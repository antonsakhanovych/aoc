use std::{env, fs, path::Path, process};

fn parse_args() -> String {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR: Wrong number of arguments.");
        eprintln!("Usage: ./day2 input.txt");
        process::exit(1);
    }
    return args.remove(1);
}

fn find_maximum(i: u32, j: u32) -> u32 {
    if i > j {
        i
    } else {
        j
    }
}

fn multiply_tuple(draw: (u32, u32, u32)) -> u32 {
    draw.0 * draw.1 * draw.2
}

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<(u32, u32, u32)>,
}

impl Game {
    fn new(line: &str) -> Self {
        let mut splitted = line.split(":");
        let id: u32 = splitted
            .next()
            .expect("Malformed line")
            .split(" ")
            .last()
            .expect("Id number of the game")
            .parse()
            .expect("ID should be a number");

        let games: Vec<_> = splitted
            .last()
            .expect("All games")
            .trim()
            .split(";")
            .map(|set| {
                let mut res: (u32, u32, u32) = (0, 0, 0);
                let cubes = set.split(",");
                for cube in cubes {
                    let mut cube = cube.trim().split(" ");
                    let number: u32 = cube
                        .next()
                        .expect("Number")
                        .parse()
                        .expect("String given is not a number");
                    match cube.last().expect("String color") {
                        "red" => res.0 = number,
                        "green" => res.1 = number,
                        "blue" => res.2 = number,
                        _ => unreachable!("Cubes cannot have any other color."),
                    }
                }
                res
            })
            .collect();
        Self { id, draws: games }
    }

    fn adheres_constraints(&self) -> bool {
        return self
            .draws
            .iter()
            .all(|(r, g, b)| *r <= MAX_RED_CUBES && *g <= MAX_GREEN_CUBES && *b <= MAX_BLUE_CUBES);
    }

    fn find_minimum(&self) -> (u32, u32, u32) {
        let mut res: (u32, u32, u32) = (0, 0, 0);
        self.draws.iter().for_each(|(r, g, b)| {
            res.0 = find_maximum(res.0, *r);
            res.1 = find_maximum(res.1, *g);
            res.2 = find_maximum(res.2, *b);
        });
        res
    }
}

fn main() {
    let file_path_str = parse_args();
    let file_path = Path::new(&file_path_str);

    if !file_path.exists() {
        eprintln!("ERROR: Invalid path provided");
        process::exit(1);
    }
    let file_content: String = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("ERROR: couldn't read the file because of: {}", e);
            process::exit(1);
        }
    };
    let games: Vec<Game> = file_content
        .trim()
        .lines()
        .map(|line| Game::new(line))
        .collect();

    let part_1 = games
        .iter()
        .filter(|game| game.adheres_constraints())
        .map(|game| game.id)
        .reduce(|acc, id| acc + id)
        .expect("Sum of ids");

    let part_2: u32 = games
        .iter()
        .map(|game| game.find_minimum())
        .map(|minimum| multiply_tuple(minimum))
        .sum();

    println!("Part1: {part_1}");
    println!("Part2: {part_2}");
}
