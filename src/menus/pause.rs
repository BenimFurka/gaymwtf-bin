use gaymwtf_core::{Menu, MenuAction, DrawBatch};
use macroquad::prelude::*;

pub struct PauseMenu {
    hovered: Option<usize>,
    options: Vec<&'static str>,
}

impl PauseMenu {
    pub fn new() -> Self {
        Self {
            hovered: None,
            options: vec!["Continue", "Exit to menu"],
        }
    }
    fn option_rect(&self, i: usize, screen_w: f32, screen_h: f32) -> Rect {
        let start_y = screen_h / 2.0 - 30.0;
        let line_height = 60.0;
        let x = screen_w / 2.0 - 120.0;
        let y = start_y + i as f32 * line_height - 20.0;
        Rect::new(x, y, 240.0, 50.0)
    }
}

impl Menu for PauseMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        let (mx, my) = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        self.hovered = None;
        for (i, _) in self.options.iter().enumerate() {
            if self.option_rect(i, screen_w, screen_h).contains(vec2(mx, my)) {
                self.hovered = Some(i);
                if is_mouse_button_pressed(MouseButton::Left) {
                    match i {
                        0 => return MenuAction::ChangeState("save".to_string()),
                        1 => return MenuAction::ChangeState("exit".to_string()),
                        _ => {}
                    }
                }
            }
        }
        MenuAction::None
    }
    fn draw(&mut self, _batch: &mut DrawBatch) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_text("Pause", screen_w / 2.0 - 60.0, screen_h / 2.0 - 80.0, 48.0, WHITE);
        let start_y = screen_h / 2.0 - 30.0;
        let line_height = 60.0;
        for (i, option) in self.options.iter().enumerate() {
            let color = if Some(i) == self.hovered { YELLOW } else { WHITE };
            draw_rectangle_lines(
                self.option_rect(i, screen_w, screen_h).x,
                self.option_rect(i, screen_w, screen_h).y,
                self.option_rect(i, screen_w, screen_h).w,
                self.option_rect(i, screen_w, screen_h).h,
                2.0,
                color,
            );
            draw_text(
                option,
                screen_w / 2.0 - 80.0,
                start_y + i as f32 * line_height + 14.0,
                36.0,
                color,
            );
        }
    }
    fn name(&self) -> &str { "pause" }
} 