use std::fs;

pub fn soln(file_path: &str) -> u32 {
    /*
     * This is a soln to an advent of code problem where given a  list of
     * strings (read here from file), each line is read scanning for digits and getting the first and
     * last digits(using the same digit if only one digit is present in the line).
     * This 2 digits per line are taken as a number consisting of 2 digits and each of them
     * are summed up to give the result
     */
    let sample_file = fs::read_to_string(file_path).unwrap(); 
    let mut lines: Vec<&str> = vec![]; // Contains all the lines in the file as a list of lines
    let mut all_digits: Vec<Vec<u32>> = Vec::new(); // Contains a vector of vectors which old the digits found in each line
    let mut sum: u32 = 0; // Holds the sum of all the digits

    for line in sample_file.lines() {
        lines.push(line);
    }

    if !lines.is_empty() {
        for line in lines {
            let mut digits_in_line: Vec<u32> = Vec::new();
            for character in line.chars() {
                if character.is_digit(10) {
                    let digit: u32 = character.to_digit(10).unwrap();
                    digits_in_line.push(digit);
                }
            }
            all_digits.push(digits_in_line);
        }
    }

    for digit_list in all_digits {
        let first_n_last_digits: u32 =
            format!("{}{}", digit_list[0], digit_list[digit_list.len() - 1])
                .parse()
                .unwrap();
        sum += first_n_last_digits;
    }
    sum
}