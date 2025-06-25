use egui::{
    self, Align2, Color32, ColorImage, Context, FontId, Pos2, RichText, TextureHandle,
    TextureOptions, Window,
};

use eframe::{egui::FontData, egui::FontDefinitions, egui::FontFamily};
use eframe::{run_native, App, Frame, NativeOptions};
use egui::ComboBox;
use egui::Vec2;
use egui::ViewportBuilder;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashMap;

struct FallingSymbol {
    pos: Pos2,
    velocity: f32,
    symbol: char,
    color: Color32,
}

//on start menu make button for go to simulator or go to helper
//return buttons

fn load_texture(ctx: &egui::Context, path: &str) -> Option<TextureHandle> {
    let image_data = std::fs::read(path).ok()?;
    let image = image::load_from_memory(&image_data).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(path, color_image, TextureOptions::LINEAR))
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let jackbot_font = FontData::from_static(include_bytes!("../resources/KarmaFuture.ttf"));

    // Create a font definitions object
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert("Block BRK".to_owned(), jackbot_font);

    // Tell egui to use "jackbot" for this font family name
    fonts.families.insert(
        FontFamily::Name("jackbot".into()),
        vec!["jackbot".to_owned()],
    );

    // Apply the font definitions to the context
    ctx.set_fonts(fonts);
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
    ten: i32,
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
            ten: 10,
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
            "10" => Some(self.ten),
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
    start_screen: bool,
    textures: HashMap<String, TextureHandle>,
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
    number_of_decks: i32,
    cards_remaining: Vec<i32>,
    bjp: BlackjackProbabilities,
    frame_count: usize,
    falling_symbols: Vec<FallingSymbol>,
    secret_pop: bool,
    initialized: bool,
    forbidden_cards_sim: Vec<(i32, i32)>, //stores the cards pulled in the blackjack simulator
                                          // stats: SimulatorStats,
}

impl Default for BlackjackAid {
    fn default() -> Self {
        let number_of_decks = 1;
        let mut rng = rand::thread_rng();
        let symbols = vec!['♠', '♥', '♦', '♣'];
        let colors = vec![
            Color32::BLACK,
            Color32::from_rgb(220, 20, 60), // red hearts
            Color32::from_rgb(220, 20, 60), // red diamonds
            Color32::BLACK,
        ];

        let mut rng = rand::thread_rng();
        let falling_symbols = (0..30)
            .map(|_| {
                let idx = rng.gen_range(0..symbols.len());
                FallingSymbol {
                    pos: Pos2::new(rng.gen_range(0.0..800.0), rng.gen_range(0.0..600.0)),
                    velocity: rng.gen_range(1.0..3.5),
                    symbol: symbols[idx],
                    color: colors[idx],
                }
            })
            .collect();

        Self {
            initialized: false,
            secret_pop: false,
            start_screen: true,
            falling_symbols,
            frame_count: 0,
            textures: HashMap::new(),
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
            number_of_decks,
            cards_remaining: vec![4 * number_of_decks; 13],
            bjp: BlackjackProbabilities::default(),
            forbidden_cards_sim: Vec::new(),
            // stats: SimulatorStats::default(),
        }
    }
}

