use eframe::egui::{self, ColorImage, ComboBox, RichText, TextureHandle, TextureOptions};
use eframe::{App, NativeOptions, run_native};
use std::collections::HashMap;
use std::path::Path;

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

/// Probability next-card gives you a blackjack (21) from your current total.
/// Probability next-card gives you a blackjack (21) from your current total.
//Args
//'player total' - 'total sum of cards on current hand 
//'card_remaining' - 'an array of size 13 with each index of the remaining cards for that value
fn probability_next_blackjack(player_total: i32, cards_remaining: &Vec<i32>) -> f64 {
    let total_cards: i32 = cards_remaining.iter().sum();
    if total_cards == 0 || player_total >= 21 {
        return 0.0;
    }

    let needed = 21 - player_total;

    let count = match needed {
        1 => cards_remaining[0], // ace low (1)
        2..=9 => cards_remaining[needed as usize - 1],
        10 => cards_remaining[9] + cards_remaining[10] + cards_remaining[11] + cards_remaining[12],
        11 => cards_remaining[13], // ace high (11)
        _ => 0,
    };

    count as f64 / total_cards as f64
}



//fn to provide the probability of busting 
//Args
//'curr_hand' - total current hand
//'card_counts' - 'card counts which holds how many total cards in the vector remaining
fn probability_busting(curr_hand: i32, card_counts: &Vec<i32>) -> f64 {
    let card_vals = [2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];
    let total_remaining: i32 = card_counts.iter().sum();

    if total_remaining == 0 {
        return 0.0;
    }
    let mut bust_prob = 0.0;

    for (&val, &count) in card_vals.iter().zip(card_counts.iter()) {  //(2, 4), (3, 4), (4,4), (card value, #of cards for that value)
        if count == 0 {
            continue;
        }
        let draw = if val == 11 && curr_hand + 11 > 21 { 1 } else { val };

        if curr_hand + draw > 21 {
            bust_prob += count as f64 / total_remaining as f64;
        }
    }

    bust_prob
}

/// computes (win_prob, tie_prob) for dealer given player's total, deck counts, and dealer total.
/// uses memoization to avoid redundant computation.(dynamic programming)
//Args
//player_total - 'total current hand for player'
//dealer_total - 'total current hand for dealer'
//cards_remaining - 'card counts which holds how many total cards in the vector remaining'
//memo - 'hashmap to store player_total, dealer_total, and cards_remaining as keys to avoid recomputing
fn probability_dealer_outcomes(
    player_total: i32,
    dealer_total: i32,
    cards_remaining: &Vec<i32>,
    memo: &mut HashMap<(i32, i32, String), (f64, f64)>,
) -> (f64, f64) {
    if dealer_total > 21 {
        return (0.0, 0.0);
    }

    let key = (
        player_total,
        dealer_total,
        format!("{:?}", cards_remaining), // use vec as string for key
    );

    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if dealer_total >= 17 {
        let result = if dealer_total > player_total {
            (1.0, 0.0)
        } else if dealer_total == player_total {
            (0.0, 1.0)
        } else {
            (0.0, 0.0)
        };
        memo.insert(key, result);
        return result;
    }

    let card_vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];
    let total_cards: i32 = cards_remaining.iter().sum();
    let mut win = 0.0;
    let mut tie = 0.0;

    for (i, &cnt) in cards_remaining.iter().enumerate() {
        if cnt == 0 {
            continue;
        }

        let mut next = cards_remaining.clone();
        next[i] -= 1;
        let prob = cnt as f64 / total_cards as f64;
        let next_total = dealer_total + card_vals[i];

        let (w, t) = probability_dealer_outcomes(player_total, next_total, &next, memo);
        win += prob * w;
        tie += prob * t;
    }
    memo.insert(key, (win, tie));
    (win, tie)
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

struct SimulatorStats {
    player_wins: bool,
    dealer_wins: bool,
    player_bust: bool,
    dealer_bust: bool,
    player_dealer_tie: bool,
    natural_blackjack: bool,
    out_of_cards: bool,
}

impl Default for SimulatorStats {
    fn default() -> Self {
        Self {
            player_wins: false,
            dealer_wins: false,
            player_bust: false,
            dealer_bust: false,
            player_dealer_tie: false,
            natural_blackjack: false,
            out_of_cards: false,
        }
    }
}

