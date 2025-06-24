
use eframe::egui::{self, ColorImage, ComboBox, TextureHandle, TextureOptions};
use eframe::{App, NativeOptions, run_native};
use std::collections::HashMap;
use std::path::Path;

//things to do:
//finish card counting function
//add New Round button
//incorporate probability functions (win, tie & blackjack)

fn load_texture(ctx: &egui::Context, path: &str) -> Option<TextureHandle> {
    println!("Loading image: {}", path);
    let img = image::open(Path::new(path)).ok()?;
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8().into_raw();
    let color_img = ColorImage::from_rgba_unmultiplied(size, &rgba);
    Some(ctx.load_texture(path, color_img, TextureOptions::default()))
}

fn hand_total(input: Vec<String>) -> i32 {
    let converter = StringToInt::new();
    let mut total = 0;
    let mut aces = 0;
    for card in input {
        if card == "ace" {
            total += 1;
            aces += 1;
        } else if let Some(v) = converter.get_value(&card) {
            total += v;
        }
    }
    // convert high aces
    if aces > 0 && total + 10 <= 21 {
        total + 10
    } else {
        total
    }
}



//fn to provide the probability of busting 
//Args
//'curr_hand' - total current hand
//'card_counts' - 'card counts which holds how many total cards in the vector remaining
fn probability_busting(
    curr_hand: i32,
    card_counts: &Vec<i32>,
) -> f64 {
    let card_vals = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,11];
    let mut bust: f64 = 0.0;
    let total_remaining_deck: i32 = card_counts.iter().sum();
     for (i, &val) in card_vals.iter().enumerate(){ //loop to get val and its current index 
        if card_counts[i] == 0{
            continue
        }
        let mut draw: i32 = val;
        if draw == 11 && curr_hand + val > 21{
            draw=1    //set to low ace 
        }
        if curr_hand + draw > 21{
            bust += card_counts[i] as f64 / total_remaining_deck as f64;
        }
}
    return bust;

}

/// Probability next-card gives you a blackjack (21) from your current total.
/// Probability next-card gives you a blackjack (21) from your current total.
fn probability_next_blackjack(player_total: i32, cards_remaining: &Vec<i32>) -> f64 {
    let total_cards: i32 = cards_remaining.iter().sum();
    if total_cards == 0 {
        return 0.0;
    }

    // How much you need to hit exactly 21:
    let needed = 21 - player_total;
    if needed < 1 || needed > 11 {
        return 0.0;
    }

    // Count how many cards of that value remain:
    let count = match needed {
        1 => {
            // low ace (value = 1)
            cards_remaining[0]
        }
        2..=9 => {
            // numeric cards 2–9 map to indices 1–8
            cards_remaining[(needed - 1) as usize]
        }
        10 => {
            // any ten-value: "10" at 9, J at 10, Q at 11, K at 12
            cards_remaining[9]
            + cards_remaining[10]
            + cards_remaining[11]
            + cards_remaining[12]
        }
        11 => {
            // high ace (value = 11)
            cards_remaining[13]
        }
        _ => 0,
    };

    count as f64 / total_cards as f64
}



fn probability_blackjack(num_decks: i32, card_counts: &Vec<i32>) -> f64{
    let mut total_aces: i32 = 0;
    let mut total_tens: i32 = 0;
    let total_remaining_deck: i32 = card_counts.iter().sum();
    let total_cards = total_remaining_deck as f64;
    
     let total_aces = card_counts[12];
    let total_tens = card_counts[8..=11].iter().sum::<i32>();
    if total_remaining_deck < 2 {
        return 0.0;
    }
    let prob_blackjack = 2.0 * (total_aces as f64 * total_tens as f64)
        / (total_cards * (total_cards - 1.0));

    prob_blackjack
}


/// Computes (win_prob, tie_prob) for dealer given player's total, deck counts, and dealer total.
fn probability_dealer_outcomes(
    player_total: i32,
    cards_remaining: &Vec<i32>,
    dealer_total: i32,
) -> (f64, f64) {
    if dealer_total > 21 { return (0.0, 0.0); }
    if dealer_total >= 17 {
        return if dealer_total > player_total { (1.0, 0.0) }
               else if dealer_total == player_total { (0.0, 1.0) }
               else                  { (0.0, 0.0) };
    }
    let card_vals = vec![1,2,3,4,5,6,7,8,9,10,10,10,10,11];
    let total_cards: i32 = cards_remaining.iter().sum();
    let mut win = 0.0;
    let mut tie = 0.0;
    for (i, &cnt) in cards_remaining.iter().enumerate() {
        if cnt == 0 { continue; }
        let p = cnt as f64 / total_cards as f64;
        let mut next = cards_remaining.clone();
        next[i] -= 1;
        let next_tot = dealer_total + card_vals[i];
        let (w, t) = probability_dealer_outcomes(player_total, &next, next_tot);
        win += p * w;
        tie += p * t;
    }
    (win, tie)
}

