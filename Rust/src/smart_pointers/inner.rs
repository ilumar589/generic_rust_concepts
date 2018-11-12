// implementing a cons list (I guess another term for linked list. Don't see any difference from the initial explanation)

enum List {
    Cons(i32, List), // because List is recursive (holds another value to itself) the compiler can't tell how much memory to allocate
    Nil,
}

enum List2 {
    Cons(i32, Box<List2>), // because it's a pointer the compiler knows it's exact size so the exact size of the List enum/struct
    Nil
}


fn use_pointer() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}