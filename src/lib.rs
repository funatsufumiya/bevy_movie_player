pub mod movie_player;
// pub mod mp4;
pub mod gv;
pub mod plugin;

pub mod prelude {
    pub use crate::plugin::MoviePlayerPlugin;
    pub use crate::movie_player::MoviePlayer;
}