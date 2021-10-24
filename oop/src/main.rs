fn main() {
    gui::main();

    blog::main();
    blog2::main();
}

mod gui {
    trait Draw {
        fn draw(&self);
    }
    
    struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    
    #[derive(Debug)]
    struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    
    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing button: {:?}", self);
        }
    }
    
    #[derive(Debug)]
    struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Drawing select box: {:?}", self);
        }
    }

    pub fn main() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("Ok"),
                }),
            ],
        };
    
        screen.run();
    }
}

mod blog {
    pub fn main() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.add_text("This shouldn't be added");
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());

        println!("content = '{}'", post.content());
    }

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            if self.state.as_ref().unwrap().is_editable() {
                self.content.push_str(text);
            }
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    trait State{
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
        fn is_editable(&self) -> bool {
            false
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {
                approval_count: 0
            })
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn is_editable(&self) -> bool {
            true
        }
    }

    struct PendingReview {
        approval_count: u8
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            self.approval_count += 1;
            if self.approval_count >= 2 {
                Box::new(Published {})
            } else {
                self
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

mod blog2 {
    pub fn main() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());

        println!("content = '{}'", post.content());
    }

    pub struct Post {
        content: String
    }

    pub struct DraftPost {
        content: String
    }

    pub struct PendingReviewPost {
        content: String
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content
            }
        }
    }
}