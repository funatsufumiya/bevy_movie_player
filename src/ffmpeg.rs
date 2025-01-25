use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::ConditionalSendFuture;
use derivative::Derivative;
use ndarray::s;
use ndarray::stack;
use ndarray::Array;
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::OwnedRepr;
use video_rs::Decoder;
use video_rs::Url;

use crate::blankable_image_data_provider::BGRAImageFrameProvider;
use crate::blankable_image_data_provider::BlankMode;
use crate::blankable_image_data_provider::Blankable;
use crate::movie_player::ImageData;
use crate::movie_player::MoviePlayerStateController;
// use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::time::Duration;
use std::fmt;

use ndarray::Axis;

#[derive(Derivative, Asset, TypePath)]
#[derivative(Debug)]
pub struct FFmpegMoviePlayer {
    #[derivative(Debug="ignore")]
    pub decoder: Decoder,
    #[derivative(Debug="ignore")]
    state_controller: MoviePlayerStateController,
    #[derivative(Debug="ignore")]
    blank_mode: BlankMode,
}

#[derive(Asset, TypePath, Derivative)]
#[derivative(Debug)]
pub struct FFmpegMovie {
    #[derivative(Debug="ignore")]
    pub player: FFmpegMoviePlayer,
}

/// Load movie from file path.
pub fn load_movie(path: &str) -> FFmpegMoviePlayer {
    let path_ = std::path::Path::new(path);
    let decoder = Decoder::new(path_).expect("failed to create decoder");

    FFmpegMoviePlayer {
        decoder,
        state_controller: MoviePlayerStateController::default(),
        blank_mode: BlankMode::default(),
    }
}

/// Load movie from url.
pub fn load_movie_from_url(url: &str) -> FFmpegMoviePlayer {
    let source = url
            .parse::<Url>()
            .unwrap();
    let decoder = Decoder::new(source).expect("failed to create decoder");

    FFmpegMoviePlayer {
        decoder,
        state_controller: MoviePlayerStateController::default(),
        blank_mode: BlankMode::default(),
    }
}

#[derive(Default)]
pub struct FFmpegMovieLoader;

impl AssetLoader for FFmpegMovieLoader {
    type Asset = FFmpegMovie;
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
        let p = Path::new(asset_dir).join(load_context.path());
        let asset_path = p.to_str().unwrap();
        // TODO: error handling
        let player = load_movie(asset_path);
        Ok(FFmpegMovie {
          player,
        })
      })
    }
  
    fn extensions(&self) -> &[&str] {
      &["mp4", "mov", "avi", "mkv", "ogv", "m4v"]
    }
}

impl MoviePlayer for FFmpegMoviePlayer {
    fn get_state_controller(&self) -> &crate::movie_player::MoviePlayerStateController {
        &self.state_controller
    }
    
    fn get_state_controller_mut(&mut self) -> &mut crate::movie_player::MoviePlayerStateController {
        &mut self.state_controller
    }
    
    fn get_duration(&self) -> Duration {
        let time = self.decoder.duration().ok().unwrap();
        Duration::from_secs_f64(time.as_secs_f64())
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
        let size = self.decoder.size();
        size
    }
}

impl Blankable for FFmpegMoviePlayer {
    fn set_blank_mode(&mut self, blank_mode: BlankMode) {
        self.blank_mode = blank_mode;
    }
    
    fn get_blank_mode(&self) -> BlankMode {
        return self.blank_mode;
    }
}

