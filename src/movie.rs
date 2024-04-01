
use std::time::Duration;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LoadMode {
    OnMemory,
    DiskStream,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MovieState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub struct Audio {
    pub volume: f32,
}

#[derive(Debug)]
pub struct Movie {
    pub path: String,
    pub duration: Duration,
    pub load_mode: LoadMode,
    pub audio: Option<Audio>,
}

pub trait Player {
    fn play(&mut self);
    fn pause(&mut self);
    fn stop(&mut self);
    fn seek(&self, time: Duration);
    fn get_state(&self) -> MovieState;
    fn set_state(&mut self, state: MovieState);
    fn get_duration(&self) -> Duration;
    fn get_position(&self) -> Duration;
    fn set_volume(&mut self, volume: f32);
}

impl Player for Movie {
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
        unimplemented!()
    }
    
    fn set_state(&mut self, state: MovieState) {
        unimplemented!()
    }

    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn get_position(&self) -> Duration {
        Duration::from_secs(0) // FIXME
    }

    fn set_volume(&mut self, volume: f32) {
        println!("Setting volume to: {}", volume);
        if let Some(audio) = &mut self.audio {
            audio.volume = volume;
        }
    }

}