fn main() {
    let inputtext = include_str!("input.txt").chars().collect::<Vec<char>>();
    let mut count = 0;
    let window = 14;
    for w in inputtext.as_slice().windows(window) {
        println!("{:?}", w);
        let mut unique = true;
        let mut i = 0;
        let mut j = 1;
        while i < window {
            j = i+1;
            while j < window {
                println!("{:?} != {:?}", w[i], w[j]);
                if(w[i] == w[j]) {
                    unique = false;
                }
                j += 1;
            }
            i += 1;
        }    

        if unique {
            println!("{}", count+window);
            return;
        } 
        count += 1;
    }
    println!("None found: {:?}", count);
}
