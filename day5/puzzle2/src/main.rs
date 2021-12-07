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
    let mut map: Vec<Vec<i32>> = Vec::new();

    for _ in 0..1000 {
        map.push(vec![0; 1000]);
    }
    for line in parsed_lines {
        if line.start.y == line.end.y {
            let mut start: usize = line.start.x as usize;
            let mut end: usize = line.end.x as usize;
            if line.start.x > line.end.x {
                start = line.end.x as usize;
                end = line.start.x as usize;
            }
            for j in start..end+1 {
                map[line.start.y as usize][j] += 1;
            }
        } else if line.start.x == line.end.x {
            let mut start: usize = line.start.y as usize;
            let mut end: usize = line.end.y as usize;
            if line.start.y > line.end.y {
                start = line.end.y as usize;
                end = line.start.y as usize;
            }
            for j in start..end+1 {
                map[j][line.start.x as usize] += 1;
            }
        } else {
            // diagonal
            let mut startx = line.start.x;
            let mut starty = line.start.y;
            let mut endy = line.end.y;
            if line.start.x > line.end.x {
                // swap so we always go right
                startx = line.end.x;
                starty = line.end.y;
                endy = line.start.y;
            }
            let direction;
            let mut x = startx;
            let mut y;
            // determine down or up
            if starty < endy {
                direction  =  1;
                y = starty;
            } else  {
                direction = -1;
                y = starty;
            }
            while  y != endy {
                map[y as usize][x as usize] += 1;
                y += direction;
                x += 1; // always right
            }
        }
    }
    println!("Count: {}", count_highs(2, &map));
}
fn count_highs(min_val: i32, grid:&Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for row in grid {
        for val in row {
            if val >= &min_val {
                count += 1;
            }
        }
    }
    return count;
}
fn print_grid(grid:&Vec<Vec<i32>>) {
    for row in grid {
        for val in row {
            print!("{},", val)
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