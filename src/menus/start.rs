use gaymwtf_core::Menu;
use gaymwtf_core::MenuAction;
use gaymwtf_core::DrawBatch;
use macroquad::prelude::*;

pub struct StartMenu {
    selected: Option<usize>,
    options: Vec<&'static str>,
}

impl StartMenu {
    pub fn new() -> Self {
        Self {
            selected: None,
            options: vec!["Start Game", "How to Play", "About", "Exit"],
        }
    }

    fn option_rect(&self, i: usize, screen_w: f32, screen_h: f32) -> Rect {
        let start_y = screen_h / 2.0 - 50.0;
        let line_height = 50.0;
        let x = screen_w / 2.0 - 100.0;
        let y = start_y + i as f32 * line_height - 35.0;
        Rect::new(x, y, 300.0, 50.0)
    }
}

impl Menu for StartMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        let mouse = mouse_position();
        let (mx, my) = (mouse.0, mouse.1);
        let screen_w = screen_width();
        let screen_h = screen_height();
        self.selected = None;
        for (i, _) in self.options.iter().enumerate() {
            if self.option_rect(i, screen_w, screen_h).contains(vec2(mx, my)) {
                self.selected = Some(i);
                if is_mouse_button_pressed(MouseButton::Left) {
                    match i {
                        0 => return MenuAction::ChangeState("worlds".to_string()),
                        1 => return MenuAction::ChangeState("howtoplay".to_string()),
                        2 => return MenuAction::ChangeState("about".to_string()),
                        3 => return MenuAction::Quit,
                        _ => {}
                    }
                }
            }
        }
        MenuAction::None
    }

    fn draw(&mut self, _batch: &mut DrawBatch) {
        clear_background(BLACK);
        let screen_w = screen_width();
        let screen_h = screen_height();
        let start_y = screen_h / 2.0 - 50.0;
        let line_height = 50.0;
        for (i, option) in self.options.iter().enumerate() {
            let color = if Some(i) == self.selected { YELLOW } else { WHITE };
            draw_text(
                option,
                screen_w / 2.0 - 100.0,
                start_y + i as f32 * line_height,
                40.0,
                color,
            );
        }
    }

    fn name(&self) -> &str {
        "start"
    }
} 