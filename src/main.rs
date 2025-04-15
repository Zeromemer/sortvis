#![windows_subsystem = "windows"]

mod methods;
mod sorter;

use eframe::egui;
use egui::Button;
use egui::{ComboBox, TextEdit};
use methods::{METHODS, MODIFIERS};
use sorter::Sorter;
use std::sync::Mutex;

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

struct SortVis {
    sorter: Sorter,
    selected_method: usize,
    data_size_text: String,
}

impl Default for SortVis {
    fn default() -> Self {
        Self {
            sorter: Sorter::new((1..=50).collect::<Vec<u32>>()),
            selected_method: 0,
            data_size_text: String::new(),
        }
    }
}

impl eframe::App for SortVis {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Sorting Visualization");

                ui.horizontal(|ui| {
                    let re =
                        ui.add(TextEdit::singleline(&mut self.data_size_text).desired_width(50.0));

                    let state = &*self.sorter.state;
                    let mut state = state.lock().unwrap();

                    let clicked = ui.button("Generate").clicked();
                    let pressed_enter =
                        re.lost_focus() && ctx.input(|input| input.key_down(egui::Key::Enter));

                    if !state.sorting && (clicked || pressed_enter) {
                        match self.data_size_text.parse::<u32>() {
                            Err(_) | Ok(0) => self.data_size_text = String::new(),
                            Ok(data_size) => state.data = (1..=data_size).collect(),
                        }
                    }
                });

                let sorting_active = self.sorter.is_sorting();

                ui.horizontal(|ui| {
                    for modifier in MODIFIERS {
                        if ui
                            .add_enabled(!sorting_active, Button::new(modifier.name))
                            .clicked()
                        {
                            let selected_method = modifier.func;
                            self.sorter.method = Some(selected_method);
                            self.sorter.start();
                        }
                    }
                });

                ComboBox::from_label("Select Sorting Method")
                    .selected_text(METHODS[self.selected_method].name)
                    .show_ui(ui, |ui| {
                        for (i, method) in METHODS.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_method, i, method.name);
                        }
                    });

                let mut global_state = GLOBAL_STATE.lock().unwrap();

                ui.horizontal(|ui| {
                    let button_label = if sorting_active { "Stop" } else { "Start" };

                    if ui.button(button_label).clicked() {
                        if sorting_active {
                            self.sorter.stop();
                            global_state.paused = false;
                        } else {
                            let selected_method = METHODS[self.selected_method].func;
                            self.sorter.method = Some(selected_method);
                            self.sorter.start();
                        }
                    }

                    let button = ui.add_enabled(
                        sorting_active,
                        egui::Button::new(if global_state.paused {
                            "Resume"
                        } else {
                            "Pause"
                        }),
                    );
                    if button.clicked() && sorting_active {
                        global_state.paused = !global_state.paused;
                        if !global_state.paused {
                            self.sorter.resume();
                        }
                    }

                    if ui.add_enabled(sorting_active, Button::new("Step")).clicked() {
                        self.sorter.resume();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Delay");
                    let mut delay_value = global_state.delay as u64;

                    if ui
                        .add(egui::Slider::new(&mut delay_value, 0..=100000).logarithmic(true))
                        .changed()
                    {
                        global_state.delay = delay_value;
                    }
                });

                let state = &*self.sorter.state;
                let state = state.lock().unwrap();

                ui.add_space(20.0);

                let (_, graph_area) =
                    ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));

                let max_value = *state.data.iter().max().unwrap();
                let painter = ui.painter_at(graph_area);
                let bar_width = graph_area.size().x / state.data.len() as f32;

                for (i, &value) in state.data.iter().enumerate() {
                    let bar_height = graph_area.size().y * (value as f32 / max_value as f32);
                    let bar_rect = egui::Rect::from_min_size(
                        egui::pos2(
                            graph_area.min.x + i as f32 * bar_width,
                            graph_area.max.y - bar_height,
                        ),
                        egui::vec2((bar_width - 2.0).max(bar_width * 0.9), bar_height),
                    );

                    let color = match state.step {
                        Some(sorter::Step::Read(j)) if j == i => egui::Color32::GREEN,
                        Some(sorter::Step::Swap(j, k)) if j == i || k == i => egui::Color32::RED,
                        _ => egui::Color32::LIGHT_BLUE,
                    };
                    painter.rect_filled(bar_rect, 0.0, color);
                }

                if state.sorting && !global_state.paused {
                    ctx.request_repaint();
                }
            });
        });
    }
}
