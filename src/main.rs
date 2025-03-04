use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use block::blockregistry::{BlockRegistry};
use block::chunk::Chunk;
use block::mesh::bake;
mod debugtext;
mod player;
mod position;
use crate::debugtext::DebugTextPlugin;
use crate::player::PlayerPlugin;
use position::*;
mod block;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, DebugTextPlugin))
        .add_systems(Startup, (set_window_title, setup))
        .add_systems(Update, translate_all_world_transforms)
        .run();
}

// Sets window title to proper name of game
fn set_window_title(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "Diřłakū".to_string();
    }
}

// summons test shit
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    
    let test_torus = meshes.add(shape::Torus::default().into());

    commands
        .spawn((PbrBundle {
            mesh: test_torus,
            material: debug_material.clone(),
            ..default()
        },))
        .insert(WorldPosition::from_xyz(0.0, 2.0, 0.0));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 9000.0,
                range: 100.,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(WorldPosition::from_xyz(8.0, 16.0, 8.0));

    // ground plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::SILVER.into()),
            ..default()
        })
        .insert(WorldPosition::from_xyz(0.0, -100.0, 0.0));

    // test chunk

    println!("making block registry");
    let block_registry = BlockRegistry::new();
    println!("making chunk");

    for x in -10..=10 {
        for z in -10..=10 {
            for y in -1..=0 {
                let chunk = Chunk::generate_chunk(&block_registry, x, y, z);
                println!("making mesh");
                let mesh = bake(&block_registry, &chunk);
                println!("spawning mesh");
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(Color::RED.into()),
                        ..default()
                    })
                    .insert(WorldPosition::from_xyz(
                        (32 * x) as f64,
                        (32 * y) as f64,
                        (32 * z) as f64,
                    ));
            }
        }
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
