use macroquad::prelude::*;
use gaymwtf_core::{World, load_texture_sync, Direction, DrawBatch, Entity};
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct PlayerTextures {
    pub idle_up: Texture2D,
    pub idle_down: Texture2D,
    pub idle_left: Texture2D,
    pub idle_right: Texture2D,
    pub walk_up: [Texture2D; 2],
    pub walk_down: [Texture2D; 2],
    pub walk_left: [Texture2D; 2],
    pub walk_right: [Texture2D; 2],
}

fn load_texture_or_empty(path: &str) -> Texture2D {
    load_texture_sync(path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load texture {}: {:?}", path, e);
            Texture2D::empty()
        })
}

static IDLE_UP: Lazy<Texture2D> = Lazy::new(|| load_texture_or_empty("assets/textures/entities/player/idle_up.png"));
static IDLE_DOWN: Lazy<Texture2D> = Lazy::new(|| load_texture_or_empty("assets/textures/entities/player/idle_down.png"));
static IDLE_LEFT: Lazy<Texture2D> = Lazy::new(|| load_texture_or_empty("assets/textures/entities/player/idle_left.png"));
static IDLE_RIGHT: Lazy<Texture2D> = Lazy::new(|| load_texture_or_empty("assets/textures/entities/player/idle_right.png"));

static WALK_UP: Lazy<[Texture2D; 2]> = Lazy::new(|| [
    load_texture_or_empty("assets/textures/entities/player/walk_up_1.png"),
    load_texture_or_empty("assets/textures/entities/player/walk_up_2.png"),
]);

static WALK_DOWN: Lazy<[Texture2D; 2]> = Lazy::new(|| [
    load_texture_or_empty("assets/textures/entities/player/walk_down_1.png"),
    load_texture_or_empty("assets/textures/entities/player/walk_down_2.png"),
]);

static WALK_LEFT: Lazy<[Texture2D; 2]> = Lazy::new(|| [
    load_texture_or_empty("assets/textures/entities/player/walk_left_1.png"),
    load_texture_or_empty("assets/textures/entities/player/walk_left_2.png"),
]);

static WALK_RIGHT: Lazy<[Texture2D; 2]> = Lazy::new(|| [
    load_texture_or_empty("assets/textures/entities/player/walk_right_1.png"),
    load_texture_or_empty("assets/textures/entities/player/walk_right_2.png"),
]);

impl PlayerTextures {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            idle_up: IDLE_UP.clone(),
            idle_down: IDLE_DOWN.clone(),
            idle_left: IDLE_LEFT.clone(),
            idle_right: IDLE_RIGHT.clone(),
            walk_up: WALK_UP.clone(),
            walk_down: WALK_DOWN.clone(),
            walk_left: WALK_LEFT.clone(),
            walk_right: WALK_RIGHT.clone(),
        })
    }
}




#[derive(Clone)]
pub struct Player {
    pos: Vec2,
    size: Vec2,
    velocity: Vec2,
    direction: Direction,
    is_moving: bool,
    animation_frame: usize,
    animation_timer: f32,
    textures: PlayerTextures,
}

impl Player {
    pub fn new(pos: Vec2, textures: PlayerTextures) -> Self {
        Self {
            pos,
            size: vec2(16.0, 16.0),
            velocity: Vec2::ZERO,
            direction: Direction::Down,
            is_moving: false,
            animation_frame: 0,
            animation_timer: 0.0,
            textures,
        }
    }

    fn handle_input(&mut self) {
        let mut input = Vec2::ZERO;
        if is_key_down(KeyCode::W) { input.y -= 1.0; }
        if is_key_down(KeyCode::S) { input.y += 1.0; }
        if is_key_down(KeyCode::A) { input.x -= 1.0; }
        if is_key_down(KeyCode::D) { input.x += 1.0; }

        if input != Vec2::ZERO {
            input = input.normalize();
            self.is_moving = true;
            if input.x.abs() > input.y.abs() {
                self.direction = if input.x > 0.0 { Direction::Right } else { Direction::Left };
            } else {
                self.direction = if input.y > 0.0 { Direction::Down } else { Direction::Up };
            }
        } else {
            self.is_moving = false;
        }
        self.velocity = input;
    }

    fn update_animation(&mut self, dt: f32) {
        if self.is_moving {
            self.animation_timer += dt;
            if self.animation_timer > 0.15 {
                self.animation_timer = 0.0;
                let anim_len = match self.direction {
                    Direction::Up => self.textures.walk_up.len(),
                    Direction::Down => self.textures.walk_down.len(),
                    Direction::Left => self.textures.walk_left.len(),
                    Direction::Right => self.textures.walk_right.len(),
                };
                self.animation_frame = (self.animation_frame + 1) % anim_len;
            }
        } else {
            self.animation_frame = 0;
        }
    }
}

impl Entity for Player {
    fn get_type_tag(&self) -> &'static str { "player" }
    fn get_pos(&self) -> Vec2 { self.pos }
    fn get_size(&self) -> Vec2 { self.size }
    fn get_velocity(&self) -> Vec2 { self.velocity }

    fn tick(&mut self, dt: f32, _world: &mut World) {
        let pos = self.get_pos();
        let velocity = self.get_velocity();
        self.set_pos(pos + velocity * 2.0);
    
        self.handle_input();
        self.update_animation(dt);
    }
    fn draw(&self, batch: &mut DrawBatch) {
        let texture = if !self.is_moving {
            match self.direction {
                Direction::Up => &self.textures.idle_up,
                Direction::Down => &self.textures.idle_down,
                Direction::Left => &self.textures.idle_left,
                Direction::Right => &self.textures.idle_right,
            }
        } else {
            match self.direction {
                Direction::Up => &self.textures.walk_up[self.animation_frame],
                Direction::Down => &self.textures.walk_down[self.animation_frame],
                Direction::Left => &self.textures.walk_left[self.animation_frame],
                Direction::Right => &self.textures.walk_right[self.animation_frame],
            }
        };
        batch.add(texture.clone(), self.pos, 1.0, Some(self.get_size()));
    }

    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn set_size(&mut self, size: Vec2) { self.size = size; }
    fn set_velocity(&mut self, velocity: Vec2) { self.velocity = velocity; }
    
    fn clone_box(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}