use std::fmt::Display;

mod inner_mod_to_show_dir_struct;

fn test11() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result  = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// what we're trying to express here is that all the references have the same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotations in Struct Definitions

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn use_important_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {part : first_sentence };
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct DataToBeContained;

impl DataToBeContained {
    fn say(&self) {
        println!("Say something!")
    }
}

impl Drop for DataToBeContained {
    fn drop(&mut self) {
        println!("Data to be contained, memory released!")
    }
}

struct Container<'a> {
    data: &'a DataToBeContained // so this specifies that the container instance has the same lifetime as the data that it contains
}

pub fn test_how_lifetime_works_with_objects() {
    let data_to_be_contained = DataToBeContained {};

    {
        let container = Container {
            data: &data_to_be_contained
        };

        // container should be destroyed here but because data_to_be_contained still lives it's life is extended
        // so that is why we get `Say something!` first then `Data to be contained, memory released!` second
    }

    data_to_be_contained.say();
}