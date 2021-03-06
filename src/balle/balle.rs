use bevy::prelude::*;
use crate::GreetTimer;

const BALLE_SPRITE: &str = "player_a_01.png"; // c'est la balle

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
                translation: Vec3::new(0., 70., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Balle)
        .insert(Name("crabe balle".into()));
}

// ============================================= systems

fn greet_balle(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Balle>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("boing : {}", name.0)
        }
    }
}

fn balle_down(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Name, With<Balle>)>
) {
    for (mut a, name, _) in query.iter_mut() {
        a.translation.x += time.delta_seconds() * 100.;
        // println!("{}", name.0);
    }
    // let mut position = query.single_mut();
    // position.translation.x += time.delta_seconds() * 100.;
}