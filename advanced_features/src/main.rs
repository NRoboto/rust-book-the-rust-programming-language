fn main() {
    unsafe_code::main();
    functions::main();
    macros::main();
}

mod unsafe_code {
    use std::slice;

    pub fn main() {
        pointers();
        functions();
        extern_functions();
    }

    fn pointers() {
        let mut num = 5;
    
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
    
        unsafe {
            println!("r1 = {}", *r1);
            println!("r2 = {}", *r2);
        }
    }

    fn functions() {
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid))
            }
        }

        let mut s = [1, 2, 3];
        println!("{:?}", split_at_mut(&mut s, 1));
    }

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    fn extern_functions() {
        unsafe {
            println!("abs(-3) from C: {}", abs(-3));
        }
    }
}

mod functions {
    pub fn main() {
        println!("{}", do_twice(add_one, 5));
    }

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

mod macros {
    #[macro_export]
    macro_rules! double_vec {
        ( $( $x: expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    pub fn main() {
        let my_vec = double_vec![1, 2, 3];

        println!("{:#?}", my_vec)
    }
}