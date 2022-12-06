use advent_of_code_2022::read_file;

const DATA_PATH: &str = "day1_data.txt";

fn get_elf_cals() -> Vec<i32> {
    let lines: Vec<String> = read_file(DATA_PATH).collect();
    let mut elf_cals: Vec<i32> = vec![];

    let mut current_cals = 0;
    for line in lines {
        if line == "" {
            elf_cals.push(current_cals);
            current_cals = 0;
            continue;
        }
        let cals: i32 = line
            .parse::<i32>()
            .expect("Couldnt convert string cals to number");

        current_cals += cals;
    }
    elf_cals
}

fn top_three() {
    let mut elf_cals = get_elf_cals();

    elf_cals.sort();
    elf_cals.reverse();

    println!(
        "Top three elves total cals is: {}",
        &elf_cals[0..=2].iter().sum::<i32>()
    );
}

fn top_elf() {
    let elf_cals = get_elf_cals();
    println!("Top elf is: {}", elf_cals.iter().max().unwrap());
}

fn main() {
    top_elf();
    top_three();
}
