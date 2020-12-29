use std::fs;
use std::str;

fn check_sum(base: i32, level: i32, nums: Vec<&str>) -> i32 {
    for x in nums.iter() {
        let nums_clone = nums.clone();
        let x_int: i32 = x.parse().unwrap();
        if level == 1 {
            let level2 = level + 1;
            let base2 = check_sum(x_int+base, level2, nums_clone);
            if base2 != 0 {
                println!("{} + {} + {:#?} == 2020", base,x_int, base2);
                println!("{} * {} * {:#?} = {}", base, x_int, base2, (base * x_int * base2));
            }
        }
        if x_int + base == 2020 { 
            return x_int;
        }
    }

    return 0;
}

fn main() {
    // read the input from the file
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");
    
    let split = contents.lines();

    let nums = split.collect::<Vec<&str>>();

    for x in nums.iter() {
        let nums_clone = nums.clone();
        let base_num: i32 = x.parse().unwrap();
        check_sum(base_num, 1, nums_clone);
    }

}
