use regex::Regex;
use std::fs::File;
use std::path::Path;

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

pub fn search_files(path: unknown) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains: \n{}", display, s),
    }
}
