pub enum Greeting {
    Hello,
    Goodbye,
}

pub fn greet(greeting: Greeting) {
    match greeting {
        Greeting::Hello => println!("Hello World!"),
        Greeting::Goodbye => println!("Goodbye World!"),
    }
}
