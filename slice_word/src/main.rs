fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    let other_word = second_word(&s);

    println!("first: {}, second: {}", word, other_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i1:usize  = 0;
    let mut crossed_boundry = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if crossed_boundry {
                return &s[i1..i]
            } else {
                crossed_boundry = true;
                i1 = i;
            }
        }
    }

    &s[..]
}
