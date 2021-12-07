fn main() {
    let text = include_str!("input.txt");
    let input = text.lines().map(|x| x.chars());
    let mut gamma_arr = [0,0,0,0,0,0,0,0,0,0,0,0];
    for chars in input {
        chars.enumerate().for_each(|(i, val)| {
            match val {
                '0' => gamma_arr[11-i] = gamma_arr[11-i] - 1,
                _ => gamma_arr[11-i] = gamma_arr[11-i] + 1
            }
        });
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for v in gamma_arr.iter().rev() {
        if *v >= 0 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    println!("{} * {}", gamma, epsilon);
    println!("{} * {}", isize::from_str_radix(&gamma, 2).unwrap(), isize::from_str_radix(&epsilon, 2).unwrap());
    println!("{}", isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap());
}
