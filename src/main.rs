use std::collections::HashSet;
use eframe::{egui, NativeOptions};
use eframe::egui::{Key, PointerButton, RawInput};
use winit::event::{Event, WindowEvent, ElementState};
use egui::{ViewportBuilder, Vec2, Pos2, Visuals};

fn main() -> eframe::Result {
    let viewport = ViewportBuilder {
        inner_size: Some(Vec2{x: 800., y: 450.}),
        position: Some(Pos2{x:0., y:0.}),
        ..Default::default()
    };

    let options = NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native("My egui App", options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))))
}

#[derive(Default)]
struct MyEguiApp {
    last_key: String,
    mouse_pos: Pos2
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            last_key: String::from("NONE"),
            ..Default::default()
        }
    }
    fn handle_key_down(&mut self, keys: HashSet<Key>) {
        if keys.contains(&Key::W) {
            self.last_key =  String::from("W");
        }
        if keys.contains(&Key::A) {
            self.last_key =  String::from("A");
        }
        if keys.contains(&Key::S) {
            self.last_key =  String::from("S");
        }
        if keys.contains(&Key::D) {
            self.last_key =  String::from("D");
        }
    }

    fn handle_mouse(&mut self, pointer: egui::PointerState) {
        if let Some(a) = pointer.hover_pos() {
            self.mouse_pos = a;
        }
    }

}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let input = ctx.input(|i| i.clone());
        self.handle_key_down(input.keys_down);
        self.handle_mouse(input.pointer);

        egui::TopBottomPanel::top("main").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            })
        });
        egui::SidePanel::right("right1").min_width(200.).show(ctx, |ui| {
            ui.label(format!("last key is {}", &self.last_key));
            ui.label(format!("mouse pos: {}", &self.mouse_pos));
        });
        egui::CentralPanel::default().show(ctx, |_ui| {
            //
        });
    }



}