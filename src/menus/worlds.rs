use gaymwtf_core::{Menu, MenuAction, DrawBatch};
use macroquad::prelude::*;
use std::fs;

pub struct WorldsMenu {
    worlds: Vec<String>,
    hovered: Option<usize>,
    create_hovered: bool,
    back_hovered: bool,
}

impl WorldsMenu {
    pub fn new() -> Self {
        Self {
            worlds: Self::load_worlds(),
            hovered: None,
            create_hovered: false,
            back_hovered: false,
        }
    }
    fn load_worlds() -> Vec<String> {
        let mut result = Vec::new();
        if let Ok(entries) = fs::read_dir("saves") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    result.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
        result
    }
    fn world_rect(&self, i: usize, screen_w: f32) -> Rect {
        Rect::new(screen_w / 2.0 - 200.0, 120.0 + i as f32 * 60.0, 400.0, 50.0)
    }
    fn create_rect(&self, screen_w: f32, screen_h: f32) -> Rect {
        Rect::new(screen_w / 2.0 - 120.0, screen_h - 180.0, 240.0, 50.0)
    }
    fn back_rect(&self, screen_w: f32, screen_h: f32) -> Rect {
        Rect::new(screen_w / 2.0 - 80.0, screen_h - 100.0, 160.0, 40.0)
    }
}

impl Menu for WorldsMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        let (mx, my) = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        self.hovered = None;
        self.create_hovered = false;
        self.back_hovered = false;

        for (i, name) in self.worlds.iter().enumerate() {
            if self.world_rect(i, screen_w).contains(vec2(mx, my)) {
                self.hovered = Some(i);
                if is_mouse_button_pressed(MouseButton::Left) {
                    return MenuAction::ChangeState(format!("play:{}", name));
                }
            }
        }

        if self.create_rect(screen_w, screen_h).contains(vec2(mx, my)) {
            self.create_hovered = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                return MenuAction::ChangeState("createworld".to_string());
            }
        }

        if self.back_rect(screen_w, screen_h).contains(vec2(mx, my)) {
            self.back_hovered = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                return MenuAction::ChangeState("start".to_string());
            }
        }
        MenuAction::None
    }
    fn draw(&mut self, _batch: &mut DrawBatch) {
        clear_background(BLACK);
        let screen_w = screen_width();
        let screen_h = screen_height();
        draw_text("My Worlds", screen_w / 2.0 - 80.0, 70.0, 40.0, WHITE);
        for (i, name) in self.worlds.iter().enumerate() {
            let color = if Some(i) == self.hovered { YELLOW } else { WHITE };
            draw_rectangle_lines(
                self.world_rect(i, screen_w).x,
                self.world_rect(i, screen_w).y,
                self.world_rect(i, screen_w).w,
                self.world_rect(i, screen_w).h,
                2.0,
                color,
            );
            draw_text(
                name,
                screen_w / 2.0 - 180.0,
                155.0 + i as f32 * 60.0,
                32.0,
                color,
            );
        }

        let color = if self.create_hovered { YELLOW } else { WHITE };
        draw_rectangle_lines(
            self.create_rect(screen_w, screen_h).x,
            self.create_rect(screen_w, screen_h).y,
            self.create_rect(screen_w, screen_h).w,
            self.create_rect(screen_w, screen_h).h,
            2.0,
            color,
        );
        draw_text(
            "Create New World",
            screen_w / 2.0 - 110.0,
            screen_h - 145.0,
            32.0,
            color,
        );
        
        let color = if self.back_hovered { YELLOW } else { WHITE };
        draw_rectangle_lines(
            self.back_rect(screen_w, screen_h).x,
            self.back_rect(screen_w, screen_h).y,
            self.back_rect(screen_w, screen_h).w,
            self.back_rect(screen_w, screen_h).h,
            2.0,
            color,
        );
        draw_text(
            "Back",
            screen_w / 2.0 - 25.0,
            screen_h - 70.0,
            28.0,
            color,
        );
    }
    fn name(&self) -> &str { "worlds" }
} 