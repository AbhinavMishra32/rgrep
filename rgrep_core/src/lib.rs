use regex::Regex;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::{env, fs};

pub fn demo() {
    let re = Regex::new(
        r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
",
    )
    .unwrap();

    if let Some(caps) = re.captures("2010-03-14") {
        println!("matched: 2010-03-14");
        println!("year={}", &caps["year"]);
        println!("month={}", &caps["month"]);
        println!("day={}", &caps["day"]);
    } else {
        println!("no match: 2010-03-14");
    }
}

pub fn file_contents(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        // Ok(_) => print!("{} contains: \n{}", display, s),
        Ok(_) => {}
    }
    return s;
}

pub fn list_files(path: &str) {
    let path = Path::new(path);
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        println!("{}", path.display());
    }
}

pub fn give_search_results(path: &str, re: &str) {
    let mut s = String::new();
    s = file_contents(path);

    let re = Regex::new(re).unwrap();

    for (line_number, line) in s.lines().enumerate() {
        if re.is_match(line) {
            println!("{}: {}", line_number + 1, line);
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!(" list <directory>");
    println!(" search <file> <regex>");
    println!();
    println!("Examples:");
    println!(" list src");
    println!(" search src/main.rs fn");
}

pub fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "list" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            list_files(path);
        }

        "search" => {
            if args.len() < 4 {
                println!("Missing args.");
                println!("Usage: engine search <file> <regex>");
                return Ok(());
            }

            let path = &args[2];
            let pattern = &args[3];

            give_search_results(path, pattern);
        }

        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }

    Ok(())
}
