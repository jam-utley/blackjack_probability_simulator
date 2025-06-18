use eframe::egui::{self, ComboBox};
use eframe::{run_native, App, CreationContext, NativeOptions};

struct MyApp {
    selected_option: String,
    options: Vec<String>,
    non_string: i32,
    non_string_vec: Vec<String>,
    selected_non_string: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_option: "Option 1".to_string(),
            options: vec!["Option 1".into(), "Option 2".into(), "Option 3".into()],
            non_string: 42,
            non_string_vec: vec!["one".into(), "two".into(),"three".into()],
            selected_non_string: "one".to_string(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Choose an option:");

            ComboBox::from_label("Options")
                .selected_text(&self.selected_option)
                .show_ui(ui, |ui| {
                    for option in &self.options {
                        ui.selectable_value(&mut self.selected_option, option.clone(), option);
                    }
                });
            ComboBox::from_label("Numbers")
                .selected_text(&self.selected_non_string)
                .show_ui(ui, |ui| {
                    for non_string_vec in &self.non_string_vec {
                        ui.selectable_value(&mut self.selected_non_string, non_string_vec.clone(), non_string_vec);
                    }
                });
            ui.separator();
            ui.label(format!("You selected: {}", self.selected_option));
            ui.label(format!("Your number is: {}", self.non_string));
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native(
        "ComboBox with egui",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}
