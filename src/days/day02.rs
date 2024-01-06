use std::fs;
use crate::{Solution, SolutionPair};

struct Round {
    count_red_box: u32,
    count_blue_box: u32,
    count_green_box: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_round(input: &str) -> Round {;
    let cubes = input.trim().split(", ").collect::<Vec<&str>>();
    let mut count_red_box = 0;
    let mut count_green_box = 0;
    let mut count_blue_box = 0;

    cubes.iter().for_each(|c| {
        let split = c.split(" ").collect::<Vec<&str>>();
        let count = split[0].parse::<u32>().expect("Cube count should be an int");

        match split[1] {
            "red" => count_red_box = count,
            "green" => count_green_box = count,
            "blue" => count_blue_box = count,
            _ => unimplemented!()
        };
    });

    Round {
        count_red_box,
        count_blue_box,
        count_green_box,
    }
}

fn parse_game(input: &str) -> Game {
    let game_id_split = input.split_at(5).1.split(": ").collect::<Vec<&str>>();
    let game_id = game_id_split[0];
    let rounds_split = game_id_split[1].split(";").collect::<Vec<&str>>();

    let rounds = rounds_split.iter().map(|round| {
        parse_round(round)
    }).collect::<Vec<Round>>();

    Game {
        id: game_id.parse::<u32>().expect("GameID should be able to parse to int"),
        rounds
    }
}

pub fn solve() -> SolutionPair {
    let input_path = "input/day2.txt";
    let input = fs::read_to_string(input_path).expect("Should be able to read the file contents");

    (Solution::from(solve_first(&input)), Solution::from(solve_second(&input)))
}

fn solve_first(input: &String) -> u32 {
    let mut games = Vec::new();

    input.lines().for_each(|line| {
        games.push(parse_game(line));
    });

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    games.iter().map(|game| {
        for round in &game.rounds {
            if round.count_red_box > MAX_RED { return 0; }
            if round.count_green_box > MAX_GREEN { return 0; }
            if round.count_blue_box > MAX_BLUE { return 0; }
        }
        return game.id
    }).sum()
}

fn solve_second(input: &String) -> u32 {
    let mut games = Vec::new();

    input.lines().for_each(|line| {
        games.push(parse_game(line));
    });

    games.iter().map(|game| {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for round in &game.rounds {
            if round.count_red_box > min_red { min_red = round.count_red_box; }
            if round.count_green_box > min_green { min_green = round.count_green_box; }
            if round.count_blue_box > min_blue { min_blue = round.count_blue_box; }
        }

        return min_red * min_green * min_blue;
    }).sum()
}
