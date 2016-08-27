fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
pub fn main() {
    let filename = "foobar.rs";
    match find(filename, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &filename[i+1..]),
    }
}
