use gaymwtf_core::{BiomeRegistry, Chunk, EntityRegistry, TileRegistry, CHUNK_SIZE, TILE_SIZE};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use ::rand::prelude::thread_rng;
use ::rand::Rng;

pub struct WorldGenerator {
    perlin: Perlin,
    scale: f64,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            scale: 0.002,
            octaves: 6,
            persistence: 0.8,
            lacunarity: 2.0,
        }
    }

    fn generate_noise(&self, x: f64, y: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_amplitude = 0.0;

        for _ in 0..self.octaves {
            total += self.perlin.get([x * frequency, y * frequency]) * amplitude;
            max_amplitude += amplitude;
            amplitude *= self.persistence;
            frequency *= self.lacunarity;
        }

        total / max_amplitude
    }

    pub fn get_values(&self, world_x: i32, world_y: i32) -> (f64, f64, f64) {
        let nx = self.scale * world_x as f64;
        let ny = self.scale * world_y as f64;

        let height = (self.generate_noise(nx, ny) + 1.0) / 2.0;

        let temp = (self.generate_noise(nx + 10000.0, ny + 10000.0) + 1.0) / 2.0;

        let moist = (self.generate_noise(nx - 10000.0, ny - 10000.0) + 1.0) / 2.0;

        (height, moist, temp)
    }
}

pub async fn generate_chunk(
    chunk_pos: (i32, i32),
    seed: u32,
    tile_registry: &TileRegistry,
    entity_registry: &EntityRegistry,
    biome_registry: &BiomeRegistry,
) -> anyhow::Result<Chunk> {
    let generator = WorldGenerator::new(seed);
    let mut tiles = Vec::new();
    let mut entities = Vec::new();
    let mut rng = thread_rng();

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let world_x = chunk_pos.0 * CHUNK_SIZE as i32 + x as i32;
            let world_y = chunk_pos.1 * CHUNK_SIZE as i32 + y as i32;
            let tile_pos = vec2(world_x as f32 * TILE_SIZE, world_y as f32 * TILE_SIZE);

            let (height, moisture, temperature) = generator.get_values(world_x, world_y);

            if let Some(biome) = biome_registry.find_biome(height, moisture, temperature) {
                if let Some(mut tile) = tile_registry.create_tile_by_id(biome.get_ground_tile_type()) {
                    tile.set_pos(tile_pos);
                    tiles.push(tile);
                }

                for (entity_type, chance) in biome.get_spawnable_entities() {
                    if rng.gen::<f32>() < chance {
                        if let Some(mut entity) = entity_registry.create_entity_by_id(entity_type) {
                            entity.set_pos(tile_pos);
                            entities.push(entity);
                        }
                    }
                }
            }
        }
    }

    let mut chunk = Chunk::new(vec2(chunk_pos.0 as f32, chunk_pos.1 as f32));
    chunk.tiles = tiles;
    chunk.entities = entities;
    Ok(chunk)
}