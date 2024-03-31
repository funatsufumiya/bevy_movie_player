use bevy::prelude::*;
use bevy_movie_player::MoviePlayerPlugin;

fn main() {
    App::new()
        .add_plugins(MoviePlayerPlugin)
        .run();
}