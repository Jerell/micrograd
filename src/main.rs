use crate::value::Value;

pub mod value;

fn main() {
    let a = Value::new(2.0, "a");
    let b = Value::new(-3.0, "b");
    let c = Value::new(10.0, "c");
    let mut e = &a * &b;
    e.label("e");
    let mut d = &e + &c;
    d.label("d");
    let f = Value::new(-2.0, "f");

    let mut l = &d * &f;
    l.label("L");

    println!("{l}");
}
