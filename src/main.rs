use std::{process::{Command, Stdio}, io::Read};

fn get_tags_as_string() -> String {
    let process = match Command::new("git")
    .args(["tag"])
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

fn turn_tags_into_number(tags: String){
    let mut tags_sorted = Vec::new();
    for tag in tags.split("\n"){
        tags_sorted.push(tag)
    }
    
    tags_sorted.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!("{:?}", tags_sorted);
}

fn main() {
    turn_tags_into_number(get_tags_as_string());
    
}
