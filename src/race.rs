//! The race module handles retrieving Races. A Race is a competition between multiple Runners.
//!
//! [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#race)

use crate::{
    get_json, get_response,
    platform::{recv_bytes, Body},
    wrapper::{
        ContainsChatMessage, ContainsChatMessages, ContainsEntries, ContainsEntry, ContainsRace,
        ContainsRaces,
    },
    Attachment, ChatMessage, Client, Download, Entry, Error, Race, Visibility,
};
use http::{header::CONTENT_TYPE, Request};
use snafu::ResultExt;
use std::ops::Deref;
use url::Url;
use uuid::Uuid;

impl Race {
    /// Gets all the currently active Races on Splits.io.
    pub async fn get_active(client: &Client) -> Result<Vec<Race>, Error> {
        self::get_active(client).await
    }

    /// Gets a Race by its ID.
    pub async fn get(client: &Client, id: Uuid) -> Result<Race, Error> {
        self::get(client, id).await
    }

    /// Creates a new Race.
    pub async fn create(client: &Client, settings: Settings<'_>) -> Result<Race, Error> {
        self::create(client, settings).await
    }

    /// Updates the Race.
    pub async fn update(
        &self,
        client: &Client,
        settings: UpdateSettings<'_>,
    ) -> Result<Race, Error> {
        self::update(client, self.id, settings).await
    }

    /// Gets all of the entries for the Race.
    pub async fn entries(&self, client: &Client) -> Result<Vec<Entry>, Error> {
        self::get_entries(client, self.id).await
    }

    /// Gets the entry in the Race that is associated with the current user.
    pub async fn my_entry(&self, client: &Client) -> Result<Entry, Error> {
        self::get_entry(client, self.id).await
    }

    /// Joins the Race for the given entry.
    pub async fn join(
        &self,
        client: &Client,
        join_as: JoinAs<'_>,
        join_token: Option<&str>,
    ) -> Result<Entry, Error> {
        self::join(client, self.id, join_as, join_token).await
    }

    /// Leaves the Race for the given entry.
    pub async fn leave(&self, client: &Client, entry_id: Uuid) -> Result<(), Error> {
        self::leave(client, self.id, entry_id).await
    }

    /// Declares the given entry as ready for the Race.
    pub async fn ready_up(&self, client: &Client, entry_id: Uuid) -> Result<Entry, Error> {
        self::ready_up(client, self.id, entry_id).await
    }

    /// Undoes a ready for the given entry in th Race.
    pub async fn unready(&self, client: &Client, entry_id: Uuid) -> Result<Entry, Error> {
        self::unready(client, self.id, entry_id).await
    }

    /// Finishes the Race for the given entry.
    pub async fn finish(&self, client: &Client, entry_id: Uuid) -> Result<Entry, Error> {
        self::finish(client, self.id, entry_id).await
    }

    /// Undoes a finish for the given entry in the Race.
    pub async fn undo_finish(&self, client: &Client, entry_id: Uuid) -> Result<Entry, Error> {
        self::undo_finish(client, self.id, entry_id).await
    }

    /// Forfeits the Race for the given entry.
    pub async fn forfeit(&self, client: &Client, entry_id: Uuid) -> Result<Entry, Error> {
        self::forfeit(client, self.id, entry_id).await
    }

    /// Undoes a forfeit for the given entry in the Race.
    pub async fn undo_forfeit(&self, client: &Client, entry_id: Uuid) -> Result<Entry, Error> {
        self::undo_forfeit(client, self.id, entry_id).await
    }

    /// Gets all of the chat messages for the Race.
    pub async fn chat_messages(&self, client: &Client) -> Result<Vec<ChatMessage>, Error> {
        self::get_chat(client, self.id).await
    }

    /// Sends a message in the chat for the Race.
    pub async fn send_chat_message(
        &self,
        client: &Client,
        message: &str,
    ) -> Result<ChatMessage, Error> {
        self::send_chat_message(client, self.id, message).await
    }
}

impl Attachment {
    /// Downloads the attachment.
    pub async fn download(&self, client: &Client) -> Result<impl Deref<Target = [u8]>, Error> {
        let response = get_response(
            client,
            Request::get(&*self.url).body(Body::empty()).unwrap(),
        )
        .await?;

        recv_bytes(response.into_body()).await.context(Download)
    }
}

/// Gets all the currently active Races on Splits.io.
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

// FIXME: get_all

/// Gets a Race by its ID.
pub async fn get(client: &Client, id: Uuid) -> Result<Race, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
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

/// The settings for a Race.
#[derive(Default, serde::Serialize)]
pub struct Settings<'a> {
    /// The ID of the Game that is being raced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<&'a str>,
    /// The ID of the Category that is being raced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<&'a str>,
    /// Any notes that are associated with the Race.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<&'a str>,
    /// The visibility of the Race.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

/// The type of update to perform on the given property.
pub enum Update<T> {
    /// Keep the previous value of the property.
    Keep,
    /// Clear the value of the property.
    Clear,
    /// Change the value of the property.
    Set(T),
}

