use egui::{
    self, Align2, Color32, ColorImage, Context, FontId, Frame, Margin, Pos2, RichText, Rounding,
    Stroke, TextureHandle, TextureOptions, Window,
};

use eframe::{egui::FontData, egui::FontDefinitions, egui::FontFamily};
use eframe::{run_native, App, NativeOptions};
use egui::ComboBox;
use egui::Vec2;
use egui::ViewportBuilder;
use rand::prelude::*;
use rand::thread_rng;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;

struct FallingSymbol {
    //defining struct for raining symbols animation
    pos: Pos2,
    velocity: f32,
    symbol: char,
    color: Color32,
}
//To do list for Future Development::
// convert repetative code in simulator into traits and structs
// in order to make the code more readable
// add suggested action box
// remove warnings on start
// remove extra thread_rng()
// don’t use BlackjackAid::default() in player_turn()
// fix cards_remaining[13] (index out of bounds)
// change deck count to 6 or make it dynamic
// move new round logic into a deal_new_round() function
// split update() into smaller parts
// show message when out of cards
// clean up unused textures (optional)

fn load_texture(ctx: &egui::Context, path: &str) -> Option<TextureHandle> {
    //'Args' -
    //egui library of the simulator and helper
    //path - file path directory to the cards
    let image_data = std::fs::read(path).ok()?;
    let image = image::load_from_memory(&image_data).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(path, color_image, TextureOptions::LINEAR))
}

