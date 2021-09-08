use tiberius_derive::FromRow;

#[derive(FromRow)]

struct Foobar<'b> {
    pub foo: Option<i32>,
    pub bar: Option<&'b str>,
}

#[derive(FromRow)]
struct FoobarNoLifetime {
    pub foo: Option<i32>,
}

// #[derive(FromRow)]
// struct FoobarOwned {
//     pub foo: Option<String>,
// }

fn main() {
    println!("Hello, world!");
}
