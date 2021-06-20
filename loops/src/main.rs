fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is {}\n", result);

    let mut number = 3;

    while number > 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Liftoff!\n");

    let arr = [1, 2, 3, 4, 5];

    for x in arr.iter() {
        println!("x is {}", x);
    }

    println!("");

    for x in (1..4).rev() {
        println!("{}!", x);
    }

    println!("Liftoff!");

    let x = String::from("Hello");
    let y = x.clone();

    println!("x = {}, y = {}", x, y);
}
