use std::{collections::HashMap, hash::Hash, thread, time::Duration};

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // generate_workout(
    //     simulated_user_specified_value,
    //     simulated_random_number,
    // );

    let v1 = vec![1, 2, 3];
    let y: Vec<i32> = v1
			.iter()
			.map(|v| v * 2)
			.collect();

    let my_counter = Counter::new();
    let even_numbers: Vec<_> = my_counter.map(|x| x * 2).collect();
    println!("even_numbers = {:#?}", even_numbers);
}

struct Cacher<I: Eq + Hash + Copy, R: Copy, F: Fn(I) -> R> {
    calculation: F,
    arg_values: HashMap<I, R>,
}

impl<I: Eq + Hash + Copy, R: Copy, F> Cacher<I, R, F>
    where F: Fn(I) -> R
{
    fn new(calculation: F) -> Cacher<I, R, F> {
        Cacher {
            calculation,
            arg_values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: I) -> R {
        match self.arg_values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.arg_values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(random_number));
        println!("Next, do {} situps!", expensive_closure.value(random_number * 2));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(random_number));
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count >= 5 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}