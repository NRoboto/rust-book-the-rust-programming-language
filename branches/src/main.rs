fn main() {
    let num = 15;

    if num < 10 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let my_val = if num % 5 == 0 {
        "True"
    } else {
        "False"
    };

    println!("my_val is {}", my_val);
}
