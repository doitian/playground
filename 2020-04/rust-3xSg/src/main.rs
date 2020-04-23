#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RPCError {
    Foo = 1,
    Bar = 2,
}

fn main() {
    println!("{:?}", RPCError::Foo);
}
