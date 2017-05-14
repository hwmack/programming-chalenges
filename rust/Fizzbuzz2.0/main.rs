use std::io;

fn main() {
    let input = io::stdin();

    println!("Input the range: ");

    let mut size = String::new();
    input.read_line(&mut size).expect("Failed to read line");
    let size: u32 = check_int(&size);

    // Check for an invalid int
    if size == 0 {
        println!("Error: Range must be a number");
        return;
    }

    // Vector to hold tuples
    let mut pairs: Vec<(u32, String)> = Vec::new();

    println!("Input your integer and string combinations in the form int string\nBlank line to perform the iteration");

    let mut line = String::new();
    loop {
        input.read_line(&mut line).expect("Failed to read line");
        if line != "\n" {
            match parse_line(line.trim()) {
                Ok(result) => pairs.push(result),
                Err(_) => println!("Invalid line")
            }

            // Clear the line
            line = String::new();
        } else {
            break;
        }
    }

    // Perform the iteration
    for i in 1..(size + 1) {
        let mut output = String::new();

        for &(divisor, ref word) in &pairs {
            if i % divisor == 0 {
                output = output + word;
            }
        }

        if output == "" {
            println!("{}", i);
        } else {
            println!("{}", output);
        }
    }

}

fn check_int(string : &str) -> u32 {
    match string.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

fn parse_line(string : &str) -> Result<(u32, String), &'static str> {
    // Split on the empty string in a temp vector
    let params: Vec<&str> = string.split(' ').collect();

    // If the line is invalid, return an error
    if params.len() != 2 {
        return Err("error");
    }

    // Get the divisor
    let divisor = check_int(&params[0]);
    let word = params[1].to_string();

    // Return the tuple
    Ok((divisor, word))
}
