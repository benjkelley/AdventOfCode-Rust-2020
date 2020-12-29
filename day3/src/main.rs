use std::fs;

fn main() {
    // read the input from the file
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");
    
    let lines: Vec<&str> = contents.lines().collect();

    let mut current_col = 0;
    let mut tree_count = 0;

    for line in lines.iter() {
        let nodes: Vec<char> = line.chars().collect();

        if current_col >= nodes.len() {
            current_col -= nodes.len();
        }

        if nodes[current_col].to_string() == "#" {
            tree_count += 1;
        }

        current_col = current_col + 3;
    }
    println!("Tree Count: {}", tree_count);



}
