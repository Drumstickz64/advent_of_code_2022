pub fn solve_part_one(input: String) -> String {
    input
        .lines()
        .map(Round::from_part_one_input)
        .map(|round| round.get_score())
        .sum::<i32>()
        .to_string()
}

pub fn solve_part_two(input: String) -> String {
    input
        .lines()
        .map(Round::from_part_two_input)
        .map(|round| round.get_score())
        .sum::<i32>()
        .to_string()
}

#[derive(Debug, Clone, Copy)]
struct Round {
    my_shape: Shape,
    opponent_shape: Shape,
}

impl Round {
    pub fn from_part_one_input(input: &str) -> Self {
        Self {
            my_shape: Shape::from_symbol(&input[2..3]),
            opponent_shape: Shape::from_symbol(&input[0..1]),
        }
    }

    pub fn from_part_two_input(input: &str) -> Self {
        let opponent_shape = Shape::from_symbol(&input[0..1]);
        let outcome = MatchOutcome::from_symbol(&input[2..3]);
        let my_shape = Shape::from_outcome(outcome, opponent_shape);
        Self {
            my_shape,
            opponent_shape,
        }
    }

    pub fn get_score(self) -> i32 {
        let shape_score = self.my_shape.get_score();
        let outcome = self.my_shape.matchup(self.opponent_shape);
        shape_score + outcome.get_score()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock = 1,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Incorrect symbol: {symbol}"),
        }
    }

    pub fn from_outcome(outcome: MatchOutcome, opponent_shape: Shape) -> Self {
        use MatchOutcome::*;
        use Shape::*;

        match (outcome, opponent_shape) {
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Draw, _) => opponent_shape,
        }
    }

    pub fn get_score(self) -> i32 {
        self as i32
    }

    pub fn matchup(self, other: Self) -> MatchOutcome {
        use MatchOutcome::*;
        use Shape::*;

        if self == other {
            return Draw;
        }

        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Lose,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MatchOutcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl MatchOutcome {
    pub fn get_score(self) -> i32 {
        self as i32
    }

    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Incorrect symbol"),
        }
    }
}
