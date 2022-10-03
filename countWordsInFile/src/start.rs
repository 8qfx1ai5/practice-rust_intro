use std::fs;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    let filecontent = fs::read_to_string(filename).expect("Unable to read file");

    println!("{}", filecontent);

    //for c in filecontent.chars() { 
        
    //}
}
