fn main() {
    let text = include_str!("input.txt");
    let input = text.lines();
    let oxygen = reduce(0, &find_most_common, &input.clone().collect::<Vec<&str>>());
    println!("Finished oxygen");
    let c02 = reduce(0, &find_least_common, &input.clone().collect::<Vec<&str>>());
    println!("{}, {}", oxygen, c02);
    println!("{}, {}", isize::from_str_radix(&oxygen, 2).unwrap(),isize::from_str_radix(&c02, 2).unwrap());
    println!("{}", isize::from_str_radix(&oxygen, 2).unwrap() * isize::from_str_radix(&c02, 2).unwrap());
}

fn find_most_common(idx: usize, values: &Vec<&str>) -> char {
    let mut ones: i32 = 0;
    let mut zeros: i32 = 0;
    for v in values {
        match v.chars().collect::<Vec<char>>()[idx] {
            '1' => ones = ones + 1,
            _ => zeros = zeros + 1 
        };
    }

    if ones < zeros {
        return '0'
    }
    return '1'
}

fn find_least_common(idx: usize, values: &Vec<&str>) -> char {
    let mut ones: i32 = 0;
    let mut zeros: i32 = 0;
    for v in values {
        match v.chars().collect::<Vec<char>>()[idx] {
            '1' => ones = ones + 1,
            _ => zeros = zeros + 1 
        };
    }

    if ones < zeros {
        return '1'
    }
    return '0'
}

fn reduce(idx:usize, find: &dyn Fn(usize, &Vec<&str>) -> char, values: &Vec<&str>) -> String {
    if values.len() == 1 {
        return values.first().unwrap().to_string();
    } 
    let winner = find(idx, values);
    println!("winner: {}", winner);
    let mut winners: Vec<&str> = Vec::new();
    for v in values.clone() {
        if v.chars().collect::<Vec<char>>()[idx] == winner {
            winners.push(v.clone());
        }
    }
    for v in winners.clone() {
        println!("{}", v);
    }
    return reduce(idx+1, find, &winners);
}