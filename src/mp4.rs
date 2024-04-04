use bevy::prelude::*;
use mp4::parse::Mp4File;

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
}

pub fn load_mp4(path: &str) -> impl MoviePlayer {
    let file = std::fs::File::open(path).unwrap();
    let mp4 = mp4::parse::parse(file.try_clone().unwrap()).unwrap();
    let reader = BufReader::new(file);

    Mp4MoviePlayer {
        mp4,
        reader: reader,
    }
}

#[allow(unused_variables)] // FIXME: just for now
impl<Reader: Read + Seek> MoviePlayer for Mp4MoviePlayer<Reader> {
    fn play(&mut self, looped: bool, bevy_time: &Time) {
        todo!()
    }

    fn pause(&mut self, bevy_time: &Time) {
        todo!()
    }

    fn stop(&mut self, bevy_time: &Time) {
        todo!()
    }

    fn seek(&mut self, to_time: Duration, bevy_time: &Time) {
        todo!()
    }

    fn update(&mut self, bevy_time: &Time) {
        todo!()
    }

    fn set_image_data(&mut self, image: &mut Image, bevy_time: &Time) {
        todo!()
    }

    fn get_image_data(&mut self, bevy_time: &Time) -> Vec<u8> {
        todo!()
    }

    fn get_state(&self) -> PlayingState {
        todo!()
    }

    fn get_duration(&self) -> Duration {
        todo!()
    }

    fn get_position(&self, bev_time: &Time) -> Duration {
        todo!()
    }

    fn set_volume(&mut self, volume: f32) {
        todo!()
    }

    fn get_volume(&self) -> f32 {
        todo!()
    }

    fn get_size(&self) -> (u32, u32) {
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
