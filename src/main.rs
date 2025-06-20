pub mod biomes;
pub mod objects;
pub mod player;
pub mod tiles;
pub mod worldgen;
pub mod menus;
pub mod utils;

use gaymwtf_core::{
    BiomeRegistry, DrawBatch, ObjectRegistry, TileRegistry, World, TILE_SIZE,
};
use macroquad::prelude::*;
use std::fs;
use serde::{Serialize, Deserialize};
extern crate serde;
extern crate serde_json;

use biomes::{
    beach::BeachBiome, desert::DesertBiome, forest::ForestBiome, plains::PlainsBiome,
    river::RiverBiome, snow_forest::SnowForestBiome, snow_plains::SnowPlainsBiome,
};
use objects::{cactus::Cactus, snow_tree::SnowTree, tree::Tree};
use player::{Player, PlayerTextures};
use tiles::{grass::GrassTile, sand::SandTile, snowgrass::SnowGrassTile, water::WaterTile};
use worldgen::generate_chunk;
use menus::start::StartMenu;
use menus::howtoplay::HowToPlayMenu;
use menus::about::AboutMenu;
use menus::worlds::WorldsMenu;
use menus::createworld::CreateWorldMenu;
use menus::game::GameMenu;
use gaymwtf_core::{Menu, MenuAction};

async fn register_tiles(registry: &mut TileRegistry) -> anyhow::Result<()> {
    registry.register(GrassTile::new(Vec2::ZERO));
    registry.register(SandTile::new(Vec2::ZERO));
    registry.register(SnowGrassTile::new(Vec2::ZERO));
    registry.register(WaterTile::new(Vec2::ZERO));
    Ok(())
}

async fn register_objects(registry: &mut ObjectRegistry) -> anyhow::Result<()> {
    registry.register(Tree::new(Vec2::ZERO));
    registry.register(SnowTree::new(Vec2::ZERO));
    registry.register(Cactus::new(Vec2::ZERO));
    registry.register(Player::new(Vec2::ZERO, PlayerTextures::new()?));
    Ok(())
}

async fn register_biomes(registry: &mut BiomeRegistry) -> anyhow::Result<()> {
    registry.register(RiverBiome);
    registry.register(BeachBiome);
    registry.register(DesertBiome);
    registry.register(SnowPlainsBiome);
    registry.register(SnowForestBiome);
    registry.register(PlainsBiome);
    registry.register(ForestBiome);
    Ok(())
}

fn init_registries() -> (TileRegistry, ObjectRegistry, BiomeRegistry) {
    let mut tile_registry = TileRegistry::new();
    let mut object_registry = ObjectRegistry::new();
    let mut biome_registry = BiomeRegistry::new();
    futures::executor::block_on(register_tiles(&mut tile_registry)).unwrap();
    futures::executor::block_on(register_objects(&mut object_registry)).unwrap();
    futures::executor::block_on(register_biomes(&mut biome_registry)).unwrap();
    (tile_registry, object_registry, biome_registry)
}

fn update_camera(camera: &mut Camera2D) {
    let base_zoom = 0.0066668;
    let aspect_ratio = screen_width() / screen_height();
    camera.zoom = if aspect_ratio > 1.0 {
        vec2(base_zoom / aspect_ratio, base_zoom)
    } else {
        vec2(base_zoom, base_zoom * aspect_ratio)
    };
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WorldGenInfo {
    pub seed: u32,
}

#[macroquad::main("gaymwtf")]
async fn main() -> anyhow::Result<()> {
    let mut current_menu: Box<dyn Menu> = Box::new(StartMenu::new());
    let mut batch = DrawBatch::new();
    loop {
        let dt = get_frame_time();
        let action = current_menu.update(dt);
        current_menu.draw(&mut batch);
        next_frame().await;
        match action {
            MenuAction::ChangeState(state) => {
                match state.as_str() {
                    "start" | "menu" => {
                        current_menu = Box::new(StartMenu::new());
                    }
                    "howtoplay" => {
                        current_menu = Box::new(HowToPlayMenu::new());
                    }
                    "about" => {
                        current_menu = Box::new(AboutMenu::new());
                    }
                    "worlds" => {
                        current_menu = Box::new(WorldsMenu::new());
                    }
                    "createworld" => {
                        current_menu = Box::new(CreateWorldMenu::new());
                    }
                    s if s.starts_with("createworld:") => {
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() == 3 {
                            let name = parts[1];
                            let seed: u32 = parts[2].parse().unwrap_or(rand::gen_range(0, u32::MAX));
                            let (tile_registry, object_registry, biome_registry) = init_registries();
                            let mut world = World::new(name, tile_registry, object_registry, biome_registry);
                            let mut initial_chunk = generate_chunk((0, 0), seed, &world.tile_registry, &world.object_registry, &world.biome_registry).await?;
                            let player_pos = vec2(TILE_SIZE * 5.0, TILE_SIZE * 5.0);
                            if let Some(mut player) = world.object_registry.create_object_by_id("player") {
                                player.set_pos(player_pos);
                                initial_chunk.objects.push(player);
                            }
                            world.add_chunk(initial_chunk);
                            world.save_world(&format!("saves/{}", name)).ok();
                            let worldgen_info = WorldGenInfo { seed };
                            let worldgen_path = format!("saves/{}/gamestate.json", name);
                            let _ = fs::write(&worldgen_path, serde_json::to_string_pretty(&worldgen_info).unwrap());
                        }
                        current_menu = Box::new(WorldsMenu::new());
                    }
                    s if s.starts_with("play:") || s.starts_with("game:") => {
                        let name = s.trim_start_matches("play:").trim_start_matches("game:");
                        current_menu = Box::new(GameMenu::new(name).await?);
                    }
                    _ => {}
                }
            }
            MenuAction::Quit => return Ok(()),
            _ => {}
        }
    }
}