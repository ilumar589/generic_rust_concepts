/// Specifying Placeholder Types in Trait Definitions with Associated Types

//Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will
// specify the concrete type to be used in this type’s place for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly
// what those types are until the trait is implemented.

pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {}
}


//The difference is that when using generics, as in Listing 19-21, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type,
// we could have multiple implementations of Iterator for Counter. In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete
// types of the generic type parameters each time. When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.

//With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times. In Listing 19-20 with the definition that uses associated types, we can only
// choose what the type of Item will be once, because there can only be one impl Iterator for Counter. We don’t have to specify that we want an iterator of u32 values everywhere that we call next
// on Counter.