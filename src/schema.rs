use uuid::Uuid;

/// A Category is a ruleset for a Game (Any%, 100%, MST, etc.) and an optional container for Runs.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#category)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Category {
    /// The time and date at which this category was created on splits.io. This field conforms to
    /// ISO 8601.
    pub created_at: Box<str>,
    /// The unique ID of the category.
    pub id: Box<str>,
    /// The name of the category.
    pub name: Box<str>,
    /// The time and date at which this category was most recently modified on splits.io. This field
    /// conforms to ISO 8601.
    pub updated_at: Box<str>,
}

/// A Chat Message is a shortform message sent by a user to a Race
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#chat-message)
#[derive(Debug, serde_derive::Deserialize)]
pub struct ChatMessage {
    /// The contents of the message.
    pub body: Box<str>,
    /// The time and date at which this message was created on splits.io. This field conforms to ISO
    /// 8601.
    pub created_at: Box<str>,
    /// Boolean indicating whether the sender was in the race when the message was sent.
    pub from_entrant: bool,
    /// The time and date at which this message was most recently modified on splits.io. This field
    /// conforms to ISO 8601.
    pub updated_at: Box<str>,
    /// The Runner that sent the message.
    pub user: Runner,
}

/// An Entry represents a Runner's participation in a Race or a ghost of a past Run.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#entry)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Entry {
    /// The time and date at which this Entry was created on splits.io. This field conforms to ISO
    /// 8601.
    pub created_at: Box<str>,
    /// The user that created this Entry; can be different from runner if the Entry is a ghost.
    pub creator: Runner,
    /// The time and date at which the runner finished this Race, if at all. This field conforms to
    /// ISO 8601.
    #[serde(default)]
    pub finished_at: Option<Box<str>>,
    /// The time and date at which the runner forfeited from this Race, if at all. This field
    /// conforms to ISO 8601.
    #[serde(default)]
    pub forfeited_at: Option<Box<str>>,
    /// Whether the Entry represents a past recording of a run (true) or a real user that has
    /// entered into the race explicitly (false).
    pub ghost: bool,
    /// The unchanging unique ID of this Entry.
    pub id: Uuid,
    /// The time and date at which the runner readied up in the Race, if at all. This field conforms
    /// to ISO 8601.
    #[serde(default)]
    pub readied_at: Option<Box<str>>,
    /// The Run linked to the current Entry. It has more detailed info about this runner's run, such
    /// as splits and history.
    pub run: Option<Run>,
    /// The user participating in the race. If the entry is a ghost, this can differ from the
    /// creator.
    pub runner: Runner,
    /// The time and date at which this Entry was most recently modified on splits.io. This field
    /// conforms to ISO 8601.
    pub updated_at: Box<str>,
}

/// A Game is a collection of information about a game, and a container for Categories.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#game)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Game {
    /// The known speedrun categories for this game.
    pub categories: Option<Vec<Category>>,
    /// The time and date at which this game was created on splits.io. This field conforms to ISO
    /// 8601.
    pub created_at: Box<str>,
    /// The unique ID of the game.
    pub id: Box<str>,
    /// The full title of the game, like "Super Mario Sunshine".
    pub name: Box<str>,
    /// A shortened title of the game, like "sms", if it is known. Where possible, this name tries
    /// to match with those on SpeedRunsLive and/or Speedrun.com.
    #[serde(default)]
    pub shortname: Option<Box<str>>,
    /// The time and date at which this game was most recently modified on splits.io. This field
    /// conforms to ISO 8601.
    pub updated_at: Box<str>,
}

/// Information about a past attempt associated with a Run.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#history)
#[derive(Debug, serde_derive::Deserialize)]
pub struct RunItemHistories {
    /// The corresponding attempt number this attempt was.
    pub attempt_number: u32,
    /// The gametime duration this attempt took in milliseconds.
    pub gametime_duration_ms: f64,
    /// The realtime duration this attempt took in milliseconds.
    pub realtime_duration_ms: f64,
}