fn hand_total(input: Vec<String>) -> i32 {
    //'Args'
    //input - calculates the total sum of cards on a hand from vector string
    //calculates the hand total of a player/dealer given the string of the number they drew/chose
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
    //Handles the conversion from a high ace to a low ace in the event of busting
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

//struct to convert a string value to i32 datatype
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
    //converts between a card name and a value
    //fn to convert the value to in t
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
    //args
    //name - reference of the name of card
    //a match function that returns a an option to see if card exists
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
//struct of probabilities
//prob_bust: f64, probability of bust for player
//prob_next_blackjack: f64, probability of next blackjack with remaining cards
//prob_win_by_stand: f64,  //probability of winning by stand
//prob_dealer_wins: f64, //probability of dealer winning
//prob_tie: f64, //probability of tying with dealer
struct BlackjackProbabilities {
    //stores the probabilities of different outcomes
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
//handles simulator popups
//prob_bust: f64, probability of bust for player
//prob_next_blackjack: f64, probability of next blackjack with remaining cards
//prob_win_by_stand: f64,  //probability of winning by stand
//prob_dealer_wins: f64, //probability of dealer winning
//prob_tie: f64, //probability of tying with dealer
struct SimulatorStats {
    //Bool vals control the simulator's popups
    player_wins: bool,
    dealer_wins: bool,
    player_bust: bool,
    dealer_bust: bool,
    player_dealer_tie: bool,
    natural_blackjack: bool,
    out_of_cards: bool,
}
//a trait to be able to call the SimulatorStats, creating a new instance when a new game, new round, or start game
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
//struct to all blackjack
struct BlackjackAid {
    //All the variables and values used in the game
    start_screen: bool,                       //start screen
    game_sim: bool,                           //game_sim
    card_counter: bool,                       //card_counter
    textures: HashMap<String, TextureHandle>, //textures in hashmap to handle pngs and backgrounds
    player: Vec<String>,                      //Picks between player and the dealer
    selected_player: String,                  //player whether it's dealer or player
    selected_suit: String,                    //selected suit
    selected_number: String,
    suit: Vec<String>,
    card_number: Vec<String>,
    recorded_cards_dealer: Vec<String>,
    recorded_cards_player1: Vec<String>,
    dealer_card_ids: Vec<String>,
    player1_card_ids: Vec<String>,
    player1_hand_total: i32, //player hand total
    dealer_hand_total: i32,  //dealer hand total
    number_of_decks: i32,
    cards_remaining: Vec<i32>,
    bjp: BlackjackProbabilities,
    frame_count: usize,
    falling_symbols: Vec<FallingSymbol>,
    secret_pop: bool, //easter egg
    initialized: bool,
    forbidden_cards_sim: Vec<(i32, i32)>, //stores the cards pulled in the blackjack simulator
    stats: SimulatorStats,
}

impl Default for BlackjackAid {
    //initializes the default state for the struct BlackjackAid
    fn default() -> Self {
        //Initializes the game
        let number_of_decks = 1; //for now one deck
        let mut rng = thread_rng(); //initialze random generator
        let symbols = vec!['♠', '♥', '♦', '♣']; //symbols for falling down background
        let colors = vec![
            Color32::BLACK,
            Color32::from_rgb(220, 20, 60), // red hearts
            Color32::from_rgb(220, 20, 60), // red diamonds
            Color32::BLACK,
        ];
        //initialze random generator
        let mut rng = thread_rng();
        let falling_symbols = (0..30) //falling symbols in the background
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
            initialized: false,  // indicates whether the app has completed initialization
            secret_pop: false,   // hidden or special feature toggle (e.g., easter egg popup)
            start_screen: true,  // whether the start menu is currently being shown
            game_sim: false,     // whether the game simulator is running
            card_counter: false, // enables card counting mode if true

            falling_symbols, // animation effect for falling symbols on the UI
            frame_count: 0,  // tracks animation frames or update cycles
            textures: HashMap::new(), // loaded textures (e.g., card images) for rendering

            player: vec!["Dealer".into(), "Player 1".into()], // available players in the UI
            selected_player: "Please choose a player".into(), // currently selected player
            selected_suit: "Please select a suit".into(), // selected suit (Hearts, Spades, etc.)
            selected_number: "Please select a number".into(), // selected card number (e.g., 7, queen)

            suit: vec!["Hearts", "Spades", "Diamonds", "Clubs"]
                .into_iter()
                .map(String::from)
                .collect(), // List of suits shown in the dropdown or UI

            card_number: vec![
                "2", "3", "4", "5", "6", "7", "8", "9", "10", "jack", "queen", "king", "ace",
            ]
            .into_iter()
            .map(String::from)
            .collect(), // List of card ranks shown in the dropdown

            recorded_cards_dealer: Vec::new(), // Tracks which cards the dealer has
            recorded_cards_player1: Vec::new(), // Tracks which cards Player 1 has
            dealer_card_ids: vec![],           // ids or paths for dealer card textures
            player1_card_ids: vec![],          // ids or paths for player card textures

            player1_hand_total: 0, // total value of Player 1’s hand
            dealer_hand_total: 0,  //total value of Dealer’s hand

            number_of_decks, //number of decks in use (passed externally)

            cards_remaining: vec![4 * number_of_decks; 13], // remaining cards of each rank (4 per deck × N decks)

            bjp: BlackjackProbabilities::default(), // Handles blackjack win/bust probability calculations

            forbidden_cards_sim: Vec::new(), // Cards that the simulator is not allowed to draw

            stats: SimulatorStats::default(), // Tracks simulation outcomes (wins, busts, etc.)
        }
    }
}
//
impl App for BlackjackAid {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Return to the start screen if the Escape key is pressed
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.start_screen = true;
        }

        let screen_rect = ctx.screen_rect(); // get the current size and position of the application window

        if self.start_screen {
            self.show_start_screen(ctx);

            ctx.request_repaint(); //if we are on the start screen, render it and request a repaint
        } else if self.card_counter {
            // Dark theme setup
            ctx.set_visuals(egui::Visuals {
                //if card counter mode is active, render the themed background and animations
                window_fill: egui::Color32::from_rgb(10, 80, 10),
                ..egui::Visuals::dark()
            });
            // draw a background rectangle covering the whole screen
            let painter = ctx.layer_painter(egui::LayerId::background());

            painter.rect_filled(screen_rect, 0.0, egui::Color32::from_rgb(41, 55, 59));
            // if it moves off screen, reset to top with new random horizontal position
            for symbol in &mut self.falling_symbols {
                symbol.pos.y += symbol.velocity;
                if symbol.pos.y > screen_rect.bottom() {
                    symbol.pos.y = 0.0;
                    symbol.pos.x = rand::thread_rng().gen_range(0.0..screen_rect.right());
                }
                // draw the symbol at its current position
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
                    Frame::none()
                        .fill(Color32::from_rgb(41, 55, 59))
                        .rounding(egui::Rounding::same(5.0))
                        .show(ui, |ui| {
                            if ui
                                .button(RichText::new("♥").size(24.0).color(Color32::RED))
                                .clicked()
                            {
                                self.secret_pop = !self.secret_pop;
                            }
                        });
                });
            // show a heart button in the bottom-right corner
            if self.secret_pop {
                Window::new("easter egg")
                    .resizable(false)
                    .collapsible(false)
                    .anchor(Align2::RIGHT_BOTTOM, [-100.0, -100.0]) // position it 10px from bottom-right
                    .frame(Frame::none().fill(Color32::from_rgb(41, 55, 59))) // set background color here
                    .show(ctx, |ui| {
                        ui.label(RichText::new("you found me!").color(Color32::WHITE));
                        // set text color properly
                    });
            }
            //home button
            egui::Area::new("home_button".into())
                .anchor(Align2::LEFT_BOTTOM, [10.0, -20.0]) // position it 10px from bottom-right // position it 10px from left, 20px from bottom
                .show(ctx, |ui| {
                    Frame::none()
                        .fill(Color32::from_rgb(41, 55, 59))
                        .rounding(egui::Rounding::same(5.0))
                        .show(ui, |ui| {
                            if ui
                                .button(RichText::new("Return 🏠").size(15.0).color(Color32::WHITE))
                                .clicked()
                            {
                                // go back to start screen
                                self.game_sim = false;
                                self.card_counter = false;
                                self.start_screen = true;
                            }
                        });
                });

            egui::Window::new("Controls")
                .anchor(egui::Align2::LEFT_TOP, [15.0, 80.0]) //position it from 15px from left top 80px from the top
                .resizable(false)
                .collapsible(true)
                .show(ctx, |ui| {
                    self.show_card_selection_ui(ui);
                    self.show_reset_buttons(ui);
                    self.show_card_display_sections(ui, ctx);
                });
            //show probabilities
            self.show_probabilities_window(ctx);
            self.show_title_banner(ctx);

            ctx.request_repaint();
        } else if self.game_sim {
            let visuals = egui::Visuals {
                //sets background color for dropown menus and windows, not the entire page
                window_fill: egui::Color32::from_rgb(10, 10, 40),
                ..egui::Visuals::dark()
            };
            ctx.set_visuals(visuals);
            // Show "Return" button in the bottom-left corner
            egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
                //Side panel for border only
                egui::Area::new("home_button".into())
                    .anchor(Align2::LEFT_BOTTOM, [10.0, -20.0])
                    .show(ctx, |ui| {
                        Frame::none()
                            .fill(Color32::from_rgb(41, 55, 59))
                            .rounding(egui::Rounding::same(5.0))
                            .show(ui, |ui| {
                                if ui
                                    .button(
                                        RichText::new("Return 🏠").size(15.0).color(Color32::WHITE),
                                    )
                                    .clicked()
                                {
                                    self.game_sim = false;
                                    self.card_counter = false;
                                    self.start_screen = true;
                                }
                            });
                    });
            });
            //panel of probabilities
            egui::Window::new("Probabilities")
                .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, 5.0]) //show from 5px right and 5px from bottom
                .show(ctx, |ui| {
                    ui.label(format!("Probability of Bust: {:.2}%", self.bjp.prob_bust)); //probability of bustin g
                    ui.label(format!(
                        "Probability of Immediate Blackjack: {:.1}%", //probabilites of blackjack
                        self.bjp.prob_next_blackjack
                    ));
                    ui.label(format!(
                        "Probability of Winning by Standing: {:.1}%", //probabilites of standing
                        self.bjp.prob_win_by_stand
                    ));
                    ui.label(format!(
                        "Probability of Dealer Wins if You Stand: {:.1}%", //probability of dealer winning
                        self.bjp.prob_dealer_wins
                    ));
                    ui.label(format!("Probability of Tie: {:.1}%", self.bjp.prob_tie));
                    //probability of tie
                });

            egui::SidePanel::right("my_right_panel").show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.vertical(|ui| {
                        if ui.button("New Round").clicked() {
                            //new round button clicked
                            //reset everything
                            self.recorded_cards_dealer.clear(); //reset recorded_cards_dealer
                            self.recorded_cards_player1.clear();
                            self.dealer_card_ids.clear();
                            self.player1_card_ids.clear();
                            self.cards_remaining = vec![4 * self.number_of_decks; 13];
                            self.bjp = BlackjackProbabilities::default(); //reset probabilities

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
                                    self.stats.out_of_cards = true; //no remaining rcards
                                } else {
                                    self.forbidden_cards_sim
                                        .push((rand_suit_index, rand_card_index));
                                    self.cards_remaining[rand_card_index as usize] -= 1;
                                    self.player1_card_ids.push(card_id.clone());
                                    self.recorded_cards_player1.push(card_value);
                                    self.player1_hand_total =
                                        hand_total(self.recorded_cards_player1.clone());
                                }
                            }
                            //initialize dealer cards
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
                                self.forbidden_cards_sim //push to forbiiden_cards vector 
                                    .push((rand_suit_index, rand_card_index));
                                self.cards_remaining[rand_card_index as usize] -= 1; //decrement index in cards_remaining
                                                                                     //println!("{:?}", self.forbidden_cards_sim);
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value); //push card value to recorded_cards_dealer
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                                //calculate total for dealer hand
                            }
                            if self.player1_hand_total == 21 {
                                self.stats.natural_blackjack = true; //check if player reach blackjack first try o next try
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
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
                                ) = player_turn(self.forbidden_cards_sim.clone()); //give it to player
                                if out_of_cards == true {
                                    ui.label("Out of cards!");
                                } else {
                                    self.forbidden_cards_sim
                                        .push((rand_suit_index, rand_card_index)); //push to forbiiden_cards vector
                                    self.cards_remaining[rand_card_index as usize] -= 1; //decrement index in cards_remaining
                                    self.player1_card_ids.push(card_id.clone());
                                    self.recorded_cards_player1.push(card_value); //push card value to recorded_cards_dealer
                                    self.player1_hand_total =
                                        hand_total(self.recorded_cards_player1.clone());
                                    //get total values in player hand
                                }
                            }
                            //initialize dealer cards
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
                                    .push((rand_suit_index, rand_card_index)); //push to forbiiden_cards vector
                                                                               //println!("{:?}", self.forbidden_cards_sim);
                                self.cards_remaining[rand_card_index as usize] -= 1; //decrement index in cards_remaining
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value); //push card value to recorded_cards_dealer
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                                //get total values in player hand
                            }
                            if self.player1_hand_total == 21 {
                                self.stats.natural_blackjack = true; //check if player reach blackjack first try o next try
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
                });
            });
            //show ui of dealer's hand
            egui::TopBottomPanel::top("my_panel_top").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Dealer's Hand");
                });
            });
            //show ui of dealers cards
            egui::TopBottomPanel::top("dealer_cards")
                .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 110, 31)))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            for card_id in &self.dealer_card_ids {
                                display_card(ui, ctx, card_id, &mut self.textures);
                                //display the card
                            }
                        });
                        self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone()); //total hand for dealer
                        ui.label(format!("Hand Total = {}", self.dealer_hand_total));
                        //show ui of dealers total cards
                    });
                });
            //show ui of player's cards
            egui::TopBottomPanel::bottom("my_panel_bottom").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Player's Hand");
                });
            });
            egui::TopBottomPanel::bottom("player_cards") //show ui of player cards 
                .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 110, 31)))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            for card_id in &self.player1_card_ids {
                                display_card(ui, ctx, card_id, &mut self.textures);
                            }
                        });
                        self.player1_hand_total = hand_total(self.recorded_cards_player1.clone()); //calculate total hand for player 
                        ui.label(format!("Hand Total = {}", self.player1_hand_total)); //show ui of total hand for player 
                    });
                });

            let button_size = eframe::egui::Vec2::new(200.0, 50.0); //button_size, size 200, 50
            let button_color = egui::Color32::from_rgb(100, 0, 0); //color of button
            let text_color = egui::Color32::from_rgb(176, 176, 176); //text colors

            egui::CentralPanel::default()
                .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 110, 31))) //sets page background color
                .show(ctx, |ui| {
                    ui.columns(2, |columns| {
                        //ui of columns
                        columns[0].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.add_space(ui.available_height() / 2.0 - 20.0); // adjust for button height
                            if ui
                                .add(
                                    egui::Button::new(RichText::new("Hit").color(text_color)) //if player click on the hit button
                                        .min_size(button_size)
                                        .fill(button_color),
                                )
                                .clicked()
                            {
                                let (
                                    card_id,
                                    card_value, //add the card for that player
                                    rand_suit_index,
                                    rand_card_index,
                                    out_of_cards,
                                ) = player_turn(self.forbidden_cards_sim.clone());
                                if out_of_cards == true {
                                    self.stats.out_of_cards = true; //check if cards are out
                                } else {
                                    self.forbidden_cards_sim
                                        .push((rand_suit_index, rand_card_index)); //if not, push to the forbidden_cards_sim to not use that specific card
                                    self.cards_remaining[rand_card_index as usize] -= 1; //decrement on that card
                                    self.player1_card_ids.push(card_id.clone());
                                    self.recorded_cards_player1.push(card_value);
                                    self.player1_hand_total =
                                        hand_total(self.recorded_cards_player1.clone());
                                    //get hand total
                                }
                                let remaining: Vec<i32> = self.cards_remaining.clone(); //clone of decks [4] * 13
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    //fn of dealer outcomes to calculate the proability
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack = probability_next_blackjack(self.player1_hand_total, &remaining) //fn of next_blackjack 
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0; //probability of winning by stand (1 - probability of dealer winning - probability of tie)
                                self.bjp.prob_bust = probability_busting(self.player1_hand_total, &remaining) //proability of bust 
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                                if self.player1_hand_total > 21 {
                                    self.stats.player_bust = true;
                                    //println!("Bustin Time!");
                                    //println!("{}", self.stats.player_bust);
                                }
                                if self.player1_hand_total == 21 {
                                    self.stats.natural_blackjack = true;
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
                                        card_id, //add the card for that dealer
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
                                            .push((rand_suit_index, rand_card_index)); //do the same for player, push to forbidden cards vector
                                                                                       //println!("{:?}", self.forbidden_cards_sim);
                                        self.cards_remaining[rand_card_index as usize] -= 1; //decrement
                                        self.dealer_card_ids.push(card_id.clone()); //push of card id instead value to dealer_card_id
                                        self.recorded_cards_dealer.push(card_value); //push to recorded_cards for dealer for value of card
                                        self.dealer_hand_total =
                                            hand_total(self.recorded_cards_dealer.clone());
                                    }
                                }
                                if self.dealer_hand_total > 21 {
                                    //check if dealer bust
                                    self.stats.dealer_bust = true;
                                } else {
                                    if self.dealer_hand_total > self.player1_hand_total {
                                        //check if dealer wins
                                        self.stats.dealer_wins = true;
                                    } else if self.dealer_hand_total == self.player1_hand_total {
                                        //check for tie
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
                            self.cards_remaining = vec![4 * self.number_of_decks; 13];

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
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value);
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
            }
            //if dealer wins, option for a new round
            if self.stats.dealer_wins {
                egui::Window::new("Dealer wins!")
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .open(&mut self.stats.dealer_wins)
                    .show(ctx, |ui| {
                        if ui.button("New Round?").clicked() {
                            //println!("{}", MyApp::default().show_popup);
                            BlackjackAid::default().stats.dealer_wins = false; //clears all vectors and renitailizes probabiliteis
                            self.recorded_cards_dealer.clear();
                            self.recorded_cards_player1.clear();
                            self.dealer_card_ids.clear();
                            self.player1_card_ids.clear();
                            self.bjp = BlackjackProbabilities::default();
                            //get two cards for player
                            for i in 0..2 {
                                let (
                                    card_id,
                                    card_value,
                                    rand_suit_index,
                                    rand_card_index,
                                    out_of_cards,
                                ) = player_turn(self.forbidden_cards_sim.clone()); //push to forbidden vecotor
                                if out_of_cards == true {
                                    self.stats.out_of_cards = true; //if run out of cards, put true
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
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value);
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
            }
            //if player busts,, renitialize two cards to player and one card to dealer
            if self.stats.player_bust {
                egui::Window::new("You've Busted!")
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .open(&mut self.stats.player_bust)
                    .show(ctx, |ui| {
                        if ui.button("New Round?").clicked() {
                            //new round selected
                            //println!("{}", MyApp::default().show_popup);
                            //clears all vectors and renitailizes probabiliteis
                            BlackjackAid::default().stats.player_bust = false;
                            self.recorded_cards_dealer.clear();
                            self.recorded_cards_player1.clear();
                            self.dealer_card_ids.clear();
                            self.player1_card_ids.clear();
                            self.bjp = BlackjackProbabilities::default();
                            //get two cards for player
                            for i in 0..2 {
                                let (
                                    card_id,
                                    card_value,
                                    rand_suit_index,
                                    rand_card_index,
                                    out_of_cards,
                                ) = player_turn(self.forbidden_cards_sim.clone()); //do the same for player, push to forbidden cards vector
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
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value);
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
            }
            //if dealer busts,, renitialize two cards to player and one card to dealer
            if self.stats.dealer_bust {
                egui::Window::new("Dealer Busts! You win!")
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .open(&mut self.stats.dealer_bust)
                    .show(ctx, |ui| {
                        if ui.button("New Round?").clicked() {
                            //println!("{}", MyApp::default().show_popup);
                            BlackjackAid::default().stats.dealer_bust = false;
                            self.recorded_cards_dealer.clear(); //clear all vectors and renitialize probabilities
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
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value);
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
            }
            //if tie, new round is selected
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
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value);
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
            }
            //if you got blackjack, new round
            if self.stats.natural_blackjack {
                egui::Window::new("You win! Blackjack!")
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
                            //initialize player cards
                            for i in 0..2 {
                                let (
                                    card_id,
                                    card_value,
                                    rand_suit_index,
                                    rand_card_index,
                                    out_of_cards,
                                ) = player_turn(self.forbidden_cards_sim.clone()); //push forbidden_cards to card simulator vectro
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
                                self.dealer_card_ids.push(card_id.clone());
                                self.recorded_cards_dealer.push(card_value);
                                self.dealer_hand_total =
                                    hand_total(self.recorded_cards_dealer.clone());
                            }
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
            }
            //no more remaining cards in the deck, button to create new game
            if self.stats.out_of_cards {
                egui::Window::new("There are no more cards in the deck!")
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .open(&mut SimulatorStats::default().out_of_cards)
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
                            //initialize player cards
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
                            self.dealer_card_ids.push(card_id.clone());
                            self.recorded_cards_dealer.push(card_value);
                            self.dealer_hand_total = hand_total(self.recorded_cards_dealer.clone());
                            if self.stats.out_of_cards == false {
                                //calculates probabilities when new round button clicked
                                let remaining: Vec<i32> = self.cards_remaining.clone();
                                let mut memo = HashMap::new(); //for memoization
                                let (w, t) = probability_dealer_outcomes(
                                    self.player1_hand_total,
                                    self.dealer_hand_total,
                                    &remaining,
                                    &mut memo,
                                );
                                self.bjp.prob_next_blackjack =
                                    probability_next_blackjack(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                                self.bjp.prob_bust =
                                    probability_busting(self.player1_hand_total, &remaining)
                                        * 100.0;
                                self.bjp.prob_dealer_wins = w * 100.0;
                                self.bjp.prob_tie = t * 100.0;
                            }
                        }
                    });
            }
        }
    }
}
//show the start screen
impl BlackjackAid {
    fn show_start_screen(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect(); // get the size of the screen

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(screen_rect.height() * 0.2); // push content down a bit

            ui.vertical_centered(|ui| {
                //centers vertically in the screen
                // big red title
                ui.heading(
                    egui::RichText::new("JACK-BOT")
                        .size(50.0)
                        .color(egui::Color32::RED),
                );

                ui.add_space(30.0); // space below title

                // gray subtitle
                ui.label(
                    egui::RichText::new("Need some help? We got you!") //First button initialization
                        .size(24.0)
                        .color(egui::Color32::LIGHT_GRAY),
                );

                ui.add_space(50.0); // space before buttons

                // "Play a Round" button (green)
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Play a Round").color(egui::Color32::WHITE), //Second button initialization
                        )
                        .min_size(egui::Vec2::new(200.0, 40.0))
                        .fill(egui::Color32::from_rgb(30, 80, 30)) // green background
                        .rounding(10.0),
                    )
                    .clicked()
                {
                    self.game_sim = true; // go to game mode
                    self.start_screen = false; // hide start screen
                    self.card_counter = false;
                }

                ui.add_space(10.0); // space between buttons

                // "Get Some Help" button (blue)
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Get Some Help").color(egui::Color32::WHITE),
                        )
                        .min_size(egui::Vec2::new(200.0, 40.0))
                        .fill(egui::Color32::from_rgb(60, 60, 100)) // blue background
                        .rounding(10.0),
                    )
                    .clicked()
                {
                    self.card_counter = true; // go to help mode
                    self.game_sim = false;
                    self.start_screen = false; // hide start screen
                }
            });
        });

        ctx.request_repaint(); // keep screen updating (useful for animations or effects)
    }

    fn show_probabilities_window(&self, ctx: &egui::Context) {
        //generates the probability display window
        egui::Window::new("Probabilities")
            .anchor(egui::Align2::RIGHT_TOP, [-15.0, 80.0]) //anchors window in top right of window
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
                ui.label(format!("Probability of Tie: {:.1}%", self.bjp.prob_tie));
            });
    }

    fn show_card_selection_ui(&mut self, ui: &mut egui::Ui) {
        //shopws selected cards in UI
        ui.label("Choose a card:");

        ComboBox::from_label("Player/Dealer") //user selects the player to assign the card to
            .selected_text(&self.selected_player)
            .show_ui(ui, |ui| {
                for player in &self.player {
                    ui.selectable_value(&mut self.selected_player, player.clone(), player);
                }
            });

        ComboBox::from_label("Suit") //user selects the suit
            .selected_text(&self.selected_suit)
            .show_ui(ui, |ui| {
                for suit in &self.suit {
                    ui.selectable_value(&mut self.selected_suit, suit.clone(), suit);
                }
            });

        ComboBox::from_label("Number") //user selects the number
            .selected_text(&self.selected_number)
            .show_ui(ui, |ui| {
                for number in &self.card_number {
                    ui.selectable_value(&mut self.selected_number, number.clone(), number);
                }
            });

        if ui.button("Add Card").clicked() {
            //adds the card to the hand & UI
            if self.selected_player != "Please choose a player" //Only runs if all three boxes are selected
                && self.selected_suit != "Please select a suit"
                && self.selected_number != "Please select a number"
            {
                let card_id = format!(
                    //Generates card_id for finding the png asset
                    "{}_of_{}",
                    self.selected_number.to_lowercase(),
                    self.selected_suit.to_lowercase()
                );

                match self.selected_player.as_str() {
                    //pushes the card_id to the particular player vector that stores the hand
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
                //computes probabilities only when there are at least two player cards and one dealer card
                let mut memo = HashMap::new(); //for memoization
                let remaining = self.cards_remaining.clone(); //remaining cards in the deck
                let (w, t) = probability_dealer_outcomes(
                    //w = wins, t = ties
                    self.player1_hand_total,
                    self.dealer_hand_total,
                    &remaining,
                    &mut memo,
                );
                self.bjp.prob_dealer_wins = w * (100.0 as f64); //assigns the values to the structs
                self.bjp.prob_win_by_stand = (1.0 - w - t) * 100.0;
                self.bjp.prob_bust =
                    probability_busting(self.player1_hand_total, &remaining) * 100.0;
                self.bjp.prob_next_blackjack =
                    probability_next_blackjack(self.player1_hand_total, &remaining);
                self.bjp.prob_tie = t * (100.0 as f64);
            }
        }
    }

    fn show_reset_buttons(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            //resets the dealer's cards to 0
            if ui.button("Reset Dealer").clicked() {
                self.recorded_cards_dealer.clear();
                self.dealer_card_ids.clear();
            }
            //resets the player's cards to 0
            if ui.button("Reset Player").clicked() {
                self.recorded_cards_player1.clear();
                self.player1_card_ids.clear();
            }
            //clears all hands, BUT keeps card counting
            if ui.button("New Round").clicked() {
                self.recorded_cards_dealer.clear();
                self.recorded_cards_player1.clear();
                self.dealer_card_ids.clear();
                self.player1_card_ids.clear();
                self.bjp = BlackjackProbabilities::default();
            }
            //clears all hands AND RESETS card counting
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
        ui.separator(); //the dividing line between the sections
                        //Section to display player cards
        ui.label("Player Cards:");
        ui.horizontal(|ui| {
            for card_id in &self.player1_card_ids {
                display_card(ui, ctx, card_id, &mut self.textures);
            }
        });
        //calculates the card value total in player's hand
        self.player1_hand_total = hand_total(self.recorded_cards_player1.clone());
        ui.label(format!("Hand Total = {}", self.player1_hand_total));

        ui.separator(); //the dividing line between the sections
                        //Section to display dealer cards
        ui.label("Dealer Cards:");
        ui.horizontal(|ui| {
            for card_id in &self.dealer_card_ids {
                display_card(ui, ctx, card_id, &mut self.textures);
            }
        });
        //calculates the card value total in dealer's hand
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

fn player_turn(forbidden_cards_sim: Vec<(i32, i32)>) -> (String, String, i32, i32, bool) {
    //Function for taking a turn in the game simulator
    let mut rng = thread_rng();
    //pick random card_number
    //pick random suit
    //if suit and card_number match up with one already picked, then pick again
    let mut rand_card_index = rng.gen_range(0..13); //selects random card
    let mut rand_suit_index = rng.gen_range(0..4); //selects random suit
    let mut counter = 0;
    let mut out_of_cards: bool = false;
    //forbidden_cards_sim keeps track of all of the cards played in the game
    while forbidden_cards_sim.contains(&(rand_suit_index, rand_card_index)) {
        rand_card_index = rng.gen_range(0..13);
        rand_suit_index = rng.gen_range(0..4);
        counter += 1;
        if counter >= 52 {
            out_of_cards = true;
            break;
        }
    }
    if out_of_cards == true {
        //What to return from thefunction if you run out of cards in the deck
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
        let card_value =
            BlackjackAid::default().card_number[rand_card_index as usize].to_lowercase();
        let card_suit = BlackjackAid::default().suit[rand_suit_index as usize].to_lowercase();
        let card_id = format!("{}_of_{}", card_value, card_suit);
        //returns the card_id for generating the png, the card value for calculating the hand total,
        //and the randomly selected indices to add to the forbidden_list_sim of used cards
        //also returns whether game is out of cards
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
    card: &str, //this is the card_id generated earlier
    textures: &mut HashMap<String, TextureHandle>,
) {
    //function to display the cards
    //initializes the png asset path for any card
    let path = format!("assets/{}.png", card);

    if !textures.contains_key(&path) {
        //show png if there, else say no png
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

fn probability_busting(curr_hand: i32, card_counts: &Vec<i32>) -> f64 {
    //fn to return probability of busting
    //ARGS
    //curr_hand - total curr hand
    //curr_count - vector of remaining deck
    let card_vals = [2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];
    let total_remaining: i32 = card_counts.iter().sum(); //counts the amount of cards remaining in the deck

    if total_remaining == 0 {
        return 0.0;
    }
    let mut bust_prob = 0.0; //initializes the probability of busting variable

    for (&val, &count) in card_vals.iter().zip(card_counts.iter()) {
        if count == 0 {
            continue;
        }
        let draw = if val == 11 && curr_hand + 11 > 21 {
            1
        } else {
            val
        };

        if curr_hand + draw > 21 {
            bust_prob += count as f64 / total_remaining as f64;
        }
    }

    return (bust_prob);
}

fn probability_dealer_outcomes(
    //fn probability dealer_outcomes
    //Args
    //player total - total sum of player
    //dealer totoal - total sum of dealer
    //cards remaining - vector of cards remainin g
    //memoization - recomputes repeated weight prbability
    player_total: i32,
    dealer_total: i32,
    cards_remaining: &Vec<i32>,
    memo: &mut HashMap<(i32, i32, String), (f64, f64)>,
) -> (f64, f64) {
    if dealer_total > 21 {
        //If dealer busts, return 0.0 for both probabilities
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
        //if dealer_total is above 17, when the dealer stops in blackjack
        let result = if dealer_total > player_total {
            //if dealer wins
            (1.0, 0.0)
        } else if dealer_total == player_total {
            //if a tie
            (0.0, 1.0)
        } else {
            //if player wins
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

fn probability_next_blackjack(player_total: i32, cards_remaining: &Vec<i32>) -> f64 {
    //Function to return next_blackjack probability
    //Args
    //player total - current hand sum of player
    //cards_remaining - vector of decks
    let total_cards: i32 = cards_remaining.iter().sum(); //counts total cards remaining
    if total_cards == 0 || player_total >= 21 {
        //if no cards left or player has a blackjack
        return 0.0;
    }

    let needed = 21 - player_total; //needed = the value of card needed for there to be an immediate blackjack

    let count = match needed {
        //counts how many of the needed card value are left in the deck.
        1 => cards_remaining[0], // ace low (1)
        2..=9 => cards_remaining[needed as usize - 1],
        10 => cards_remaining[9] + cards_remaining[10] + cards_remaining[11] + cards_remaining[12],
        11 => cards_remaining[13], // ace high (11)
        _ => 0,
    };
    //returns the percentage of the amount of needed cards for a blackjack
    //that remain by the total amount of cards in the deck
    count as f64 / total_cards as f64
}

fn main() {
    //sets up the options variable used in the run_native() function
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };
    //This starts the UI
    run_native(
        "Blackjack Assistant",
        options,
        Box::new(|_cc| Box::new(BlackjackAid::default())),
    );
}
