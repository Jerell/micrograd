use crate::value::Value;

pub mod value;

fn main() {
    {
        // backprop example
        let mut x1 = Value::new(2.0, "x1");
        let x2 = Value::new(0.0, "x2");
        let w1 = Value::new(-3.0, "w1");
        let w2 = Value::new(1.0, "w2");
        let b = Value::new(8.0, "b");

        let mut x1w1 = &x1 * &w1;
        x1w1.label("x1w1");
        let mut x2w2 = &x2 * &w2;
        x2w2.label("x1w1");

        let mut prodsum = &x1w1 + &x2w2;
        prodsum.label("x1w1 + x2w2");

        let mut n = &prodsum + &b;
        n.label("n");

        let mut o = n.tanh();
        o.grad(1.0);

        println!("{o}");
    }
}
