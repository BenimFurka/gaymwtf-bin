use gaymwtf_core::{
    Menu, MenuAction, DrawBatch, TileRegistry, ObjectRegistry, 
    BiomeRegistry, World, CHUNK_PIXELS, TILE_SIZE
};
use macroquad::prelude::*;
use macroquad::text::draw_text_ex;
use macroquad::text::TextParams;
use crate::utils::system::SystemInfo;
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
    system_info: SystemInfo,
}

impl GameMenu {
    pub async fn new(world_name: &str) -> anyhow::Result<Self> {
        let mut tile_registry = TileRegistry::new();
        crate::register_tiles(&mut tile_registry).await?;
        let mut object_registry = ObjectRegistry::new();
        crate::register_objects(&mut object_registry).await?;
        let player_textures = PlayerTextures::new()?;
        let player_pos = vec2(TILE_SIZE * 5.0, TILE_SIZE * 5.0);
        object_registry.register(Player::new(player_pos, player_textures));
        let mut biome_registry = BiomeRegistry::new();
        crate::register_biomes(&mut biome_registry).await?;
        let mut world = World::load_world(&format!("saves/{}", world_name), tile_registry, object_registry, biome_registry)
            .map_err(|e| anyhow::anyhow!(e))?;
            
        let worldgen_path = format!("saves/{}/gamestate.json", world_name);
        let seed = WorldGenInfo::load(&worldgen_path)?.seed;

        let mut player_pos = Vec2::ZERO;
        'outer: for chunk in world.chunks.values() {
            for object in &chunk.objects {
                if object.get_type_tag() == "player" {
                    player_pos = object.get_pos();
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
            system_info: SystemInfo::new(),
        })
    }
}

impl Menu for GameMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        self.system_info.update();
        
        if is_key_pressed(KeyCode::F3) {
            self.system_info.toggle_debug();
        }
        let dt = get_frame_time();
        let screen_size = vec2(screen_width(), screen_height());
        if !self.paused {
            crate::update_camera(&mut self.camera);
            let player_objects = self.world.get_objects_by_type("player");
            let player_pos: Vec2 = player_objects.first().map(|e| e.get_pos()).unwrap_or(Vec2::ZERO);
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
                                &self.world.object_registry,
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
            
            if self.system_info.is_debug_visible() {
                self.draw_chunk_borders();
            }
            
            set_default_camera();
            
            if self.system_info.is_debug_visible() {
                self.draw_debug_info();
            }
        } else {
            self.pause_menu.draw(&mut DrawBatch::new());
        }
    }
    fn name(&self) -> &str { "game" }
} 

impl GameMenu {

    fn draw_debug_info(&self) {
        let x = 10.0;
        let mut y = 20.0;
        let line_height = 22.0;
        let font_size = 20.0;
        
        let fps = self.system_info.fps;
        let cpu = self.system_info.cpu_usage;
        let mem = self.system_info.process_memory;
        
        let (fps_value, fps_color) = if fps >= 45 {
            (format!("{}", fps), GREEN)
        } else if fps >= 30 {
            (format!("{}", fps), YELLOW)
        } else {
            (format!("{}", fps), RED)
        };
        
        let (cpu_value, cpu_color) = if cpu < 10.0 {
            (format!("{:.1}%", cpu), GREEN)
        } else if cpu < 15.0 {
            (format!("{:.1}%", cpu), YELLOW)
        } else {
            (format!("{:.1}%", cpu), RED)
        };
        
        let (mem_value, mem_unit, mem_color) = if mem < 128 {
            (format!("{}", mem), "MB".to_string(), GREEN)
        } else if mem < 256 {
            (format!("{:.1}", mem as f32 / 1024.0), "GB".to_string(), YELLOW)
        } else {
            (format!("{:.1}", mem as f32 / 1024.0), "GB".to_string(), RED)
        };


        draw_rectangle(5.0, 5.0, 200.0, 140.0, Color::new(0.0, 0.0, 0.0, 0.3));
        
        draw_text("FPS: ", x, y, font_size, WHITE);
        draw_text(&fps_value, x + 50.0, y, font_size, fps_color);
        y += line_height;
        
        draw_text("CPU: ", x, y, font_size, WHITE);
        draw_text(&cpu_value, x + 50.0, y, font_size, cpu_color);
        y += line_height;
        
        draw_text("MEM: ", x, y, font_size, WHITE);
        let mem_text = format!("{} {}", mem_value, mem_unit);
        draw_text(&mem_text, x + 70.0, y, font_size, mem_color);
        y += line_height;
        
        if let Some(player) = self.world.get_objects_by_type("player").first() {
            let pos = player.get_pos();
            draw_text("POS: ", x, y, font_size, WHITE);
            draw_text(&format!("{:.1}, {:.1}", pos.x / TILE_SIZE as f32, pos.y / TILE_SIZE as f32), x + 60.0, y, font_size, WHITE);
            y += line_height;
            
            let chunk_x = (pos.x / CHUNK_PIXELS as f32).floor() as i32;
            let chunk_y = (pos.y / CHUNK_PIXELS as f32).floor() as i32;
            draw_text("CHUNK: ", x, y, font_size, WHITE);
            draw_text(&format!("{}:{}", chunk_x, chunk_y), x + 90.0, y, font_size, WHITE);
        }
    }
    
    fn draw_chunk_borders(&self) {
        let chunk_size = CHUNK_PIXELS as f32;
        let camera_pos = self.camera.target;
        let screen_size = vec2(screen_width(), screen_height());
        
        let start_x = ((camera_pos.x - screen_size.x / 2.0) / chunk_size).floor() as i32 - 1;
        let end_x = ((camera_pos.x + screen_size.x / 2.0) / chunk_size).ceil() as i32 + 1;
        let start_y = ((camera_pos.y - screen_size.y / 2.0) / chunk_size).floor() as i32 - 1;
        let end_y = ((camera_pos.y + screen_size.y / 2.0) / chunk_size).ceil() as i32 + 1;

        for x in start_x..=end_x {
            let line_x = x as f32 * chunk_size;
            draw_line(
                line_x,
                (start_y as f32) * chunk_size,
                line_x,
                (end_y as f32) * chunk_size,
                1.0,
                Color::new(1.0, 1.0, 1.0, 0.3)
            );
            
            if self.camera.target.distance(Vec2::new(line_x, camera_pos.y)) < screen_size.x / 1.5 {
                let text = format!("{}:{}", x, (camera_pos.y / chunk_size) as i32);
                draw_text_ex(
                    &text,
                    line_x + 5.0,
                    camera_pos.y - 10.0,
                    TextParams {
                        font_size: 14,
                        color: Color::new(1.0, 1.0, 1.0, 0.5),
                        ..Default::default()
                    }
                );
            }
        }
        
        for y in start_y..=end_y {
            let line_y = y as f32 * chunk_size;
            draw_line(
                (start_x as f32) * chunk_size,
                line_y,
                (end_x as f32) * chunk_size,
                line_y,
                1.0,
                Color::new(1.0, 1.0, 1.0, 0.3)
            );
            
            if self.camera.target.distance(Vec2::new(camera_pos.x, line_y)) < screen_size.y / 1.5 {
                let text = format!("{}:{}", (camera_pos.x / chunk_size) as i32, y);
                draw_text_ex(
                    &text,
                    camera_pos.x + 5.0,
                    line_y - 5.0,
                    TextParams {
                        font_size: 14,
                        color: Color::new(1.0, 1.0, 1.0, 0.5),
                        ..Default::default()
                    }
                );
            }
        }
    }
}