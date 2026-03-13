use std::fs;

/*
Purpose: to read a text file and print out the frequencies of each word
Credits:
- Gemini
    - Reading files
- w3Schools
*/

fn main() {

    let textFile = "input.txt";
    let contents = fs::read_to_string(textFile)
        .expect("File has been read");

    println!("{}", contents);
}