pub mod movie_player;
pub mod mp4_video;
pub mod plugin;

pub mod prelude {
    pub use crate::plugin::MoviePlayerPlugin;
    pub use crate::movie_player::MoviePlayer;
}