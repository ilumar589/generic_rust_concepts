pub fn creating_pointers_from_references() {
    let mut number = 5;

    let p1 = &number as *const i32;
    let p2 = &number as *mut i32;

//    Notice that we don’t include the unsafe keyword in this code. We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block

    unsafe {
          println!("pointer1's value is {}", *p1);
          println!("pointer2's value is {}", *p2);
    }
}

// calling unsafe functions

unsafe fn dangerous() {}

pub fn call_unsafe_function() {
    unsafe {
        dangerous();
    }
}