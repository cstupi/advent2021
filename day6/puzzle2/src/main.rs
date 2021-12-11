fn main() {
    let input = include_str!("input.txt");
    let mut animals: [u64; 9] = [0; 9];
    for line in input.lines() {
        for fish in line.split(',') {
            animals[fish.parse::<usize>().unwrap()] += 1;
        }
    }

    for _ in 0..256 {
        let mut prev = animals[8];
        for i in (1..animals.len()-1).rev() {
            let mut temp = animals[i];
            animals[i] = prev;
            prev = temp;
        }
        animals[6] += animals[0];
        animals[8] = animals[0];
        animals[0] = prev;
    }
    println!("Count: {}", animals.iter().sum::<u64>());
}
