fn my_func_1() {
    println!("a");
}

fn main() {
    let mut x = 0o77;
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);

    let y = 5;
    println!("The value of y is {}", y);
    let y = y * 2;
    println!("The value of y is {}", y);
    let y = y + 3;
    println!("The value of y is {}", y);
    let y = y.to_string() + "!";
    println!("The value of y is {}", y);

    let z: i32 = "42".parse().expect("Not a number!");
    println!("The value of z is {}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    let z = tup.2;
    println!("The value of y is {} and z is {}", y, z);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a[2] is {}", a[2]);

    my_func_1();
    my_func_2(5);
    my_func_2(10);

    println!("5^2 + 5 is {}", square_plus_x(5));
    
    println!("Please enter an index:");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Input must be a number!");
    println!("The value of a[{}] is {}", index, a[index]);
}

fn my_func_2(x: i32) {
    println!("The value of x is {}", x);
}

fn square_plus_x(x: i32) -> i32 {
    let y = x * x;
    y + x
}