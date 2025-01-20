mod sorter;

use eframe::egui;
use rand::seq::SliceRandom;
use sorter::Sorter;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sorting Visualization",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    sorter: Sorter,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut data = (1..=50).collect::<Vec<u32>>();
        data.shuffle(&mut rand::thread_rng());

        Self {
            sorter: Sorter::new(data, |int| {
                let len = int.len();
                for i in 0..len {
                    for j in 0..len - i - 1 {
                        if int.read(j) > int.read(j + 1) {
                            int.swap(j, j + 1);
                        }
                    }
                }
            }),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Sorting Visualization");

                if ui.button("Start Sorting").clicked() {
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
