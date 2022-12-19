use core::cell::RefCell;

pub enum SnailfishNumber {
    Pair {
        left: Box<RefCell<SnailfishNumber>>,
        right: Box<RefCell<SnailfishNumber>>,
    },
    Number {
        value: u32,
    },
}

use std::mem;

use std::rc::Rc;

pub trait ObjectInterface {}

pub type Object = Rc<dyn ObjectInterface>;

pub type IntObject = Rc<i32>;

impl ObjectInterface for i32 {}

pub fn is_same(left: &Object, right: &Object) -> bool {
    let a = left.as_ref() as *const _;
    let b = right.as_ref() as *const _;
    let r = a == b;
    println!("comparing: {:p} == {:p} -> {}", a, b, r);

    r
}


#[test]
fn test_foo() {
    is_same(&Rc::new(5u32), &Rc::new(5u32));
}