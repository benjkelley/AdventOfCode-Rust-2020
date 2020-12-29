use std::fs;

fn tree_checker(run: usize, rise: usize, lines: & mut Vec<&str>) -> i32 {
    let mut current_col = 0;
    let mut tree_count = 0;

    let mut current_row = 0;

    while current_row < lines.len() {
        let nodes: Vec<char> = lines[current_row].chars().collect();

        if current_col >= nodes.len() {
            current_col -= nodes.len();
        }

        if nodes[current_col].to_string() == "#" {
            tree_count += 1;
        }

        current_col = current_col + run;
        current_row = current_row + rise;
    }

    return tree_count;
}


fn main() {
    // read the input from the file
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");
    
    let mut lines: Vec<&str> = contents.lines().collect();
   
    let test_1 = tree_checker(1, 1, &mut lines);
    println!("Test 1: {} Trees", test_1);
    let test_2 = tree_checker(3, 1, &mut lines);
    println!("Test 2: {} Trees", test_2);
    let test_3 = tree_checker(5, 1, &mut lines);
    println!("Test 3: {} Trees", test_3);
    let test_4 = tree_checker(7, 1, &mut lines);
    println!("Test 4: {} Trees", test_4);
    let test_5 = tree_checker(1, 2, &mut lines);
    println!("Test 5: {} Trees", test_5);

    let total: u128 = (test_1 * test_2 * test_3 * test_4 * test_5) as u128;

    println!("Tree Count: {}", total);

}
