fn main() {
    {
        let s1 = String::from("Hello");

        let s2 = takes_and_gives_ownership(s1);

        let length = calculate_length(&s2);

        // println!("s1 = {}", s1); // Error: borrow of moved value: `s1`
        println!("s2 = {}", s2);
        println!("s2 length = {}", length);

        let mut hello = String::from("Hello");
        exclaim(&mut hello);
        println!("{}", hello);

        let r1 = &hello;
        let r2 = &hello;
        println!("r1 = {}", r1);
        println!("r2 = {}", r2);
        let r3 = &mut hello;
        println!("r3 = {}", r3);
    }

    let s = String::from("Hello, World!");

    let s1 = &s[..5];
    let s2 = &s[7..12];
    let s3 = &s[7..];

    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);

    println!("First word of {} is {}", s, first_word(&s));
}

fn takes_and_gives_ownership(s: String) -> String {
    s + "!"
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn exclaim(s: &mut String) {
    s.push('!');
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