use bevy::prelude::*;
use crate::{GreetTimer, WinSize};

const BALLE_SPRITE: &str = "boule.png"; // c'est la balle

// ================================= components
pub struct BallePlugin;

impl Plugin for BallePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "game_setup_balle",
                SystemStage::single(spawn_balle)
            )
            .add_system(greet_balle)
            .add_system(balle_down);
    }
}
// ================================= components

#[derive(Component)]
struct Balle;

#[derive(Component)]
struct Name(String);


fn spawn_balle(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(BALLE_SPRITE),
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                // scale: Vec3::new(0.031, 0.031, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Balle)
        .insert(Name("crabe balle".into()));
}

// ============================================= systems

fn greet_balle(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&mut Transform, &Name, With<Balle>)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (position, name, _) in query.iter() {
            println!("boing : {}", name.0);
            println!("position : {:?}", position)
        }
    }
}

fn balle_down(
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(&mut Transform, With<Balle>)>
) {
    for (mut position, _) in query.iter_mut() {

        if position.translation.y > -(win_size.height / 2.) {
            // la balle tombe
            position.translation.y -= time.delta_seconds() * 100.;
        }

    }
}