use itertools::Itertools;
fn main() {
    let inputtext = include_str!("input.txt");
    let mut count = 0;
    for line in inputtext.lines() {
        let ranges = line.split(",").collect::<Vec<&str>>();
        let (start1,end1) = ranges.get(0).unwrap().split("-").map(|c| c.parse::<i32>().unwrap()).into_iter().next_tuple().unwrap();
        let (start2, end2) = ranges.get(1).unwrap().split("-").map(|c| c.parse::<i32>().unwrap()).into_iter().next_tuple().unwrap();
        if (start1 <= start2 && end1 >= start2) || (start1 >= start2 && end2 >= start1) {
            count += 1;
        }
    }
    println!("Enclosures: {}", count);
}
