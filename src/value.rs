use std::{
    fmt,
    ops::{Add, Mul},
};

#[derive(Debug)]
pub struct Value<'a> {
    data: f32,
    grad: f32,
    _prev: (Option<&'a Value<'a>>, Option<&'a Value<'a>>),
    _op: Option<Operation>,
    label: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
}

impl Default for Value<'_> {
    fn default() -> Self {
        Value {
            data: 0.0,
            grad: 0.0,
            _prev: (None, None),
            _op: None,
            label: String::from(""),
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Value( {} | data={} | grad={:.4})",
            self.label, self.data, self.grad
        )
    }
}

impl PartialEq for Value<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'a> Add<&'a Value<'a>> for &'a Value<'a> {
    type Output = Value<'a>;
    fn add(self, other: &'a Value) -> Value<'a> {
        Value::new_with_children(self.data + other.data, self, other, Operation::Add)
    }
}

impl<'a> Mul<&'a Value<'_>> for &'a Value<'_> {
    type Output = Value<'a>;
    fn mul(self, other: &'a Value) -> Value<'a> {
        Value::new_with_children(self.data * other.data, self, other, Operation::Mul)
    }
}

impl Value<'_> {
    pub fn new(v: f32, label: &str) -> Value<'static> {
        Value {
            data: v,
            _prev: (None, None),
            _op: None,
            label: String::from(label),
            ..Default::default()
        }
    }

    fn new_with_children<'a>(
        v: f32,
        s: &'a Value<'a>,
        o: &'a Value<'a>,
        op: Operation,
    ) -> Value<'a> {
        Value {
            data: v,
            _prev: (Some(s), Some(o)),
            _op: Some(op),
            label: String::from(""),
            ..Default::default()
        }
    }

    pub fn label(&mut self, label: &str) {
        self.label = String::from(label);
    }

    pub fn grad(&mut self, grad: f32) {
        self.grad = grad
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let nums = (2.0, -3.0);
        let answer = Value::new(nums.0 + nums.1, "answer");

        let a = Value::new(nums.0, "a");
        let b = Value::new(nums.1, "b");

        let mut sum = &a + &b;
        sum.label("sum");

        assert_eq!(sum, answer);
        assert_eq!(sum.label, "sum");
        assert_eq!(sum._prev.0, Some(&a));
        assert_eq!(sum._prev.1, Some(&b));
        assert_eq!(sum._op, Some(Operation::Add));
    }

    #[test]
    fn mul() {
        let nums = (2.0, -3.0);

        let answer = Value::new(nums.0 * nums.1, "answer");

        let a = Value::new(nums.0, "a");
        let b = Value::new(nums.1, "b");

        let mut prod = &a * &b;
        prod.label("product");

        assert_eq!(prod, answer);
        assert_eq!(prod.label, "product");
        assert_eq!(prod._prev.0, Some(&a));
        assert_eq!(prod._prev.1, Some(&b));
        assert_eq!(prod._op, Some(Operation::Mul));
    }
}
