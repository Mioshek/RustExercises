use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_five_biggest_fixedboi(arr: &Vec<i32>) -> [i32; 5] {
    let mut output = [0, 0, 0, 0, 0];
    for i in 0..arr.len() {
        let mut value = arr[i];
        // DEBUGGING:
        // println!("{} - {:?}", value, output);
        for j in 0..output.len() {
            if value > output[j] {
                let temp = output[j];
                output[j] = value;
                value = temp;
            }
        }
    }
    return output;
}

// Did some small code edits c:
//      ~Kihau
fn main() {
    let mut numbers = Vec::new();

    let file_handle = File::open("test.txt").expect("File not found???????");
    let lines = BufReader::new(file_handle).lines();

    for line in lines {
        let line = line.unwrap();
        if let Ok(number) = line.parse::<i32>() {
            numbers.push(number);
        }
    }
    println!("Initial array = {:?}", numbers);
    println!("Max 5 = {:?}", get_five_biggest_fixedboi(&numbers));
}
