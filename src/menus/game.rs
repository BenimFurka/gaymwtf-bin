use gaymwtf_core::{Menu, MenuAction, DrawBatch, TileRegistry, EntityRegistry, BiomeRegistry, World, CHUNK_PIXELS, TILE_SIZE};
use macroquad::prelude::*;
use crate::player::{Player, PlayerTextures};
use crate::menus::pause::PauseMenu;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WorldGenInfo {
    pub seed: u32,
}

impl WorldGenInfo {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }
}

pub struct GameMenu {
    world: World,
    camera: Camera2D,
    paused: bool,
    pause_menu: PauseMenu,
    world_name: String,
    seed: u32,
}

impl GameMenu {
    pub async fn new(world_name: &str) -> anyhow::Result<Self> {
        let mut tile_registry = TileRegistry::new();
        crate::register_tiles(&mut tile_registry).await?;
        let mut entity_registry = EntityRegistry::new();
        crate::register_entities(&mut entity_registry).await?;
        let player_textures = PlayerTextures::new()?;
        let player_pos = vec2(TILE_SIZE * 5.0, TILE_SIZE * 5.0);
        entity_registry.register(Player::new(player_pos, player_textures));
        let mut biome_registry = BiomeRegistry::new();
        crate::register_biomes(&mut biome_registry).await?;
        let mut world = World::load_world(&format!("saves/{}", world_name), tile_registry, entity_registry, biome_registry)
            .map_err(|e| anyhow::anyhow!(e))?;
            
        let worldgen_path = format!("saves/{}/gamestate.json", world_name);
        let seed = WorldGenInfo::load(&worldgen_path)?.seed;

        let mut player_pos = Vec2::ZERO;
        'outer: for chunk in world.chunks.values() {
            for entity in &chunk.entities {
                if entity.get_type_tag() == "player" {
                    player_pos = entity.get_pos();
                    break 'outer;
                }
            }
        }
        let camera = Camera2D {
            target: player_pos,
            zoom: Vec2::ZERO,
            ..Default::default()
        };
        let screen_size = vec2(screen_width(), screen_height());
        world.update(camera.target, screen_size);
        Ok(Self {
            world,
            camera,
            paused: false,
            pause_menu: PauseMenu::new(),
            world_name: world_name.to_string(),
            seed,
        })
    }
}

impl Menu for GameMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        let dt = get_frame_time();
        let screen_size = vec2(screen_width(), screen_height());
        if !self.paused {
            crate::update_camera(&mut self.camera);
            let player_entities = self.world.get_entities_by_type("player");
            let player_pos: Vec2 = player_entities.first().map(|e| e.get_pos()).unwrap_or(Vec2::ZERO);
            self.camera.target = player_pos;
            let player_chunk_pos = (
                (player_pos.x / CHUNK_PIXELS).floor() as i32,
                (player_pos.y / CHUNK_PIXELS).floor() as i32,
            );
            let render_dist = 2;
            for y in -render_dist..=render_dist {
                for x in -render_dist..=render_dist {
                    let chunk_pos_to_check = (player_chunk_pos.0 + x, player_chunk_pos.1 + y);
                    if !self.world.chunks.contains_key(&chunk_pos_to_check) {
                        let new_chunk = futures::executor::block_on(
                            crate::worldgen::generate_chunk(
                                chunk_pos_to_check,
                                self.seed,
                                &self.world.tile_registry,
                                &self.world.entity_registry,
                                &self.world.biome_registry,
                            )
                        ).unwrap();
                        self.world.add_chunk(new_chunk);
                    }
                }
            }
            self.world.update(self.camera.target, screen_size);
            if is_key_pressed(KeyCode::Escape) {
                self.paused = true;
            }
        } else {
            let action = self.pause_menu.update(dt);
            match action {
                MenuAction::ChangeState(ref state) if state == "save" => {
                    self.world.save_world(&format!("saves/{}", self.world_name)).ok();
                    self.paused = false;
                }
                MenuAction::ChangeState(ref state) if state == "exit" => {
                    self.world.save_world(&format!("saves/{}", self.world_name)).ok();
                    return MenuAction::ChangeState("menu".to_string());
                }
                _ => {}
            }
        }
        MenuAction::None
    }
    fn draw(&mut self, _batch: &mut DrawBatch) {
        let screen_size = vec2(screen_width(), screen_height());
        if !self.paused {
            clear_background(BLACK);
            set_camera(&self.camera);
            self.world.draw(self.camera.target, screen_size);
            set_default_camera();
            draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        } else {
            self.pause_menu.draw(&mut DrawBatch::new());
        }
    }
    fn name(&self) -> &str { "game" }
} 