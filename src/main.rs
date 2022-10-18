use std::{process::{Command, Stdio}, io::Read, str::FromStr};

use regex::Regex;

fn get_tags_as_string() -> String {
    let process = match Command::new("git")
    .args(["ls-remote", "--tags", "--sort=committerdate"])
    .stdout(Stdio::piped())
    .spawn() {
        Err(why) => {
            eprintln!("couldn't spawn git tag: {}", why);
            std::process::exit(1);
        },
        Ok(process) => process,
    };
    
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s){
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error trying to read from git tag : {}", e);
            std::process::exit(1);
        },
    };
    s
}

fn get_last_tag(tags: String) -> String {
    let mut tags_sorted = Vec::new();
    for tag in tags.split("\n"){
        if tag != ""{
            tags_sorted.push(tag)
        }
    }    
    let mut last_tag = tags_sorted[tags_sorted.len()-1];
    let re = Regex::new(r"/.*/(.*)$").unwrap();
    
    last_tag = match re.captures(last_tag){
        Some(x) => x.get(1).unwrap().as_str(),
        None => panic!("The right tag cannot be found!"),
    };
    last_tag.to_string()
}

fn next_tag(tag: String) -> String {
    let vec: Vec<&str> = tag.split(".").collect();
    let mut c: i32 = FromStr::from_str(vec[2]).unwrap();
    c+=1;
    let result = format!("{}.{}.{}", vec[0], vec[1], c.to_string());    
    result
}

fn main() {
    let last_tag = get_last_tag(get_tags_as_string());
    let new_tag = next_tag(last_tag);
    println!("{}", new_tag);
}
