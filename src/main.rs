use std::collections::HashMap; //hashmap used to store images that are from a file 
use std::path::Path;

use eframe::egui::{self, ColorImage, TextureHandle, TextureOptions};

fn load_texture(ctx: &egui::Context, path: &str) -> Option<TextureHandle> { //loads png and turns into texture to be read by UI
    let img = image::open(Path::new(path)).ok()?;
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8().into_raw();
    let color_img = ColorImage::from_rgba_unmultiplied(size, &rgba);
    Some(ctx.load_texture(path, color_img, TextureOptions::default()))
}

struct CardClickApp {
    textures: HashMap<String, TextureHandle>, //stores loaded images so they don't reload everytime
    clicked: Option<String>, //stores which card is clicked
}

impl Default for CardClickApp { //defining what happens when app is first started 
    fn default() -> Self {
        Self {
            textures: HashMap::new(), //empty image cache
            clicked: None, //nothing clicked yet
        }
    }
}

impl eframe::App for CardClickApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //drawing window frame
        egui::CentralPanel::default().show(ctx, |ui| { //establishing title
            ui.heading("Click a card");

            let cards = vec!["10_of_hearts", "ace_of_spades", "2_of_diamonds", "king_of_hearts"]; //choosing which cards to display

            for name in &cards { //looping through card names checking to make sure loaded into cache
                let path = format!("assets/{}.png", name);
                if !self.textures.contains_key(&path) {
                    if let Some(tex) = load_texture(ctx, &path) {
                        self.textures.insert(path.clone(), tex);
                    }
                }
            }

            ui.horizontal(|ui| { //displaying cards
                for name in &cards {
                    let path = format!("assets/{}.png", name); //pulling from file of pngs
                    if let Some(tex) = self.textures.get(&path) {
                        if ui
                            .add(egui::ImageButton::new(
                                egui::Image::new(tex).fit_to_exact_size(egui::vec2(80.0, 120.0)),
                            ))
                            .clicked() //letting user click card
                        {
                            self.clicked = Some(name.to_string());
                        }
                    }
                }
            });

            if let Some(card) = &self.clicked {
                ui.label(format!("You clicked: {}", card)); //once clicked displays name 
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .into(),
        ..Default::default()
    };

    eframe::run_native(
        "Card Clicker",
        options,
        Box::new(|_cc| Box::<CardClickApp>::default()),
    )
}
