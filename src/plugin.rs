use bevy::prelude::*;

use crate::gv::{GVMovie, GVMovieLoader, GVMovieOnMemory, GVMovieOnMemoryLoader};
#[cfg(feature = "lottie")]
use crate::lottie::{LottieMovie, LottieMovieLoader};
pub struct MoviePlayerPlugin;

// fn hello_world() {
//     println!("Hello, world!");
// }

impl Plugin for MoviePlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<GVMovie>()
            .init_asset::<GVMovieOnMemory>()
            .init_asset_loader::<GVMovieLoader>()
            .init_asset_loader::<GVMovieOnMemoryLoader>()
            ;
        #[cfg(feature = "lottie")]
        app
            .init_asset::<LottieMovie>()
            .init_asset_loader::<LottieMovieLoader>()
            ;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .add_plugins(MoviePlayerPlugin)
            .update(); // run once
    }
}