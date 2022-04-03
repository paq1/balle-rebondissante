mod balle;

use bevy::prelude::*;
use crate::balle::balle::BallePlugin;


// ================================= resources

struct GreetTimer(Timer);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04,0.04)))
        .insert_resource(WindowDescriptor {
            title: "balle rebondissante".to_string(),
            width: 600.0,
            height: 600.0,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(GreetTimer(Timer::from_seconds(2., true)))
        .add_startup_system(setup)
        .add_plugin(BallePlugin)
        .run()
}

// ============================================= Load system

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // change la position de la fenetre de jeu
    // let mut window: &mut Window = windows.get_primary_mut().unwrap();
    let window = match windows.get_primary_mut() {
        Some(v) => v,
        _ => panic!("Pas de fenetre !!")
    };

    window.set_position(IVec2::new(0, 0));
}