impl App for BlackjackAid {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Setup fonts and initialization
        if !self.initialized {
            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "block_brk".to_owned(),
                FontData::from_owned(include_bytes!("../resources/KarmaFuture.ttf").to_vec()),
            );

            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "block_brk".to_owned());

            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, "block_brk".to_owned());

            ctx.set_fonts(fonts);
            self.initialized = true;
        }

        let screen_rect = ctx.screen_rect();

        if self.start_screen {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(screen_rect.height() * 0.2);
                ui.vertical_centered(|ui| {
                    ui.heading("JACK-BOT");

                    ui.add_space(30.0);

                    ui.label(
                        egui::RichText::new("Need some help? We got you!")
                            .size(24.0)
                            .color(egui::Color32::LIGHT_GRAY),
                    );

                    ui.add_space(50.0);

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Play a Round").color(egui::Color32::WHITE),
                            )
                            .min_size(egui::Vec2::new(200.0, 40.0))
                            .fill(egui::Color32::from_rgb(30, 80, 30))
                            .rounding(10.0),
                        )
                        .clicked()
                    {
                        // Play logic here
                    }

                    ui.add_space(10.0);

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Get Some Help").color(egui::Color32::WHITE),
                            )
                            .min_size(egui::Vec2::new(200.0, 40.0))
                            .fill(egui::Color32::from_rgb(60, 60, 100))
                            .rounding(10.0),
                        )
                        .clicked()
                    {
                        self.start_screen = false;
                    }
                });
            });

            ctx.request_repaint();
        } else {
            // Dark theme setup
            ctx.set_visuals(egui::Visuals {
                window_fill: egui::Color32::from_rgb(10, 80, 10),
                ..egui::Visuals::dark()
            });

            let painter = ctx.layer_painter(egui::LayerId::background());

            painter.rect_filled(screen_rect, 0.0, egui::Color32::from_rgb(41, 55, 59));

            for symbol in &mut self.falling_symbols {
                symbol.pos.y += symbol.velocity;
                if symbol.pos.y > screen_rect.bottom() {
                    symbol.pos.y = 0.0;
                    symbol.pos.x = rand::thread_rng().gen_range(0.0..screen_rect.right());
                }

                painter.text(
                    symbol.pos,
                    egui::Align2::CENTER_CENTER,
                    symbol.symbol,
                    egui::FontId::proportional(24.0),
                    symbol.color,
                );
            }

            egui::Area::new("button".into())
                .anchor(Align2::RIGHT_BOTTOM, [-10.0, -10.0])
                .show(ctx, |ui| {
                    if ui.button(RichText::new("♥").size(24.0)).clicked() {
                        self.secret_pop = !self.secret_pop;
                    }
                });

            if self.secret_pop {
                Window::new("secret")
                    .resizable(false)
                    .collapsible(false)
                    .anchor(Align2::RIGHT_BOTTOM, [-100.0, -100.0])
                    .show(ctx, |ui| ui.label("secret plan"));
            }

            egui::Window::new("Controls")
                .anchor(egui::Align2::LEFT_TOP, [15.0, 20.0])
                .resizable(false)
                .collapsible(true)
                .show(ctx, |ui| {
                    self.show_card_selection_ui(ui);
                    self.show_reset_buttons(ui);
                    self.show_card_display_sections(ui, ctx);
                });

            self.show_probabilities_window(ctx);

            ctx.request_repaint();
        }
    }
}

impl BlackjackAid {
    fn setup_custom_fonts(ctx: &egui::Context) {
        let jackbot_font = FontData::from_static(include_bytes!("../resources/KarmaFuture.ttf"));

        let mut fonts = FontDefinitions::default();

        // Add your custom font data
        fonts.font_data.insert("jackbot".to_owned(), jackbot_font);

        // Use it for the Proportional family (or Name-based)
        fonts.families.insert(
            FontFamily::Name("jackbot".into()),
            vec!["jackbot".to_owned()],
        );

        ctx.set_fonts(fonts);
    }
    fn show_probabilities_window(&self, ctx: &egui::Context) {
        egui::Window::new("Probabilities")
            .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
            .show(ctx, |ui| {
                ui.label(format!("Probability of Bust: {:.2}%", self.bjp.prob_bust));
                ui.label(format!(
                    "Probability of Immediate Blackjack: {:.1}%",
                    self.bjp.prob_next_blackjack
                ));
                ui.label(format!(
                    "Probability of Winning by Standing: {:.1}%",
                    self.bjp.prob_win_by_stand
                ));
                ui.label(format!(
                    "Probability of Dealer Wins if You Stand: {:.1}%",
                    self.bjp.prob_dealer_wins
                ));
            });
    }

    fn show_card_selection_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Choose a card:");

        ComboBox::from_label("Player/Dealer")
            .selected_text(&self.selected_player)
            .show_ui(ui, |ui| {
                for player in &self.player {
                    ui.selectable_value(&mut self.selected_player, player.clone(), player);
                }
            });

        ComboBox::from_label("Suit")
            .selected_text(&self.selected_suit)
            .show_ui(ui, |ui| {
                for suit in &self.suit {
                    ui.selectable_value(&mut self.selected_suit, suit.clone(), suit);
                }
            });

        ComboBox::from_label("Number")
            .selected_text(&self.selected_number)
            .show_ui(ui, |ui| {
                for number in &self.card_number {
                    ui.selectable_value(&mut self.selected_number, number.clone(), number);
                }
            });

