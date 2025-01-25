pub mod movie_player;
#[cfg(feature = "mp4")]
pub mod mp4;
pub mod gv;
pub mod plugin;
#[cfg(feature = "lottie")]
pub mod lottie;

pub mod prelude {
    pub use crate::plugin::MoviePlayerPlugin;
    pub use crate::movie_player::MoviePlayer;
}
