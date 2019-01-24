extern crate rand;

mod lifetime_test;
mod functional_test;
mod iterators;
mod smart_pointers;
mod a_dir;
mod thread_examples;


use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::fmt::Display;

use functional_test::inner as functional_inner;
use iterators::inner as iterators_inners;

use smart_pointers::inner as smart_pointer_inner;

use thread_examples::inner_thread_examples as my_threads;

// example of a struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function to create a rectangle instance
    // similar to String::from
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

impl Rectangle {
    // this works, we can have multiple implementation blocks for the same struct/enum
}

fn main() {

    my_threads::main();

    lifetime_test::test_how_lifetime_works_with_objects();

    smart_pointer_inner::main();

//  A string literal has a known size at compile time and is hardcoded into the executable
    let string_literal = "a string literal";

//  String type is allocated on the heap because of the unknown size
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}, String literal: {}", s, string_literal);

//    Using structs and traits
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Vup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL")
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    let second_tweet = Tweet {
        username: String::new(),
        content: String::new(),
        reply: false,
        retweet: false
    };

    let copy_of_tweet = second_tweet; // because the Tweet class doesn't implement the Copy trait
    // copy_of_tweet doesn't actually copy the value from the second_tweet, it just points to the same
    // memory on the stack but now owns that zone for memory release once it's gone out of scope

    notify(copy_of_tweet);

//    notify(article);
//    notify(tweet);
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!")
        }
    }
}

fn ownership_example() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // ownership has been moved to the some_string backing pointer

    // if we tried to use s after the takes_ownership function the compiler would throw an error
    // because s no longer holds ownership over the backing memory

    let x = 5;

    makes_copy(x);

    // x can be used because primitive values are only stored on the stack and copies are made
    // so both x and the some_integer param in the function hold a copy of the data

    // same rules apply to returning values

    let s1 = gives_ownership(); // gives_ownership moves its return value to s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back_ownership(s2); // s2 is moved into takes_and_gives_back_ownership
                                                                // which also moves its return value into s3

//    let reference_to_nothing = dangling_pointer(); // compiler will throw an error because the dangling_pointer function
                                                            // returns a reference to a deallocated memory zone
} // s1 goes out of scope and it's backing memory is dropped, s2 goes out of scope and nothing happens, s3 goes out of scope and it's backing memory is dropped

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and the backing memory is released

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn big_thing_to_remember_about_references() {
    let s = String::from("hello");

    // reference in the smaller context of things, in rust, has the main goal
    // of borrowing, meaning pointing to the desired variable without taking it's
    // ownership away

    // we can have many immutable references to a variable

    let imm_ref1 = &s;
    let imm_ref2 = &s;

    // but we can't have more than one mutable refrence

    // shadow s. We can't borrow a immutable variable with a mutable reference, which makes sense
    let mut s = String::from("hello");

    let muta_ref1 = &mut s;
//    let muta_ref2 = &mut s; this will generate a compiler error
}

fn custom_scopes_for_references() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problem

    let r2 = &mut s;
}

//fn dangling_pointer() -> &String {
//    let s = String::from("hello!");
//
//    &s
//}

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // so one we declare the largest variable we should not own the memory of the first element
    // so either we create largest as let &mut largest (reference) or declare that only elements that implement the Copy trait
    // can be used so largest will copy the first element data into a new memory zone

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we can restrict generics differently for each method using multiple impl declarations

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {

    // associated function (has no self in parameters)
    fn new(x: T, y:T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cm_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// This is interesting, we can conditionally implement traits for every struct definition that has certain traits
// For example the standard library implements the ToString trait for every struct that has the Display trait
// looks like this: impl<T: Display> for T {// snip}