fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(localhost);
    route(loopback);

    let m = Message::Write(String::from("Hello!"));
    m.call();

    let some_int = Some(5);
    let some_string = Some(String::from("Hello!"));
    let none_int: Option<i32> = None;

    handle_message(Message::Write(String::from("Hello!")));
    handle_message(Message::ChangeColour(53, 10, 200));
    handle_message(Message::Move { x: 10, y: -3 });
    handle_message(Message::Quit);

    let my_option_int = Some(5);
    println!("my_option_int = {:?}", my_option_int);
    println!("my_option_int + 1 = {:?}", plus_one(my_option_int));
    println!("None + 1 = {:?}", plus_one(None));

    let some_u8_value = 1u8;
    let some_string_value = match some_u8_value {
        1 => "one",
        3 => "three",
        5 => "five",
        7 => "seven",
        _ => "some other value"
    };

    println!("{} as a string is {}", some_u8_value, some_string_value);

    let someother_u8_value = Some(3u8);
    match someother_u8_value {
        Some(3) => println!("Three!"),
        _ => (),
    }

    if let Some(3) = someother_u8_value {
        println!("Three!")
    }

    let move_msg = Message::Move { x: -5, y: 2 };
    if let Message::Move { x, y } = move_msg {
        println!("Moving to ({}, {})", x, y);
    } else {
        println!("Some other kind of message!");
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddr) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // ...
    }
}

enum Status_Code {
    OK,
    Exited,
}

fn handle_message(message: Message) -> Status_Code {
    match message {
        Message::Quit => {
            println!("Quit");
            Status_Code::Exited
        },
        Message::Move {x, y} => {
            println!("Move to ({}, {})", x, y);
            Status_Code::OK
        },
        Message::Write(m) => {
            println!("Write \"{}\"", m);
            Status_Code::OK
        },
        Message::ChangeColour(r, g, b) => {
            println!("Change colour to ({}, {}, {})", r, g, b);
            Status_Code::OK
        },
    }
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        Some(x) => {
            Some(x + 1)
        },
        None => {
            None
        },
    }
}