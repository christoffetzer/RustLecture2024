pub fn do_match(c: char) -> &'static str {
    match c {
        'a'..='z' => "lower case",
        _ => "not lower case",
    }
}

fn main() {
    let char_list = ['a', 'd', 'G', '$', 'z', '0'];

    for c in char_list {
        println!("{c}=>{result}", result = do_match(c));
    }
}
