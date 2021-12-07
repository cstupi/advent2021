fn main() -> Result<(), std::io::Error> {
    let text = include_str!("input.txt");
    let mut count = 0;
    let mut previous = std::i32::MAX;
    let data:Vec<i32> = text.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    for v in data {
        if v > previous {
            count = count + 1;
        }
        previous = v;
    }
    println!("{}", count);
    Ok(())
}
