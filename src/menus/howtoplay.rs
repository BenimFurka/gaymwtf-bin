use gaymwtf_core::{Menu, MenuAction, DrawBatch};
use macroquad::prelude::*;

pub struct HowToPlayMenu {
    back_hovered: bool,
}

impl HowToPlayMenu {
    pub fn new() -> Self {
        Self { back_hovered: false }
    }
    fn back_rect(&self, screen_w: f32, screen_h: f32) -> Rect {
        Rect::new(screen_w / 2.0 - 80.0, screen_h - 120.0, 160.0, 50.0)
    }
}

impl Menu for HowToPlayMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        let (mx, my) = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        self.back_hovered = self.back_rect(screen_w, screen_h).contains(vec2(mx, my));
        if self.back_hovered && is_mouse_button_pressed(MouseButton::Left) {
            return MenuAction::ChangeState("start".to_string());
        }
        MenuAction::None
    }
    fn draw(&mut self, _batch: &mut DrawBatch) {
        clear_background(BLACK);
        let screen_w = screen_width();
        let _screen_h = screen_height();
        let lines = [
            "Controls:",
            "WASD - movement",
            // TODO: Add "F3 - debug",
        ];
        let start_y = 120.0;
        let line_height = 40.0;
        for (i, line) in lines.iter().enumerate() {
            draw_text(
                line,
                screen_w / 2.0 - 200.0,
                start_y + i as f32 * line_height,
                32.0,
                WHITE,
            );
        }
        let color = if self.back_hovered { YELLOW } else { WHITE };
        draw_text(
            "Back",
            screen_w / 2.0 - 60.0,
            _screen_h - 90.0,
            36.0,
            color,
        );
    }
    fn name(&self) -> &str { "howtoplay" }
} 