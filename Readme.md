
# Animated LDTK IntGrid Tiles with Bevy

This repository contains an example project that integrates animated IntGrid tiles in a LDTK (Level Designer Toolkit) project using the Bevy game engine. The example showcases how to set up a basic Bevy application with LDTK, adding animations to IntGrid tiles.

## References

- [Rust](https://www.rust-lang.org/tools/install)
- [Bevy](https://bevyengine.org/)
- [LDTK (Level Designer Toolkit)](https://ldtk.io/)

## Getting Started

Clone this repository and navigate to the project directory:

```sh
git clone https://github.com/your-repo/animated-ldtk-tiles.git
cd animated-ldtk-tiles
```

## Running the Project

To run the project, use the following command:

```sh
cargo run
```

## Project Structure

- `main.rs`: The main entry point for the application. It sets up the Bevy app, adds necessary plugins, and defines the system for setting up the scene.
- `assets/minimal.ldtk`: A sample LDTK project file used for loading the level.
- `assets/Sprite.png`: A sample spritemap.

## Code Overview

### Main Function

The `main` function initializes the Bevy application and adds the required plugins, systems, and resources.

```rust
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
```

### Setup System

The `setup` function spawns the camera and loads the LDTK world.

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.125;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("minimal.ldtk"),
        ..Default::default()
    });
}
```

### Animated LDTK Tile

The `AnimatedLDTKTile` struct defines the properties for an animated tile.

```rust
#[derive(Debug, Bundle, LdtkIntCell, Reflect)]
struct AnimatedLDTKTile {
    animation: AnimatedTile,
}

impl Default for AnimatedLDTKTile {
    fn default() -> Self {
        Self {
            animation: AnimatedTile {
                start: 0,
                end: 1,
                speed: 0.95,
            },
        }
    }
}
```

### AnimatedTile Struct

The `AnimatedTile` struct holds animation parameters such as the start frame, end frame, and speed.

```rust
#[derive(Debug, Reflect)]
struct AnimatedTile {
    start: usize,
    end: usize,
    speed: f32,
}
```
