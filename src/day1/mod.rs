type Calories = i32;

pub fn solve_part_one(input: String) -> String {
    let elves_calories = calculate_elves_calories(input);
    elves_calories.into_iter().max().unwrap().to_string()
}

pub fn solve_part_two(input: String) -> String {
    let mut elves_calories = calculate_elves_calories(input);
    elves_calories.sort_unstable();
    let length = elves_calories.len();
    elves_calories[(length - 3)..length]
        .iter()
        .sum::<Calories>()
        .to_string()
}

fn calculate_elves_calories(input: String) -> Vec<Calories> {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<Calories>().unwrap())
                .sum()
        })
        .collect()
}
