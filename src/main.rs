use eframe::{egui::{self, Color32, Rect, Vec2, Pos2, Event}, Frame};

struct GameEngine {
    rectangles: Vec<Rectangle>,
    player_rect: Option<usize>,
    grid_size: Vec2,
    edit_mode: bool,
}

struct Rectangle {
    bounds: Rect,
    color: Color32,
    momentum: Vec2,
    is_player: bool,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self {
            rectangles: Vec::new(),
            player_rect: None,
            grid_size: Vec2::new(50.0, 50.0),
            edit_mode: true,
        }
    }
}

impl GameEngine {
    fn add_rectangle(&mut self, rect: Rect) {
        self.rectangles.push(Rectangle {
            bounds: rect,
            color: Color32::GREEN,
            momentum: Vec2::ZERO,
            is_player: false,
        });
    }

    fn handle_player_movement(&mut self, dir: Vec2) {
        if let Some(player_index) = self.player_rect {
            let rect = &mut self.rectangles[player_index];

            if rect.is_player {
                rect.momentum += dir * 5.0;

                if rect.momentum.length() > 10.0 {
                    rect.momentum = rect.momentum.normalized() * 10.0;
                }
            }
        }
    }

    fn update_physics(&mut self) {
        for rect in &mut self.rectangles {
            rect.bounds = rect
                .bounds
                .translate(rect.momentum);

            // Simulate momentum dissipation
            rect.momentum *= 0.9;

            // Collision detection and bounce
            if rect.bounds.min.x < 0.0 || rect.bounds.max.x > 800.0 {
                rect.momentum.x = -rect.momentum.x;
            }
            if rect.bounds.min.y < 0.0 || rect.bounds.max.y > 600.0 {
                rect.momentum.y = -rect.momentum.y;
            }
        }
    }
}

impl eframe::App for GameEngine {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let events = ctx.input(|i| i.events.clone());
        events.iter().for_each(|event| {
            match event {
                Event::PointerButton { button, pressed, pos, .. } => {
                    if *pressed {
                        if *button == egui::PointerButton::Primary {
                            self.add_rectangle(Rect::from_min_size(*pos, self.grid_size));
                        } else if *button == egui::PointerButton::Secondary {
                            // Right click to toggle player assignment
                            let mut index_to_update = None;
                            let current_player = self.player_rect;

                            // First, find the rectangle that was clicked
                            for (i, rect) in self.rectangles.iter().enumerate() {
                                if rect.bounds.contains(*pos) {
                                    index_to_update = Some(i);
                                    break;
                                }
                            }

                            // Then update the rectangles based on what we found
                            if let Some(i) = index_to_update {
                                if current_player == Some(i) {
                                    // Clicking on current player - remove player status
                                    self.rectangles[i].is_player = false;
                                    self.player_rect = None;
                                } else {
                                    // Clicking on a new rectangle - update player status
                                    if let Some(prev_player) = current_player {
                                        self.rectangles[prev_player].is_player = false;
                                    }
                                    self.rectangles[i].is_player = true;
                                    self.player_rect = Some(i);
                                }
                            }
                        }
                    }
                }
                Event::Key { key, pressed, modifiers: _, .. } => {
                    if let (true, Some(player_index)) = (*pressed, self.player_rect) {
                        let dir = match key {
                            egui::Key::W => Vec2::new(0.0, -1.0),
                            egui::Key::A => Vec2::new(-1.0, 0.0),
                            egui::Key::S => Vec2::new(0.0, 1.0),
                            egui::Key::D => Vec2::new(1.0, 0.0),
                            _ => Vec2::ZERO,
                        };

                        if dir.length() > 1e-2 {
                            self.handle_player_movement(dir * self.grid_size);
                        }
                    }
                }
                _ => {}
            }
        });

        self.update_physics();

        egui::CentralPanel::default().show(ctx, |ui| {
            for rect in &self.rectangles {
                ui.painter().rect_filled(
                    rect.bounds,
                    0.0,
                    if rect.is_player { Color32::RED } else { rect.color },
                );
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Potato Game Engine",
        options,
        Box::new(|_cc| Ok(Box::new(GameEngine::default()))),
    )
}