use eframe::egui::{self, ComboBox};
use eframe::{App, CreationContext, NativeOptions, run_native};

//things to do:
//Add Hit Button to draw a new card ?
//Add Stand Button ?
//Add Section for probabilities

struct BlackjackAid {
    player: Vec<String>, //Picks between player and the dealer
    selected_player: String,
    selected_suit: String,
    suit: Vec<String>,
    card_number: Vec<String>,
    selected_number: String,
    recorded_cards_dealer: String,
    recorded_cards_player1: String,
    bjp: BlackjackProbabilities,
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

impl Default for BlackjackAid {
    fn default() -> Self {
        Self {
            player: vec!["Dealer".into(), "Player 1".into()],
            selected_player: "Please choose a player".to_string(),
            selected_suit: "Please select a suit".to_string(),
            suit: vec![
                "Hearts".into(),
                "Spades".into(),
                "Diamonds".into(),
                "Clubs".into(),
            ],
            card_number: vec![
                "one".into(),
                "two".into(),
                "three".into(),
                "four".into(),
                "five".into(),
                "six".into(),
                "seven".into(),
                "eight".into(),
                "nine".into(),
                "ten".into(),
                "jack".into(),
                "queen".into(),
                "king".into(),
                "ace".into(),
            ],
            selected_number: "Please select a number".to_string(),
            recorded_cards_dealer: String::new(),
            recorded_cards_player1: String::new(),
            bjp: BlackjackProbabilities{
                prob_bust: 0.0,
                prob_next_blackjack: 0.0,
                prob_win_by_stand: 0.0,
        },
        }
    }
}

impl App for BlackjackAid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let visuals = egui::Visuals {
            //sets background ccolor for dropown menus and windows, not the entire page
            window_fill: egui::Color32::from_rgb(10, 10, 40),
            ..egui::Visuals::dark() //Starts from dark theme
        };

        ctx.set_visuals(visuals);

        //creates floating window. anchored at top right, offset of -5.0,5.0
        egui::Window::new("Dealer's Hand")
            .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(format!("{}", self.recorded_cards_dealer))
                        .color(egui::Color32::BLUE),
                );
            });
        egui::Window::new("Player's Hand")
            .anchor(egui::Align2::RIGHT_TOP, [-5.0, 100.0])
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(format!("{}", self.recorded_cards_player1))
                        .color(egui::Color32::RED),
                );
            });
        egui::Window::new("Probabilities")
            .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, 5.0])
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(format!("Probability of Bust: {}%", self.bjp.prob_bust))
                        .color(egui::Color32::from_rgb(158, 101, 186)),
                );
                ui.label(
                    egui::RichText::new(format!("Probability of Immediate BLackjack: {}%", self.bjp.prob_next_blackjack))
                        .color(egui::Color32::from_rgb(158, 101, 186)),
                );
                ui.label(
                    egui::RichText::new(format!("Probability of Winning by Standing: {}%", self.bjp.prob_win_by_stand))
                        .color(egui::Color32::from_rgb(158, 101, 186)),
                );
            });

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
                        for card_number in &self.card_number {
                            ui.selectable_value(
                                &mut self.selected_number,
                                card_number.clone(),
                                card_number,
                            );
                        }
                    });
                if ui.button("Add").clicked() {
                    if self.selected_player == "Dealer" {
                        //appends selected number and suit to a rolling string of values
                        self.recorded_cards_dealer +=
                            &format!("the {} of {}\n", self.selected_number, self.selected_suit)
                                .to_string();
                    }
                    if self.selected_player == "Player 1" {
                        //appends selected number and suit to a rolling string of values
                        self.recorded_cards_player1 +=
                            &format!("the {} of {}\n", self.selected_number, self.selected_suit)
                                .to_string();
                    }
                }
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Reset Dealer").clicked() {
                        self.recorded_cards_dealer = String::new();
                    }
                    if ui.button("Reset Player 1").clicked() {
                        self.recorded_cards_player1 = String::new();
                    }
                    if ui.button("New Game").clicked() {
                        self.recorded_cards_dealer = String::new();
                        self.recorded_cards_player1 = String::new();
                    }
                });

                ui.label(format!("You selected:"));
                //sets this text color different
                ui.label(
                    egui::RichText::new(format!("{}", self.recorded_cards_dealer))
                        .color(egui::Color32::BLUE),
                );
                ui.label(
                    egui::RichText::new(format!("{}", self.recorded_cards_player1))
                        .color(egui::Color32::RED),
                );
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
