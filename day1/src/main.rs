use std::fs;
use std::str;

fn main() {
    // read the input from the file
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");
    
    let split = contents.lines();

    let nums = split.collect::<Vec<&str>>();

    for x in nums.iter() {
        let base_num: u32 = x.parse().unwrap();
        for y in nums.iter() {
            let base_num2: u32 = y.parse().unwrap();
            if base_num2 + base_num == 2020 {
                println!("{} + {} = 2020", base_num, base_num2);
                println!("{} * {} = {}", base_num, base_num2, (base_num * base_num2));
            }
        }
    }

}
