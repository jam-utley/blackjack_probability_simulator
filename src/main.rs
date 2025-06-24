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

//Args
// `val`: value of the card
//'curr_hand' - total current hand
//'card_Val' - vector of card values with faces and numbers
//'card_counts' - 'card counts which holds how many total cards in the vector remaining

fn probability_busting(curr_hand: i32) -> f64 {
    let bust_number = 21 - curr_hand;
    let mut bust_cards_sum = 0; //sums the total of the cards remaining that could bust the hand on the next draw
    for i in (bust_number + 1)..(BlackjackAid::default().cards_remaining.len() as i32) {
        bust_cards_sum += BlackjackAid::default().cards_remaining[i as usize];
    }
    let mut cards_remaining_in_deck = 0;
    for i in BlackjackAid::default().cards_remaining {
        cards_remaining_in_deck += i;
    }

    let prob_bust: f64 = bust_cards_sum as f64 / cards_remaining_in_deck as f64;
    println!("{prob_bust}");
    return prob_bust;
}

fn probability_dealer_win(curr_hand: i32, card_counts: &Vec<i32>, curr_dealer_hand: i32) -> f64 {
    //replace card_vals with calls to the struct

    let card_vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11]; //values of the cards

    //check if dealer busts if current dealer hand
    if curr_dealer_hand > 21 {
        //println!("{:?}",curr_dealer_hand);
        return 0.0;
    }
    //check if dealer stand if current dealer <=17 and less than or equal to 21
    if curr_dealer_hand >= 17 && curr_dealer_hand <= 21 {
        if curr_dealer_hand > curr_hand {
            //check if current dealer hand is greater than players hand then return 1.0 for the weight probability
            //println!("Dealer wins with {} vs {}", curr_dealer_hand, curr_hand);
            return 1.0;
        } else {
            //println!("Dealer stands with {} — not enough to beat {}", curr_dealer_hand, curr_hand);
            return 0.0; //else return 0.0
        }
    }

    //This variable tracks the sum total of all of the remaining cards?
    let total_remaining_deck: i32 = card_counts.iter().sum(); //sum all remaining decks
    //println!("{total_remaining_deck}");
    let cards_remaining = BlackjackAid::default().cards_remaining.clone();
    let mut win_prob: f64 = 0.0;
    for (i, &val) in cards_remaining.iter().enumerate() {
        //loop through each remaining card if exists in card_count vector deck
        if cards_remaining[i] == 0 {
            continue;
        }
        let mut next_card_count: Vec<i32> = cards_remaining.clone(); //create clone to prevent mutate globally
        next_card_count[i] -= 1;
        let mut curr_prob: f64 = cards_remaining[i] as f64 / total_remaining_deck as f64; //calculate current probability
        let mut next_total_hand: i32 = card_vals[i] + curr_dealer_hand; //sum the total value of the next dealer hand
        win_prob +=
            curr_prob * probability_dealer_win(curr_hand, &next_card_count, next_total_hand);
    }
    //win_prob *= 100.0;
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

struct SimulatorStats{
    player_wins: i32,
    dealer_wins: i32,
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
    number_of_decks: i32,
    cards_remaining: Vec<i32>,
    bjp: BlackjackProbabilities,
    textures: HashMap<String, TextureHandle>,
    forbidden_cards_sim: Vec<(i32, i32)>, //stores the cards pulled in the blackjack simulator
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

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label(format!("{}", self.forbidden_cards_sim.len()));
        });

        egui::SidePanel::right("my_right_panel").show(ctx, |ui| {
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
                self.bjp.prob_dealer_wins = 0.0;
                self.bjp.prob_next_blackjack = 0.0;
                self.bjp.prob_win_by_stand = 0.0;
                self.bjp.prob_bust = 0.0;
            }
            if ui.button("New Game").clicked() {
                //RESETS CARD COUNTING
                self.recorded_cards_dealer.clear();
                self.recorded_cards_player1.clear();
                self.dealer_card_ids.clear();
                self.player1_card_ids.clear();
                self.forbidden_cards_sim.clear();
                self.cards_remaining = vec![4 * self.number_of_decks; 13];
                self.bjp.prob_dealer_wins = 0.0;
                self.bjp.prob_next_blackjack = 0.0;
                self.bjp.prob_win_by_stand = 0.0;
                self.bjp.prob_bust = 0.0;
            }
            if ui.button("Start Game").clicked() {
                //initialize player cards
                for i in 0..2 {
                    let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                        player_turn(self.forbidden_cards_sim.clone());
                    if out_of_cards == true {
                        ui.label("Out of cards!");
                    } else {
                        self.forbidden_cards_sim
                            .push((rand_suit_index, rand_card_index));
                        //println!("{:?}", self.forbidden_cards_sim);
                        self.player1_card_ids.push(card_id.clone());
                        self.recorded_cards_player1.push(card_value);
                        self.player1_hand_total = hand_total(self.recorded_cards_player1.clone());
                    }
                }
                //initialize dealer cards
                let (card_id, card_value, rand_suit_index, rand_card_index, out_of_cards) =
                    player_turn(self.forbidden_cards_sim.clone());
                if out_of_cards == true {
                    ui.label("Out of cards!");
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
        egui::TopBottomPanel::top("my_panel_top").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Dealer's Hand");
            });
        });
        egui::TopBottomPanel::top("dealer_cards").show(ctx, |ui| {
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
        egui::TopBottomPanel::bottom("player_cards").show(ctx, |ui| {
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

        //new game button resets the cards and counter
        //new round button resets the cards
        //generate cards: two for player, one for dealer
        //player hits until bust or stand
        //when stand, dealer draws new cards until 17 -- 21 or bust.
        //Highest hand wins
        //new round button to play another hand

        //add a gambling aspect with fake money?

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
                                ui.label("Out of cards!");
                            } else {
                                self.forbidden_cards_sim
                                    .push((rand_suit_index, rand_card_index));
                                println!("{:?}", self.forbidden_cards_sim);
                                self.player1_card_ids.push(card_id.clone());
                                self.recorded_cards_player1.push(card_value);
                                self.player1_hand_total =
                                    hand_total(self.recorded_cards_player1.clone());
                            }
                            println!("{:?}", self.player1_card_ids);
                            println!("{}", self.player1_hand_total);
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
                                    ui.label("Out of Cards!");
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
                            println!("{}", self.dealer_hand_total);
                        }
                    });
                });
            });
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