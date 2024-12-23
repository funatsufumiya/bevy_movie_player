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

use crate::movie_player::BlankMode;
use crate::movie_player::Blankable;
use crate::movie_player::CompressedImageDataProvider;
use crate::movie_player::ImageData;
use crate::movie_player::ImageDataProvider;
use crate::movie_player::LoopMode;
// use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use rlottie::Animation as LottieAnimation;
use rlottie::Surface as LottieSurface;

use core::slice;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Derivative, Asset, TypePath)]
#[derivative(Debug)]

pub struct LottieMoviePlayer {
    pub lottie: Arc<Mutex<LottieAnimation>>,
    #[derivative(Debug="ignore")]
    pub lottie_surface: LottieSurface,
    state: PlayingState,
    bevy_elapsed_time: Duration,
    play_started_time: Option<Duration>,
    pause_started_time: Option<Duration>,
    seek_position: Duration,
    loop_mode: LoopMode,
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
        state: PlayingState::Stopped,
        bevy_elapsed_time: Duration::from_secs(0),
        play_started_time: None,
        pause_started_time: None,
        seek_position: Duration::from_secs(0),
        loop_mode: LoopMode::default(),
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
        state: PlayingState::Stopped,
        bevy_elapsed_time: Duration::from_secs(0),
        play_started_time: None,
        pause_started_time: None,
        seek_position: Duration::from_secs(0),
        loop_mode: LoopMode::default(),
        blank_mode: BlankMode::default(),
    }
}

impl MoviePlayer for LottieMoviePlayer {
    fn play(&mut self) {
        if self.state == PlayingState::Playing {
            warn!("Already playing");
            return;
        } else if self.state == PlayingState::Paused {
            let paused_duration = self.bevy_elapsed_time - self.pause_started_time.unwrap();
            self.play_started_time = Some(self.play_started_time.unwrap() + paused_duration);
            self.pause_started_time = None;
        } else if self.state == PlayingState::Stopped {
            self.play_started_time = Some(self.bevy_elapsed_time);
        }
        self.state = PlayingState::Playing;
    }

    fn pause(&mut self) {
        if self.state == PlayingState::Paused {
            warn!("Already paused");
            return;
        } else if self.state == PlayingState::Stopped {
            warn!("Not playing");
            return;
        } else if self.state == PlayingState::Playing {
            self.state = PlayingState::Paused;
            self.pause_started_time = Some(self.bevy_elapsed_time);
            self.seek_position = (self.bevy_elapsed_time - self.play_started_time.unwrap()) + self.seek_position;
            self.play_started_time = self.pause_started_time;
        }
    }

    fn stop(&mut self) {
        if self.state == PlayingState::Stopped {
            warn!("Already stopped");
        }

        self.state = PlayingState::Stopped;
        self.seek_position = Duration::from_secs(0);
        self.play_started_time = None;
        self.pause_started_time = None;
    }

    fn seek(&mut self, to_time: Duration) {
        self.seek_position = to_time;
        self.play_started_time = Some(self.bevy_elapsed_time);
    }

    fn get_state(&self) -> PlayingState {
        self.state
    }

    fn get_duration(&self) -> Duration {
        let lottie = self.lottie.lock().unwrap();
        Duration::from_secs_f64(lottie.duration())
    }

    fn update(&mut self, bevy_elapsed_time: Duration) {
        self.bevy_elapsed_time = bevy_elapsed_time;

        if self.state == PlayingState::Playing {
            let position = self.get_position();
            if position >= self.get_duration() {
                match self.loop_mode {
                    LoopMode::Stop => {
                        self.stop();
                    },
                    LoopMode::Loop => {
                        self.seek(Duration::from_secs(0));
                    },
                    LoopMode::PauseAtEnd => {
                        // self.seek(self.get_duration(), bevy_elapsed_time);

                        // FIXME: not working
                        // let last_frame_pos = self.gv.get_fps() * (self.gv.get_frame_count() as f32 - 1.0);
                        // self.seek(Duration::from_secs_f32(last_frame_pos), bevy_elapsed_time);

                        // WORKAROUND: seek to the end - 0.1ms
                        self.seek(self.get_duration() - Duration::from_secs_f32(0.0001));
                        self.pause();
                    },
                }
            }
        }
    }

    fn get_position(&self) -> Duration {
        match self.state {
            PlayingState::Stopped => Duration::from_secs(0),
            // PlayingState::Paused => self.seek_position,
            PlayingState::Paused => self.seek_position,
            PlayingState::Playing => (self.bevy_elapsed_time - self.play_started_time.unwrap()) + self.seek_position,
        }
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
    
    fn get_loop_mode(&self) -> LoopMode {
        self.loop_mode
    }
    
    fn set_loop_mode(&mut self, loop_mode: LoopMode) {
        self.loop_mode = loop_mode;
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

const fn black_frame_bgra_1x1() -> &'static [u8] {
    &[0, 0, 0, 255]
}

const fn white_frame_bgra_1x1() -> &'static [u8] {
    &[255, 255, 255, 255]
}

const fn transparent_frame_bgra_1x1() -> &'static [u8] {
    &[0, 0, 0, 0]
}

