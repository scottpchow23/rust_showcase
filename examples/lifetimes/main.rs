fn shorter_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() < str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let str1 = String::from("blah blah blah");
    {
        let str2 = String::from("ha ha ha ha");
        let shortest = shorter_string(str1.as_str(), str2.as_str());
        println!("the shorter string is {}", shortest);
    }
}
