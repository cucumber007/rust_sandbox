#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    dbg!("kek");
    println!("Searching '{}' in: {}", query, file_path);

    let contents = fs::read_to_string(file_path);

    // match contents.map(|x| search_for(&query, &x)) {
    match search(query, &contents) {
        Ok(Some(value)) => println!("{}", value),
        Ok(None) => println!("{}", "Nothing found"),
        Err(e) => panic!("{}", e.kind().to_string()),
    }
}

fn search<'a>(query: &'a String, result: &'a Result<String, io::Error>) -> Result<Option<&'a str>, &'a io::Error> {
    return match result {
        Ok(success) => {
            let r = search_for(query, success);
            let fr = Ok::<Option<&'a str>, &'a io::Error>(r); 
            fr
        },
        Err(e) => Err(e),
    };
}

fn search_for<'a>(query: &'a String, data:&'a String) -> Option<&'a str> {
    for part in data.split('\n') {
        if part.contains(query.as_str()) {
            return Some(part);
        }
    }
    return None;
}