use crate::value::Value;

pub mod value;

fn main() {
    let a = Value::new(1.0, "a");
    println!("Hello, world! {a}");
}
