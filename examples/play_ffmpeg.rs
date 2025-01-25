use std::{fs::File, io::{BufReader, Cursor}, time::Duration};
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension}}};

use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_movie_player::{ffmpeg::{load_movie, load_movie_from_url, FFmpegMovie, FFmpegMoviePlayer}, image_data_provider::{ImageCreator, ImageDataProvider}, movie_player::{LoopMode, PlayingState}, prelude::*};

fn main() {
    use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

    let url = "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4";
    let movie_player = load_movie_from_url(url);

    // // or:
    // let path = "path/to/file.mp4";
    // let movie_player = load_movie(path);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MoviePlayerPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .init_state::<AssetLoadingState>()
        // .add_loading_state(
        //     LoadingState::new(AssetLoadingState::Loading)
        //         .continue_to_state(AssetLoadingState::Loaded)
        //         .load_collection::<MovieAssets>()
        // )
        .insert_resource(ImageHandle {
            handle: None,
        })
        .insert_resource(MovieRes {
            last_update_time: None,
            player: movie_player,
        })
        // .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Startup, setup)
        .add_systems(Update, update.run_if(is_asset_ready))
        .add_systems(Update, update_fps)
        .add_systems(Update, key_handler)
        .run();
}

#[derive(Resource)]
pub struct MovieRes {
  pub player: FFmpegMoviePlayer,
  last_update_time: Option<Duration>,
}


#[derive(Resource)]
struct ImageHandle {
    handle: Option<Handle<Image>>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Component)]
struct FpsText;

fn is_asset_ready (
    image_handle_res: Res<ImageHandle>,
) -> bool
{
    image_handle_res.handle.is_some() 
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut image_handle: ResMut<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    // mut assets: ResMut<Assets<FFmpegMovie>>,
    // mut asset_server: Res<AssetServer>,
    // time: Res<Time>,
) {
    let movie_player = &mut movie_res.player;

    println!("movie width: {}", movie_player.get_resolution().0);
    println!("movie height: {}", movie_player.get_resolution().1);
    println!("movie duration: {:?}", movie_player.get_duration());

    movie_player.set_loop_mode(LoopMode::Loop);
    movie_player.play();

    commands.spawn(Camera2d::default());

    let handle = movie_player.register_image_handle(&mut images);

    image_handle.handle = Some(handle);

    // background plane
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
    ));

    commands.spawn((
        Sprite {
            image: image_handle.handle.clone().unwrap(),
            custom_size: Some(Vec2::new(540.0, 360.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.1),
            ..default()
        },
    ));

    // fps text
    commands.spawn((
        Text::new("FPS: "),
        TextFont { font_size: 30.0, ..default() },
    )).with_children(|parent| {
        parent.spawn(
        (
            TextSpan::new("0"),
            TextFont { font_size: 30.0, ..default() },
            FpsText,
        ));
    });
}

fn update(
    mut images: ResMut<Assets<Image>>,
    image_handle: Res<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    // mut movie_assets: ResMut<MovieAssets>,
    // mut assets: ResMut<Assets<FFmpegMovie>>,
    time: Res<Time>,
) {
    // skip update to be fps 30 (msec 33)
    if movie_res.last_update_time.is_some() {
        let last_update_time = movie_res.last_update_time.unwrap();
        let time_since_startup = time.elapsed();
        if time_since_startup - last_update_time < Duration::from_millis(33) {
            return;
        }
    }

    // let lottie_movie = assets.get_mut(&movie_assets.test).unwrap();
    // let movie_player = &mut lottie_movie.player;
    let movie_player = &mut movie_res.player;
    movie_player.update(time.elapsed());

    // get image from handle
    let handle = image_handle.handle.clone().unwrap();
    let image = images.get_mut(&handle).unwrap();

    // println!("Update image data with time: {}", time.elapsed_seconds());

    movie_player.set_image_data(image);
    
    movie_res.last_update_time = Some(time.elapsed());
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.0 = format!("{value:.2}");
            }
        }
    }
}

fn key_handler(
    // mut assets: ResMut<Assets<LottieMovie>>,
    // mut movie_assets: ResMut<MovieAssets>,
    mut movie_res: ResMut<MovieRes>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // time: Res<Time>,
) {
    let movie_player = &mut movie_res.player;
    if keyboard_input.just_pressed(KeyCode::Space) {
        // movie_player.pause(&time);

        // toggle play/pause
        match movie_player.get_state() {
            PlayingState::Playing => movie_player.pause(),
            PlayingState::Paused => movie_player.play(),
            PlayingState::Stopped => movie_player.play(),
        }
    }
    if keyboard_input.just_pressed(KeyCode::Enter) {
        // movie_player.stop(&time);
        
        // toggle stop/play
        match movie_player.get_state() {
            PlayingState::Playing => movie_player.stop(),
            PlayingState::Paused => movie_player.stop(),
            PlayingState::Stopped => movie_player.play(),
        }
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        let pos = movie_player.get_position();
        movie_player.seek(pos + Duration::from_secs_f32(1.0));
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        let pos = movie_player.get_position();
        let to_time =
            if pos.as_secs_f32() > 1.0 {
                pos - Duration::from_secs_f32(1.0)
            } else {
                Duration::from_secs_f32(0.0)
            };
        movie_player.seek(to_time);
    }
}