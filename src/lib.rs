pub mod movie;
pub mod loader;
pub mod mp4_loader;
pub mod mp4_player;
pub mod plugin;

pub mod prelude {
    pub use crate::plugin::MoviePlayerPlugin;
    pub use crate::movie::Player;
}