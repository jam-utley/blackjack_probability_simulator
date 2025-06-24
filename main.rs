use eframe::egui::{self, ColorImage, ComboBox, TextureHandle, TextureOptions};
use eframe::{App, NativeOptions};
use std::collections::HashMap;
use std::path::Path;
use egui::ViewportBuilder;
//things to do:
//finish card counting function
//add New Round button
//incorporate probability functions

fn load_texture(ctx: &egui::Context, path: &str) -> Option<TextureHandle> {
    println!("Loading image: {}", path);
    let img = image::open(Path::new(path)).ok()?;
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8().into_raw();
    let color_img = ColorImage::from_rgba_unmultiplied(size, &rgba);
    Some(ctx.load_texture(path, color_img, TextureOptions::default()))
}

fn hand_total(input: Vec<String>) -> i32 {
    let mut output = Vec::new();
    let mut hand_value: i32 = 0;
    let mut ace_in_hand: bool = false;
    let mut number_of_aces: i8 = 0;
    let converter = StringToInt::new();
    for mut i in input {
        if i == "ace".to_string() {
            i = "ace_high".to_string();
            number_of_aces += 1;
            ace_in_hand = true;
        }
        match converter.get_value(i.as_str()) {
            Some(value) => output.push(value),
            None => println!("Failed to convert: invalid input"),
        }
    }

    for j in output {
        hand_value += j;
    }
    if ace_in_hand {
        'ace_conversion: for _k in 1..=number_of_aces {
            if hand_value > 21 {
                hand_value -= 10; //converts high ace to a low ace
            } else {
                break 'ace_conversion;
            }
        }
    }
    return hand_value;
}

//fn generate_all_cards(suits: &Vec<String>, numbers: &Vec<String>) {
//THIS MUST BE FINISHED
// for i in suits{
// for j in numbers{
//  BlackjackAid::default().cards_remaining.push([i as usize],[j as usize])};//.push([i as usize][j as usize]);
// }
// }

fn probability_dealer_win(
    curr_player1_hand: i32,
    card_vals: &Vec<i32>,
    card_counts: &Vec<i32>,
    curr_dealer_hand: i32,
) -> f64 {
    //Args
    //'curr_player1_hand' - total current player1 hand
    //'card_Vals' - vector of card values with faces and numbers
    //'card_counts' - 'card counts which holds how many total cards in the vector remaining
    //'curr_dealer_hand` - total amount of current dealer

    //check if dealer busts if current dealer hand
    if curr_dealer_hand > 21 {
        //println!("{:?}",curr_dealer_hand);
        return 0.0;
    }
    //check if dealer stand if current dealer <=17 and less than or equal to 21
    if curr_dealer_hand >= 17 && curr_dealer_hand <= 21 {
        if curr_dealer_hand > curr_player1_hand {
            //check if current dealer hand is greater than players hand then return 1.0 for the weight probability
            //println!("Dealer wins with {} vs {}", curr_dealer_hand, curr_hand);
            return 1.0;
        } else {
            //            println!("Dealer stands with {} — not enough to beat {}", curr_dealer_hand, curr_hand);
            return 0.0; //else return 0.0
        }
    }
    let total_remaining_deck: i32 = card_counts.iter().sum(); //sum all remaining decks
    let mut win_prob: f64 = 0.0;
    for (i, &_val) in card_counts.iter().enumerate() {
        //loop through each remaining card if exists in card_count vector deck
        if card_counts[i] == 0 {
            continue;
        }
        let mut next_card_count: Vec<i32> = card_counts.clone(); //create clone to prevent mutate globally
        next_card_count[i] -= 1;
        let curr_prob: f64 = card_counts[i] as f64 / total_remaining_deck as f64; //calculate current probability
        let next_total_hand: i32 = card_vals[i] + curr_dealer_hand; //sum the total value of the next dealer hand
        win_prob += curr_prob
            * probability_dealer_win(
                curr_player1_hand,
                &card_vals,
                &next_card_count,
                next_total_hand,
            );
    }
    return win_prob;
}

struct StringToInt {
    ace_low: i32,
    two: i32,
    three: i32,
    four: i32,
    five: i32,
    six: i32,
    seven: i32,
    eight: i32,
    nine: i32,
    jack: i32,
    queen: i32,
    king: i32,
    ace_high: i32,
}

