fn main() -> Result<(), std::io::Error> {
    let text = include_str!("input.txt");
    let mut count = 0;
    let data:Vec<i32> = text.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 2..data.len()-1 {
        let left:i32 = data.iter().nth(i-2).unwrap() + data.iter().nth(i-1).unwrap() + data.iter().nth(i).unwrap();
        let right:i32 = data.iter().nth(i-1).unwrap() + data.iter().nth(i).unwrap() + data.iter().nth(i+1).unwrap();
        if left < right {
            count = count + 1;
        }
    }
    println!("{}", count);
    Ok(())
}
