use std::{thread, time::Duration};

use eframe::egui;
use rand::seq::SliceRandom;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sorting Visualization",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    data: Vec<u32>,
    swaped: (usize, usize),
    sorting: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            data: {
                let mut data = (1..=50).collect::<Vec<u32>>();
                data.shuffle(&mut rand::thread_rng());
                data
            }, // Initial dataset
            swaped: (0, 0),
            sorting: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Create a vertical layout for the widgets
            ui.vertical(|ui| {
                // Label
                ui.label("Sorting Visualization");

                // Button
                if ui.button("Start Sorting").clicked() {
                    self.sorting = true;
                }

                // Add more space
                ui.add_space(20.0);

                // Reserve specific space for the graph
                let (_, graph_area) =
                    ui.allocate_space(egui::vec2(ui.available_width(), ui.available_height()));

                // Draw the graph within the reserved area
                let painter = ui.painter_at(graph_area);
                let bar_width = graph_area.size().x / self.data.len() as f32;

                for (i, &value) in self.data.iter().enumerate() {
                    let bar_height = graph_area.size().y
                        * (value as f32 / *self.data.iter().max().unwrap() as f32);
                    let bar_rect = egui::Rect::from_min_size(
                        egui::pos2(
                            graph_area.min.x + i as f32 * bar_width,
                            graph_area.max.y - bar_height,
                        ),
                        egui::vec2(bar_width - 2.0, bar_height),
                    );

                    let color = if self.sorting && (i == self.swaped.0 || i == self.swaped.1) {
                        egui::Color32::RED
                    } else {
                        egui::Color32::LIGHT_BLUE
                    };

                    painter.rect_filled(bar_rect, 0.0, color);
                }
            });
        });

        // Sorting logic (Bubble Sort)
        if self.sorting {
            'outer: loop {
                let mut sorted = true;
                for i in 0..(self.data.len() - 1) {
                    if self.data[i] > self.data[i + 1] {
                        self.data.swap(i, i + 1);

                        self.swaped.0 = i;
                        self.swaped.1 = i + 1;
                        ctx.request_repaint(); // Why doesn't it update?

                        sorted = false;
                    }

                    thread::sleep(Duration::from_millis(1));
                }

                if sorted {
                    break 'outer;
                }
            }

            self.sorting = false;
            ctx.request_repaint();
        }
    }
}
