use std::{fmt, ops};

#[derive(Debug, PartialEq)]
pub struct Value {
    data: f32,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value(data={})", self.data)
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, other: Value) -> Value {
        Value {
            data: self.data + other.data,
        }
    }
}

impl Value {
    fn new(v: f32) -> Value {
        Value { data: v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let nums = (2.0, -3.0);

        let a = Value::new(nums.0);
        let b = Value::new(nums.1);

        assert_eq!(a + b, Value::new(nums.0 + nums.1));
    }
}
