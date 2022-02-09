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
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>
){
    // Watches for changes in files
    asset_server.watch_for_changes().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window to top left
    let mut window = windows.get_primary_mut().unwrap();
    // window.set_position(IVec2::new(1600,200));

    // spawn a sprite
    let bottom = -window.height() / 2.;
    commands.spawn_bundle(SpriteBundle {
        // material:materials.add(Color::rgb(1.,0.7,0.7,).into()),
        sprite: Sprite {
            custom_size: Some(Vec2::new(200., 100.)),
            color: Color::rgb(1.,0.7,0.7,),
            ..Default::default()
        },
        transform: Transform {
            // x,y,z
            translation: Vec3::new(0., bottom + 75. / 4. + 20., 10.),
            scale: Vec3::new(0.5,0.5,1.),
            ..Default::default()
        },
        texture: asset_server.load(PLAYER_SPRITE),
        ..Default::default()
    });
}