struct BlackjackProbabilities {
    prob_bust: f64,
    prob_next_blackjack: f64,
    prob_win_by_stand: f64,
    prob_dealer_wins: f64,
    prob_tie: f64,
}

impl Default for BlackjackProbabilities {
    fn default() -> Self {
        Self {
            prob_bust: 0.0,
            prob_next_blackjack: 0.0,
            prob_win_by_stand: 0.0,
            prob_dealer_wins: 0.0,
            prob_tie: 0.0,
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
    number_of_decks: i32,
    cards_remaining: Vec<i32>,
    bjp: BlackjackProbabilities,
    textures: HashMap<String, TextureHandle>,
    forbidden_cards_sim: Vec<(i32, i32)>, //stores the cards pulled in the blackjack simulator
    stats: SimulatorStats,
}

impl Default for BlackjackAid {
    fn default() -> Self {
        let number_of_decks = 1;
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
            recorded_cards_dealer: Vec::new(),
            recorded_cards_player1: Vec::new(),
            dealer_card_ids: vec![],
            player1_card_ids: vec![],
            player1_hand_total: 0,
            dealer_hand_total: 0,
            number_of_decks,
            cards_remaining: vec![4 * number_of_decks; 13],
            bjp: BlackjackProbabilities::default(),
            textures: HashMap::new(),
            forbidden_cards_sim: Vec::new(),
            stats: SimulatorStats::default(),
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

        egui::Window::new("Probabilities")
            .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, 5.0])
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
                ui.label(format!(
                    "Probability of Tie: {:.1}%",
                    self.bjp.prob_tie
                ));
            });

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            /*ui.label(format!("Probability of Bust: {:.2}%", self.bjp.prob_bust));
            ui.label(format!(
                "Probability of Immediate Blackjack: {:.1}%",
                self.bjp.prob_next_blackjack
            ));
            ui.label(format!(
                "Probability of Winning by Standing: {:}%",
                self.bjp.prob_win_by_stand
            ));
            ui.label(format!(
                "Probability of Dealer Wins if You Stand: {:.1}%",
                self.bjp.prob_dealer_wins
            ));*/
        });

        egui::SidePanel::right("my_right_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    if ui.button("New Round").clicked() {
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.bjp = BlackjackProbabilities::default();

                        //initializes a new hand
                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                    if ui.button("New Game").clicked() {
                        //RESETS CARD COUNTING
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.forbidden_cards_sim.clear();
                        self.cards_remaining = vec![4 * self.number_of_decks; 13];
                        self.bjp = BlackjackProbabilities::default();
                    }
                    if ui.button("Start Game").clicked() {
                        //initialize player cards
                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                ui.label("Out of cards!");
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                //println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                });
            });
        });
        egui::TopBottomPanel::top("my_panel_top").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Dealer's Hand");
            });
        });
        egui::TopBottomPanel::top("dealer_cards")
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 110, 31)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        for card_id in &self.dealer_card_ids {
                            display_card(ui, ctx, card_id, &mut self.textures);
                        }
                    });
                    self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                    ui.label(format!("Hand Total = {}", self.dealer_hand_total));
                });
            });

        egui::TopBottomPanel::bottom("my_panel_bottom").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Player's Hand");
            });
        });
        egui::TopBottomPanel::bottom("player_cards")
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 110, 31)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        for card_id in &self.player1_card_ids {
                            display_card(ui, ctx, card_id, &mut self.textures);
                        }
                    });
                    self.player1_hand_total = hand_total(self.recorded_cards_player1.clone());
                    ui.label(format!("Hand Total = {}", self.player1_hand_total));
                });
            });

        let button_size = eframe::egui::Vec2::new(200.0, 50.0);
        let button_color = egui::Color32::from_rgb(100, 0, 0);
        let text_color = egui::Color32::from_rgb(176, 176, 176);

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 110, 31))) //sets page background color
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    columns[0].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(ui.available_height() / 2.0 - 20.0); // adjust for button height
                        if ui
                            .add(
                                egui::Button::new(RichText::new("Hit").color(text_color))
                                    .min_size(button_size)
                                    .fill(button_color),
                            )
                            .clicked()
                        {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                            let remaining: Vec<i32> = self.cards_remaining.clone();
                           let mut memo = HashMap::new(); //for memoization
                    let (w, t) = probability_dealer_outcomes(
                        self.player1_hand_total,
                        self.dealer_hand_total,
                        &remaining,
                        &mut memo,
                    );
                    self.bjp.prob_next_blackjack =  probability_next_blackjack(self.player1_hand_total, &remaining) * 100.0;
                    self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                    self.bjp.prob_bust = probability_busting(self.player1_hand_total, &remaining) * 100.0;
                    self.bjp.prob_dealer_wins = w * 100.0;
                    self.bjp.prob_tie = t * 100.0;
                            if self.player1_hand_total > 21 {
                                self.stats.player_bust = true;
                                println!("Bustin Time!");
                                println!("{}", self.stats.player_bust);
                            }
                        }
                    });
                    columns[1].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(ui.available_height() / 2.0 - 20.0); // adjust for button height
                        if ui
                            .add(
                                egui::Button::new(RichText::new("Stand").color(text_color))
                                    .min_size(button_size)
                                    .fill(button_color),
                            )
                            .clicked()
                        {
                            while self.dealer_hand_total <= 17 {
                                let (
                                    card_id,
                                    card_value,
                                    rand_suit_index,
                                    rand_card_index,
                                    out_of_cards,
                                ) = player_turn(self.forbidden_cards_sim.clone());
                                if out_of_cards == true {
                                    self.stats.out_of_cards = true;
                                    break;
                                } else {
                                    self.forbidden_cards_sim
                                        .push((rand_suit_index, rand_card_index));
                                    println!("{:?}", self.forbidden_cards_sim);
                                    self.dealer_card_ids.push(card_id.clone());
                                    self.recorded_cards_dealer.push(card_value);
                                    self.dealer_hand_total =
                                        hand_total(self.recorded_cards_dealer.clone());
                                }
                            }
                            if self.dealer_hand_total > 21 {
                                self.stats.dealer_bust = true;
                            } else {
                                if self.dealer_hand_total > self.player1_hand_total {
                                    self.stats.dealer_wins = true;
                                } else if self.dealer_hand_total == self.player1_hand_total {
                                    self.stats.player_dealer_tie = true;
                                } else {
                                    self.stats.player_wins = true;
                                }
                            }
                        }
                    });
                });
            });
        //pop-up windows
        if self.stats.player_wins {
            egui::Window::new("You win!")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut self.stats.player_wins)
                .show(ctx, |ui| {
                    if ui.button("New Round?").clicked() {
                        //println!("{}", MyApp::default().show_popup);
                        BlackjackAid::default().stats.player_wins = false;
                        //clears cards
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.bjp = BlackjackProbabilities::default();

                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                //println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                });
        }
        if self.stats.dealer_wins {
            egui::Window::new("Dealer wins!")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut self.stats.dealer_wins)
                .show(ctx, |ui| {
                    if ui.button("New Round?").clicked() {
                        //println!("{}", MyApp::default().show_popup);
                        BlackjackAid::default().stats.dealer_wins = false;
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.bjp = BlackjackProbabilities::default();

                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                //println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                });
        }
        if self.stats.player_bust {
            egui::Window::new("You've Busted!")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut self.stats.player_bust)
                .show(ctx, |ui| {
                    if ui.button("New Round?").clicked() {
                        //println!("{}", MyApp::default().show_popup);
                        BlackjackAid::default().stats.player_bust = false;
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.bjp = BlackjackProbabilities::default();

                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                //println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                });
        }
        if self.stats.dealer_bust {
            egui::Window::new("Dealer Busts! You win!")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut self.stats.dealer_bust)
                .show(ctx, |ui| {
                    if ui.button("New Round?").clicked() {
                        //println!("{}", MyApp::default().show_popup);
                        BlackjackAid::default().stats.dealer_bust = false;
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.bjp = BlackjackProbabilities::default();
                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                //println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                });
        }
        if self.stats.player_dealer_tie {
            egui::Window::new("You tie!")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut self.stats.player_dealer_tie)
                .show(ctx, |ui| {
                    if ui.button("New Round?").clicked() {
                        //println!("{}", MyApp::default().show_popup);
                        BlackjackAid::default().stats.player_dealer_tie = false;
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.bjp = BlackjackProbabilities::default();

                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                //println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                });
        }
        if self.stats.natural_blackjack {
            egui::Window::new("You win! Natural Blackjack!")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut self.stats.natural_blackjack)
                .show(ctx, |ui| {
                    if ui.button("New Round?").clicked() {
                        //println!("{}", MyApp::default().show_popup);
                        BlackjackAid::default().stats.natural_blackjack = false;
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.bjp = BlackjackProbabilities::default();

                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            if out_of_cards == true {
                                self.stats.out_of_cards = true;
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                //println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        if out_of_cards == true {
                            self.stats.out_of_cards = true;
                        } else {
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                        }
                    }
                });
        }
        if self.stats.out_of_cards {
            egui::Window::new("There are no more cards in the deck!")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut self.stats.out_of_cards)
                .show(ctx, |ui| {
                    if ui.button("New Game?").clicked() {
                        //println!("{}", MyApp::default().show_popup);
                        BlackjackAid::default().stats.out_of_cards = false;
                        //RESETS CARD COUNTING
                        self.recorded_cards_dealer.clear();
                        self.recorded_cards_player1.clear();
                        self.dealer_card_ids.clear();
                        self.player1_card_ids.clear();
                        self.forbidden_cards_sim.clear();
                        self.cards_remaining = vec![4 * self.number_of_decks; 13];
                        self.bjp = BlackjackProbabilities::default();

                        for i in 0..2 {
                            let (
                                card_id,
                                card_value,
                                rand_suit_index,
                                rand_card_index,
                                out_of_cards,
                            ) = player_turn(self.forbidden_cards_sim.clone());
                            self.forbidden_cards_sim
                                .push((rand_suit_index, rand_card_index));
                            //println!("{:?}", self.forbidden_cards_sim);
                            self.player1_card_ids.push(card_id.clone());
                            self.recorded_cards_player1.push(card_value);
                            self.player1_hand_total =
                                hand_total(self.recorded_cards_player1.clone());
                        }
                        //initialize dealer cards
                        let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                            player_turn(self.forbidden_cards_sim.clone());
                        self.forbidden_cards_sim
                            .push((rand_suit_index, rand_card_index));
                        //println!("{:?}", self.forbidden_cards_sim);
                        self.dealer_card_ids.push(card_id.clone());
                        self.recorded_cards_dealer.push(card_value);
                        self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                    }
                });
        }
    }
}

fn player_turn(forbidden_cards_sim: Vec<(i32, i32)>) -> (String, String, i32, i32, bool) {
    //pick random card_number
    //pick random suit
    //if suit and card_number match up with one already picked, then pick again
    let mut rand_card_index = rand::random_range(0..13);
    let mut rand_suit_index = rand::random_range(0..4);
    let mut counter = 0;
    let mut out_of_cards: bool = false;
    while forbidden_cards_sim.contains(&(rand_suit_index, rand_card_index)) {
        rand_card_index = rand::random_range(0..13);
        rand_suit_index = rand::random_range(0..4);
        counter += 1;
        if counter >= 52 {
            out_of_cards = true;
            break;
        }
    }
    if out_of_cards == true {
        let card_value = "-1".to_string();
        let card_suit = "None".to_string();
        let card_id = format!("{}_of_{}", card_value, card_suit);
        return (
            card_id,
            card_value,
            rand_suit_index,
            rand_card_index,
            out_of_cards,
        );
    } else {
        println!("{:?}", forbidden_cards_sim);
        let card_value =
            BlackjackAid::default().card_number[rand_card_index as usize].to_lowercase();
        let card_suit = BlackjackAid::default().suit[rand_suit_index as usize].to_lowercase();
        let card_id = format!("{}_of_{}", card_value, card_suit);
        println!("{}", card_id);

        //BlackjackAid::default().player1_card_ids.push(card_id.clone());
        //println!("{:?}", BlackjackAid::default().player1_card_ids);
        return (
            card_id,
            card_value,
            rand_suit_index,
            rand_card_index,
            out_of_cards,
        );
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
            .inner_margin(egui::Margin::same(1.0))
            .rounding(egui::Rounding::same(5.0))
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
        Box::new(|_cc| Box::new(BlackjackAid::default())),
    );
}
