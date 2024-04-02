use crate::movie::LoadMode;
use crate::movie::Audio;
use crate::movie::MovieState;
use crate::movie::Player;

use std::time::Duration;
use mp4::Mp4Reader;

#[derive(Debug)]
pub struct Mp4Movie<Reader> {
    pub path: String,
    pub duration: Duration,
    pub load_mode: LoadMode,
    pub audio: Option<Audio>,
    pub reader: Mp4Reader<Reader>,
}

impl<Reader> Player for Mp4Movie<Reader> {
    fn play(&mut self) {
        println!("Playing movie: {}", self.path);
        self.set_state(MovieState::Playing);
    }

    fn pause(&mut self) {
        println!("Pausing movie: {}", self.path);
        self.set_state(MovieState::Paused);
    }

    fn stop(&mut self) {
        println!("Stopping movie: {}", self.path);
        self.set_state(MovieState::Stopped);
    }

    fn seek(&self, time: Duration) {
        println!("Seeking movie: {} to {:?}", self.path, time);
    }
    
    fn get_state(&self) -> MovieState {
        todo!()
    }
    
    fn set_state(&mut self, state: MovieState) {
        todo!()
    }
    
    fn get_duration(&self) -> Duration {
        todo!()
    }
    
    fn get_position(&self) -> Duration {
        todo!()
    }
    
    fn set_volume(&mut self, volume: f32) {
        todo!()
    }
}