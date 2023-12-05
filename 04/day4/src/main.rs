use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Card {
    id: i32,
    winning_nums: HashSet<String>,
    my_nums: Vec<String>
}

impl Card {
    fn new(s: &str) -> Card {
        let card_regex = Regex::new(r"Card[\s]+(?<id>\d+): (?<winning_nums>.*) \| (?<my_nums>.*)").unwrap();
        let Some(split_card) = card_regex.captures(&s) else { todo!() };

        let id = &split_card["id"];

        let winning_nums = split_card["winning_nums"].split(" ").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        let my_nums = split_card["my_nums"].split(" ").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

        Card {
            id: id.parse().unwrap(),
            winning_nums: winning_nums,
            my_nums: my_nums,
        }
    }

    fn get_card_score_1(&self) -> i32 {
        let count = self.my_nums.iter().filter(|s| self.winning_nums.contains(*s)).count() as u32;
        if count == 0 {
            0
        } else {
            2_i32.pow(count - 1)
        }
    }

    fn get_card_score_2(&self) -> i32 {
        self.my_nums.iter().filter(|s| self.winning_nums.contains(*s)).count() as i32
    }
}

fn get_card_score_1(cards: Vec<Card>) -> i32 {
    cards.iter().map(|card| card.get_card_score_1()).sum()
}

fn get_card_score_2(cards: Vec<Card>) -> i32 {
    let mut cards_for_id = HashMap::new();

    let mut total_cards = 0;
    
    cards.iter().for_each(|card| {
        let id_factor = *cards_for_id.entry(card.id).or_insert(1);

        let score = card.get_card_score_2();
        
        for i in card.id + 1..=card.id + score {
            let mut score_cards = cards_for_id.entry(i).or_insert(1);
            *score_cards += id_factor;
        }
        total_cards += id_factor;
    });

    total_cards
}

fn main() {
    let input_file = "input.txt";

    let mut cards = Vec::new();

    for line in read_to_string(input_file).unwrap().lines() {
        let card = Card::new(line);
        cards.push(card);
    }

    let result = get_card_score_2(cards);
    

    println!("{}", result);
}
