use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
    math::Vec3,
    prelude::{Commands, OnEnter, Res, Resource, Transform},
    sprite::{Sprite, SpriteBundle},
};
use noise::{NoiseFn, Perlin};

use crate::state::AppState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::generate(123456, 200, 200))
            .add_systems(OnEnter(AppState::InGame), spawn_map);
    }
}

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>, // Flat array of tiles
}

impl Map {
    pub fn generate(seed: u32, width: usize, height: usize) -> Self {
        let perlin = Perlin::new(seed); // Perlin noise for smooth transitions

        let mut tiles = Vec::new();

        for y in 0..height {
            for x in 0..width {
                // Normalize x and y to [-1, 1] for Perlin noise
                let nx = x as f64 / width as f64 - 0.5;
                let ny = y as f64 / height as f64 - 0.5;

                // Get noise value (ranges from -1 to 1)
                let noise_value = perlin.get([nx, ny]);

                // Map noise value to tile type
                let tile_type = if noise_value < -0.2 {
                    TileType::Water
                } else if noise_value < 0.2 {
                    TileType::Grass
                } else {
                    TileType::Dirt
                };

                tiles.push(tile_type);
            }
        }

        Self {
            width,
            height,
            tiles,
        }
    }
}

pub enum TileType {
    Water,
    Grass,
    Dirt,
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<Map>) {
    // Load textures for each tile type
    let water_texture = asset_server.load("water.png");
    let grass_texture = asset_server.load("grass.png");
    let dirt_texture = asset_server.load("dirt.png");

    let tile_size = 32.0; // Size of each tile in pixels

    for y in 0..map.height {
        for x in 0..map.width {
            let tile_type: &TileType = &map.tiles[y * map.width + x];
            let texture = match tile_type {
                TileType::Water => water_texture.clone(),
                TileType::Grass => grass_texture.clone(),
                TileType::Dirt => dirt_texture.clone(),
            };

            let readjusted_x = x as f32 - map.width as f32 / 2.0;
            let readjusted_y = y as f32 - map.height as f32 / 2.0;

            commands.spawn((
                Sprite::from_image(texture),
                Transform {
                    translation: Vec3::new(readjusted_x * tile_size, readjusted_y * tile_size, 0.0),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
            ));
        }
    }
}
