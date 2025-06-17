fn main() {
    let num_decks = 1; // will be inputted by user

    // names of the cards to call indices for count and value vectors
    let card_names = vec!["ace".to_string(), "two".into(), "three".into(), 
        "four".into(), "five".into(), "six".into(), "seven".into(), "eight".into(), 
        "nine".into(), "ten".into(), "jack".into(), "queen".into(), "king".into()];

    let card_vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10]; // values of the cards in order

    
    // hands--eventually will be selected from display
    let hand = ["five".to_string(), "seven".into()];
    let dealer_hand = ["king".to_string()];



    
    let mut card_counts = vec![4*num_decks; 13]; // number of cards remaining in order

    // calculate player hand value and change card counts
    let mut curr_hand = 0;
    for card in hand {
        let index = card_index(&card);
        curr_hand += card_vals[index];
        card_counts[index] -= 1;
    }

    // calculate dealer hand value and change card counts
    let mut curr_dealer_hand = 0;
    for card in dealer_hand {
        let index = card_index(&card);
        curr_dealer_hand += card_vals[index];
        card_counts[index] -= 1;
    }


    // temporary prints to track
    println!("{:?}", card_counts);
    println!("Current player hand: {}\nDealer hand: {}", curr_hand, curr_dealer_hand);
     probability_chosen_card(4,curr_hand,&card_vals,&mut card_counts);
}



// function matching the names of the cards to the value vector and counts-tracking vector
fn card_index(card: &str) -> usize {
    match card {
        "ace"   => 0,
        "two"   => 1,
        "three" => 2,
        "four"  => 3,
        "five"  => 4,
        "six"   => 5,
        "seven" => 6,
        "eight" => 7,
        "nine"  => 8,
        "ten"   => 9,
        "jack"  => 10,
        "queen" => 11,
        "king"  => 12,
        other   => panic!("Unknown card face: {}", other),
    }
}



//if you chose this num, fn to provide the probability of busting 
//Args
// `val`: value of the card 
fn probability_chosen_card(
    val: i32,
    curr_hand: i32,
    card_vals: &Vec<i32>,
    card_counts: &mut Vec<i32>,
){
      //calculate of a player getting a bust 
    if let Some(index) = card_vals.iter().position(|&x| x == val) {
        let mut temp_hand = curr_hand + card_vals[index];
        if card_counts[index] > 0 {
            card_counts[index] -= 1;
        }
    println!("{:?}", card_counts);
    let mut remaining_bust_nums: f64 = 0.0;
    let total_remaining_deck: i32 = card_counts.iter().sum();  //total remaining cards 
     for (i, &val) in card_vals.iter().enumerate(){ //loop to get val and its current index 
        if val > 21 - temp_hand && card_counts[i] > 0{ //check if val greater than 
          //  println!("Probability of drawing that a {}: {:.2}", card_names[i], card_counts[i] as f64/total_remaining_deck as f64);
            remaining_bust_nums  +=  card_counts[i] as f64
    }
}
    println!("{:.2}", remaining_bust_nums/total_remaining_deck as f64);

}
}



