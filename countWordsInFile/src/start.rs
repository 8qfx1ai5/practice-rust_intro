use std::fs;
use std::env;
use std::collections::HashMap;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    let filecontent = fs::read_to_string(filename).expect("Unable to read file");

    // println!("{}", filecontent);

    let mut word_counters: HashMap<&str, u128> = HashMap::new();
    let mut current_word = String::from("");
    let seperator_string = " -.,;:/\\_\n";
    let seperator_vec: Vec<char> = seperator_string.chars().collect();

    for c in filecontent.chars() {
        print!("{}", c);
        if !seperator_vec.contains(&c){
            // just a normal character from a word
            current_word.push(c);
            continue;
        }
        if current_word.len() == 0 {
            // we found an seperator, but the current word is empty
            // maybe because two seperators in a row
            continue;
        }

        //if word_counters.contains_key(&current_word[..]) {
        //let count = word_counters.entry(&current_word[..]).or_insert(0);
            // let count = map.entry(word).or_insert(0);
        //*count += 1;
        //}
    }

    println!("{}","");
    //for (key, value) in &word_counters {
      //  println!("{}: {}", key, value);
    //}

}