fn opt_rgb_to_bgra_u8(opt_rgb_ndarray: Option<(video_rs::Time, ArrayBase<OwnedRepr<u8>, Dim<[usize; 3]>>)>) -> Option<Vec<u8>> {
    if let Some(rgb) = opt_rgb_ndarray {

        // for faster, first just convert rgb into vec
        // let rgb_ndarray = rgb.1.view();
        // let size = rgb_ndarray.shape()[0] * rgb_ndarray.shape()[1];
        // let rgb_raw = rgb_ndarray.to_shape((rgb_ndarray.shape()[0] * rgb_ndarray.shape()[1] * rgb_ndarray.shape()[2])).unwrap();
        // let mut rgb_vec = rgb_raw.to_vec();
        // // finally add size (fill 255)
        // rgb_vec.append(&mut vec![255; size]);
        // return Some(rgb_vec);

        // slower

        let rgb_ndarray = rgb.1.view();
        let size = rgb_ndarray.shape()[0] * rgb_ndarray.shape()[1];
        let w: usize = rgb_ndarray.shape()[0];
        let h: usize = rgb_ndarray.shape()[1];
        let r_array = rgb_ndarray.index_axis(Axis(2), 0);
        let g_array = rgb_ndarray.index_axis(Axis(2), 1);
        let b_array = rgb_ndarray.index_axis(Axis(2), 2);
        // let mut a_array = Array::<u8,_>::zeros((rgb_ndarray.shape()[0], rgb_ndarray.shape()[1]));
        // a_array.fill(255);
        let a_array = Array::<u8,_>::ones((rgb_ndarray.shape()[0], rgb_ndarray.shape()[1])) * 255;
        // let a_array = Array::<u8,_>::from(vec![255; size]).to_shape((w, h)).unwrap();
        // let rgba_ndarray = stack!(Axis(0), r_array, g_array, b_array, a_array);
        // let rgba_ndarray_raw = rgba_ndarray.to_shape((rgba_ndarray.shape()[0] * rgba_ndarray.shape()[1] * rgba_ndarray.shape()[2])).unwrap();
        // let rgba_ndarray_as_u8 = rgba_ndarray_raw.to_vec();
        // return Some(rgba_ndarray_as_u8.to_vec());

        let bgra_ndarray = stack!(Axis(2), b_array, g_array, r_array, a_array);
        // let bgra_ndarray2 = bgra_ndarray.to_shape((bgra_ndarray.shape()[2], bgra_ndarray.shape()[0], bgra_ndarray.shape()[1])).unwrap();
        let bgra_ndarray_raw = bgra_ndarray.to_shape((bgra_ndarray.shape()[0] * bgra_ndarray.shape()[1] * bgra_ndarray.shape()[2])).unwrap();
        let bgra_ndarray_as_u8 = bgra_ndarray_raw.to_vec();
        return Some(bgra_ndarray_as_u8.to_vec());
    } else {
        None
    }
}


impl BGRAImageFrameProvider for FFmpegMoviePlayer {
    fn get_first_frame_bgra(&mut self) -> Option<Vec<u8>> {
        // seek to first frame
        self.decoder.seek_to_start().unwrap();
        let frame_or_not = self.decoder.decode().ok();
        opt_rgb_to_bgra_u8(frame_or_not)
    }

    fn get_last_frame_bgra(&mut self) -> Option<Vec<u8>> {
        // seek to last frame
        let frame_count: usize = (self.get_duration().as_secs_f64() * (self.decoder.frame_rate() as f64)).round() as usize;
        self.decoder.seek_to_frame((frame_count as i64) - 1).unwrap();
        let frame_or_not: Option<(video_rs::Time, ArrayBase<OwnedRepr<u8>, Dim<[usize; 3]>>)> = self.decoder.decode().ok();
        opt_rgb_to_bgra_u8(frame_or_not)
    }

    fn get_paused_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let position = self.get_position();
        // let frame_number = (position.as_secs_f64() * (self.decoder.frame_rate() as f64)).round() as i64;
        // self.decoder.seek_to_frame(frame_number).unwrap();
        let msec = position.as_millis() as i64;
        self.decoder.seek(msec).unwrap();
        let frame_or_not = self.decoder.decode().ok();
        opt_rgb_to_bgra_u8(frame_or_not)
    }

    fn get_playing_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let position = self.get_position();
        // let frame_number = (position.as_secs_f64() * (self.decoder.frame_rate() as f64)).round() as i64;
        // println!("frame_number: {}", frame_number);
        // self.decoder.seek_to_frame(frame_number).unwrap();
        let msec = position.as_millis() as i64;
        // println!("msec: {}", msec);
        self.decoder.seek(msec).unwrap();
        let frame_or_not = self.decoder.decode().ok();
        opt_rgb_to_bgra_u8(frame_or_not)
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
