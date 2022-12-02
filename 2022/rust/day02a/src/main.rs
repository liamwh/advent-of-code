use std::collections::HashMap;
fn main() {
    // Make a hashmap with predefined scores
    let scores = new_score_map();
    let decryption_map = new_decryption_map();
    let results_map = new_results_map();
    // Read the input file
    let strategy_guide = include_str!("../input.txt")
        .lines();

    let mut opponent_total_score = 0;
    let mut my_total_score = 0;
    // Iterate over the lines
    for round in strategy_guide {
        // Split the line into two parts
        let (encrypted_opponent_play, encrypted_my_play) = round.split_once(' ').unwrap();

        // Get the decrypted plays
        let decrypted_opponent_play = decryption_map.get(encrypted_opponent_play).unwrap();
        let decrypted_my_play = decryption_map.get(encrypted_my_play).unwrap();

        // Get the scores for each individual play
        let opponent_play_score = scores.get(decrypted_opponent_play).unwrap();
        let my_play_score = scores.get(decrypted_my_play).unwrap();

        // Get the scores for the round
        let (opponent_round_score, my_round_score) = get_round_score(decrypted_opponent_play, decrypted_my_play, &results_map, &scores);

        // Add the scores to the total
        opponent_total_score += opponent_round_score;
        my_total_score += my_round_score;
        opponent_total_score += opponent_play_score;
        my_total_score += my_play_score;

        // Print the round scores
        println!("{}: {} - {}: {}", decrypted_opponent_play, opponent_round_score + opponent_play_score, decrypted_my_play, my_round_score + my_play_score);
    }
    // Print the total scores
    println!("Opponent: {}", opponent_total_score);
    println!("Me: {}", my_total_score);
}

fn get_round_score(opponent_play: &String, my_play: &String, results_map: &ResultsMap, scores: &HashMap<String, i32>) -> (i32, i32) {
    let (opponent_result, my_result) = results_map.get(&(opponent_play.clone(), my_play.clone())).unwrap();
    let opponent_score = scores.get(opponent_result).unwrap();
    let my_score = scores.get(my_result).unwrap();
    (*opponent_score, *my_score)
}

fn new_score_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Round Win"), 6);
    scores.insert(String::from("Round Draw"), 3);
    scores.insert(String::from("Round Loss"), 0);
    scores.insert(String::from("Rock"), 1);
    scores.insert(String::from("Paper"), 2);
    scores.insert(String::from("Scissors"), 3);
    scores
}

fn new_decryption_map() -> HashMap<String, String> {
    let mut scores = HashMap::new();
    scores.insert(String::from("A"), String::from("Rock"));
    scores.insert(String::from("B"), String::from("Paper"));
    scores.insert(String::from("C"), String::from("Scissors"));
    scores.insert(String::from("X"), String::from("Rock"));
    scores.insert(String::from("Y"), String::from("Paper"));
    scores.insert(String::from("Z"), String::from("Scissors"));
    scores
}

type ResultsMap = HashMap<(String, String), (String, String)>;

// new_results_map creates a ResultsMap where the opponent play is the left key and my play is the right key
fn new_results_map() -> ResultsMap {
    let mut results_map = HashMap::new();
    results_map.insert((String::from("Rock"), String::from("Rock")), (String::from("Round Draw"), String::from("Round Draw")));
    results_map.insert((String::from("Rock"), String::from("Paper")), (String::from("Round Loss"), String::from("Round Win")));
    results_map.insert((String::from("Rock"), String::from("Scissors")), (String::from("Round Win"), String::from("Round Loss")));
    results_map.insert((String::from("Paper"), String::from("Rock")), (String::from("Round Win"), String::from("Round Loss")));
    results_map.insert((String::from("Paper"), String::from("Paper")), (String::from("Round Draw"), String::from("Round Draw")));
    results_map.insert((String::from("Paper"), String::from("Scissors")), (String::from("Round Loss"), String::from("Round Win")));
    results_map.insert((String::from("Scissors"), String::from("Rock")), (String::from("Round Loss"), String::from("Round Win")));
    results_map.insert((String::from("Scissors"), String::from("Paper")), (String::from("Round Win"), String::from("Round Loss")));
    results_map.insert((String::from("Scissors"), String::from("Scissors")), (String::from("Round Draw"), String::from("Round Draw")));
    results_map
}