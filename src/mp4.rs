use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use mp4::parse::Mp4File;

use crate::movie_player::ImageData;
use crate::movie_player::MoviePlayerStateController;
// use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::time::Duration;
use std::fmt;

impl<Reader: Read + Seek> fmt::Debug for Mp4MoviePlayer<Reader> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mp4MoviePlayer")
            .field("mp4", &self.mp4)
            // .field("reader",  &"Reader")
            .finish()
    }
}
pub struct Mp4MoviePlayer<Reader: Read + Seek> {
    pub mp4: Mp4File,
    pub reader: Reader,
    state_controller: MoviePlayerStateController,
}

pub fn load_mp4(path: &str) -> impl MoviePlayer {
    let file = std::fs::File::open(path).unwrap();
    let mp4 = mp4::parse::parse(file.try_clone().unwrap()).unwrap();
    let reader = BufReader::new(file);

    Mp4MoviePlayer {
        mp4,
        reader: reader,
        state_controller: MoviePlayerStateController::default(),
    }
}

impl<Reader: Read + Seek> MoviePlayer for Mp4MoviePlayer<Reader> {
    fn get_state_controller(&self) -> &crate::movie_player::MoviePlayerStateController {
        &self.state_controller
    }

    fn get_state_controller_mut(&mut self) -> &mut crate::movie_player::MoviePlayerStateController {
        &mut self.state_controller
    }

    fn get_duration(&self) -> Duration {
        todo!()
    }

    fn get_resolution(&self) -> (u32, u32) {
        todo!()
    }

    fn set_volume(&mut self, _volume: f32) {
        todo!()
    }

    fn get_volume(&self) -> f32 {
        todo!()
    }
}

// test
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let mut movie = load_mp4("assets/test.mp4");
    //     movie.play();
    //     movie.pause();
    //     movie.stop();
    // }
}