impl StringToInt {
    fn new() -> Self {
        Self {
            ace_low: 1,
            two: 2,
            three: 3,
            four: 4,
            five: 5,
            six: 6,
            seven: 7,
            eight: 8,
            nine: 9,
            jack: 10,
            queen: 10,
            king: 10,
            ace_high: 11,
        }
    }
    fn get_value(&self, name: &str) -> Option<i32> {
        match name {
            "ace_low" => Some(self.ace_low),
            "2" => Some(self.two),
            "3" => Some(self.three),
            "4" => Some(self.four),
            "5" => Some(self.five),
            "6" => Some(self.six),
            "7" => Some(self.seven),
            "8" => Some(self.eight),
            "9" => Some(self.nine),
            "jack" => Some(self.jack),
            "queen" => Some(self.queen),
            "king" => Some(self.king),
            "ace_high" => Some(self.ace_high),
            _ => None,
        }
    }
}

struct BlackjackProbabilities {
    prob_bust: f64,
    prob_next_blackjack: f64,
    prob_win_by_stand: f64,
    prob_dealer_wins: f64,
}

impl Default for BlackjackProbabilities {
    fn default() -> Self {
        Self {
            prob_bust: 0.0,
            prob_next_blackjack: 0.0,
            prob_win_by_stand: 0.0,
            prob_dealer_wins: 0.0,
        }
    }
}

struct BlackjackAid {
    player: Vec<String>, //Picks between player and the dealer
    selected_player: String,
    selected_suit: String,
    selected_number: String,
    suit: Vec<String>,
    card_number: Vec<String>,
    recorded_cards_dealer: Vec<String>,
    recorded_cards_player1: Vec<String>,
    dealer_card_ids: Vec<String>,
    player1_card_ids: Vec<String>,
    player1_hand_total: i32,
    dealer_hand_total: i32,
    cards_remaining: Vec<i32>,
    bjp: BlackjackProbabilities,
    textures: HashMap<String, TextureHandle>,

    dogs: Vec<Puppies>,
    dog_texture: Option<TextureHandle>,
    visuals_dogs: bool,
    selected_dog: usize,
    visuals_set: bool,

    poker_table: Option<TextureHandle>,
}

struct Puppies {
    x: f32,
    y: f32,
    frame_width: usize,
    frame_height: usize,
    total_frames: usize,
    current_frame: usize,
    frame_timer: f32,
    sprite_sheet: TextureHandle,
}

impl Default for BlackjackAid {
    fn default() -> Self {
        Self {
            poker_table: None,
            player: vec!["Dealer".into(), "Player 1".into()],
            selected_player: "Please choose a player".into(),
            selected_suit: "Please select a suit".into(),
            selected_number: "Please select a number".into(),
            suit: vec!["Hearts", "Spades", "Diamonds", "Clubs"]
                .into_iter()
                .map(String::from)
                .collect(),
            card_number: vec![
                "2", "3", "4", "5", "6", "7", "8", "9", "10", "jack", "queen", "king", "ace",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            recorded_cards_dealer: Vec::new(),
            recorded_cards_player1: Vec::new(),
            dealer_card_ids: vec![],
            player1_card_ids: vec![],
            player1_hand_total: 0,
            dealer_hand_total: 0,
            cards_remaining: Vec::new(),
            bjp: BlackjackProbabilities::default(),
            textures: HashMap::new(),
            dog_texture: None,
            visuals_dogs: false,
            selected_dog: 0,
            visuals_set: false,
            dogs: Vec::new(),
        }
    }
}

impl BlackjackAid {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut instance = Self::default();

        // Load poker_table texture here
        let ctx = &cc.egui_ctx;
        let image = image::load_from_memory(include_bytes!("../assets/poker_table.png"))
            .expect("Failed to load image")
            .to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_vec();

        let texture = cc.egui_ctx.load_texture(
            "poker_table",
            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
            Default::default(),
        );

        instance.poker_table = Some(texture);

        instance
    }
}