fn texture_1x1_bgra(data: &[u8]) -> ImageData {
    ImageData {
        data: data.to_vec(),
        format: TextureFormat::Bgra8UnormSrgb,
        resolution: (1, 1),
    }
}
// fn texture_bgra(data: &Vec<u8>, width: u32, height: u32) -> ImageData {
//     ImageData {
//         data: data.clone(),
//         format: TextureFormat::Bgra8UnormSrgb,
//         resolution: (width, height),
//     }
// }

fn get_blank_frame_bgra(blank_mode: BlankMode, state:PlayingState, last_or_first_frame: Option<ImageData>) -> ImageData {
    // NOTE: BGRA format, black is (0, 0, 0, 255), white is (255, 255, 255, 255)
    match blank_mode {
        BlankMode::Black => texture_1x1_bgra(black_frame_bgra_1x1()),
        BlankMode::White => texture_1x1_bgra(white_frame_bgra_1x1()),
        BlankMode::Transparent => texture_1x1_bgra(transparent_frame_bgra_1x1()),
        BlankMode::LastFrameOnPause_TransparentOnStop => {
            if state == PlayingState::Paused {
                if let Some(last_frame) = last_or_first_frame {
                    last_frame
                } else {
                    texture_1x1_bgra(transparent_frame_bgra_1x1())
                }
            } else {
                texture_1x1_bgra(transparent_frame_bgra_1x1())
            }
        },
        BlankMode::LastFrameOnPause_FirstFrameOnStop => {
            if state == PlayingState::Paused {
                if let Some(last_frame) = last_or_first_frame {
                    last_frame
                } else {
                    texture_1x1_bgra(transparent_frame_bgra_1x1())
                }
            } else if state == PlayingState::Stopped {
                if let Some(first_frame) = last_or_first_frame {
                    first_frame
                } else {
                    texture_1x1_bgra(transparent_frame_bgra_1x1())
                }
            } else {
                texture_1x1_bgra(transparent_frame_bgra_1x1())
            }
        },
        BlankMode::LastFrameOnPauseAndStop => {
            if let Some(last_or_first_frame) = last_or_first_frame {
                last_or_first_frame
            } else {
                texture_1x1_bgra(transparent_frame_bgra_1x1())
            }
        },
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

impl ImageDataProvider for LottieMoviePlayer {
    fn set_image_data(&mut self, image: &mut Image) {
        let image_data = self.get_image_data();
        image.data = image_data.data;
        image.texture_descriptor.format = image_data.format;
        image.texture_descriptor.size = Extent3d {
            width: image_data.resolution.0,
            height: image_data.resolution.1,
            depth_or_array_layers: 1,
        };
    }

    fn get_image_data(&mut self) -> ImageData {
        match self.state {
            PlayingState::Stopped => {
                // FIXME: slow? need cached for first and last frame?
                let mut lottie= self.lottie.lock().unwrap();
                let frame_or_not = if self.blank_mode == BlankMode::LastFrameOnPause_FirstFrameOnStop {
                    read_frame(&mut lottie, &mut self.lottie_surface, 0)
                } else if self.blank_mode == BlankMode::LastFrameOnPauseAndStop {
                    let total_frame = lottie.totalframe();
                    read_frame(&mut lottie, &mut self.lottie_surface, total_frame - 1)
                } else {
                    None
                };
                let frame_data = if let Some(frame) = frame_or_not {
                    Some(ImageData {
                        data: get_bgra_from_data(frame),
                        format: TextureFormat::Bgra8Unorm,
                        resolution: get_resolution_of_lottie(&lottie),
                    })
                } else {
                    None
                };
                
                get_blank_frame_bgra(self.blank_mode, self.state, frame_data)
            }
            PlayingState::Paused => {
                // FIXME: slow? need cached for first and last frame?
                let mut lottie= self.lottie.lock().unwrap();
                let last_frame = {
                    lottie.render(0, &mut self.lottie_surface);
                    Some(self.lottie_surface.data())
                };
                let last_frame_data = if let Some(frame) = last_frame {
                    Some(ImageData {
                        data: get_bgra_from_data(frame),
                        format: TextureFormat::Bgra8Unorm,
                        resolution: get_resolution_of_lottie(&lottie),
                    })
                } else {
                    None
                };
                
                get_blank_frame_bgra(self.blank_mode, self.state, last_frame_data)
            }
            PlayingState::Playing => {
                let mut lottie= self.lottie.lock().unwrap();
                let position = self.get_position();
                let frame = {
                    read_frame_at(&mut lottie, &mut self.lottie_surface, position)
                };
                let frame_data = if let Some(frame) = frame {
                    ImageData {
                        data: get_bgra_from_data(frame),
                        format: TextureFormat::Bgra8Unorm,
                        resolution: get_resolution_of_lottie(&lottie),
                    }
                } else {
                    // WORKAROUND
                    get_blank_frame_bgra(self.blank_mode, self.state, None)
                };
                frame_data
            }
        }
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
