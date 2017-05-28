use std::fs::File;
use std::io::Read;
use std::io;

// Location of the dictionary file
const DICT_FILE : &str = "wordlist";

fn main() {
    println!("Loading dictionary");
    let lines : Vec<String> = get_lines(DICT_FILE);

    println!("Input string: ");
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();

    println!("Searching for matches...");

    let matches : Vec<String> = find_matches(&lines, &get_subsets(&input));

    println!("Matches found:");
    for m in matches {
        println!("{}", m);
    }
}

/// Return a Vector of lines in a file
fn get_lines(filename : &str) -> Vec<String> {
    let mut file = File::open(filename).expect("File can not be found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect()
}

/// Generates all subsets of a string and returns the vector
fn get_subsets(input : &str) -> Vec<String> {
    let mut subsets : Vec<String> = Vec::new();
    subsets.push("".to_string());

    let mut updated : Vec<String> = Vec::new();

    for character in input.chars() {
        for mut x in subsets.clone() {
            x.push(character);
            updated.push(x);
        }
        subsets.append(&mut updated);
    }

    // Remove useless subsets
    subsets.retain(|ref elem| elem.len() > 1);
    subsets
}

/// Returns a list of matches found in the string
fn find_matches(dict : &Vec<String>, subsets : &Vec<String>) -> Vec<String> {
   let mut results : Vec<String> = Vec::new();
   for subset in subsets {
        // Find if the dict vector has the contents
        if dict.contains(&subset) {
            results.push(subset.clone());
        }
   }
   results // Return the resulting vector
}
