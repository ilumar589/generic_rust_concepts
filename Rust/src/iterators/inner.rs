//trait Iterator {
//    type Item;
//
//    fn next(&mut self) -> Option<Self::Item>;
//
//    // methods with default implementations elided
//}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32; // associated item type, don't know what this concept is yet

    fn next(&mut self) -> Option<<Self as Iterator>::Item> { //Self refers to type (Counter in out case). as Iterator syntax is to specify that our type is an iterator I guess, in order to work with for loops
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn use_custom_iterator() {
    for item in Counter::new() { // for construct takes iterators and unwraps the result
        println!("Item count: {}", item)
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}