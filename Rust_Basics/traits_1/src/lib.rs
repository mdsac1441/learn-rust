pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits as Parameters : impl Trait syntax

pub fn notify1(item: &impl Summary) {
    println!(
        "Traits as Parameters : impl Trait syntax --> Breaking news! {}",
        item.summarize()
    );
}

// Trait Bound Syntax:
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound;it looks like this:
pub fn notify2<T: Summary>(item: &T) {
    println!("Trait Bound Syntax --> Breaking news! {}", item.summarize());
}

// This longer form is equivalent to the example in the previous section but is more verbose.
// We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.

// Specifying Multiple Trait Bounds with the + Syntax:
use std::fmt::Display;
pub fn notify3(item: &(impl Summary + Display)) {}

// The + syntax is also valid with trait bounds on generic types:

pub fn notify4<T: Summary + Display>(item: &T) {}
// With the two trait bounds specified, the body of notify can call summarize and use {} to format item.

// Clearer Trait Bounds with where Clauses:

// Using too many trait bounds has its downsides.
// Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read.
// For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So instead of writing this:
use std::fmt::Debug;
use std::fmt::Error;
pub fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> Result<i32,Error> {Ok(1)}

// we can use a where clause, like this:
pub fn some_function2<T, U>(t: &T, u: &U) -> Result<i32,Error>
where
    T: Display + Clone,
    U: Clone + Debug,
{
    Ok(1)
}
// Returning Types that Implement Traits:
fn returns_summarizable1() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

//  Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler.
// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
