fn main() {
    let input = include_str!("input.txt");
    let locations: Vec<i32> = input.lines().flat_map(|l| l.split(',').flat_map(|x| x.parse::<i32>())).collect();
    let avg = average(&locations);
    let min = search(1, 500, &locations, avg);
    println!("{}", min);
}

fn average(arr: &[i32]) -> i32 {
    if arr.len() == 0 {
        return 0;
    }
    let mut x = 0;
    for v in arr {
        x += v;
    }
    return x/(arr.len() as i32);
}

fn calc_move_cost(point: i32, locations: &[i32]) -> i32 {
    let mut total_distance = 0;
    for l in locations {
        let n = (point - l).abs();
        total_distance += (n * (n+1))/2;
    }
    return total_distance;
}

fn search(min_spread: usize, max_spread: usize, arr:&[i32], start_point: i32) -> i32 {
    let mut min = calc_move_cost(start_point, &arr);
    for i in min_spread..max_spread {
        let low = calc_move_cost(start_point - (i as i32), &arr);
        let high = calc_move_cost(start_point + (i as i32), &arr);
        if low < min {
            min = low;
        }
        if high < min {
            min = high;
        }
    }
    return min;
}