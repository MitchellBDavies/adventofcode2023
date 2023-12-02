use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

struct Game {
    id: i32,
    cube_pulls: Vec<HashMap<String, i32>>,
}

impl Game {
    fn new(s: String) -> Game {
        let id_game_split = Regex::new(r"Game (?<id>[0-9]+): (?<pulls>.*)").unwrap();
        let score_color_split = Regex::new(r"(?<score>[0-9]+) (?<color>.*)").unwrap();
        let Some(caps) = id_game_split.captures(&s) else { todo!() };
        let mut pulls: Vec<HashMap<String, i32>> = Vec::new();
        for i in caps["pulls"].split(";") {
            let rounds = i.split(',');
            let mut hash_map = HashMap::new();
            for j in rounds {
                let Some(parsed_round) = score_color_split.captures(&j) else { todo!() };
                hash_map.insert(parsed_round["color"].to_string(), parsed_round["score"].parse().unwrap());
            }
            pulls.push(hash_map);
        }
        Game {
            id: caps["id"].parse().unwrap(),
            cube_pulls: pulls,
        }
    }
}

fn get_game_score_1(parsed_game: Game) -> i32 {
    let game_results = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    let impossible_scores = parsed_game.cube_pulls.iter()
        .flat_map(|hm| {
            hm.iter()
            .map(move |(k,v)| {
                (k, v)
            }).filter(|(k, v)| {
                game_results.get(*k).unwrap() < *v
            })
        })
        .count();
    if impossible_scores == 0 {
        parsed_game.id
    } else {
        0
    }
}

fn get_game_score_2(parsed_game: Game) -> i32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    parsed_game.cube_pulls.iter()
        .for_each(|hm| {
            hm.iter()
                .for_each(|(k,v)| {
                    if k == "red" {
                        red = std::cmp::max(red, *v);
                    }
                    if k == "green" {
                        green = std::cmp::max(green, *v);
                    }
                    if k == "blue" {
                        blue = std::cmp::max(blue, *v);
                    }
                });
        });
    
    red * green * blue
}

fn main() {
    let input_file = "input.txt";
    let mut result = 0;



    for line in read_to_string(input_file).unwrap().lines() {
        let parsed_game = Game::new(line.to_string());
        result += get_game_score_2(parsed_game);
    }

    println!("{}", result);
}
