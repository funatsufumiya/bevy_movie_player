use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::ConditionalSendFuture;
use derivative::Derivative;
use mp4::parse::Mp4File;

use crate::blankable_image_data_provider::BlankMode;
use crate::blankable_image_data_provider::Blankable;
use crate::movie_player::ImageData;
use crate::movie_player::MoviePlayerStateController;
// use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
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
    pub mp4: Arc<Mutex<Mp4File>>,
    pub reader: Reader,
    state_controller: MoviePlayerStateController,
    blank_mode: BlankMode,
}

#[derive(Asset, TypePath, Derivative)]
#[derivative(Debug)]
pub struct Mp4Movie {
    #[derivative(Debug="ignore")]
    pub player: Mp4MoviePlayer<BufReader<File>>,
}

unsafe impl Send for Mp4Movie {}
unsafe impl Sync for Mp4Movie {}

#[derive(Default)]
pub struct Mp4MovieLoader;

impl AssetLoader for Mp4MovieLoader {
    type Asset = Mp4Movie;
    type Settings = ();
    type Error = std::io::Error;
  
    fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
      Box::pin(async move {
        let asset_dir = "assets"; // FIXME: just WORKAROUND

        // let mut bytes = Vec::new();
        // reader.read_to_end(&mut bytes).await?;
        // let player = load_mp4_from_data(bytes.as_slice());

        let p = Path::new(asset_dir).join(load_context.path());
        let asset_path = p.to_str().unwrap();
        let player = load_mp4(asset_path);

        // println!("Loaded Mp4 {:?}", player);
        // println!("duration: {:?}", player.get_duration());
        Ok(Mp4Movie {
          player,
        })
      })
    }
  
    fn extensions(&self) -> &[&str] {
      &["mp4"]
    }
}

/// Load a mp4 movie from a file
pub fn load_mp4(path: &str) -> impl MoviePlayer {
    let file = std::fs::File::open(path).unwrap();
    let mp4 = mp4::parse::parse(file.try_clone().unwrap()).unwrap();
    let reader = BufReader::new(file);

    Mp4MoviePlayer {
        mp4,
        reader: reader,
        state_controller: MoviePlayerStateController::default(),
        blank_mode: BlankMode::default(),
    }
}

/// Load a mp4 movie from a data
pub fn load_mp4_from_data(data: &[u8]) -> impl MoviePlayer {
    let mp4 = mp4::parse::parse(data).unwrap();
    let reader = BufReader::new(data);
    let player = Mp4MoviePlayer {
        mp4,
        reader: reader,
        state_controller: MoviePlayerStateController::default(),
        blank_mode: BlankMode::default(),
    };
    player
}

impl<Reader: Read + Seek> MoviePlayer for Mp4MoviePlayer<Reader> {
    fn get_state_controller(&self) -> &crate::movie_player::MoviePlayerStateController {
        &self.state_controller
    }
    
    fn get_state_controller_mut(&mut self) -> &mut crate::movie_player::MoviePlayerStateController {
        &mut self.state_controller
    }
    
    fn get_duration(&self) -> Duration {
        // let lottie = self.lottie.lock().unwrap();
        // Duration::from_secs_f64(lottie.duration())
        let mp4 = self.mp4.lock().unwrap();
        Duration::from_secs_f64(mp4.duration())
    }

    fn set_volume(&mut self, _volume: f32) {
        warn!("Volume is not supported");
        // do nothing
    }

    fn get_volume(&self) -> f32 {
        warn!("Volume is not supported");
        0.0
    }
    
    fn get_resolution(&self) -> (u32, u32) {
        let lottie = self.lottie.lock().unwrap();
        let size = lottie.size();
        (size.width as u32, size.height as u32)
    }
}

impl<Reader: Read + Seek> Blankable for Mp4MoviePlayer<Reader> {
    fn set_blank_mode(&mut self, blank_mode: BlankMode) {
        self.blank_mode = blank_mode;
    }
    
    fn get_blank_mode(&self) -> BlankMode {
        return self.blank_mode;
    }
}

pub fn get_bgra_data_as_bytes(data: &[Bgra]) -> &[u8] {
    unsafe {
        slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * mem::size_of::<Bgra>()
        )
    }
}

fn get_bgra_from_data(data: &[Bgra]) -> Vec<u8> {
    let bytes = get_bgra_data_as_bytes(data);
    bytes.to_vec()
}

fn read_frame<'a>(lottie: &'a mut rlottie::Animation, surface: &'a mut LottieSurface, frame_num: usize) -> Option<&'a [Bgra]> {
    lottie.render(frame_num, surface);
    Some(surface.data())
}

fn read_frame_at<'a>(lottie: &'a mut rlottie::Animation, surface: &'a mut LottieSurface, t: Duration) -> Option<&'a [Bgra]> {
    let frame_num = lottie.frame_at_pos(t.as_secs_f32());
    read_frame(lottie, surface, frame_num)
}

fn get_resolution_of_lottie(lottie: &rlottie::Animation) -> (u32, u32) {
    let size = lottie.size();
    (size.width as u32, size.height as u32)
}


fn opt_bgra_to_u8(frame_or_not: Option<&[Bgra]>) -> Option<Vec<u8>> {
    if let Some(frame) = frame_or_not {
        let bgra_data = get_bgra_from_data(frame);
        Some(bgra_data)
    } else {
        None
    }
}

impl BGRAImageFrameProvider for LottieMoviePlayer {
    fn get_first_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let mut lottie= self.lottie.lock().unwrap();
        let frame_or_not = read_frame(&mut lottie, &mut self.lottie_surface, 0);
        opt_bgra_to_u8(frame_or_not)
    }

    fn get_last_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let mut lottie= self.lottie.lock().unwrap();
        let total_frame = lottie.totalframe();
        let frame_or_not = read_frame(&mut lottie, &mut self.lottie_surface, total_frame - 1);
        opt_bgra_to_u8(frame_or_not)
    }

    fn get_paused_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let mut lottie= self.lottie.lock().unwrap();
        let position = self.get_position();
        let frame_or_not =
            read_frame_at(&mut lottie, &mut self.lottie_surface, position);
        opt_bgra_to_u8(frame_or_not)
    }

    fn get_playing_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let mut lottie= self.lottie.lock().unwrap();
        let position = self.get_position();
        let frame_or_not =
            read_frame_at(&mut lottie, &mut self.lottie_surface, position);
        opt_bgra_to_u8(frame_or_not)
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