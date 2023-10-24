use crate::{Category, ChatMessage, Entry, Game, Race, Run, Runner};

#[derive(serde_derive::Deserialize)]
pub struct ContainsRun {
    pub run: Run,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsRunner {
    pub runner: Runner,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsGame {
    pub game: Game,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsCategory {
    pub category: Category,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsRace {
    pub race: Race,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsEntry {
    pub entry: Entry,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsChatMessage {
    pub chat_message: ChatMessage,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsRuns {
    pub runs: Vec<Run>,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsPBs {
    pub pbs: Vec<Run>,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsRunners {
    pub runners: Vec<Runner>,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsGames {
    pub games: Vec<Game>,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsCategories {
    pub categories: Vec<Category>,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsRaces {
    pub races: Vec<Race>,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsEntries {
    pub entries: Vec<Entry>,
}

#[derive(serde_derive::Deserialize)]
pub struct ContainsChatMessages {
    pub chat_messages: Vec<ChatMessage>,
}
