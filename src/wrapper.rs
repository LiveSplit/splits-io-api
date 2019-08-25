use crate::{Category, ChatMessage, Entry, Game, Race, Run, Runner};

#[derive(serde::Deserialize)]
pub struct ContainsRun {
    pub run: Run,
}

#[derive(serde::Deserialize)]
pub struct ContainsRunner {
    pub runner: Runner,
}

#[derive(serde::Deserialize)]
pub struct ContainsGame {
    pub game: Game,
}

#[derive(serde::Deserialize)]
pub struct ContainsCategory {
    pub category: Category,
}

#[derive(serde::Deserialize)]
pub struct ContainsRace {
    pub race: Race,
}

#[derive(serde::Deserialize)]
pub struct ContainsEntry {
    pub entry: Entry,
}

#[derive(serde::Deserialize)]
pub struct ContainsChatMessage {
    pub chat_message: ChatMessage,
}

#[derive(serde::Deserialize)]
pub struct ContainsRuns {
    pub runs: Vec<Run>,
}

#[derive(serde::Deserialize)]
pub struct ContainsPBs {
    pub pbs: Vec<Run>,
}

#[derive(serde::Deserialize)]
pub struct ContainsRunners {
    pub runners: Vec<Runner>,
}

#[derive(serde::Deserialize)]
pub struct ContainsGames {
    pub games: Vec<Game>,
}

#[derive(serde::Deserialize)]
pub struct ContainsCategories {
    pub categories: Vec<Category>,
}

#[derive(serde::Deserialize)]
pub struct ContainsRaces {
    pub races: Vec<Race>,
}

#[derive(serde::Deserialize)]
pub struct ContainsEntries {
    pub entries: Vec<Entry>,
}

#[derive(serde::Deserialize)]
pub struct ContainsChatMessages {
    pub chat_messages: Vec<ChatMessage>,
}
