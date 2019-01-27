/// Ensuring one lifetime outlives another with lifetime subtyping

//struct Context<'a>(&'a str);
//
//struct Parser<'a> {
//    context: &'a Context<'a> // weird how the lifetime has to be specified everywhere
//}
//
//impl<'a> Parser<'a> {
//    fn parse(&self) -> Result<(), &str> {
//        Err(&self.context.0[1..])
//    }
//
//    fn parse_context(context: Context) -> Result<(), &str> {
//        Parser {
//            context: &context
//        }.parse()
//    }
//}

//This code compiles just fine. It tells Rust that a Parser holds a reference to a Context with lifetime 'a and that Context holds a string slice that also lives as long as the reference
// to the Context in Parser. Rust’s compiler error message stated that lifetime parameters were required for these references, and we’ve now added lifetime parameters



//First, we’ll try giving Parser and Context different lifetime parameters, as shown in Listing 19-15. We’ll use 's and 'c as lifetime parameter names to clarify which lifetime goes with
// the string slice in Context and which goes with the reference to Context in Parser. Note that this solution won’t completely fix the problem, but it’s a start. We’ll look at why this fix
// isn’t sufficient when we try to compile
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

//Rust doesn’t know of any relationship between 'c and 's. To be valid, the referenced data in Context with lifetime 's needs to be constrained to guarantee that it lives longer than
// the reference with lifetime 'c. If 's is not longer than 'c, the reference to Context might not be valid.

//Now we get to the point of this section: the Rust feature lifetime subtyping specifies that one lifetime parameter lives at least as long as another one. In the angle brackets where
// we declare lifetime parameters, we can declare a lifetime 'a as usual and declare a lifetime 'b that lives at least as long as 'a by declaring 'b using the syntax 'b: 'a.

//In our definition of Parser, to say that 's (the lifetime of the string slice) is guaranteed to live at least as long as 'c (the lifetime of the reference to Context),
// we change the lifetime declarations to look like this:

//struct Parser<'c, 's: 'c> {
//    context: &'c Context<'s>,
//}

