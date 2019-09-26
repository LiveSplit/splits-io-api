use crate::{get_response_unchecked, ChatMessage, Client, Error, Race};
use futures::prelude::*;
use hyper::{
    header::{CONNECTION, UPGRADE},
    Body, Request, StatusCode,
};
use snafu::ResultExt;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_tungstenite::{
    tungstenite::{protocol::Role, Message},
    WebSocketStream,
};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum RawEvent {
    General(GeneralEvent),
    Targeted {
        identifier: Box<str>,
        message: TargetedEvent,
    },
    UnknownTargeted {
        identifier: Box<str>,
        message: UnknownTargetedEvent,
    },
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum GeneralEvent {
    Ping,
    Welcome,
    ConfirmSubscription { identifier: Box<str> },
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum TargetedEvent {
    RaceCreated { data: Data<Race> },
    GlobalState { data: Data<Vec<Race>> },
    RaceUpdated { data: Data<Race> },
    NewMessage { data: Data<ChatMessage> },
    NewAttachment { data: Data<Race> },
    RaceState { data: Data<Race> },
    RaceNotFound { message: Box<str> },
    RaceInvalidJoinToken { message: Box<str> },
    RaceStartScheduled { data: Data<Option<Race>> },
    RaceEnded { data: Data<Option<Race>> },
    RaceEntriesUpdated { data: Data<Option<Race>> },
    FatalError { message: Box<str> },
    ConnectionError { message: Box<str> },
}

#[derive(Debug, serde::Deserialize)]
struct Data<T> {
    message: Box<str>,
    #[serde(rename = "race")]
    #[serde(alias = "races")]
    #[serde(alias = "chat_message")]
    data: T,
}

#[derive(Debug, serde::Deserialize)]
struct UnknownTargetedEvent {
    #[serde(rename = "type")]
    ty: serde::de::IgnoredAny,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "channel")]
pub enum Identifier {
    #[serde(rename = "Api::V4::GlobalRaceChannel")]
    GlobalRaceChannel,
    #[serde(rename = "Api::V4::RaceChannel")]
    RaceChannel { race_id: Uuid, join_token: Box<str> },
}

fn inner_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, EventError> {
    serde_json::from_str(s).context(Parse)
}

impl RawEvent {
    fn convert(self) -> Result<Option<Event>, EventError> {
        use TargetedEvent::*;
        Ok(Some(match self {
            RawEvent::General(GeneralEvent::ConfirmSubscription { identifier }) => Event {
                identifier: inner_str(&identifier)?,
                kind: EventKind::ConfirmSubscription,
            },
            RawEvent::General(_) => return Ok(None),
            RawEvent::Targeted {
                message,
                identifier,
            } => Event {
                identifier: inner_str(&identifier)?,
                kind: match message {
                    RaceCreated { data } => EventKind::RaceCreated(data.data),
                    GlobalState { data } => EventKind::GlobalState(data.data),
                    RaceUpdated { data } => EventKind::RaceUpdated(data.data),
                    NewMessage { data } => EventKind::NewMessage(data.data),
                    NewAttachment { data } => EventKind::NewAttachment(data.data),
                    RaceState { data } => EventKind::RaceState(data.data),
                    RaceNotFound { message } => return Err(EventError::RaceNotFound { message }),
                    RaceInvalidJoinToken { message } => {
                        return Err(EventError::RaceInvalidJoinToken { message })
                    }
                    RaceStartScheduled { data } => EventKind::RaceStartScheduled(data.data),
                    RaceEnded { data } => EventKind::RaceEnded(data.data),
                    RaceEntriesUpdated { data } => EventKind::RaceEntriesUpdated(data.data),
                    FatalError { message } => return Err(EventError::FatalError { message }),
                    ConnectionError { message } => {
                        return Err(EventError::ConnectionError { message })
                    }
                },
            },
            RawEvent::UnknownTargeted { .. } => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub struct Event {
    pub identifier: Identifier,
    pub kind: EventKind,
}

#[derive(Debug)]
pub enum EventKind {
    RaceCreated(Race),
    GlobalState(Vec<Race>),
    RaceUpdated(Race),
    NewMessage(ChatMessage),
    NewAttachment(Race),
    RaceState(Race),
    RaceStartScheduled(Option<Race>),
    RaceEnded(Option<Race>),
    RaceEntriesUpdated(Option<Race>),
    ConfirmSubscription,
}

#[derive(Debug, snafu::Snafu)]
pub enum EventError {
    /// No Race found for the given ID.
    #[snafu(display("{}", message))]
    RaceNotFound { message: Box<str> },
    /// The join token is not valid for the Race.
    #[snafu(display("{}", message))]
    RaceInvalidJoinToken { message: Box<str> },
    /// An error occurred when processing the message.
    #[snafu(display("{}", message))]
    FatalError { message: Box<str> },
    /// There was a connection error. The OAuth Token might've been invalid.
    #[snafu(display("{}", message))]
    ConnectionError { message: Box<str> },
    /// The message could not be parsed.
    Parse { source: serde_json::Error },
    /// Failed to receive the message.
    Receive {
        source: tokio_tungstenite::tungstenite::Error,
    },
}

#[derive(Debug, snafu::Snafu)]
pub enum SendError {
    /// Failed to send the message.
    Send {
        source: tokio_tungstenite::tungstenite::Error,
    },
}

pub struct Events {
    ws: WebSocketStream<hyper::upgrade::Upgraded>,
}

impl Stream for Events {
    type Item = Result<Event, EventError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            return Poll::Ready(
                match futures::ready!(Pin::new(&mut self.ws).poll_next(cx)) {
                    Some(Ok(Message::Text(text))) => match serde_json::from_str(&text)
                        .context(Parse)
                        .and_then(RawEvent::convert)
                        .transpose()
                    {
                        // Relevant message
                        Some(val) => Some(val),
                        // Irrelevant ActionCable message
                        None => continue,
                    },
                    // Underlying error
                    Some(Err(source)) => Some(Err(EventError::Receive { source })),
                    // Irrelevant WebSocket message
                    Some(_) => continue,
                    // Stream ended
                    None => None,
                },
            );
        }
    }
}

pub enum Command {
    Subscribe { identifier: Identifier },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "command")]
enum RawCommand {
    Subscribe { identifier: String },
}

impl Events {
    pub async fn send(&mut self, command: Command) -> Result<(), SendError> {
        let command = match command {
            Command::Subscribe { identifier } => RawCommand::Subscribe {
                identifier: serde_json::to_string(&identifier).unwrap(),
            },
        };
        let command = serde_json::to_string(&command).unwrap();
        self.ws.send(Message::Text(command)).await.context(Send)
    }

    pub async fn subscribe(&mut self, identifier: Identifier) -> Result<(), SendError> {
        self.send(Command::Subscribe { identifier }).await
    }

    pub async fn close(&mut self) -> Result<(), SendError> {
        self.ws.close(None).await.context(Send)
    }
}

pub async fn events(client: &Client) -> Result<Events, Error> {
    let response = get_response_unchecked(
        client,
        Request::get("https://splits.io/api/cable")
            .header(CONNECTION, "Upgrade")
            .header(UPGRADE, "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", "TotallyRandomBytesHere==")
            .body(Body::empty())
            .unwrap(),
    )
    .await?;

    let status = response.status();
    if status != StatusCode::SWITCHING_PROTOCOLS {
        return Err(Error::Status { status });
    }

    let upgraded = response
        .into_body()
        .on_upgrade()
        .await
        .context(super::Download)?;

    let ws = WebSocketStream::from_raw_socket(upgraded, Role::Client, None).await;

    Ok(Events { ws })
}
