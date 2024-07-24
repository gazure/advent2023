use std::str::FromStr;

#[derive(Debug)]
struct CubeSet {
    blue: u32,
    red: u32,
    green: u32,
}

impl CubeSet {
    pub fn new(blue: u32, red: u32, green: u32) -> Self {
        Self {
            blue,
            red,
            green,
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.blue >= other.blue && self.red >= other.red && self.green >= other.green
    }

    pub fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

impl FromStr for CubeSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors: Vec<_> = s.split(",").collect();
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        colors.iter().for_each(|color_str| {
            let color_str = color_str.trim();
            let split: Vec<_> = color_str.split(" ").collect();
            let quantity = split.first().unwrap().parse::<u32>().unwrap();
            let color = split.last().unwrap();
            match *color {
                "blue" => { blue = quantity }
                "red" => { red = quantity }
                "green" => { green = quantity }
                _ => {}
            }
        });

        Ok(Self::new(blue, red, green))
    }
}

#[derive(Debug)]
struct CubeGame {
    game_num: u32,
    grabs: Vec<CubeSet>,
}

impl CubeGame {
    pub fn new(game_num: u32, grabs: Vec<CubeSet>) -> Self {
        Self {
            game_num,
            grabs,
        }
    }

    pub fn valid_with_cubeset(&self, cube_set: &CubeSet) -> u32 {
        let mut game_is_valid = true;
        for grab in &self.grabs {
            if !cube_set.contains(grab) {
                game_is_valid = false;
            }
        }
        if game_is_valid {
            self.game_num
        } else {
            0
        }
    }
}

impl FromStr for CubeGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(":").collect();
        if parts.len() != 2 {
            return Err(());
        }
        let game_num_str = parts[0];
        let cubesets_str = parts[1];

        let game_num = game_num_str.split(" ").last().unwrap().parse::<u32>().unwrap();

        let cubesets: Vec<CubeSet> = cubesets_str.split(";").map(|s| s.parse::<CubeSet>().unwrap()).collect();

        Ok(Self::new(game_num, cubesets))
    }
}

fn part_one(input: &Vec<CubeGame>) {
    let cube_set = CubeSet::new(14, 12, 13);
    let mut sum = 0;
    for game in input {
        sum += game.valid_with_cubeset(&cube_set);
    }
    println!("{sum}");
}

fn part_two(input: &Vec<CubeGame>) {
    let mut power = 0;

    for game in input {
        let mut min_cube_set = CubeSet::new(0, 0, 0);
        for grab in &game.grabs {
            min_cube_set.red = std::cmp::max(min_cube_set.red, grab.red);
            min_cube_set.green = std::cmp::max(min_cube_set.green, grab.green);
            min_cube_set.blue = std::cmp::max(min_cube_set.blue, grab.blue);
        }
        power += min_cube_set.power();
    }

    println!("{power}")
}

fn parse(input: &str) -> Vec<CubeGame> {
    input.lines().map(|line| line.parse::<CubeGame>().unwrap()).collect()
}

pub fn run() {
    let input = parse(include_str!("../data/day2input.txt"));
    part_one(&input);
    part_two(&input);
}