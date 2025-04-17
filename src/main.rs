#![windows_subsystem = "windows"]

mod methods;
mod sorter;

use eframe::egui;
use egui::Button;
use egui::{ComboBox, TextEdit};
use methods::{METHODS, MODIFIERS};
use sorter::Sorter;
use std::sync::Mutex;
use std::time::Duration;

struct State {
    paused: bool,
    delay: u64,
}

lazy_static::lazy_static! {
    static ref GLOBAL_STATE: Mutex<State> = Mutex::new(State {
        paused: false,
        delay: 3000,
    });
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Sorting Visualization",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);

            Ok(Box::new(SortVis::default()))
        }),
    )
}

struct SortResult {
    name: &'static str,
    data_size: u32,
    delay: u64,
    time: Duration,
}

struct SortVis {
    sorter: Sorter,
    selected_method: usize,
    data_size_text: String,
    history: Vec<SortResult>,
}

impl Default for SortVis {
    fn default() -> Self {
        Self {
            sorter: Sorter::new((1..=50).collect::<Vec<u32>>()),
            selected_method: 0,
            data_size_text: String::new(),
            history: Vec::new(),
        }
    }
}

impl eframe::App for SortVis {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Sorting Visualization");

            ui.horizontal(|ui| {
                let mut global = GLOBAL_STATE.lock().unwrap();
                // ─── Left: controls ───
                ui.vertical(|ui| {
                    // Row 1: Generate
                    ui.horizontal(|ui| {
                        let re = ui.add(
                            TextEdit::singleline(&mut self.data_size_text).desired_width(50.0),
                        );
                        let clicked = ui.button("Generate").clicked();
                        let pressed_enter =
                            re.lost_focus() && ctx.input(|i| i.key_down(egui::Key::Enter));
                        if clicked || pressed_enter {
                            if let Ok(n) = self.data_size_text.parse::<u32>() {
                                if n > 0 {
                                    if let Ok(mut s) = self.sorter.state.lock() {
                                        if !s.sorting {
                                            s.data = (1..=n).collect();
                                        }
                                    }
                                } else {
                                    self.data_size_text.clear();
                                }
                            } else {
                                self.data_size_text.clear();
                            }
                        }
                    });

                    let sorting_active = self.sorter.is_sorting();

                    // Row 2: Modifiers
                    ui.horizontal(|ui| {
                        for m in MODIFIERS {
                            if ui
                                .add_enabled(!sorting_active, Button::new(m.name))
                                .clicked()
                            {
                                self.sorter.method = Some(m.func);
                                self.sorter.start(false);
                            }
                        }
                    });

                    // Row 3: ComboBox
                    ComboBox::from_label("Select Sorting Method")
                        .selected_text(METHODS[self.selected_method].name)
                        .show_ui(ui, |ui| {
                            for (i, method) in METHODS.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_method, i, method.name);
                            }
                        });

                    // Row 4: Start/Stop, Pause/Resume, Step
                    ui.horizontal(|ui| {
                        let label = if sorting_active { "Stop" } else { "Start" };
                        if ui.button(label).clicked() {
                            if sorting_active {
                                self.sorter.stop();
                                global.paused = false;
                            } else {
                                self.sorter.method = Some(METHODS[self.selected_method].func);
                                self.sorter.start(true);
                            }
                        }
                        let btn = ui.add_enabled(
                            sorting_active,
                            Button::new(if global.paused { "Resume" } else { "Pause" }),
                        );
                        if btn.clicked() && sorting_active {
                            global.paused = !global.paused;
                            if !global.paused {
                                self.sorter.resume();
                            }
                        }
                        if ui
                            .add_enabled(sorting_active, Button::new("Step"))
                            .clicked()
                        {
                            self.sorter.resume();
                        }
                    });

                    // Row 5: Delay slider
                    ui.horizontal(|ui| {
                        ui.label("Delay (μs)");
                        let mut d = global.delay;
                        if ui
                            .add(egui::Slider::new(&mut d, 0..=100_000).logarithmic(true))
                            .changed()
                        {
                            global.delay = d;
                        }
                    });
                });

                // Spacer to push the table right
                let cell_width = 75.0;
                let spacing = 10.0;
                let table_width = 4.0 * cell_width + 4.0 * spacing;
                let avail = ui.available_width();
                if avail > table_width {
                    ui.add_space(avail - table_width);
                }

                // Update history
                let mut state = self.sorter.state.lock().unwrap();
                if let Some(stop_time) = state.stop_time.take() {
                    let elapsed = stop_time.duration_since(state.start_time.unwrap());
                    let result = SortResult {
                        name: METHODS[self.selected_method].name,
                        data_size: state.data.len() as u32,
                        delay: global.delay,
                        time: elapsed,
                    };
                    self.history.push(result);
                    if self.history.len() > 4 {
                        self.history.remove(0);
                    }
                }

                // Right: 5×5 table
                ui.vertical(|ui| {
                    egui::Grid::new("value_table")
                        .striped(true)
                        .spacing(egui::vec2(spacing, 4.0))
                        .min_col_width(cell_width)
                        .show(ui, |ui| {
                            ui.label("Sort");
                            ui.label("Size");
                            ui.label("Delay (μs)");
                            ui.label("Time (ms)");
                            ui.end_row();
                            for row in self.history.iter().rev() {
                                ui.label(row.name);
                                ui.label(row.data_size.to_string());
                                ui.label(row.delay.to_string());
                                let time_us = row.time.as_millis();
                                if time_us == 0 {
                                    ui.label(format!("{:.6}", row.time.as_secs_f64() * 1_000.0));
                                } else {
                                    ui.label(time_us.to_string());
                                }
                                ui.end_row();
                            }
                        });
                });
            });

            // Graph
            let state = self.sorter.state.lock().unwrap();
            ui.add_space(20.0);
            let (_, graph_area) =
                ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));
            let maxv = *state.data.iter().max().unwrap_or(&1) as f32;
            let painter = ui.painter_at(graph_area);
            let bar_w = graph_area.size().x / state.data.len().max(1) as f32;

            for (i, &v) in state.data.iter().enumerate() {
                let h = graph_area.size().y * (v as f32 / maxv);
                let rect = egui::Rect::from_min_size(
                    egui::pos2(graph_area.min.x + i as f32 * bar_w, graph_area.max.y - h),
                    egui::vec2((bar_w - 2.0).max(bar_w * 0.9), h),
                );
                let color = match state.step {
                    Some(sorter::Step::Read(j)) if j == i => egui::Color32::GREEN,
                    Some(sorter::Step::Swap(j, k)) if j == i || k == i => egui::Color32::RED,
                    _ => egui::Color32::LIGHT_BLUE,
                };
                painter.rect_filled(rect, 0.0, color);
            }

            if state.sorting && !GLOBAL_STATE.lock().unwrap().paused {
                ctx.request_repaint();
            }
        });
    }
}
