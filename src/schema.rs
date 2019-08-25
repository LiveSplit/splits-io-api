use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct Category {
    pub created_at: Box<str>,
    pub id: Box<str>,
    pub name: Box<str>,
    pub updated_at: Box<str>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatMessage {
    pub body: Box<str>,
    pub created_at: Box<str>,
    pub from_entrant: bool,
    pub updated_at: Box<str>,
    pub user: Runner,
}

#[derive(Debug, serde::Deserialize)]
pub struct Entry {
    /// The time this entry was created.
    pub created_at: Box<str>,
    /// The user who created this entry. If the entry is a ghost, this can differ from the runner.
    pub creator: Runner,
    /// The time at which the runner finished, if at all.
    #[serde(default)]
    pub finished_at: Option<Box<str>>,
    /// The time at which the runner forfeited, if at all.
    #[serde(default)]
    pub forfeited_at: Option<Box<str>>,
    /// Whether the entry is a real user (false) or a ghost of user's past run (true).
    pub ghost: bool,
    /// The unchanging unique ID of this entry.
    pub id: Uuid,
    /// The time at which the runner readied, if at all.
    #[serde(default)]
    pub readied_at: Option<Box<str>>,
    /// The Run linked to this Entry, if any. A linked Run will let the Race show realtime splits
    /// and extra stats on the Race page.
    pub run: Option<Run>,
    /// The user participating in the race. If the entry is a ghost, this can differ from the
    /// creator.
    pub runner: Runner,
    /// The time this entry was most recently changed.
    pub updated_at: Box<str>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Game {
    pub categories: Option<Vec<Category>>,
    pub created_at: Box<str>,
    pub id: Box<str>,
    pub name: Box<str>,
    #[serde(default)]
    pub shortname: Option<Box<str>>,
    pub updated_at: Box<str>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RunItemHistories {
    pub attempt_number: u32,
    pub gametime_duration_ms: f64,
    pub realtime_duration_ms: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Run {
    #[serde(default)]
    pub attempts: Option<u32>,
    pub category: Category,
    pub created_at: Box<str>,
    pub default_timing: Box<str>,
    pub game: Game,
    #[serde(default)]
    pub gametime_duration_ms: Option<f64>,
    #[serde(default)]
    pub gametime_sum_of_best_ms: Option<f64>,
    pub histories: Option<Vec<RunItemHistories>>,
    #[serde(default)]
    pub id: Option<Box<str>>,
    #[serde(default)]
    pub image_url: Option<Box<str>>,
    pub program: Box<str>,
    #[serde(default)]
    pub realtime_duration_ms: Option<f64>,
    #[serde(default)]
    pub realtime_sum_of_best_ms: Option<f64>,
    pub runners: Vec<Runner>,
    pub segments: Vec<Segment>,
    #[serde(default)]
    pub srdc_id: Option<Box<str>>,
    pub updated_at: Box<str>,
    #[serde(default)]
    pub video_url: Option<Box<str>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Runner {
    #[serde(default)]
    pub avatar: Option<Box<str>>,
    pub created_at: Box<str>,
    pub display_name: Option<Box<str>>,
    pub id: Box<str>,
    pub name: Box<str>,
    pub twitch_id: Option<Box<str>>,
    pub twitch_name: Option<Box<str>>,
    pub updated_at: Box<str>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SegmentItemHistories {
    pub attempt_number: u32,
    pub gametime_duration_ms: f64,
    pub realtime_duration_ms: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Segment {
    #[serde(default)]
    pub gametime_duration_ms: Option<f64>,
    #[serde(default)]
    pub gametime_end_ms: Option<f64>,
    pub gametime_gold: bool,
    pub gametime_reduced: bool,
    #[serde(default)]
    pub gametime_shortest_duration_ms: Option<f64>,
    pub gametime_skipped: bool,
    #[serde(default)]
    pub gametime_start_ms: Option<f64>,
    pub histories: Option<Vec<SegmentItemHistories>>,
    pub id: Uuid,
    pub name: Box<str>,
    pub realtime_duration_ms: f64,
    pub realtime_end_ms: f64,
    pub realtime_gold: bool,
    pub realtime_reduced: bool,
    pub realtime_shortest_duration_ms: f64,
    pub realtime_skipped: bool,
    pub realtime_start_ms: f64,
    pub segment_number: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Race {
    pub id: Uuid,
    pub path: Box<str>,
    pub game: Option<Game>,
    pub category: Option<Category>,
    pub visibility: Visibility,
    pub join_token: Option<Box<str>>,
    pub notes: Option<Box<str>>,
    pub owner: Runner,
    pub entries: Vec<Entry>,
    pub chat_messages: Vec<ChatMessage>,
    pub attachments: Vec<Attachment>,
    pub started_at: Option<Box<str>>,
    pub created_at: Box<str>,
    pub updated_at: Box<str>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub created_at: Box<str>,
    pub filename: Box<str>,
    pub url: Box<str>,
}

#[derive(Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    InviteOnly,
    Secret,
}
