use eframe::egui::{self, ComboBox};
use eframe::{run_native, App, CreationContext, NativeOptions};

struct BlackjackAid {
    player: Vec<String>, //Picks between player and the dealer
    selected_player: String,
    selected_suit: String,
    suit: Vec<String>,
    card_number: Vec<String>,
    selected_number: String,
    recorded_cards: String,
}

impl Default for BlackjackAid {
    fn default() -> Self {
        Self {
            player: vec!["Dealer".into(), "Player".into()],
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
            recorded_cards: String::new(),
        }
    }
}

impl App for BlackjackAid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                //appends selected number and suit to a rolling string of values
                self.recorded_cards +=
                    &format!("the {} of {}\n", self.selected_number, self.selected_suit)
                        .to_string();
            }
            ui.separator();

            ui.label(format!("You selected:"));
            ui.label(format!("{}", self.recorded_cards));
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