struct StringToInt {
    ace_low: i32, two: i32, three: i32, four: i32, five: i32,
    six: i32, seven: i32, eight: i32, nine: i32, ten: i32,
    jack: i32, queen: i32, king: i32, ace_high: i32,
}
impl StringToInt {
    fn new() -> Self {
        Self { ace_low:1, two:2, three:3, four:4, five:5,
               six:6, seven:7, eight:8, nine:9, ten:10,
               jack:10, queen:10, king:10, ace_high:11 }
    }
    fn get_value(&self, name: &str) -> Option<i32> {
        match name {
            "1" => Some(self.ace_low),
            "2" => Some(self.two), "3" => Some(self.three), "4" => Some(self.four),
            "5" => Some(self.five), "6" => Some(self.six), "7" => Some(self.seven),
            "8" => Some(self.eight), "9" => Some(self.nine), "10" => Some(self.ten),
            "jack" => Some(self.jack), "queen" => Some(self.queen), "king" => Some(self.king),
            "ace_high" => Some(self.ace_high), _ => None,
        }
    }
}

struct BlackjackProbabilities {
    prob_bust: f64,
    prob_next_blackjack: f64,
    prob_player_win: f64,
    prob_dealer_wins: f64,
    prob_tie: f64,
}
impl Default for BlackjackProbabilities {
    fn default() -> Self {
        Self { prob_bust:0.0, prob_next_blackjack:0.0,
               prob_player_win:0.0, prob_dealer_wins:0.0,
               prob_tie:0.0 }
    }
}

struct BlackjackAid {
    player: Vec<String>, selected_player:String,
    selected_suit:String, selected_number:String,
    suit: Vec<String>, card_number:Vec<String>,
    recorded_cards_dealer:Vec<String>, recorded_cards_player1:Vec<String>,
    dealer_card_ids:Vec<String>, player1_card_ids:Vec<String>,
    player1_hand_total:i32, dealer_hand_total:i32,
    number_of_decks:i32, cards_remaining:Vec<i32>,
    bjp:BlackjackProbabilities, textures:HashMap<String,TextureHandle>,
}
impl Default for BlackjackAid {
    fn default() -> Self {
        let decks = 1;
        Self {
            player: vec!["Dealer".into(), "Player 1".into()],
            selected_player: "Please choose a player".into(),
            selected_suit: "Please select a suit".into(),
            selected_number: "Please select a number".into(),
            suit: vec!["Hearts","Spades","Diamonds","Clubs"].iter().map(|s|s.to_string()).collect(),
            card_number: vec!["2","3","4","5","6","7","8","9","10","jack","queen","king","ace"].iter().map(|s|s.to_string()).collect(),
            recorded_cards_dealer: Vec::new(), recorded_cards_player1: Vec::new(),
            dealer_card_ids: Vec::new(), player1_card_ids: Vec::new(),
            player1_hand_total: 0, dealer_hand_total: 0,
            number_of_decks: decks, cards_remaining: vec![4*decks;14],
            bjp: BlackjackProbabilities::default(), textures: HashMap::new(),
        }
    }
}
impl App for BlackjackAid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals { window_fill: egui::Color32::from_rgb(10,10,40), ..egui::Visuals::dark() });
        self.show_probabilities_window(ctx);
        egui::CentralPanel::default().frame(egui::Frame::default().fill(egui::Color32::from_rgb(40,110,31))).show(ctx, |ui| {
            self.show_card_selection_ui(ui);
            self.show_reset_buttons(ui);
            self.show_card_display_sections(ui, ctx);
        });
    }
}
impl BlackjackAid {
    fn show_probabilities_window(&self, ctx: &egui::Context) {
        egui::Window::new("Probabilities").anchor(egui::Align2::RIGHT_TOP, [-5.0,5.0]).show(ctx, |ui| {
            ui.label(format!("Probability of Bust: {:.6}%", self.bjp.prob_bust));
            ui.label(format!("Probability of Immediate Blackjack: {:.6}%", self.bjp.prob_next_blackjack));
            ui.label(format!("Probability of Dealer Winning: {:.6}%", self.bjp.prob_dealer_wins));
            ui.label(format!("Probability of Tie: {:.6}%", self.bjp.prob_tie));
            ui.label(format!("Probability of Player Win: {:.6}%", self.bjp.prob_player_win));
        });
    }
    fn show_card_selection_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Choose a card:");
        ComboBox::from_label("Player/Dealer").selected_text(&self.selected_player).show_ui(ui, |ui| {
            for p in &self.player { ui.selectable_value(&mut self.selected_player, p.clone(), p); }
        });
        ComboBox::from_label("Suit").selected_text(&self.selected_suit).show_ui(ui, |ui| {
            for s in &self.suit { ui.selectable_value(&mut self.selected_suit, s.clone(), s); }
        });
        ComboBox::from_label("Number").selected_text(&self.selected_number).show_ui(ui, |ui| {
            for n in &self.card_number { ui.selectable_value(&mut self.selected_number, n.clone(), n); }
        });
        if ui.button("Add Card").clicked() {
            if self.selected_player != "Please choose a player" && self.selected_suit != "Please select a suit" && self.selected_number != "Please select a number" {
                let cid = format!("{}_of_{}", self.selected_number.to_lowercase(), self.selected_suit.to_lowercase());
                match self.selected_player.as_str() {
                    "Dealer" => { self.recorded_cards_dealer.push(self.selected_number.clone()); self.dealer_card_ids.push(cid); }
                    "Player 1" => { self.recorded_cards_player1.push(self.selected_number.clone()); self.player1_card_ids.push(cid); }
                    _ => {}
                }
            }
            if self.recorded_cards_dealer.len() >= 1 && self.recorded_cards_player1.len() >= 2 {
                self.player1_hand_total = hand_total(self.recorded_cards_player1.clone());
                self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());

                // compute blackjack chance
                let bj = probability_next_blackjack(self.player1_hand_total, &self.cards_remaining);
                self.bjp.prob_next_blackjack = bj * 100.0;

                let (w, t) = probability_dealer_outcomes(
                    self.player1_hand_total,
                    &self.cards_remaining,
                    self.dealer_hand_total,
                );
                self.bjp.prob_dealer_wins = w * 100.0;
                self.bjp.prob_tie = t * 100.0;
<<<<<<< HEAD
                    self.bjp.prob_next_blackjack =  probability_next_blackjack(self.player1_hand_total, &remaining) * 100.0;
                    self.bjp.prob_win_by_stand  = 100.0 - self.bjp.prob_dealer_wins;
                    self.bjp.prob_bust = probability_busting(self.player1_hand_total, &remaining) * 100.0;
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
                        self.cards_remaining.clear();
                    }
                });
