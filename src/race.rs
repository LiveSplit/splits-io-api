use crate::{
    get_json,
    wrapper::{
        ContainsChatMessage, ContainsChatMessages, ContainsEntries, ContainsEntry, ContainsRace,
        ContainsRaces,
    },
    ChatMessage, Client, Entry, Error, Race, Visibility,
};
use hyper::{header::CONTENT_TYPE, Body, Request};
use url::Url;
use uuid::Uuid;

pub async fn get_active(client: &Client) -> Result<Vec<Race>, Error> {
    let ContainsRaces { races } = get_json(
        client,
        Request::get("https://splits.io/api/v4/races")
            .body(Body::empty())
            .unwrap(),
    )
    .await?;

    Ok(races)
}

// TODO: get_all

pub async fn get(client: &Client, id: Uuid) -> Result<Race, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut()
        .unwrap()
        .push(id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()));

    let ContainsRace { race } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(race)
}

#[derive(Default, serde::Serialize)]
pub struct Settings<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

pub enum Update<T> {
    Keep,
    Clear,
    Set(T),
}

impl<T> Update<T> {
    fn is_keep(&self) -> bool {
        match self {
            Update::Keep => true,
            _ => false,
        }
    }
}

impl<T> Default for Update<T> {
    fn default() -> Self {
        Update::Keep
    }
}

impl<T: serde::Serialize> serde::Serialize for Update<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Update::Set(val) => serializer.serialize_some(val),
            _ => serializer.serialize_none(),
        }
    }
}

#[derive(Default, serde::Serialize)]
pub struct UpdateSettings<'a> {
    #[serde(skip_serializing_if = "Update::is_keep")]
    pub game_id: Update<&'a str>,
    #[serde(skip_serializing_if = "Update::is_keep")]
    pub category_id: Update<&'a str>,
    #[serde(skip_serializing_if = "Update::is_keep")]
    pub notes: Update<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

pub async fn create(client: &Client, settings: Settings<'_>) -> Result<Race, Error> {
    let ContainsRace { race } = get_json(
        client,
        Request::post("https://splits.io/api/v4/races")
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&settings).unwrap()))
            .unwrap(),
    )
    .await?;

    Ok(race)
}

pub async fn update(
    client: &Client,
    id: Uuid,
    settings: UpdateSettings<'_>,
) -> Result<Race, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut()
        .unwrap()
        .push(id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()));

    let ContainsRace { race } = get_json(
        client,
        Request::patch(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&settings).unwrap()))
            .unwrap(),
    )
    .await?;

    Ok(race)
}

// TODO: Not sure if this is even a thing.
pub async fn get_entries(client: &Client, id: Uuid) -> Result<Vec<Entry>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()),
        "entries",
    ]);

    let ContainsEntries { entries } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(entries)
}

pub async fn get_entry(client: &Client, id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()),
        "entry",
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(entry)
}

// TODO: Arguments
pub async fn join(client: &Client, id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()),
        "entry",
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::put(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(entry)
}

// TODO: Update

pub async fn leave(client: &Client, id: Uuid) -> Result<(), Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()),
        "entry",
    ]);

    // TODO: Idk what this returns
    let ContainsEntry { .. } = get_json(
        client,
        Request::delete(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(())
}

pub async fn get_chat(client: &Client, id: Uuid) -> Result<Vec<ChatMessage>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()),
        "chat",
    ]);

    let ContainsChatMessages { chat_messages } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(chat_messages)
}

#[derive(serde::Serialize)]
struct SendMessage<'a> {
    chat_message: SendMessageBody<'a>,
}

#[derive(serde::Serialize)]
struct SendMessageBody<'a> {
    body: &'a str,
}

pub async fn send_chat_message(
    client: &Client,
    id: Uuid,
    message: &str,
) -> Result<ChatMessage, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races/").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        id.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()),
        "chat",
    ]);

    let ContainsChatMessage { chat_message } = get_json(
        client,
        Request::post(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&SendMessage {
                    chat_message: SendMessageBody { body: message },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(chat_message)
}
