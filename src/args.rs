use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PlayerArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Play a track.
    Play(PlayTrack),
    /* /// List information about one or all tracks.
    List(ListTrack), */
    /// Add a track to the queue.
    Queue(QueueTrack),
}

#[derive(Debug, Args)]
pub struct PlayTrack {
    /// Path to file of track
    pub path: String,
}

#[derive(Debug, Args)]
pub struct QueueTrack {
    /// Path to file of track
    pub path: String,
}

/* #[derive(Debug, Args)]
pub struct ListTrack {
    /// Path of file to list out
    pub path: String,
}
 */
