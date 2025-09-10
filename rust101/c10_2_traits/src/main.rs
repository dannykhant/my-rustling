#![allow(unused)]

fn main() {
    use aggregator::{SocialPost, Summary, NewArticle};

    let post = SocialPost {
        username: String::from("ebooks"),
        content: String::from("this is the content for the post"),
        reply: false,
        repost: false
    };
    println!("1 new post: {}", post.summarize());

    let article = NewArticle {
        headline: String::from("Penguins Championship!"),
        location: String::from("USA"),
        author: String::from("Iceberg"),
        content: String::from("Penguins are good at hockey.")
    };
    println!("New article: {}", article.summarize());
}

mod aggregator {
    use std::fmt::Display;

    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String
    }

    impl Summary for NewArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool
    }

    impl Summary for SocialPost {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // trait-bound sugar
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // trait-bound
    pub fn notify_v2<T: Summary>(item1: &T, item2: &T) {}

    // multiple trait-bounds sugar
    pub fn notify_v3(item: &(impl Summary + Display)) {}

    // multiple trait-bounds
    pub fn notify_v4<T: Summary + Display>(item: &T) {}

    // where clause for multiple trait bounds
    // fn func<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug
    // {}

    // Return type that implement trait
    fn return_summarizable() -> impl Summary {
        SocialPost {
            username: String::from("ebooks"),
            content: String::from("this is the content for the post"),
            reply: false,
            repost: false
        }
    }
}

mod pair_mod {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x > self.y {
                println!("Largest: {}", self.x);
            } else {
                println!("Largest: {}", self.y);
            }
        }
    }
}
