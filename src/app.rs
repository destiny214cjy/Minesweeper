use eframe::egui;
use crate::fonts::setup_fonts;
use crate::board::{Board, WIDTH, HEIGHT, CellState};
use egui::{Color32, Vec2, RichText};

pub struct MyApp {
    board: Board,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self {
            board: Board::new(),
        }
    }
}

// 彩色数字
fn number_color(n: u8) -> Color32 {
    match n {
        1 => Color32::from_rgb(0, 0, 255),
        2 => Color32::from_rgb(0, 128, 0),
        3 => Color32::from_rgb(255, 0, 0),
        4 => Color32::from_rgb(0, 0, 128),
        5 => Color32::from_rgb(128, 0, 0),
        6 => Color32::from_rgb(0, 128, 128),
        7 => Color32::from_rgb(0, 0, 0),
        8 => Color32::from_rgb(128, 128, 128),
        _ => Color32::BLACK,
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 顶部信息
            ui.horizontal(|ui| {
                ui.heading("扫雷小游戏");
                ui.add_space(20.0);

                // 剩余雷数
                let flagged_count = self.board.cells.iter()
                    .flatten()
                    .filter(|c| c.state == CellState::Flagged)
                    .count();
                let total_mines = self.board.cells.iter()
                    .flatten()
                    .filter(|c| c.is_mine)
                    .count();
                let remaining = total_mines.saturating_sub(flagged_count);
                ui.label(format!("剩余雷数: {}", remaining));
            });

            let game_won = self.board.check_win_dual();

            if self.board.game_over {
                ui.label("💥 游戏结束！点击重新开始按钮");
            } else if game_won {
                ui.label("🎉 恭喜！你赢了！");
            }

            let cell_size = Vec2::splat(40.0);

            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;

                for y in 0..HEIGHT {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;

                        for x in 0..WIDTH {
                            let cell = &mut self.board.cells[y][x];

                            let (label, color) = match cell.state {
                                CellState::Covered => ("■".to_string(), Color32::WHITE),
                                CellState::Flagged => ("⚑".to_string(), Color32::RED),
                                CellState::Uncovered => {
                                    if cell.is_mine {
                                        ("💣".to_string(), Color32::RED)
                                    } else if cell.neighbor_mines > 0 {
                                        (cell.neighbor_mines.to_string(), number_color(cell.neighbor_mines))
                                    } else {
                                        (" ".to_string(), Color32::WHITE)
                                    }
                                }
                            };

                            // 左键翻格条件
                            let enabled_left = cell.state == CellState::Covered
                                && !self.board.game_over
                                && !game_won;

                            // 右键标旗/撤旗条件
                            let enabled_right = (cell.state == CellState::Covered || cell.state == CellState::Flagged)
                                && !self.board.game_over
                                && !game_won;

                            let button = egui::Button::new(RichText::new(label).color(color))
                                .min_size(cell_size);
                            let response = ui.add(button);

                            // 左键翻格
                            if response.clicked() && enabled_left {
                                if self.board.first_click {
                                    self.board.place_mines(x, y);
                                    self.board.first_click = false;
                                }
                                self.board.uncover(x, y);
                            }

                            // 右键标旗/撤旗
                            if response.secondary_clicked() && enabled_right {
                                self.board.toggle_flag(x, y);
                            }
                        }
                    });
                }
            });

            if ui.button("重新开始").clicked() {
                self.board = Board::new();
            }
        });

        ctx.request_repaint();
    }
}
