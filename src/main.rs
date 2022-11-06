use crate::value::Value;

pub mod value;

fn main() {
    {
        //forward pass example
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

        l.grad(1.0);

        println!("{l}");
    }

    {
        // backprop example
        let x1 = Value::new(2.0, "x1");
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

        let o = n.tanh();

        println!("{o}")
    }
}
