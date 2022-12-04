fn main() {
    let input = include_str!("input.txt");
    let commands = input.lines().map(|x| x.split_whitespace());
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;
    let mut aim: i32 = 0;
    for split in commands {
        let instructions: Vec<&str> = split.collect();
        let command = instructions.first().unwrap();
        let distance: i32 = instructions.last().unwrap().parse::<i32>().unwrap();
        match *command {
            "up" => aim = aim - distance,
            "down" => { aim = aim + distance;
            println!("Aim increasing by {} to {}", distance, aim) },
            _ => {
                println!("{}", command);
                vertical = vertical + (aim * distance);
                horizontal = horizontal + distance;  
            }
        }
    }
    println!("{},{}, {}", horizontal, vertical, aim);
    println!("{}", horizontal * vertical);
}