=======
                let player = 1.0 - w - t;
                self.bjp.prob_player_win = player * 100.0;
                self.bjp.prob_bust = probability_busting(self.player1_hand_total, &self.cards_remaining) * 100.0;
            }
        }
    }
    fn show_reset_buttons(&mut self, ui: &mut egui::Ui) {
        ui.separator(); ui.horizontal(|ui| {
            if ui.button("Reset Dealer").clicked() { self.recorded_cards_dealer.clear(); self.dealer_card_ids.clear(); }
            if ui.button("Reset Player").clicked() { self.recorded_cards_player1.clear(); self.player1_card_ids.clear(); }
            if ui.button("New Round").clicked() {
                self.recorded_cards_dealer.clear(); self.recorded_cards_player1.clear();
                self.dealer_card_ids.clear(); self.player1_card_ids.clear();
                self.bjp = BlackjackProbabilities::default();
            }
            if ui.button("New Game").clicked() {
                self.recorded_cards_dealer.clear(); self.recorded_cards_player1.clear();
                self.dealer_card_ids.clear(); self.player1_card_ids.clear();
                self.cards_remaining = vec![4*self.number_of_decks;13];
                self.bjp = BlackjackProbabilities::default();
            }
        });
    }
    fn show_card_display_sections(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.separator();
        ui.label("Player Cards:"); ui.horizontal(|ui| {
            for cid in &self.player1_card_ids { display_card(ui, ctx, cid, &mut self.textures); }
        });
        ui.label(format!("Hand Total = {}", self.player1_hand_total));
>>>>>>> 9d3bfa2959be33a2c29577441fc71919f2c7ceef

        ui.separator();
        ui.label("Dealer Cards:"); ui.horizontal(|ui| {
            for cid in &self.dealer_card_ids { display_card(ui, ctx, cid, &mut self.textures); }
        });
        ui.label(format!("Hand Total = {}", self.dealer_hand_total));
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
        egui::Frame::default().fill(egui::Color32::WHITE)
            .inner_margin(egui::Margin::same(1.0))
            .rounding(egui::Rounding::same(5.0))
            .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
            .show(ui, |ui| {
                ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(80.0,110.0)));
            });
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native("Blackjack Assistant", options, Box::new(|_cc| Box::new(BlackjackAid::default())));
}
