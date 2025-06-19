use gaymwtf_core::{load_texture_sync, DrawBatch, Tile, TILE_SIZE};
use macroquad::prelude::*;
use once_cell::sync::Lazy;

static WATER_TEXTURE: Lazy<Texture2D> = Lazy::new(|| {
    load_texture_sync("assets/textures/tiles/water.png").expect("Failed to load water texture")
});

#[derive(Clone, Debug)]
pub struct WaterTile {
    pos: Vec2,
}

impl WaterTile {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }

    pub fn get_texture(&self) -> Texture2D {
        WATER_TEXTURE.clone()
    }
}

impl Tile for WaterTile {
    fn get_type_tag(&self) -> &'static str { "water" }
    fn get_pos(&self) -> Vec2 { self.pos }
    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn get_size(&self) -> Vec2 { vec2(TILE_SIZE, TILE_SIZE) }
    fn clone_box(&self) -> Box<dyn Tile> { Box::new(self.clone()) }

    fn draw(&self, batch: &mut DrawBatch, pos: Vec2) {
        batch.add(self.get_texture(), pos, 0.0, None);
    }
}
