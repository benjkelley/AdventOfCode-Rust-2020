use std::fs;
use std::collections::HashMap;

fn main() {
    // read the input from the file
    let filename = "input_test.txt";

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");
    
    let lines: Vec<&str> = contents.lines().collect();
    let required_fields: [&str; 8] = ["byr",
                                        "iyr",
                                        "eyr",
                                        "hgt",
                                        "hcl",
                                        "ecl",
                                        "pid",
                                        "cid"];

    let mut passports: Vec<HashMap<&str, &str>> = Vec::new();

    for line in lines.iter() {
        let mut current_passport = HashMap::new();
        if line.to_string() != "" {
            let elements: Vec<&str> = line.split(" ").collect();
            for item in elements.iter() {
                let key_value: Vec<&str> = item.split(":").collect();
                println!("{} {}", key_value[0], key_value[1]);
                current_passport.insert(key_value[0], key_value[1],);
            }
        }
        else {
            println!("{}", current_passport.len());
            passports.push(current_passport);
        }
    }

    let mut valid_total = 0;

    for passport in passports.iter() {
        let mut valid_passport = true;

        for (key, value) in &*passport {
            println!("{} {} {}", passports.len(), key, value);
        }

        for field in required_fields.iter() {
            if !(passport.contains_key(field)) {
                println!("Didn't find {}", field);
                valid_passport = false;
                break;
            }
        }

        if valid_passport == true {
            valid_total += 1;
        }

    }

    println!("Total Valid Passports: {}", valid_total);


}
