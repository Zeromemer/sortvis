#![windows_subsystem = "windows"]

mod methods;
mod sorter;

use eframe::egui;
use egui::TextEdit;
use methods::METHODS;
use sorter::Sorter;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sorting Visualization",
        options,
        Box::new(|_cc| Ok(Box::new(SortVis::default()))),
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

                    let (state, _) = &*self.sorter.state;
                    let mut state = state.lock().unwrap();

                    let clicked = ui.button("Generate").clicked();
                    let pressed_enter =
                        re.lost_focus() && ctx.input(|input| input.key_down(egui::Key::Enter));

                    if !state.sorting && clicked || pressed_enter {
                        if let Ok(data_size) = self.data_size_text.parse::<u32>() {
                            state.data = (1..=data_size).collect();
                        }
                        self.data_size_text = String::new();
                    }
                });

                egui::ComboBox::from_label("Select Sorting Method")
                    .selected_text(METHODS[self.selected_method].name)
                    .show_ui(ui, |ui| {
                        for (i, method) in METHODS.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_method, i, method.name);
                        }
                    });

                if ui.button("Start Sorting").clicked() {
                    let selected_method = METHODS[self.selected_method].func;
                    self.sorter.method = Some(selected_method);
                    self.sorter.start();
                }

                let (state, condv) = &*self.sorter.state;
                let state = state.lock().unwrap();

                ui.add_space(20.0);

                let (_, graph_area) =
                    ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));

                let painter = ui.painter_at(graph_area);
                let bar_width = graph_area.size().x / state.data.len() as f32;

                for (i, &value) in state.data.iter().enumerate() {
                    let bar_height = graph_area.size().y
                        * (value as f32 / *state.data.iter().max().unwrap() as f32);
                    let bar_rect = egui::Rect::from_min_size(
                        egui::pos2(
                            graph_area.min.x + i as f32 * bar_width,
                            graph_area.max.y - bar_height,
                        ),
                        egui::vec2(bar_width - 2.0, bar_height),
                    );

                    let color = match state.step {
                        Some(sorter::Step::Read(j)) if j == i => egui::Color32::GREEN,
                        Some(sorter::Step::Swap(j, k)) if j == i || k == i => egui::Color32::RED,
                        _ => egui::Color32::LIGHT_BLUE,
                    };
                    painter.rect_filled(bar_rect, 0.0, color);
                }

                if state.sorting {
                    condv.notify_all();
                    ctx.request_repaint();
                }
            });
        });
    }
}
