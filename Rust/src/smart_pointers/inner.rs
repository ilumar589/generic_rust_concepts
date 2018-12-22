use std::ops::Deref;
use std::mem::drop;

// implementing a cons list (I guess another term for linked list. Don't see any difference from the initial explanation)

enum List {
//    Cons(i32, List), // because List is recursive (holds another value to itself) the compiler can't tell how much memory to allocate
    Nil,
}

enum List2 {
    Cons(i32, Box<List2>), // because it's a pointer the compiler knows it's exact size so the exact size of the List enum/struct
    Nil
}


fn use_pointer() {
    let list = List2::Cons(1, Box::new(List2::Cons(2, Box::new(List2::Cons(3, Box::new(List2::Nil))))));
}

struct MyBox<T>(T); // tuple struct with one value ? yeah this is a tuple struct

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// example implementing the Drop trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

// can use manual release of memory function -> drop()
enum AnotherList {
    Cons(i32, Box<AnotherList>),
    Nil
}


pub fn main() {
    let a = AnotherList::Cons(5,Box::new(AnotherList::Cons(10, Box::new(AnotherList::Nil))));
    let b = AnotherList::Cons(3, Box::new(a));
//    let c = AnotherList::Cons(4, Box::new(a));
}