pub mod movie_player;
pub mod image_data_provider;
pub mod blankable_image_data_provider;
pub mod plugin;

#[cfg(feature = "ffmpeg")]
pub mod ffmpeg;
#[cfg(feature = "gv")]
pub mod gv;
#[cfg(feature = "mp4")]
pub mod mp4;
#[cfg(feature = "lottie")]
pub mod lottie;

pub mod prelude {
    pub use crate::plugin::MoviePlayerPlugin;
    pub use crate::movie_player::MoviePlayer;
    pub use crate::movie_player::SeekOutOfBoundsError;
    pub use crate::image_data_provider::{ImageDataProvider, CompressedImageDataProvider, ImageCreator};
}
