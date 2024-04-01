use std::time::Duration;
use crate::movie::LoadMode;
use crate::movie::Audio;
use crate::movie::MovieState;
use crate::movie::Player;

#[derive(Debug)]
pub struct Mp4Movie {
    pub path: String,
    pub duration: Duration,
    pub load_mode: LoadMode,
    pub audio: Option<Audio>,
}

impl Player for Mp4Movie {
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