use std::collections::hash_map::OccupiedEntry;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::process::Output;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_if_one_of_biggest(element:i8, current_bigest_nums:[i8;5])->[i8;5]{
   let mut cbn = current_bigest_nums;
   let mut el = element;
   println!("{}{:?}", el, cbn);
    for j in 0..cbn.len(){
        if el > cbn[j]{
            let temp_el = cbn[j];
            cbn[j] = el;
            el = temp_el;
        }
        else {
            continue;
        }
    }
    return cbn;
}

fn get_five_biggest(arr:Vec<i8>)-> [i8;5]{
    let mut output:[i8;5] = [0,0,0,0,0];
    for i in 0..arr.len(){
        let el:i8 = arr[i];
        output = check_if_one_of_biggest(el, output);
    }
    return output;
}

fn main() {
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines("/home/mioshek/Programming_Stuff/Programming/Rust/exercises/five_largest_numbers/test.txt"){
        for line in lines{
            if let Ok(int) = line{
                let number = int.parse::<i8>().unwrap();
                numbers.push(number);
            }
        }
    }
    println!("{:?}",get_five_biggest(numbers));
}
