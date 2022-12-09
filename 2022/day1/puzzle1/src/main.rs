fn main() {
    let inputtext = include_str!("input.txt");
    let mut carried_cals = 0;
    let mut calories_per_elf: Vec<i32> = Vec::new();
    for line in inputtext.lines() {
        if let Ok(cals) = line.parse::<i32>() {
            carried_cals = carried_cals + cals;
        } else {
            calories_per_elf.push(carried_cals);
            carried_cals = 0;
        }
    }
    calories_per_elf.sort();
    let top_3:i32 = calories_per_elf.iter().rev().take(3).sum();
    println!("Top 3 Sum: {}", top_3);
}
