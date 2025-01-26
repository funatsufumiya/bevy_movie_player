use bevy::asset::io::Reader;
use bevy::asset::Asset;
use bevy::asset::AssetLoader;
use bevy::asset::LoadContext;
use bevy::reflect::TypePath;
use bevy::utils::ConditionalSendFuture;
use derivative::Derivative;
use mp4::parse::Mp4File;
use openh264::decoder::Decoder;
use openh264::nal_units;

use mp4::track::H264VideoTrack;
use mp4::track::VideoTrack;
use mp4::track::VideoCodec;
use mp4::nalu::Nalu;

use std::fs::{ self, OpenOptions, };
use std::io::BufReader;
use std::io::Cursor;
use std::io::{ Read, Write, Seek, SeekFrom };
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use crate::blankable_image_data_provider::BlankMode;
use crate::movie_player::MoviePlayerStateController;

#[derive(Derivative, Asset, TypePath)]
#[derivative(Debug)]
pub struct Mp4MoviePlayer {
    #[derivative(Debug="ignore")]
    mp4file: Mp4File,
    #[derivative(Debug="ignore")]
    decoder: Decoder,
    #[derivative(Debug="ignore")]
    state_controller: MoviePlayerStateController,
    blank_mode: BlankMode,
}

unsafe impl Send for Mp4MoviePlayer {}
unsafe impl Sync for Mp4MoviePlayer {}

#[derive(Asset, TypePath, Derivative)]
#[derivative(Debug)]
pub struct Mp4Movie {
    #[derivative(Debug="ignore")]
    pub player: Mp4MoviePlayer,
}

#[derive(Asset, TypePath, Derivative)]
#[derivative(Debug)]
pub struct Mp4MovieOnMemory {
    #[derivative(Debug="ignore")]
    pub player: Mp4MoviePlayer,
}

impl From<Mp4MoviePlayer> for Mp4Movie  {
    fn from(player: Mp4MoviePlayer) -> Self {
        Self { player }
    }
}

impl From<Mp4MoviePlayer> for Mp4MovieOnMemory {
    fn from(player: Mp4MoviePlayer) -> Self {
        Self { player }
    }
}

#[cfg(not(feature = "ffmpeg"))] /// ffmpeg extensions conflicts with mp4
#[derive(Default)]
pub struct Mp4MovieLoader;

#[cfg(not(feature = "ffmpeg"))] /// ffmpeg extensions conflicts with mp4
#[derive(Default)]
pub struct Mp4MovieOnMemoryLoader;

#[cfg(not(feature = "ffmpeg"))] /// ffmpeg extensions conflicts with mp4
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
        let p = Path::new(asset_dir).join(load_context.path());
        let asset_path = p.to_str().unwrap();
        // TODO: error handling
        let player = load_mp4(asset_path);
        Ok(player.into())
      })
    }
  
    fn extensions(&self) -> &[&str] {
      &["mp4"]
    }
}

#[cfg(not(feature = "ffmpeg"))] /// ffmpeg extensions conflicts with mp4
impl AssetLoader for Mp4MovieOnMemoryLoader {
    type Asset = Mp4MovieOnMemory;
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
        // TODO: error handling
        let player = load_mp4_from_reader(Cursor::new(bytes));
        Ok(player.into())
      })
    }
  
    fn extensions(&self) -> &[&str] {
      &["mp4"]
    }
}

/// Load a Mp4 video from a file (disk stream)
pub fn load_mp4(path: &str) -> Mp4MoviePlayer {
    let mut file = std::fs::File::open(path).unwrap();
    let mp4file = mp4::parse::parse(&mut file).unwrap();
    
    Mp4MoviePlayer {
        mp4file,
        decoder: Decoder::new().unwrap(),
        blank_mode: BlankMode::default(),
        state_controller: MoviePlayerStateController::default(),
    }
}

/// Load a Mp4 video from a reader
pub fn load_mp4_from_reader<R>(mut reader: R) -> Mp4MoviePlayer
    where R: Read
{
    let mp4file = mp4::parse::parse(&mut reader).unwrap();
    
    Mp4MoviePlayer {
        mp4file,
        decoder: Decoder::new().unwrap(),
        blank_mode: BlankMode::default(),
        state_controller: MoviePlayerStateController::default(),
    }
}