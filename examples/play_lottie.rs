use std::{fs::File, io::{BufReader, Cursor}, time::Duration};
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension}}};

use bevy_asset_loader::asset_collection::AssetCollection;
#[cfg(feature = "lottie")]
use bevy_movie_player::{lottie::LottieMovie, movie_player::{CompressedImageDataProvider, ImageDataProvider, LoopMode}, prelude::*};
#[cfg(feature = "lottie")]
use bevy_movie_player::lottie::LottieMoviePlayer;
#[cfg(feature = "lottie")]
use bevy_movie_player::lottie::load_lottie;

#[cfg(feature = "lottie")]
fn main() {
    use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MoviePlayerPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<MovieAssets>()
        )
        .insert_resource(ImageHandle {
            handle: None,
        })
        .insert_resource(MovieRes {
            last_update_time: None,
        })
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, update.run_if(is_asset_ready))
        .add_systems(Update, update_fps)
        .run();
}

#[cfg(feature = "lottie")]
#[derive(AssetCollection, Resource)]
pub struct MovieAssets {
  #[asset(path = "test.json")]
  pub test: Handle<LottieMovie>,
}

#[cfg(feature = "lottie")]
#[derive(Resource)]
struct MovieRes {
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

#[cfg(feature = "lottie")]
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut image_handle: ResMut<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    mut movie_assets: ResMut<MovieAssets>,
    mut assets: ResMut<Assets<LottieMovie>>,
    // mut asset_server: Res<AssetServer>,
    // time: Res<Time>,
) {
    use bevy_movie_player::movie_player::ImageCreator;


    let lottie_movie = assets.get_mut(&movie_assets.test).unwrap();
    let movie_player = &mut lottie_movie.player;

    movie_player.set_loop_mode(LoopMode::Loop);
    movie_player.play();

    commands.spawn(Camera2dBundle::default());

    let handle = movie_player.register_image_handle(&mut images);

    image_handle.handle = Some(handle);

    // background plane
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        ..default()
    });
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(360.0, 360.0)),
            ..default()
        },
        texture: image_handle.handle.clone().unwrap(),
        ..default()
    });

    // fps text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
        ]),
        FpsText,
    ));
}

#[cfg(feature = "lottie")]
fn update(
    mut images: ResMut<Assets<Image>>,
    image_handle: Res<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    mut movie_assets: ResMut<MovieAssets>,
    mut assets: ResMut<Assets<LottieMovie>>,
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

    let lottie_movie = assets.get_mut(&movie_assets.test).unwrap();
    let movie_player = &mut lottie_movie.player;
    movie_player.update(time.elapsed());

    // get image from handle
    let handle = image_handle.handle.clone().unwrap();
    let image = images.get_mut(handle).unwrap();

    // println!("Update image data with time: {}", time.elapsed_seconds());

    movie_player.set_image_data(image);
    
    movie_res.last_update_time = Some(time.elapsed());
}

#[cfg(feature = "lottie")]
fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

#[cfg(not(feature = "lottie"))]
fn main() {
    println!("This example requires `--features lottie`");
}