use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set((ImagePlugin::default_nearest())))
        .add_plugins(LdtkPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_int_cell_for_layer::<AnimatedLDTKTile>("IntGrid", 1)
        .register_type::<AnimatedTile>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.125;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("minimal.ldtk"),
        ..Default::default()
    });
}

#[derive(Debug, Bundle, LdtkIntCell, Reflect)]
struct AnimatedLDTKTile{
    animation: AnimatedTile,
}

impl Default for AnimatedLDTKTile {
    fn default() -> Self {
        Self {
            animation: AnimatedTile{
                start: 0,
                end: 1,
                speed: 0.95,
            }
        }
    }
}