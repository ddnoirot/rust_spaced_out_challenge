/* 
Prompt:
  - Given N input defining an N by N grid of farm space.
  - Every 2 by 2 grid must contain exactly 2 cows.
  - In a space where a cow 'exists', the total beauty of the farm is increased by the value of that cell. 
  - Find the cow placement that provides the maximum beauty.  
  
  Steps:
  1. Read input file.
  2. Map the "beauty" grid.
  3. Place cows in every configuration that satisfies prompt requirements.
  4. Calculate the "beauty" of each configuration.
  5. Determine the configuration that produces maximum "beauty".
  6. Print the solution to the console.  
  */

// Library calls

use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();    
}

fn main() {
    // Step 1: Read input file. Transform string input to multidimensional vector.
    let input_file = "input_files/5.in";
    let lines = read_lines(input_file.to_string());
    let mut farm_dim: u32 = 0;
    let mut farm_val: Vec<Vec<u32>> = Vec::new();
   
    for (i,line) in lines.into_iter().enumerate() {
        if i == 0 {
            let first_line  = line.unwrap();
            let n: u32 = first_line.parse().unwrap();
            farm_dim = n;
        }
        else { 
            let n_line = line.unwrap();
            
            let n_line: Vec<u32> = n_line
                    .split_whitespace()
                    .map(|s| s.parse().expect("parse error"))
                    .collect();
                
                farm_val.push(n_line);
        }
        
    }
    
    println!("{farm_dim}");
    println!("{:?}", farm_val);
    
    // Step 2
    
    // Step 3
    
    // Step 4
    
    // Step 5
    
    // Step 6
    
}