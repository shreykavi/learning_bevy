#![allow(unused)] // Silences any unused warnings

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";

//  ECS

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .insert_resource(WindowDescriptor{
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}