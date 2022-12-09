use itertools::Itertools;
fn main() {
    let inputtext = include_str!("input.txt");
    let mut score = 0;
    let lines_iter = inputtext.lines();
    for mut chunk in &lines_iter.chunks(3) {
        let (first, second, third) = &chunk.next_tuple().unwrap();
        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                println!("The character is: {}", c);
                if c as u32 <= 90 && c as u32 >= 65 {
                    score += (c as u32) - 38;
                } else if c as u32 >= 97 && c as u32 <= 122 {
                    score += (c as u32) - 96;
                }
                break;
            }
        }
        println!("{},{},{}", first,second,third);
    }
    score = score;

    println!("{}", score);
}
/*
if c as u32 <= 90 && c as u32 >= 65 {
    score += (c as u32) - 38;
} else if c as u32 >= 97 && c as u32 <= 122 {
    score += (c as u32) - 96;
}
*/