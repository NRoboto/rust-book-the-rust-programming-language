use std::fmt::Display;

fn main() {
    let number_list = [1, 20, -3, 4, 5];
    println!("The largest number is {}", largest(&number_list));

    let char_list = ['a', 'b', 'x', 'd', 'e'];
    println!("The largest char is {}", largest(&char_list));

    let p1 = Point {
        x: 1,
        y: 2,
    };

    let p2 = Point {
        x: 1.0,
        y: 2.0,
    };

    println!("p2 distance from origin = {}", p2.distance_from_origin());

    let tweet = traits::Tweet {
        username: String::from("Alice"),
        content: String::from("My tweet!"),
    };

    // println!("New tweet: {}", tweet.summarize());
    traits::notify(tweet);

    let news_article = traits::NewsArticle {
        headline: String::from("My Article"),
        location: String::from("Sweden"),
        author: String::from("Alice"),
        content: String::from("Some news..."),
    };

    // println!("New news article: {}", news_article.summarize());
    traits::notify(news_article);

    let other_article = traits::OtherArticle {
        content: String::from("Some content..."),
        author: String::from("Alice"),
    };

    // print!("New article: {}", other_article.summarize());
    traits::notify(other_article);

    let p3 = Point {
        x: traits::Tweet {
            username: String::from("Alice"),
            content: String::from("My tweet!"),
        },
        y: traits::Tweet {
            username: String::from("Liv"),
            content: String::from("Another tweet!"),
        }
    };

    // p3.cmp_display(); // Doesn't work because Tweet doesn't implement Display or PartialOrd

    println!("\n");

    lifetimes::main();
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("x > y, x = {}", self.x);
        } else {
            println!("y > x, y = {}", self.y);
        }
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

mod traits {
    pub trait Summary {
        fn summarize(&self) -> String {
            format!("(Read more from {})", self.summarize_author())
        }

        fn summarize_author(&self) -> String;
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.summarize_author(), self.location)
        }

        fn summarize_author(&self) -> String {
            self.author.clone()
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.summarize_author(), self.content)
        }

        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    pub struct OtherArticle {
        pub content: String,
        pub author: String,
    }

    impl Summary for OtherArticle {
        fn summarize_author(&self) -> String {
            self.author.clone()
        }
    }

    pub fn notify(item: impl Summary) {
        println!("New item! {}", item.summarize());
    }
}

mod lifetimes {
    pub fn main() {
        let s1 = String::from("abcd");
        let s2 = "xyz";

        println!("The longest string is {}", longest(s1.as_str(), s2));
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    struct ImportantExcerpt<'a> {
        part: &'a str
    }
}