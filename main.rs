use eframe::egui::{self, ColorImage, ComboBox, TextureHandle, TextureOptions};
use eframe::{App, NativeOptions, run_native};
use std::collections::HashMap;
use std::path::Path;

//things to do:
//Add Hit Button to draw a new card ?
//Add Stand Button ?
//Add Section for probabilities

fn load_texture(ctx: &egui::Context, path: &str) -> Option<TextureHandle> {
    println!("Loading image: {}", path);
    let img = image::open(Path::new(path)).ok()?;
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8().into_raw();
    let color_img = ColorImage::from_rgba_unmultiplied(size, &rgba);
    Some(ctx.load_texture(path, color_img, TextureOptions::default()))
}

struct BlackjackProbabilities {
    prob_bust: f64,
    prob_next_blackjack: f64,
    prob_win_by_stand: f64,
}

impl Default for BlackjackProbabilities {
    fn default() -> Self {
        Self {
            prob_bust: 0.0,
            prob_next_blackjack: 0.0,
            prob_win_by_stand: 0.0,
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
    recorded_cards_dealer: String,
    recorded_cards_player1: String,
    dealer_card_ids: Vec<String>,
    player1_card_ids: Vec<String>,
    bjp: BlackjackProbabilities,
    textures: HashMap<String, TextureHandle>,
}

impl Default for BlackjackAid {
    fn default() -> Self {
        Self {
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
            recorded_cards_dealer: String::new(),
            recorded_cards_player1: String::new(),
            dealer_card_ids: vec![],
            player1_card_ids: vec![],
            bjp: BlackjackProbabilities::default(),
            textures: HashMap::new(),
        }
    }
}

impl App for BlackjackAid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let visuals = egui::Visuals {
            //sets background color for dropown menus and windows, not the entire page
            window_fill: egui::Color32::from_rgb(10, 10, 40),
            ..egui::Visuals::dark()
        };
        ctx.set_visuals(visuals);

        //creates floating window. anchored at top right, offset of -5.0,5.0
/*        egui::Window::new("Dealer's Hand")
            .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(&self.recorded_cards_dealer).color(egui::Color32::BLUE),
                );
            });

        egui::Window::new("Player's Hand")
            .anchor(egui::Align2::RIGHT_TOP, [-5.0, 100.0])
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(&self.recorded_cards_player1).color(egui::Color32::RED),
                );
            });
*/
        egui::Window::new("Probabilities")
            .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, 5.0])
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
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 110, 31))) //sets page background color
            .show(ctx, |ui| {
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

                // Add Button
                if ui.button("Add Card").clicked() {
                    if self.selected_player != "Please choose a player"
                    //appends selected number and suit to a rolling string of values
                        && self.selected_suit != "Please select a suit"
                        && self.selected_number != "Please select a number"
                    {
                        let card_id = format!(
                            "{}_of_{}",
                            self.selected_number.to_lowercase(),
                            self.selected_suit.to_lowercase()
                        );

                        let label =
                            format!("the {} of {}\n", self.selected_number, self.selected_suit);

                        match self.selected_player.as_str() {
                            "Dealer" => {
                                self.recorded_cards_dealer += &label;
                                self.dealer_card_ids.push(card_id);
                            }
                            "Player 1" => {
                                self.recorded_cards_player1 += &label;
                                self.player1_card_ids.push(card_id);
                            }
                            _ => {}
                        }
                    }
                }

                // Reset buttons
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
                    if ui.button("New Game").clicked() {
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                    }
                });

                // Display selected cards
                ui.separator();
                ui.label("Player Cards:");
                ui.horizontal(|ui| {
                    for card_id in &self.player1_card_ids {
                        display_card(ui, ctx, card_id, &mut self.textures);
                    }
                });

                ui.separator();
                ui.label("Dealer Cards:");
                ui.horizontal(|ui| {
                    for card_id in &self.dealer_card_ids {
                        display_card(ui, ctx, card_id, &mut self.textures);
                    }
                });
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
        let mut frame = egui::Frame::default()
            .fill(egui::Color32::WHITE)
            .inner_margin(egui::Margin::same(1))
            .rounding(egui::Rounding::same(5))
            .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK));
        frame.show(ui, |ui| {
            ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(80.0, 110.0)));
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native(
        "Blackjack Assistant",
        options,
        Box::new(|_cc| Ok(Box::new(BlackjackAid::default()))),
    );
}
