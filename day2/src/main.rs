use regex::Regex;
use std::fs;
use std::io;
use std::io::BufRead;

static INPUT_PATH: &str = "./rsrc/input.txt";

static GAME_ID_REG: &str = "Game (?P<id>\\d+):";
static GAME_GREEN_REG: &str = "(?i)(?P<n>\\d+) green";
static GAME_BLUE_REG: &str = "(?i)(?P<n>\\d+) blue";
static GAME_RED_REG: &str = "(?i)(?P<n>\\d+) red";

fn main() {
    println!("Hello, world!");
    let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");

    // compile the regexes
    let reg_id = Regex::new(GAME_ID_REG).unwrap();
    let reg_green = Regex::new(GAME_GREEN_REG).unwrap();
    let reg_red = Regex::new(GAME_RED_REG).unwrap();
    let reg_blue = Regex::new(GAME_BLUE_REG).unwrap();

    // loop sur les lignes
    let reader = io::BufReader::new(f);
    let mut valid_id: Vec<i32> = Vec::new();
    let mut power_sum: u32 = 0;
    for line in reader.lines() {
        let line = line.expect("Pas du UTF-8 valide...");
        let id = reg_id.captures(&line).expect("Pas d'ID de partie");
        let pull_start = id.get(0).unwrap().end();
        let id: i32 = id.name("id").unwrap().as_str().parse().unwrap();

        let valid_fn = is_valid;
        let pulls = line[pull_start..].split(';');
        let pulls: Vec<Pull> = pulls
            .map(|pull| {
                let red = reg_red
                    .captures(pull)
                    .map_or(0, |m| m.name("n").unwrap().as_str().parse().unwrap());
                let blue = reg_blue
                    .captures(pull)
                    .map_or(0, |m| m.name("n").unwrap().as_str().parse().unwrap());
                let green = reg_green
                    .captures(pull)
                    .map_or(0, |m| m.name("n").unwrap().as_str().parse().unwrap());
                Pull { red, blue, green }
            })
            .collect();
        println!("Game {}:", id);
        let mut pull_num = 0;
        let mut max_pull = Pull::default();
        let mut v = true;
        pulls.iter().for_each(|p| {
            pull_num += 1;
            v = v && valid_fn(p);
            println!("\tPull {}: {} {}", pull_num, p, if v { '✅' } else { '❌' });
            if p.red > max_pull.red {
                max_pull.red = p.red;
            }
            if p.green > max_pull.green {
                max_pull.green = p.green;
            }
            if p.blue > max_pull.blue {
                max_pull.blue = p.blue;
            }
        });
        if v {
            valid_id.push(id);
        }
        let power = max_pull.red * max_pull.green * max_pull.blue;
        power_sum += power;
        println!("\tPower: {} {}", max_pull, power);
    }
    println!("Valid ids: {:?}", valid_id);
    println!("sum: {}", valid_id.iter().sum::<i32>());
    println!("Power Sum: {}", power_sum);
}

#[derive(Debug, Default, Clone, Copy)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}
impl std::fmt::Display for Pull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}; {}; {})", self.red, self.green, self.blue)
    }
}

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;
fn is_valid(pull: &Pull) -> bool {
    pull.red <= MAX_RED && pull.green <= MAX_GREEN && pull.blue <= MAX_BLUE
}
