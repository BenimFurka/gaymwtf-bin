use gaymwtf_core::{load_texture_sync, DrawBatch, Object};
use macroquad::prelude::*;
use once_cell::sync::Lazy;

static SNOW_TREE_TEXTURE: Lazy<Texture2D> = Lazy::new(|| {
    load_texture_sync("assets/textures/objects/snowtree/snowtree.png").expect("Failed to load snow_tree texture")
});

#[derive(Clone, Debug)]
pub struct SnowTree {
    pos: Vec2,
    size: Vec2,
}

impl SnowTree {
    pub fn new(pos: Vec2) -> Self {
        Self { pos, size: vec2(16.0, 32.0) }
    }
}

impl SnowTree {
    pub fn get_texture(&self) -> Texture2D {
        SNOW_TREE_TEXTURE.clone()
    }
}

impl Object for SnowTree {
    fn get_type_tag(&self) -> &'static str { "snow_tree" }
    fn get_pos(&self) -> Vec2 { self.pos }
    fn get_size(&self) -> Vec2 { self.size }
    fn get_velocity(&self) -> Vec2 { Vec2::ZERO }

    fn tick(&mut self, _dt: f32, _world: &mut gaymwtf_core::World) { }
    fn draw(&self, batch: &mut DrawBatch) {
        batch.add(self.get_texture(), self.pos, 1.0, Some(self.size));
    }

    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn set_size(&mut self, size: Vec2) { self.size = size; }
    fn set_velocity(&mut self, _velocity: Vec2) { }
    
    fn clone_box(&self) -> Box<dyn Object> { Box::new(self.clone()) }
}