/// A Run maps 1:1 to an uploaded splits file.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#run)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Run {
    /// The number of run attempts recorded by the timer that generated the run's source file, if
    /// supported by the source timer.
    #[serde(default)]
    pub attempts: Option<u32>,
    /// The category which was run, if it was supplied by the runner and determined from the source
    /// file.
    pub category: Option<Category>,
    /// The time and date at which this run's source file was uploaded to splits.io. This field
    /// conforms to ISO 8601.
    pub created_at: Box<str>,
    /// The timing method used for the run. Will be either real or game.
    pub default_timing: Box<str>,
    /// The game which was run, if it was supplied by the runner and determined from the source
    /// file.
    pub game: Option<Game>,
    /// Gametime duration in milliseconds of the run.
    #[serde(default)]
    pub gametime_duration_ms: Option<f64>,
    /// Gametime sum of best in milliseconds of the run.
    #[serde(default)]
    pub gametime_sum_of_best_ms: Option<f64>,
    /// Ordered history objects of all previous attempts. The first item is the first run recorded
    /// by the runner's timer into the source file. The last item is the most recent one. This field
    /// is only nonempty if the source timer records history.
    pub histories: Option<Vec<RunItemHistories>>,
    /// Unique ID for identifying the run on splits.io. This can be used to construct a user-facing
    /// URL or an API-facing one.
    #[serde(default)]
    pub id: Option<Box<str>>,
    /// A screenshot of the timer after a finished run, if it was supplied by the runner. This is
    /// typically supplied automatically by timers which support auto-uploading runs to splits.io.
    #[serde(default)]
    pub image_url: Option<Box<str>>,
    /// The name of the timer with which the run was recorded. This is typically an all lowercase,
    /// no-spaces version of the program name.
    pub program: Box<str>,
    /// Realtime duration in milliseconds of the run.
    #[serde(default)]
    pub realtime_duration_ms: Option<f64>,
    /// Realtime sum of best in milliseconds of the run.
    #[serde(default)]
    pub realtime_sum_of_best_ms: Option<f64>,
    /// The runner(s) who performed the run, if they claim credit.
    pub runners: Vec<Runner>,
    /// The associated segments for the run.
    pub segments: Vec<Segment>,
    /// Unique ID for identifying the run on Speedrun.com. This is typically supplied by the runner
    /// manually.
    #[serde(default)]
    pub srdc_id: Option<Box<str>>,
    /// The time and date at which this run was most recently modified on splits.io (modify events
    /// include disowning, adding a video or Speedrun.com association, and changing the run's
    /// game/category). This field conforms to ISO 8601.
    pub updated_at: Box<str>,
    /// A URL for a Twitch, YouTube, or Hitbox video which can be used as proof of the run. This is
    /// supplied by the runner.
    #[serde(default)]
    pub video_url: Option<Box<str>>,
}

/// A Runner is a user who has at least one run tied to their account.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#runner)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Runner {
    /// The avatar of the user.
    #[serde(default)]
    pub avatar: Option<Box<str>>,
    /// The time and date at which this user first authenticated with splits.io. This field conforms
    /// to ISO 8601.
    pub created_at: Box<str>,
    /// The display name of the user.
    pub display_name: Option<Box<str>>,
    /// The unique ID of the user.
    pub id: Box<str>,
    /// The splits.io username of the user.
    pub name: Box<str>,
    /// The Twitch ID of the user.
    pub twitch_id: Option<Box<str>>,
    /// The Twitch name of the user.
    pub twitch_name: Option<Box<str>>,
    /// The time and date at which this user was most recently modified on splits.io. This field
    /// conforms to ISO 8601.
    pub updated_at: Box<str>,
}

/// Information about a past attempt of a segment.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#history)
#[derive(Debug, serde_derive::Deserialize)]
pub struct SegmentItemHistories {
    /// The corresponding attempt number this attempt was.
    pub attempt_number: u32,
    /// The gametime duration this attempt took in milliseconds.
    pub gametime_duration_ms: f64,
    /// The realtime duration this attempt took in milliseconds.
    pub realtime_duration_ms: f64,
}