impl App for BlackjackAid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let visuals = egui::Visuals {
            //sets background color for dropown menus and windows, not the entire page
            window_fill: egui::Color32::from_rgb(10, 10, 40),
            ..egui::Visuals::dark()
        };

        if self.dog_texture.is_none() {
            let image = image::open("assets/spritesheet_white.png")
                .unwrap()
                .to_rgba8();
            let size = [image.width() as usize, image.height() as usize];
            let pixels = image.into_raw();

            let texture = ctx.load_texture(
                "white_dog",
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                egui::TextureOptions::NEAREST,
            );

            self.dog_texture = Some(texture.clone());

            self.dogs.push(Puppies {
                x: 100.0,
                y: 100.0,
                sprite_sheet: texture,
                frame_width: 74, // CHANGE if your frame size is different
                frame_height: 128,
                total_frames: 28, // CHANGE based on how many frames in your sprite sheet
                current_frame: 0,
                frame_timer: 0.0,
            });
        }

        let visuals = egui::Visuals {
            window_fill: egui::Color32::from_rgb(10, 10, 40),
            ..egui::Visuals::dark()
        };

        let mut delta_x = 0.0;
        let mut delta_y = 0.0;

        if self.visuals_dogs {
            for dog in &self.dogs {
                // draw dog sprite
            }
        }

        if ctx.input(|i| i.key_down(egui::Key::ArrowRight)) {
            delta_x += 5.0;
        }
        if ctx.input(|i| i.key_down(egui::Key::ArrowLeft)) {
            delta_x -= 5.0;
        }
        if ctx.input(|i| i.key_down(egui::Key::ArrowDown)) {
            delta_y += 5.0;
        }
        if ctx.input(|i| i.key_down(egui::Key::ArrowUp)) {
            delta_y -= 5.0;
        }

        if let Some(puppy) = self.dogs.get_mut(self.selected_dog) {
            puppy.x += delta_x;
            puppy.y += delta_y;
        }

        if let Some(dog) = self.dogs.get_mut(self.selected_dog) {
            dog.x = (dog.x + delta_x).clamp(0.0, ctx.screen_rect().max.x - 32.0);
            dog.y = (dog.y + delta_y).clamp(0.0, ctx.screen_rect().max.y - 32.0);
        }

        if !self.visuals_set {
            ctx.set_visuals(visuals);
            self.visuals_set = true;
        }

        egui::Window::new("Probabilities")
            .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
            .show(ctx, |ui| {
                ui.label(format!("Probability of Bust: {:.1}%", self.bjp.prob_bust));
                ui.label(format!(
                    "Probability of Immediate Blackjack: {:.1}%",
                    self.bjp.prob_next_blackjack
                ));
                ui.label(format!(
                    "Probability of Winning by Standing: {:.1}%",
                    self.bjp.prob_win_by_stand
                ));
            });

        //Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();

            // 1. Try drawing image
            if let Some(tex) = &self.poker_table {
                ui.label("✅ Texture is loaded. Drawing image.");
                ui.painter()
                    .image(tex.id(), rect, rect, egui::Color32::WHITE);
            } else {
                ui.label("❌ Texture not loaded.");
            }

            // 2. Green overlay with transparency (optional)
            let overlay = egui::Color32::from_rgba_unmultiplied(40, 110, 31, 160);
            ui.painter().rect_filled(rect, 0.0, overlay);

            // 3. Add dummy UI
            ui.label("UI overlays here.");
        });
    }
}

fn display_card(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    card: &str,
    textures: &mut HashMap<String, TextureHandle>,
) {
    let path = format!("assets/{}.png", card);

    if !textures.contains_key(&path) {
        if let Some(tex) = load_texture(ctx, &path) {
            textures.insert(path.clone(), tex);
        } else {
            ui.label(format!("PNG not found: {}", card));
            return;
        }
    }

    if let Some(tex) = textures.get(&path) {
        let frame = egui::Frame::default()
            .fill(egui::Color32::WHITE)
            .inner_margin(egui::Margin::same(1.0))
            .rounding(egui::Rounding::same(5.0))
            .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK));
        frame.show(ui, |ui| {
            ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(80.0, 110.0)));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 750.0]) //initial window size when opened
            .with_resizable(true), //allows window to be resizable
        ..Default::default()
    };
    eframe::run_native(
        "Blackjack Assistant",
        options,
        Box::new(|cc| Box::new(BlackjackAid::new(cc))),
    );
}

