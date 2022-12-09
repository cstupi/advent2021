fn main() {
    let inputtext = include_str!("input.txt");
    let mut score = 0;
    for line in inputtext.lines() {
        let (container1, container2) = line.split_at(line.len()/2);
        let container2_list = container2.chars().collect::<Vec<char>>();
        for c in container1.chars() {
            if container2_list.contains(&c) {
                if c as u32 <= 90 && c as u32 >= 65 {
                    score += (c as u32) - 38;
                } else if c as u32 >= 97 && c as u32 <= 122 {
                    score += (c as u32) - 96;
                }
                break;
            }
        }


    }
    println!("A: {} - Z: {}. a: {} - z: {}", 'A' as u8, 'Z' as u8, 'a' as u8, 'z' as u8);
    println!("Score: {}", score);
}
