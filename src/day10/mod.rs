use crate::common::array::to_2d_index;

pub fn solve_part_one(input: String) -> String {
    const IMPORTANT_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

    let mut cycles = 0;
    let mut x_register = 1;
    let mut signal_strengths = Vec::new();

    for instruction in parse_input(&input) {
        let mut instruction_cycles = instruction.num_cycles();
        while instruction_cycles > 0 {
            cycles += 1;
            if IMPORTANT_CYCLES.contains(&cycles) {
                signal_strengths.push(cycles * x_register);
            }
            if instruction_cycles == 1 {
                match instruction {
                    Instruction::Addx(num) => x_register += num,
                    Instruction::Noop => (),
                }
            }
            instruction_cycles -= 1;
        }
    }

    signal_strengths.into_iter().sum::<i32>().to_string()
}

pub fn solve_part_two(input: String) -> String {
    const CRT_WIDTH: usize = 40;

    let mut cycles = 0;
    let mut sprite_x = 1;
    let mut screen = String::new();

    for instruction in parse_input(&input) {
        let mut instruction_cycles = instruction.num_cycles();
        while instruction_cycles > 0 {
            cycles += 1;
            let (x_position, _) = to_2d_index(cycles - 1, CRT_WIDTH as usize);
            let ch = if (sprite_x - 1..=sprite_x + 1).contains(&x_position) {
                '#'
            } else {
                '.'
            };
            screen.push(ch);
            let reached_newline = cycles % 40 == 0;
            if reached_newline {
                screen.push('\n');
            }
            if instruction_cycles == 1 {
                match instruction {
                    Instruction::Addx(num) => sprite_x = (sprite_x as i32 + num) as usize,
                    Instruction::Noop => (),
                }
            }
            instruction_cycles -= 1;
        }
    }
    screen
}

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    pub const ADDX_NUM_CYCLES: u32 = 2;
    pub const NOOP_NUM_CYCLES: u32 = 1;

    pub fn from_input(input: &str) -> Self {
        let instruction_name = &input[0..4];
        match instruction_name {
            "noop" => Self::Noop,
            "addx" => {
                let count = &input[5..];
                Self::Addx(count.parse().unwrap())
            }
            _ => unreachable!(),
        }
    }

    pub const fn num_cycles(&self) -> u32 {
        match self {
            Instruction::Addx(_) => Self::ADDX_NUM_CYCLES,
            Instruction::Noop => Self::NOOP_NUM_CYCLES,
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_input).collect()
}
