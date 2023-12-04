pub struct Round {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

pub struct Game {
    pub id: i32,
    pub rounds: Vec<Round>,
    pub max_red: i32,
    pub max_green: i32,
    pub max_blue: i32,
}

impl Game {
    pub fn is_valid(&self) -> bool {
        const RED_CUBES: i32 = 12;
        const GREEN_CUBES: i32 = 13;
        const BLUE_CUBES: i32 = 14;

        for round in &self.rounds {
            if round.red > RED_CUBES {
                return false;
            }

            if round.green > GREEN_CUBES {
                return false;
            }

            if round.blue > BLUE_CUBES {
                return false;
            }
        }

        true
    }
}
