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
     probability_busting(4,curr_hand,&card_vals,&mut card_counts);
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
//'curr_hand' - total current hand
//'card_Val' - vector of card values with faces and numbers
//'card_counts' - 'card counts which holds how many total cards in the vector remaining
fn probability_busting(
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

fn probability_dealer_win(
    curr_hand: i32,
    card_vals: &Vec<i32>,
    card_counts: &mut Vec<i32>, curr_dealer_hand: i32
) -> f64{
    if curr_dealer_hand > 21{
         println!("{:?}",curr_dealer_hand);
        return 0.0;
    }
    if curr_dealer_hand >= 17{
        if curr_dealer_hand > curr_hand{
            return 1.0;
        }
        else{
            return 0.0;
        }
    }
    let total_remaining_deck: i32 = card_counts.iter().sum(); 
    let mut win_prob: f64 = 0.0;
    for (i, &val) in card_counts.iter().enumerate(){
        if card_counts[i] == 0{
            continue;
        }
         let mut next_card_count: Vec<i32> = card_counts.clone();
         next_card_count[i] -= 1;
          println!("Current state of card count vector for that branch{:?}", next_card_count);
         let mut curr_prob: f64 = card_counts[i] as f64/total_remaining_deck as f64;
         let mut next_total_hand: i32 = val + curr_dealer_hand;
         win_prob += curr_prob * probability_dealer_win(curr_hand,&card_vals,&mut next_card_count, next_total_hand);
    }
    return win_prob;

}



