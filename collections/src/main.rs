use std::collections::HashMap;
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:#?}", v);

    let mut v2 = Vec::new();
    v2.push(3);

    let second = &v[1];
    println!("Second = {}", second);

    match v.get(1) {
        Some(x) => println!("x = {}", x),
        None => println!("None!")
    }

    for i in &mut v {
        *i *= *i;
    }

    for i in &v {
        println!("i = {}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("Hello!")),
        SpreadsheetCell::Float(5.4),
        SpreadsheetCell::Bool(true),
    ];

    println!("Row = {:?}", row);

    let s = format!("{}-{}-{}", v[0], v[1], v[2]);
    println!("s: {}", s);

    for c in s.chars() {
        print!("'{}', ", c);
    }
    println!();

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("Scores: {:#?}", scores);
    println!("Scores from vecs: {:#?}", scores2);
    println!("Blue score is {}", match scores.get("Blue") { Some(x) => x.to_string(), None => String::from("not found") });

    for (key, value) in &scores {
        println!("Team: {}, Score: {}", key, value);
    }

    let sentence = "the quick brown fox jumps over the lazy dog";
    let mut letterCount = HashMap::new();

    for l in sentence.split("").filter(|&x| x != "" && x != " ") {
        letterCount
            .entry(l)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
    }

    println!("{:?} | length = {}", letterCount, letterCount.len());
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    Bool(bool),
}