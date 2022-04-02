fn main() {
    let s0 = String::from("Hello World");
    let s1 = String::from("HelloWorld");

    println!("{}", first_word(&s0));
    println!("can use s0 {}", s0);

    println!("{}", first_word(&s1));
    println!("can use s1 {}", s1);


    let sl0 = first_slice(&s0);
    println!("{}", sl0);
    println!("can use s0 {}", s0);

    let sl1 = first_slice(&s1);
    println!("{}", sl1);
    println!("can use s1 {}", s1);


    takes_ownership(s0);
    // println!("can't use s0 {}", s0);
    // println!("can't use sl0 {}", sl0);
    println!("can use s1 {}", s1);
    println!("can use sl1 {}", sl1);

    takes_ownership(s1);
    // println!("can't use s1 {}", s1);
    // println!("can't use sl1 {}", sl1);
    
}

fn first_word(s: &String) -> String {
    let bytes = s.as_bytes();

    let mut final_position = s.len();

    for (i, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            final_position = i;
            break
        }
    }
    
    let word = &s[..final_position];
    String::from(word)
}

// fn first_slice(s: &String) -> &str {
fn first_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn takes_ownership(_s: String) {
    return
}