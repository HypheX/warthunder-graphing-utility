



mod best_fit;
mod graph;

use eframe::egui::{self, Color32, Vec2b};
use egui_plot::{AxisHints, HPlacement, Line, Placement, Plot, PlotBounds, PlotPoints, VPlacement};
use graph::{load_factor, range_iterator, turn_radius, turn_rate_of_load_factor, turn_rate_of_turn_radius};

fn main() {

    

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Warthunder EM Graph", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct MyEguiApp {
    initialized: bool,
}

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


        let load_factor_curves: Vec<Line> = (1..=10).map(
            |load_factor| {
                Line::new(
                    range_iterator(0.0, 1600.0, 1000)
                    .map(|x| [x, turn_rate_of_load_factor(x, load_factor as f64)])
                    .filter(|[true_airspeed, turn_rate]| turn_radius(*true_airspeed, *turn_rate) >= 100.0)
                    .collect::<PlotPoints>()
                )
            }
        ).collect();

        let turn_radius_lines: Vec<Line> = [100, 150, 200, 250, 300, 400, 500, 700, 1000].into_iter().map(
            |turn_radius| {
                Line::new(
                    range_iterator(0.0, 1600.0, 1000)
                    .map(|true_airspeed| [true_airspeed, turn_rate_of_turn_radius(true_airspeed, turn_radius as f64)])
                    .filter(|[true_airspeed, turn_rate]| load_factor(*true_airspeed, *turn_rate) >= 1.0)
                    .filter(|[true_airspeed, turn_rate]| load_factor(*true_airspeed, *turn_rate) <= 10.0)
                    .collect::<PlotPoints>()
                )
            }
        ).collect();

        
        //let line2 = Line::new(lf);
        

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("La-7");
            });
            Plot::new("my_plot")
                /*.view_aspect(2.0)*/
                .custom_x_axes(vec![
                    AxisHints::new_x().label("TAS [km/h]"),
                    AxisHints::new_x().label("IAS [km/h]").placement(VPlacement::Top),
                ])
                .custom_y_axes(vec![
                    AxisHints::new_y().label("Turn Rate [deg/s]"),
                ])
                .show(ui, |plot_ui| {
                    //plot_ui.line(line);
                    
                    if !self.initialized {  //only set bounds once
                        plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0,0.0], [1600.0, 60.0]));
                        self.initialized = true;
                    }

                    load_factor_curves
                        .into_iter()
                        .for_each(|curve| 
                            plot_ui.line(curve.color(Color32::DARK_RED))
                        );

                    turn_radius_lines
                        .into_iter()
                        .for_each(|curve| 
                            plot_ui.line(curve.color(Color32::BLUE))
                        );
                });
        });

        
    }
}