use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::BoxedFuture;
use bevy::utils::ConditionalSendFuture;
use derivative::Derivative;
use rlottie::Bgra;

use crate::blankable_image_data_provider::BGRAImageFrameProvider;
use crate::blankable_image_data_provider::BlankMode;
use crate::blankable_image_data_provider::Blankable;
use crate::blankable_image_data_provider::CompressedImageFrameProvider;
use crate::movie_player;
use crate::movie_player::MoviePlayerStateController;
use crate::movie_player::ImageData;
use crate::movie_player::LoopMode;
// use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use rlottie::Animation as LottieAnimation;
use rlottie::Surface as LottieSurface;

use core::slice;
use std::mem;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Duration;

#[derive(Derivative, Asset, TypePath)]
#[derivative(Debug)]
pub struct LottieMoviePlayer {
    pub lottie: Arc<Mutex<LottieAnimation>>,
    #[derivative(Debug="ignore")]
    pub lottie_surface: LottieSurface,
    #[derivative(Debug="ignore")]
    state_controller: MoviePlayerStateController,
    blank_mode: BlankMode,
}

#[derive(Asset, TypePath, Derivative)]
#[derivative(Debug)]
pub struct LottieMovie {
    #[derivative(Debug="ignore")]
    pub player: LottieMoviePlayer,
}

#[derive(Default)]
pub struct LottieMovieLoader;

impl AssetLoader for LottieMovieLoader {
    type Asset = LottieMovie;
    type Settings = ();
    type Error = std::io::Error;
  
    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
      Box::pin(async move {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let cache_key = "".to_string();
        let resource_path = "".to_string();
        let player = load_lottie_from_data(bytes, cache_key, resource_path);
        // println!("Loaded Lottie {:?}", player);
        // println!("duration: {:?}", player.get_duration());
        Ok(LottieMovie {
          player,
        })
      })
    }
  
    fn extensions(&self) -> &[&str] {
      #[cfg(not(feature = "disable-json-extension-for-lottie"))]
      {
          &["json", "lottie.json"]
      }

      #[cfg(feature = "disable-json-extension-for-lottie")]
      {
            &["lottie.json"]
      } 
    }
  }
  

/// Load a Lottie movie from a file
pub fn load_lottie(path: &str) -> LottieMoviePlayer {
    let lottie = LottieAnimation::from_file(path).unwrap();
    let size = lottie.size();
    let lottie_surface = LottieSurface::new(size);
    
    LottieMoviePlayer {
        lottie: Arc::new(Mutex::new(lottie)),
        lottie_surface,
        state_controller: MoviePlayerStateController::default(),
        blank_mode: BlankMode::default(),
    }
}

/// Load a Lottie movie from a data
pub fn load_lottie_from_data<D, K, P>(json_data: D, cache_key: K, resource_path: P) -> LottieMoviePlayer
	where
		D: Into<Vec<u8>>,
		K: Into<Vec<u8>>,
		P: AsRef<std::path::Path>
{
    let lottie = LottieAnimation::from_data(
        json_data,
        cache_key,
        resource_path,
    ).unwrap();

    let size = lottie.size();
    let lottie_surface = LottieSurface::new(size);
    
    LottieMoviePlayer {
        lottie: Arc::new(Mutex::new(lottie)),
        lottie_surface,
        state_controller: MoviePlayerStateController::default(),
        blank_mode: BlankMode::default(),
    }
}

impl MoviePlayer for LottieMoviePlayer {
    fn get_state_controller(&self) -> &crate::movie_player::MoviePlayerStateController {
        &self.state_controller
    }
    
    fn get_state_controller_mut(&mut self) -> &mut crate::movie_player::MoviePlayerStateController {
        &mut self.state_controller
    }
    
    fn get_duration(&self) -> Duration {
        let lottie = self.lottie.lock().unwrap();
        Duration::from_secs_f64(lottie.duration())
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

impl Blankable for LottieMoviePlayer {
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
    use super::*;

    #[test]
    fn it_works() {
        let mut movie = load_lottie("test_assets/test.json");
        let size = movie.get_resolution();
        let duration = movie.get_duration();
        assert_eq!(size, (1024, 1024));
        assert_eq!(duration, Duration::from_secs_f64(3.0));
        movie.play();
        movie.pause();
        movie.stop();
    }

    // TODO: add loop test
    // TODO: add seek test
    // TODO: add image data test
}