        if ui.button("Add Card").clicked() {
            if self.selected_player != "Please choose a player"
                && self.selected_suit != "Please select a suit"
                && self.selected_number != "Please select a number"
            {
                let card_id = format!(
                    "{}_of_{}",
                    self.selected_number.to_lowercase(),
                    self.selected_suit.to_lowercase()
                );

                match self.selected_player.as_str() {
                    "Dealer" => {
                        self.recorded_cards_dealer
                            .push(self.selected_number.clone());
                        self.dealer_card_ids.push(card_id);
                    }
                    "Player 1" => {
                        self.recorded_cards_player1
                            .push(self.selected_number.clone());
                        self.player1_card_ids.push(card_id);
                    }
                    _ => {}
                }
            }

            if self.recorded_cards_dealer.len() >= 1 && self.recorded_cards_player1.len() >= 2 {
                println!("Computing probabilities!");
                self.bjp.prob_dealer_wins = self.probability_dealer_win(
                    self.player1_hand_total,
                    &self.cards_remaining,
                    self.dealer_hand_total,
                ) * 100.0;

                self.bjp.prob_win_by_stand = (1.0 - (self.bjp.prob_dealer_wins / 100.0)) * 100.0;
                self.bjp.prob_bust = self.probability_busting(self.player1_hand_total) * 100.0;
            }
        }
    }
    fn probability_busting(&self, curr_hand: i32) -> f64 {
        let bust_number = 21 - curr_hand;
        let mut bust_cards_sum = 0;

        for i in (bust_number + 1)..(self.cards_remaining.len() as i32) {
            bust_cards_sum += self.cards_remaining[i as usize];
        }

        let cards_remaining_in_deck: i32 = self.cards_remaining.iter().sum();

        bust_cards_sum as f64 / cards_remaining_in_deck as f64
    }

    fn probability_dealer_win(
        &self,
        curr_hand: i32,
        card_counts: &Vec<i32>,
        curr_dealer_hand: i32,
    ) -> f64 {
        let card_vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];

        if curr_dealer_hand > 21 {
            return 0.0;
        }
        if curr_dealer_hand >= 17 && curr_dealer_hand <= 21 {
            return if curr_dealer_hand > curr_hand {
                1.0
            } else {
                0.0
            };
        }

        let total_remaining_deck: i32 = card_counts.iter().sum();
        let mut win_prob: f64 = 0.0;

        for (i, &count) in card_counts.iter().enumerate() {
            if count == 0 || i >= card_vals.len() {
                continue;
            }

            let draw = card_vals[i];
            let mut next_total_hand = curr_dealer_hand + draw;
            if draw == 11 && next_total_hand > 21 {
                next_total_hand -= 10;
            }

            let mut next_card_counts = card_counts.clone();
            next_card_counts[i] -= 1;

            let prob = count as f64 / total_remaining_deck as f64;
            win_prob +=
                prob * self.probability_dealer_win(curr_hand, &next_card_counts, next_total_hand);
        }

        win_prob
    }

    fn show_reset_buttons(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Reset Dealer").clicked() {
                self.recorded_cards_dealer.clear();
                self.dealer_card_ids.clear();
            }
            if ui.button("Reset Player").clicked() {
                self.recorded_cards_player1.clear();
                self.player1_card_ids.clear();
            }
            if ui.button("New Round").clicked() {
                self.recorded_cards_dealer.clear();
                self.recorded_cards_player1.clear();
                self.dealer_card_ids.clear();
                self.player1_card_ids.clear();
                self.bjp = BlackjackProbabilities::default();
            }
            if ui.button("New Game").clicked() {
                self.recorded_cards_dealer.clear();
                self.recorded_cards_player1.clear();
                self.dealer_card_ids.clear();
                self.player1_card_ids.clear();
                self.cards_remaining = vec![4 * self.number_of_decks; 13];
                self.bjp = BlackjackProbabilities::default();
            }
        });
    }

    fn show_card_display_sections(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.separator();
        ui.label("Player Cards:");
        ui.horizontal(|ui| {
            for card_id in &self.player1_card_ids {
                display_card(ui, ctx, card_id, &mut self.textures);
            }
        });
        self.player1_hand_total = hand_total(self.recorded_cards_player1.clone());
        ui.label(format!("Hand Total = {}", self.player1_hand_total));

        ui.separator();
        ui.label("Dealer Cards:");
        ui.horizontal(|ui| {
            for card_id in &self.dealer_card_ids {
                display_card(ui, ctx, card_id, &mut self.textures);
            }
        });
        self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
        ui.label(format!("Hand Total = {}", self.dealer_hand_total));
    }

    fn show_title_banner(&mut self, ctx: &egui::Context) {
        // UI on top of the background
        egui::TopBottomPanel::top("top_controls").show(ctx, |ui| {
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.heading(
                                egui::RichText::new("🃏 Blackjack Assistant")
                                    .size(32.0)
                                    .strong(),
                            );
                        },
                    );
                });
            });
            ui.add_space(10.0);
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
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    run_native(
        "Blackjack Assistant",
        options,
        Box::new(|_cc| Box::new(BlackjackAid::default())),
    );
}
