fn main() {
    let native_option = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        initial_window_size: Some(egui::Vec2 {x:400.0, y:200.0}),
        ..Default::default()
    };
    let _ = eframe::run_native("My egui App", native_option, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    pub value:bool,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            value: true,
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

            let msg = format!("checked = {}.", self.value);
            let label_text = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_text);
            ui.add(label);

            ui.separator();

            let check_text = egui::RichText::new("Checkbox").size(24.0);
            let check = egui::Checkbox::new(&mut self.value, check_text);
            let _resp = ui.add(check);
        });
    }
}
