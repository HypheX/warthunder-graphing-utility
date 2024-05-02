



mod best_fit;
//mod iterators;
mod graph;

use eframe::egui;
use egui_plot::{AxisHints, HPlacement, Line, Placement, Plot, PlotPoints};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Warthunder EM Graph", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        let sin: PlotPoints = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        }).collect();
        let line = Line::new(sin);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("La-7");
            });
            Plot::new("my_plot")
                /*.view_aspect(2.0)*/
                .custom_x_axes(vec![
                    AxisHints::new_x().label("IAS [km/h]"),
                ])
                .custom_y_axes(vec![
                    AxisHints::new_y().label("Turn Rate [deg/s]"),
                ])
                .show(ui, |plot_ui| plot_ui.line(line));
        });

        
    }
}