impl<T> Update<T> {
    fn is_keep(&self) -> bool {
        matches!(self, Update::Keep)
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

/// The new properties to use for a Race when performing an update.
#[derive(Default, serde::Serialize)]
pub struct UpdateSettings<'a> {
    /// The update to perform for the ID of the Game that is being raced.
    #[serde(skip_serializing_if = "Update::is_keep")]
    pub game_id: Update<&'a str>,
    /// The update to perform for the ID of the Category that is being raced.
    #[serde(skip_serializing_if = "Update::is_keep")]
    pub category_id: Update<&'a str>,
    /// The update to perform for any notes that are associated with the Race.
    #[serde(skip_serializing_if = "Update::is_keep")]
    pub notes: Update<&'a str>,
    /// The update to perform for the visibility of the Race.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

/// Creates a new Race.
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

/// Updates a Race.
pub async fn update(
    client: &Client,
    id: Uuid,
    settings: UpdateSettings<'_>,
) -> Result<Race, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
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

/// Gets all of the entries for a Race.
pub async fn get_entries(client: &Client, id: Uuid) -> Result<Vec<Entry>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
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

/// Gets the entry in a Race that is associated with the current user.
pub async fn get_entry(client: &Client, id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
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

/// The type of racer to join the Race as.
pub enum JoinAs<'a> {
    /// Join the Race as a regular user.
    Myself,
    /// Join the Race as a ghost of a past Run.
    Ghost(&'a str),
}

#[derive(serde::Serialize)]
struct JoinToken<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    join_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<JoinEntry<'a>>,
}

#[derive(serde::Serialize)]
struct JoinEntry<'a> {
    run_id: &'a str,
}

/// Joins the Race for the given entry.
pub async fn join(
    client: &Client,
    race_id: Uuid,
    join_as: JoinAs<'_>,
    join_token: Option<&str>,
) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::post(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&JoinToken {
                    join_token,
                    entry: match join_as {
                        JoinAs::Myself => None,
                        JoinAs::Ghost(run_id) => Some(JoinEntry { run_id }),
                    },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(entry)
}

/// Leaves the Race for the given entry.
pub async fn leave(client: &Client, race_id: Uuid, entry_id: Uuid) -> Result<(), Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
        entry_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ]);

    get_response(
        client,
        Request::delete(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(())
}

#[derive(serde::Serialize)]
struct UpdateEntry<T> {
    entry: T,
}

#[derive(serde::Serialize)]
struct ReadyState {
    readied_at: Option<&'static str>,
}

#[derive(serde::Serialize)]
struct FinishState {
    finished_at: Option<&'static str>,
}

#[derive(serde::Serialize)]
struct ForfeitState {
    forfeited_at: Option<&'static str>,
}

/// Declares the given entry as ready for a Race.
pub async fn ready_up(client: &Client, race_id: Uuid, entry_id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
        entry_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::patch(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&UpdateEntry {
                    entry: ReadyState {
                        readied_at: Some("now"),
                    },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(entry)
}

/// Undoes a ready for the given entry in a Race.
pub async fn unready(client: &Client, race_id: Uuid, entry_id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
        entry_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::patch(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&UpdateEntry {
                    entry: ReadyState { readied_at: None },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(entry)
}

/// Finishes the Race for the given entry.
pub async fn finish(client: &Client, race_id: Uuid, entry_id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
        entry_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::patch(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&UpdateEntry {
                    entry: FinishState {
                        finished_at: Some("now"),
                    },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(entry)
}

/// Undoes a finish for the given entry in a Race.
pub async fn undo_finish(client: &Client, race_id: Uuid, entry_id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
        entry_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::patch(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&UpdateEntry {
                    entry: FinishState { finished_at: None },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(entry)
}

/// Forfeits the Race for the given entry.
pub async fn forfeit(client: &Client, race_id: Uuid, entry_id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
        entry_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::patch(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&UpdateEntry {
                    entry: ForfeitState {
                        forfeited_at: Some("now"),
                    },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(entry)
}

/// Undoes a forfeit for the given entry in a Race.
pub async fn undo_forfeit(client: &Client, race_id: Uuid, entry_id: Uuid) -> Result<Entry, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
    url.path_segments_mut().unwrap().extend(&[
        race_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
        "entries",
        entry_id
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ]);

    let ContainsEntry { entry } = get_json(
        client,
        Request::patch(url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&UpdateEntry {
                    entry: ForfeitState { forfeited_at: None },
                })
                .unwrap(),
            ))
            .unwrap(),
    )
    .await?;

    Ok(entry)
}

/// Gets all of the chat messages for a Race.
pub async fn get_chat(client: &Client, id: Uuid) -> Result<Vec<ChatMessage>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
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

/// Sends a message in the chat for a Race.
pub async fn send_chat_message(
    client: &Client,
    id: Uuid,
    message: &str,
) -> Result<ChatMessage, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/races").unwrap();
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
