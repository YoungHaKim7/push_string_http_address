use regex::Regex;
use std::{
    fs::{read_to_string, File},
    io::Write,
};

fn starts_with_url_prefix(s: &str) -> bool {
    s.starts_with("https://younghakim7.github.io/")
}

fn insert_investment(line: &str) -> String {
    if starts_with_url_prefix(line) && !line.contains("/Investment/") {
        if let Some(pos) = line.rfind('/') {
            // Insert "/Investment" after the prefix and before the last part
            format!(
                "{}{}/Investment{}",
                &line[..pos], // Everything before the last "/"
                "",
                &line[pos..] // Including the last "/"
            )
        } else {
            // Fallback for lines without a trailing "/"
            format!("{}/Investment", line)
        }
    } else {
        line.to_string()
    }
}

fn main() {
    let input = read_to_string("./input/input.txt").expect("Failed to read input file");

    // Regex to match <loc>...</loc> tags and extract URLs
    let re = Regex::new(r#"<loc>(.*?)</loc>"#).unwrap();

    let mut result = input.clone();

    // For each match, insert "Investment/"
    for caps in re.captures_iter(&input) {
        if let Some(url) = caps.get(1) {
            let updated_url = insert_investment(url.as_str());
            result = result.replace(url.as_str(), &updated_url);
        }
    }

    // Print the modified content
    println!("{}", result);
    let mut file = File::create("output.xml").expect("create failed");
    file.write_all(result.as_bytes()).expect("write failed");
}
