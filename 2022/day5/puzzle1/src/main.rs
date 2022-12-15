use itertools::Itertools;
struct ShippingMove {
    initial_stack: usize,
    end_stack: usize,
    count: i32
}

fn main() {
    let inputtext = include_str!("input.txt");
    let mut stacks_input: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<ShippingMove> = Vec::new();
    let mut stack_count = 0;
    // 4n+1 = bucket (i-1)/4
    for line in inputtext.lines() {
        if stack_count == 0 {
            stack_count = (line.chars().count()/4)+1;
            stacks_input.resize(stack_count, Vec::new());
        }
        for c in line.chars() {
            if c == 'm' {
                moves.push(parse_move_line(line));
                break;
            } else if c == '[' {
                let containers = parse_container_line(line, stack_count);
                for (i, v) in containers.iter().enumerate() {
                    stacks_input[i].push(*v);
                }
                break;
            }
        }
    }
    let mut stacks: Vec<Vec<char>> = stacks_input.into_iter().map(|x| x.into_iter().filter(|v| *v != '\0').rev().collect()).collect(); 



    for m in moves {
        let mut crane: Vec<char> = Vec::new();
        for _c in 0..m.count {
            crane.push(stacks[m.initial_stack-1].pop().unwrap());
        }
        for c in crane.into_iter().rev() {
            stacks[m.end_stack-1].push(c);
        }

    }

    for c in stacks {
        println!("{:?}", c);
    }
}


fn parse_container_line(line: &str, length: usize) -> Vec<char> {
    let mut containers: Vec<char> = Vec::new();
    containers.resize(length, '\0');
    for (i, c) in line.chars().enumerate() {
        if (c as u32 >= 97 && c as u32 <= 122) || c as u32 <= 90 && c as u32 >= 65 {
            let idx = (i-1)/4;
            containers[idx] = c;
        }
    }
    return containers;
}

fn parse_move_line(line: &str) -> ShippingMove {
    let (_,count,_, start,_, end) = line.split(' ').into_iter().next_tuple().unwrap();
    println!("{}, moving from {} to {}", count, start, end);
    return ShippingMove { 
        count: count.parse::<i32>().unwrap(), 
        initial_stack: start.parse::<usize>().unwrap(), 
        end_stack: end.parse::<usize>().unwrap()
    };
}

