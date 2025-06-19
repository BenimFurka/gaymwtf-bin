use gaymwtf_core::{Menu, MenuAction, DrawBatch};
use macroquad::prelude::*;
use std::fs;

pub struct CreateWorldMenu {
    pub world_name: String,
    pub seed_input: String,
    selected_field: usize,
    error_message: Option<String>,
}

impl CreateWorldMenu {
    pub fn new() -> Self {
        Self {
            world_name: String::new(),
            seed_input: String::new(),
            selected_field: 0,
            error_message: None,
        }
    }
    fn field_rect(&self, field: usize, center_x: f32, y: f32) -> Rect {
        match field {
            0 => Rect::new(center_x + 160.0, y - 28.0, 200.0, 36.0),
            1 => Rect::new(center_x + 160.0, y - 28.0, 200.0, 36.0),
            2 => Rect::new(center_x, y, 180.0, 48.0),
            3 => Rect::new(center_x + 220.0, y, 140.0, 48.0),
            _ => Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl Menu for CreateWorldMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        let (mx, my) = mouse_position();
        let screen_w = screen_width();
        let _screen_h = screen_height();
        let center_x = screen_w / 2.0 - 180.0;
        let mut y = 120.0 + 50.0 * 2.0;

        if self.field_rect(0, center_x, y).contains(vec2(mx, my)) && is_mouse_button_pressed(MouseButton::Left) {
            self.selected_field = 0;
        }
        y += 50.0;
        if self.field_rect(1, center_x, y).contains(vec2(mx, my)) && is_mouse_button_pressed(MouseButton::Left) {
            self.selected_field = 1;
        }
        y += 50.0 * 1.5;
        if self.field_rect(2, center_x, y).contains(vec2(mx, my)) && is_mouse_button_pressed(MouseButton::Left) {
            self.selected_field = 2;
            let name = self.world_name.trim();
            if name.is_empty() {
                self.error_message = Some("World name cannot be empty".to_string());
                return MenuAction::None;
            }
            let seed = if self.seed_input.trim().is_empty() {
                rand::gen_range(0, u32::MAX)
            } else {
                match self.seed_input.trim().parse::<u32>() {
                    Ok(s) => s,
                    Err(_) => {
                        self.error_message = Some("Seed must be a number".to_string());
                        return MenuAction::None;
                    }
                }
            };
            let save_dir = format!("saves/{}", name);
            if let Err(e) = fs::create_dir_all(&save_dir) {
                self.error_message = Some(format!("Error creating folder: {}", e));
                return MenuAction::None;
            }
            return MenuAction::ChangeState(format!("createworld:{}:{}", name, seed));
        }
        if self.field_rect(3, center_x, y).contains(vec2(mx, my)) && is_mouse_button_pressed(MouseButton::Left) {
            self.selected_field = 3;
            return MenuAction::ChangeState("worlds".to_string());
        }

        if self.selected_field == 0 {
            if let Some(c) = get_char_pressed() {
                if c.is_ascii() && (c.is_alphanumeric() || c == ' ') && self.world_name.len() < 20 {
                    self.world_name.push(c);
                }
            }
            if is_key_pressed(KeyCode::Backspace) && !self.world_name.is_empty() {
                self.world_name.pop();
            }
        }
        if self.selected_field == 1 {
            if let Some(c) = get_char_pressed() {
                if c.is_ascii() && c.is_numeric() && self.seed_input.len() < 10 {
                    self.seed_input.push(c);
                }
            }
            if is_key_pressed(KeyCode::Backspace) && !self.seed_input.is_empty() {
                self.seed_input.pop();
            }
        }
        MenuAction::None
    }
    fn draw(&mut self, _batch: &mut DrawBatch) {
        clear_background(BLACK);
        let screen_w = screen_width();
        let _screen_h = screen_height();
        let center_x = screen_w / 2.0 - 180.0;
        let mut y = 120.0;
        let line_height = 50.0;
        draw_text("Creating new world", center_x, y, 36.0, WHITE);
        y += line_height * 2.0;
        draw_text("World Name:", center_x, y, 28.0, WHITE);
        let color = if self.selected_field == 0 { YELLOW } else { WHITE };
        draw_rectangle(center_x + 160.0, y - 28.0, 200.0, 36.0, DARKGRAY);
        draw_text(&self.world_name, center_x + 170.0, y, 28.0, color);
        y += line_height;
        draw_text("Seed:", center_x, y, 28.0, WHITE);
        let color = if self.selected_field == 1 { YELLOW } else { WHITE };
        draw_rectangle(center_x + 160.0, y - 28.0, 200.0, 36.0, DARKGRAY);
        draw_text(&self.seed_input, center_x + 170.0, y, 28.0, color);
        y += line_height * 1.5;
        let color = if self.selected_field == 2 { YELLOW } else { WHITE };
        draw_rectangle_lines(center_x, y, 180.0, 48.0, 2.0, color);
        draw_text("Create", center_x + 30.0, y + 36.0, 32.0, color);
        let color = if self.selected_field == 3 { YELLOW } else { WHITE };
        draw_rectangle_lines(center_x + 220.0, y, 140.0, 48.0, 2.0, color);
        draw_text("Back", center_x + 240.0, y + 36.0, 32.0, color);
        
        if let Some(msg) = &self.error_message {
            draw_text(msg, center_x, y + 80.0, 24.0, RED);
        }
    }
    fn name(&self) -> &str { "createworld" }
} 