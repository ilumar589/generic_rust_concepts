pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>> // dyn is used in Rust to explicitly say that we are referring to a trait. How unnecessary
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//This works differently than defining a struct that uses a generic type parameter with trait bounds.
// A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
// For example, we could have defined the Screen struct using a generic type and a trait bound as in Listing 17 - 6:

pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//This restricts us to a Screen instance that has a list of components all of type Button or all of type TextField.
// If you’ll only ever have homogeneous collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.
//
//On the other hand, with the method using trait objects, one Screen instance can hold a Vec that contains a Box<Button> as well as a Box<TextField>.
// Let’s look at how this works, and then we’ll talk about the runtime performance implications.


/// structs for gui

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!()
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        unimplemented!()
    }
}

pub fn execute() {
    let screen = Screen {
      components: vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ]
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK")
        })
      ]
    };

    screen.run();
}