/// A Segment maps to a single piece of a run, also called a split.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#segment)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Segment {
    /// Gametime duration in milliseconds of the segment.
    #[serde(default)]
    pub gametime_duration_ms: Option<f64>,
    /// The total elapsed time of the run at the moment when this segment was finished in gametime
    /// (such that the run's duration is equal to the final split's finish time). Provided in
    /// milliseconds.
    #[serde(default)]
    pub gametime_end_ms: Option<f64>,
    /// Whether or not this split was the shortest duration the runner has ever gotten on this
    /// segment in gametime. This field is shorthand for duration == best.
    pub gametime_gold: bool,
    /// Whether or not this segment was "reduced" in gametime; that is, had its duration affected by
    /// previous splits being skipped.
    pub gametime_reduced: bool,
    /// The shortest duration the runner has ever gotten on this segment in gametime. Provided in
    /// milliseconds.
    #[serde(default)]
    pub gametime_shortest_duration_ms: Option<f64>,
    /// Whether or not this split was skipped in gametime -- some timers let the runner skip over a
    /// split in case they forgot to hit their split button on time. Beware that a skipped split's
    /// duration is considered 0, and instead is rolled into the following split.
    pub gametime_skipped: bool,
    /// The total elapsed time of the run at the moment when this segment was started in gametime.
    /// Provided in milliseconds.
    #[serde(default)]
    pub gametime_start_ms: Option<f64>,
    /// Ordered history objects of all previous attempts of the segment. The first item is the first
    /// run recorded by the runner's timer into the source file. The last item is the most recent
    /// one. This field is only nonempty if the source timer records history.
    pub histories: Option<Vec<SegmentItemHistories>>,
    /// Internal ID of the segment.
    pub id: Uuid,
    /// Name of the segment. This value is an exact copy of timers' fields.
    pub name: Box<str>,
    /// Realtime duration in milliseconds of the segment.
    pub realtime_duration_ms: f64,
    /// The total elapsed time of the run at the moment when this segment was finished in realtime
    /// (such that the run's duration is equal to the final split's finish time). Provided in
    /// milliseconds.
    pub realtime_end_ms: f64,
    /// Whether or not this split was the shortest duration the runner has ever gotten on this
    /// segment in realtime. This field is shorthand for realtime_duration_ms ==
    /// realtime_shortest_duration_ms.
    pub realtime_gold: bool,
    /// Whether or not this segment was "reduced" in realtime; that is, had its duration affected by
    /// previous splits being skipped.
    pub realtime_reduced: bool,
    /// The shortest duration the runner has ever gotten on this segment in realtime. Provided in
    /// milliseconds.
    #[serde(default)]
    pub realtime_shortest_duration_ms: Option<f64>,
    /// Whether or not this split was skipped in realtime -- some timers let the runner skip over a
    /// split in case they forgot to hit their split button on time. Beware that a skipped split's
    /// duration is considered 0, and instead is rolled into the following split.
    pub realtime_skipped: bool,
    /// The total elapsed time of the run at the moment when this segment was started in realtime.
    /// Provided in milliseconds.
    pub realtime_start_ms: f64,
    /// The index of the segment within the run. (This value starts at 0.)
    pub segment_number: u32,
}

/// A Race is a live competition between multiple Runners who share a start time for their run.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#race)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Race {
    /// Any attachments supplied by the race creator for the benefit of other entrants (e.g. for
    /// randomizers).
    pub attachments: Vec<Attachment>,
    /// The category being raced.
    pub category: Option<Category>,
    /// Chat messages for the Race. Only present when fetching the Race individually.
    pub chat_messages: Vec<ChatMessage>,
    /// The time and date at which this Race was created on splits.io. This field conforms to ISO
    /// 8601.
    pub created_at: Box<str>,
    /// All Entries currently in the Race.
    pub entries: Vec<Entry>,
    /// The game being raced.
    pub game: Option<Game>,
    /// The unique ID of the Race.
    pub id: Uuid,
    /// The token needed to join the race if it's invite-only or secret. Only provided to the owner
    /// as a response to creation.
    pub join_token: Option<Box<str>>,
    /// Any notes associatied with the Race.
    pub notes: Option<Box<str>>,
    /// The Runner who created the Race.
    pub owner: Runner,
    /// The user-friendly URL to the Race, to be given to a user when necessary.
    pub path: Box<str>,
    /// The time and date at which this Race was started on splits.io. This field conforms to ISO
    /// 8601.
    pub started_at: Option<Box<str>>,
    /// The time and date at which this Race was most recently modified on splits.io. This field
    /// conforms to ISO 8601.
    pub updated_at: Box<str>,
    /// The permission set for the Race.
    pub visibility: Visibility,
}

/// A file that is attached to a Race.
///
/// [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#attachment)
#[derive(Debug, serde_derive::Deserialize)]
pub struct Attachment {
    /// The unique ID of the attachment.
    pub id: Uuid,
    /// The time and date at which this attachment was created on splits.io. This field conforms to ISO 8601.
    pub created_at: Box<str>,
    /// The filename of the attachment.
    pub filename: Box<str>,
    /// The URL to use in order to download the attachment.
    pub url: Box<str>,
}

/// The permission set for a Race.
#[derive(Copy, Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    /// Anyone can join the race.
    Public,
    /// An invitation is required in order to join the race.
    InviteOnly,
    /// The race can only be viewed by certain users.
    Secret,
}
