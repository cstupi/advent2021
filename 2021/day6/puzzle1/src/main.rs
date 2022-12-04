use std::fmt;

fn main() {
    let input = include_str!("input.txt");
    let mut animals: Vec<Lanternfish> = Vec::new();
    for line in input.lines() {
        animals = line.split(',').map(|x| Lanternfish::new(x.parse::<i32>().unwrap())).collect();
    }
    let mut count = animals.len();
    for _ in 0..256 {
        let temp = count;
        for i in 0..temp {
            match animals[i].pass_time(1) {
                Some(v) => {
                    count += 1;
                    animals.push(v);
                },
                None => {}
            }

        }
    }
    println!("Count: {}", count);
}


#[derive(Copy, Clone)]
struct Lanternfish {
    days_until_birth: i32
}

impl Lanternfish {
    fn new(days: i32) -> Lanternfish {
        return Lanternfish { days_until_birth: days };
    }
    fn give_birth(&mut self) -> Lanternfish {
        let baby = Lanternfish::new(8);
        self.days_until_birth = 6;
        return baby;
    }
    fn pass_time(&mut self, time:i32) -> Option<Lanternfish> {
        if self.days_until_birth == 0 {
            return Some(self.give_birth());
        }
        self.days_until_birth -= time;
        
        return None
    }
}

impl fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", &self.days_until_birth)
    }
}