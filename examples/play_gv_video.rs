use std::{fs::File, io::{BufReader, Cursor}, time::Duration};

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension}}};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}};
use bevy_movie_player::{gv::{self, load_gv, load_gv_on_memory, GVMovie, GVMovieOnMemory, GVMoviePlayer}, movie_player::{CompressedImageDataProvider, ImageCreator, ImageDataProvider, LoopMode}, prelude::*};

fn main() {
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
        // .add_systems(Update, setup.run_if(setup_needed)) // WORKAROUND
        .add_systems(Update, update.run_if(is_asset_ready))
        .add_systems(Update, update_fps)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource)]
pub struct MovieAssets {
  #[asset(path = "test.gv")]
  pub test: Handle<GVMovie>,

  // // if you want to load on memory:
  // #[asset(path = "test.gv")]
  // pub test: Handle<GVMovieOnMemory>,
}

#[derive(Resource)]
struct MovieRes {
    last_update_time: Option<Duration>,
}

#[derive(Resource)]
struct ImageHandle {
    handle: Option<Handle<Image>>,
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
    mut image_handle_res: ResMut<ImageHandle>,
    mut _movie_res: ResMut<MovieRes>,
    mut movie_assets: ResMut<MovieAssets>,
    mut assets: ResMut<Assets<GVMovie>>,
    // mut assets: ResMut<Assets<GVMovieOnMemory>>,
    // mut asset_server: Res<AssetServer>,
    // time: Res<Time>,
) {

    let gv_movie = assets.get_mut(&movie_assets.test).unwrap();
    let movie_player = &mut gv_movie.player;

    movie_player.set_loop_mode(LoopMode::Loop);
    movie_player.play();

    commands.spawn(Camera2d::default());

    let image_handle = movie_player.register_image_handle(&mut images);
    image_handle_res.handle = Some(image_handle);

    // background plane
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.25, 0.25, 0.75),
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
            custom_size: Some(Vec2::new(640.0, 360.0)),
            image: image_handle_res.handle.clone().unwrap(),
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
    mut movie_assets: ResMut<MovieAssets>,
    mut assets: ResMut<Assets<GVMovie>>,
    // mut assets: ResMut<Assets<GVMovieOnMemory>>,
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

    let gv_movie = assets.get_mut(&movie_assets.test).unwrap();
    let movie_player = &mut gv_movie.player;

    // mut asset_server: Res<AssetServer>,
    // time: Res<Time>,

    movie_player.update(time.elapsed());

    // get image from handle
    let handle = image_handle.handle.clone().unwrap();
    let image = images.get_mut(&handle).unwrap();

    // println!("Update image data with time: {}", time.elapsed_seconds());

    // movie_player.set_image_data(image);
    movie_player.set_compressed_image_data(image); // faster
    
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