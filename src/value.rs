use std::ops::{Neg, Add, Sub, Mul, Div, Not};

use crate::object::{Object, ObjString};

#[derive(Clone, Debug)]
pub enum Value {
    ValBool(bool),
    ValVoid(()),
    ValNumber(f64),
    ValObject(Object)
}

impl From<Value> for bool {
    fn from(value: Value) -> bool {
        match value{
            Value::ValBool(bool_val) => bool_val,
            _ => panic!("Error. Value is not boolean"),
        }
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> f64 {
        match value {
            Value::ValNumber(num_val) => num_val,
            _ => panic!("Error. Value is not numeric"),
        }
    }
}

impl From<Value> for () {
    fn from(value: Value) -> () {
        match value {
            Value::ValVoid(void_val) => void_val,
            _ => panic!("Error. Value is not nill"),
        }
    }
}

impl From<bool> for Value {
    fn from(bool_val: bool) -> Value {
        Value::ValBool(bool_val)
    }
}

impl From<f64> for Value {
    fn from(num_val: f64) -> Value {
        Value::ValNumber(num_val)
    }
}

impl From<()> for Value {
    fn from(void_val: ()) -> Value {
        Value::ValVoid(void_val)
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::ValNumber(num) => Value::ValNumber(-num),
            _ => panic!("Error. Negating a non numeric value is not possible"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a + b),
            (Value::ValNumber(_), _) => panic!("Error. Invalid right argument"),
            (_, Value::ValNumber(_)) => panic!("Error. Invalid left argument"),
            (_, _) => panic!("Error. Invalid arguments for arithmetic addition"),
        }
    }
}


impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a - b),
            (Value::ValNumber(_), _) => panic!("Error. Invalid right argument"),
            (_, Value::ValNumber(_)) => panic!("Error. Invalid left argument"),
            (_, _) => panic!("Error. Invalid arguments for arithmetic subtraction"),
        }
    }
}


impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a * b),
            (Value::ValNumber(_), _) => panic!("Error. Invalid right argument"),
            (_, Value::ValNumber(_)) => panic!("Error. Invalid left argument"),
            (_, _) => panic!("Error. Invalid arguments for arithmetic multiplication"),
        }
    }
}


impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValNumber(_), Value::ValNumber(0.0)) => panic!("Error. Cannot divide by zero"),
            (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a / b),
            (Value::ValNumber(_), _) => panic!("Error. Invalid right argument"),
            (_, Value::ValNumber(_)) => panic!("Error. Invalid left argument"),
            (_, _) => panic!("Error. Invalid arguments for arithmetic addition"),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Value {
        match self {
            Value::ValBool(boolean) => Value::ValBool(!boolean),
            _ => panic!("Error. Invalid argument for not operator"),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::ValNumber(a), Value::ValNumber(b)) => a.partial_cmp(b),
            _ => panic!("Error. Using non numeric values for comparison."),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::ValObject(object), Value::ValObject(other_object)) => object.get_object_data() == other_object.get_object_data(),
            (_, _) => self == other,
        }
    }
}

impl Value {
    pub fn is_number(&self) -> bool {
        match self {
            Value::ValNumber(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::ValBool(_) => true,
            _ => false,
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Value::ValBool(true) => true,
            _ => false,
        }
    }
    
    pub fn is_false(&self) -> bool {
        match self {
            Value::ValBool(false) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::ValObject(Object) => true,
            _ => false,
        }
    }

    pub fn get_inner_string(&self) -> Option<&str> {
        match self {
            Value::ValObject(object) => {
                object.get_object_data()
            }
            _ => None
        } 
    }


    pub fn print_value(&self) {
        match self {
            Value::ValBool(boolean) => print!("'{}'", boolean),
            Value::ValVoid(()) => print!("'nil'"),
            Value::ValNumber(val) => print!("'{}'", val),
            Value::ValObject(object) => print!("'{}'", object.get_object_data().unwrap_or("")),
            //_ => panic!("Value not recognised, cannot print"),
        }
    }

   // pub fn read_value(&self, which: usize)
}