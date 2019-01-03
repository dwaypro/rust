// The aim of references is to prevent dangling references
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    println!("Hello, world!");
    let String1 = String::from("abcd");
    let String2 = "xyz";
    let result = longest(String1.as_str(), String2);
    println!("the longest string is {}", result);
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
