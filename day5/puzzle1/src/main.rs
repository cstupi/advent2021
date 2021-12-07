use std::fmt;

fn main() {
    let input = include_str!("input.txt");
    let parsed_lines = input.lines().map(|l| {
        let mut split = l.split_whitespace();
        let mut start = split.next().unwrap().split(',');
        split.next();
        let mut end = split.next().unwrap().split(',');
        let start_point = Point {
            x: start.next().unwrap().parse::<i32>().unwrap(),
            y: start.next().unwrap().parse::<i32>().unwrap()
        };
        let end_point = Point {
            x: end.next().unwrap().parse::<i32>().unwrap(),
            y: end.next().unwrap().parse::<i32>().unwrap()
        };
        return Line { start: start_point, end: end_point };
    });
    let mut map: Vec<Vec<usize>> = Vec::new();

    for _ in 0..10 {
        map.push(vec![0; 10]);
    }
    for line in parsed_lines {
        println!("{}", line);
        if line.start.y <= line.end.y {
            if line.start.x <= line.end.x {
                for i in line.start.y..line.end.y+1 {
                    for j in line.start.x..line.end.x+1 {
                        map[i as usize][j as usize] = map[i as usize][j as usize] + 1
                    }
                }
            } else {
                println!("starty <= endy startx > endx");
                for i in line.start.y..line.end.y+1 {
                    let mut j:i32 = line.start.x;
                    while j >= line.end.x {
                        println!("{},{} for point {}", j, i, line);
                        map[i as usize][j as usize] += 1;
                        j = j-1;
                    }
                }
            }
        } else {
            if line.start.x <= line.end.x {
                for i in (line.start.y..line.end.y+1).rev() {
                    for j in line.start.x..line.end.x+1 {
                        map[i as usize][j as usize] = map[i as usize][j as usize] + 1
                    }
                }
            } else {
                for i in (line.start.y..line.end.y+1).rev() {
                    for j in (line.start.x..line.end.x+1).rev() {
                        map[i as usize][j as usize] = map[i as usize][j as usize] + 1;
                    }
                }
            }
        }
        print_grid(&map);
    }
    print_grid(&map);
}
fn print_grid(grid:&Vec<Vec<usize>>) {
    for row in grid {
        for val in row {
            print!("{}\t", val)
        }
        println!("");
    }
}
struct Point {
    x:i32,
    y:i32
}
struct Line {
    start:Point,
    end:Point
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{},{} -> {},{}", self.start.x, self.start.y, self.end.x, self.end.y);
    }
}