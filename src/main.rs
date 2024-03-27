fn main() {
    let native_option = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        initial_window_size: Some(egui::Vec2 {x:400.0, y:200.0}),
        ..Default::default()
    };
    let _ = eframe::run_native("My egui App", native_option, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    pub value:usize,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            value: 0,
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.spacing();

            let msg = format!("click {} times.", self.value);
            let label_text = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_text);
            ui.add(label);

            ui.separator();

            let btn_txt = egui::RichText::new("Click!").font(egui::FontId::proportional(24.0));
            let btn = egui::Button::new(btn_txt);
            let resp = ui.add_sized(egui::Vec2 {x:150.0, y:40.0}, btn);
            if resp.clicked() {
                self.value += 1;
            }
        });
    }
}
