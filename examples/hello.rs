use bevy::prelude::*;
use bevy_movie_player::prelude::*;

fn main() {
    App::new()
        .add_plugins(MoviePlayerPlugin)
        .run();
}