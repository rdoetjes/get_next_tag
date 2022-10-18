use std::{process::{Command, Stdio}, io::Read, str::FromStr};

use regex::Regex;

/// get_tags_as_string returns a string with all the tags from the central repository
/// 
/// Generates a string by issuing the following command: 
/// git ls-remote --tags --sort=committerdate and reading the output
/// via a pipe into a variable.
/// The --sort=comtterdate is very important because that's the only way to determine the last tag
/// 
/// Example:
/// println!("{}", get_tags_as_string());
/// 
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

///get_last_tag(tags) returns the last tag
/// 
/// It gets the last summited tag fbe 
/// This gets the last tag based on the last entry that is created in the tags_sorted vector
/// then running a regular expression to only collect the tag name (at the end of the line using regex /*/(.*)/)
fn get_last_tag(tags: String) -> String {
    let mut tags_sorted = Vec::new();
    for tag in tags.split("\n"){
        if tag != ""{
            tags_sorted.push(tag)
        }
    }   

    //when no match found return 0.0.0 as the fist ever version
    if tags_sorted.len() ==0{
        return "0.0.0".to_string();
    }

    let mut last_tag = tags_sorted[tags_sorted.len()-1];
    let re = Regex::new(r"/.*/(.*)$").unwrap();
    
    last_tag = match re.captures(last_tag){
        Some(x) => x.get(1).unwrap().as_str(),
        None => {
            eprintln!("The right tag cannot be found!");
            std::process::exit(1)
        },
    };
    last_tag.to_string()
}

/// next_tag(tag) creates the last minor number of the 1.0.1 string returning 1.0.2
/// 
/// next_tag(tag) parses the x.y.z tag and cover z to i32 adds 1 and returns the string
/// x.y.z+1
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
    println!("##vso[task.setvariable variable=next_tag_version]{}", new_tag);
}
