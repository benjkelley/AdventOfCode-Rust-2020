use std::fs;

fn check_valid(low: i32, high: i32, letter: &str, pass: &str) -> bool {
    let mut letter_count = 0;
    
    let split: Vec<char> = pass.chars().collect();

    for l in split.iter() {
        if l.to_string() == letter
        {
            letter_count += 1;
        }
    }

    if letter_count >= low && letter_count <= high {
        return true;
    }
    else {
        return false;
    }
}


fn main() {
    // read the input from the file
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");
    
    let split = contents.lines();

    let lines = split.collect::<Vec<&str>>();
    let mut valid_count = 0;
    
    for line in lines.iter() {
        let split = line.split(":");
        let items = split.collect::<Vec<&str>>();
        let rule = items[0];
        let pwd = items[1].trim();

        let split2 = rule.split(" ");
        let vals = split2.collect::<Vec<&str>>();
        let range = vals[0];
        let letter = vals[1];

        let split3 = range.split("-");
        let vals2 = split3.collect::<Vec<&str>>();
        let low: i32 = vals2[0].parse().unwrap();
        let high: i32 = vals2[1].parse().unwrap();
        
        let is_valid = check_valid(low, high, &letter, &pwd);
        if is_valid == true {
            valid_count += 1;
        }
    }
    println!("Valid Passwords: {}", valid_count);
}
