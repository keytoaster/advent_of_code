use core::ops::Add;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::tuple;
use nom::IResult;
use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

pub struct SnailfishNumber {
    data: Rc<Sn>,
}

impl PartialEq for SnailfishNumber {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.data, &other.data)
    }
}

impl Eq for SnailfishNumber {}

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.data.deref() {
            Sn::Pair {
                left: l, right: r, ..
            } => f.write_fmt(format_args!("[{:?},{:?}]", l.borrow(), r.borrow())),
            Sn::Number { value: v, .. } => f.write_str(&v.borrow().to_string()),
        }
    }
}

impl SnailfishNumber {
    pub fn from(input: &str) -> IResult<&str, SnailfishNumber> {
        let (input, data) = snailfish_number(input)?;

        Ok((input, data))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn magnitude(&self) -> u32 {
        match self.data.deref() {
            Sn::Pair {
                left: l, right: r, ..
            } => 3 * l.borrow().magnitude() + 2 * r.borrow().magnitude(),
            Sn::Number { value: v, .. } => *v.borrow(),
        }
    }

    fn set_children_parent_pointers(&self) {
        if let Sn::Pair {
            left: l, right: r, ..
        } = self.data.deref()
        {
            let l_borrow = l.borrow();
            let (Sn::Pair { parent: p, .. } | Sn::Number { parent: p, .. }) =
                (*l_borrow).deref().data.deref();
            p.replace(Rc::downgrade(&self.data));

            let r_borrow = r.borrow();
            let (Sn::Pair { parent: p, .. } | Sn::Number { parent: p, .. }) =
                (*r_borrow).deref().data.deref();
            p.replace(Rc::downgrade(&self.data));
        }
    }

    fn leftmost_nested_pair(&self, depth: usize) -> Option<SnailfishNumber> {
        match self.data.deref() {
            Sn::Pair {
                left: l, right: r, ..
            } => {
                if depth == 0 {
                    // Some(Rc::clone(&self.data))
                    Some(SnailfishNumber {
                        data: Rc::clone(&self.data),
                    })
                } else {
                    l.borrow()
                        .leftmost_nested_pair(depth - 1)
                        .or(r.borrow().leftmost_nested_pair(depth - 1))
                }
            }
            Sn::Number { .. } => None,
        }
    }

    fn leftmost_number_to_split(&self) -> Option<SnailfishNumber> {
        match self.data.deref() {
            Sn::Pair {
                left: l, right: r, ..
            } => l
                .borrow()
                .leftmost_number_to_split()
                .or(r.borrow().leftmost_number_to_split()),
            Sn::Number { value: v, .. } => {
                if *v.borrow() >= 10 {
                    Some(SnailfishNumber {
                        data: Rc::clone(&self.data),
                    })
                } else {
                    None
                }
            }
        }
    }

    fn verify_parents(&self) {
        self.verify_parents_internal(None);
    }

    fn verify_parents_internal(&self, expected_parent: Option<&SnailfishNumber>) {
        match self.data.deref() {
            Sn::Pair {
                left: l,
                right: r,
                parent: p,
            } => {
                l.borrow().verify_parents_internal(Some(self));

                r.borrow().verify_parents_internal(Some(self));
            }
            Sn::Number {
                value: v,
                parent: p,
            } => {
                if expected_parent.is_none() {
                    if !p.borrow().upgrade().is_none() {
                        panic!("parent not expected, but found");
                    }
                } else {
                    if p.borrow().upgrade().is_none() {
                        panic!("did not find a parent for {:?}", self);
                    }
                    if p.borrow().upgrade().unwrap() != expected_parent.unwrap().data {
                        panic!("fail");
                    }
                }
            }
        }
    }
}

impl Clone for SnailfishNumber {
    fn clone(&self) -> Self {
        match self.data.deref() {
            Sn::Pair {
                left: l, right: r, ..
            } => {
                let retval = SnailfishNumber {
                    data: Rc::new(Sn::Pair {
                        left: RefCell::new(Rc::new((**l.borrow()).clone())),
                        right: RefCell::new(Rc::new((**r.borrow()).clone())),
                        parent: RefCell::new(Weak::new()),
                    }),
                };
                retval.set_children_parent_pointers();
                retval
            }
            Sn::Number { value: v, .. } => SnailfishNumber {
                data: Rc::new(Sn::Number {
                    value: v.clone(),
                    parent: RefCell::new(Weak::new()),
                }),
            },
        }
    }
}

fn get_parent(n: &SnailfishNumber) -> Option<SnailfishNumber> {
    match n.data.deref() {
        Sn::Pair { parent: p, .. } => Some(SnailfishNumber {
            data: (*p.borrow()).upgrade()?,
        }),
        Sn::Number { parent: p, .. } => Some(SnailfishNumber {
            data: (*p.borrow()).upgrade()?,
        }),
    }
}

fn get_max(n: &SnailfishNumber) -> SnailfishNumber {
    match n.data.deref() {
        Sn::Pair { right: r, .. } => get_max(r.borrow().deref()),
        Sn::Number { .. } => SnailfishNumber {
            data: Rc::clone(&n.data),
        },
    }
}

fn get_min(n: &SnailfishNumber) -> SnailfishNumber {
    match n.data.deref() {
        Sn::Pair { left: l, .. } => get_min(l.borrow().deref()),
        Sn::Number { .. } => SnailfishNumber {
            data: Rc::clone(&n.data),
        },
    }
}

fn get_left(n: &SnailfishNumber) -> Option<SnailfishNumber> {
    let mut node = SnailfishNumber {
        data: Rc::clone(&n.data),
    };

    loop {
        // println!("iteration with node: {:?}", node);
        let parent = get_parent(&node);
        if let None = parent {
            return None;
        }
        let parent = parent.unwrap();

        if let Sn::Pair {
            left: l, right: r, ..
        } = parent.data.deref()
        {
            let r = r.borrow();
            // println!("determining right child: {:?}", r);
            if node == **r {
                return Some(get_max(&l.borrow()));
            }
        }

        node = SnailfishNumber { data: parent.data };
        // println!("new node: {:?}", node);
    }
}

fn get_right(n: &SnailfishNumber) -> Option<SnailfishNumber> {
    let mut node = SnailfishNumber {
        data: Rc::clone(&n.data),
    };

    loop {
        let parent = get_parent(&node);
        if let None = parent {
            return None;
        }
        let parent = parent.unwrap();

        if let Sn::Pair {
            left: l, right: r, ..
        } = parent.data.deref()
        {
            let l = l.borrow();
            if node == **l {
                return Some(get_min(&r.borrow()));
            }
        }

        node = SnailfishNumber { data: parent.data }
    }
}

impl Add for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, other: &SnailfishNumber) -> SnailfishNumber {
        let retval = SnailfishNumber {
            data: Rc::new(Sn::Pair {
                left: RefCell::new(Rc::new((*self).clone())),
                right: RefCell::new(Rc::new((*other).clone())),
                parent: RefCell::new(Weak::new()),
            }),
        };
        retval.set_children_parent_pointers();

        // println!("after add: {:?}", retval);
        retval.verify_parents();

        loop {
            while let Some(pair_to_explode) = retval.leftmost_nested_pair(4) {
                if let Sn::Pair {
                    left: l, right: r, ..
                } = pair_to_explode.data.deref()
                {
                    // println!("left node of nested pair: {:?}", l.borrow().deref());
                    let left_neighbour = get_left(l.borrow().deref());
                    let right_neighbour = get_right(r.borrow().deref());

                    // println!("left neighbour: {:?}", left_neighbour);
                    // println!("right neighbour: {:?}", right_neighbour);
                    if let Some(left_neighbour) = left_neighbour {
                        if let Sn::Number {
                            value: explode_left,
                            ..
                        } = l.borrow().data.deref()
                        {
                            if let Sn::Number { value: v, .. } = left_neighbour.data.deref() {
                                let mut v_borrow = v.borrow_mut();
                                *v_borrow = *v_borrow + *explode_left.borrow();
                            } else {
                                panic!("Neighbour found, but was not a number");
                            }
                        } else {
                            panic!("exploding pair's left child is not a Number");
                        }
                    }
                    if let Some(right_neighbour) = right_neighbour {
                        if let Sn::Number {
                            value: explode_right,
                            ..
                        } = r.borrow().data.deref()
                        {
                            if let Sn::Number { value: v, .. } = right_neighbour.data.deref() {
                                let mut v_borrow = v.borrow_mut();
                                *v_borrow = *v_borrow + *explode_right.borrow();
                            } else {
                                panic!("Neighbour found, but was not a number");
                            }
                        } else {
                            panic!("exploding pair's left child is not a Number");
                        }
                    }

                    replace_with_number(pair_to_explode, 0);
                } else {
                    panic!("pair expected");
                }

                // println!("after explode:  {:?}", retval);
                retval.verify_parents();
                // println!("parents ok");
            }

            if let Some(number_to_split) = retval.leftmost_number_to_split() {
                replace_with_pair(number_to_split);
                // println!("after split  :  {:?}", retval);
                retval.verify_parents();
                // println!("parents ok");
            } else {
                break; // Overall action loop.
            }
        }

        retval
    }
}

fn replace_with_number(node: SnailfishNumber, new_value: u32) {
    let parent = get_parent(&node);

    if let None = parent {
        panic!("unsupported");
    }
    let parent = parent.unwrap();

    if let Sn::Pair {
        left: l, right: r, ..
    } = parent.data.deref()
    {
        if node == **l.borrow() {
            *l.borrow_mut() = Rc::new(SnailfishNumber {
                data: Rc::new(Sn::Number {
                    value: RefCell::new(new_value),
                    parent: RefCell::new(Weak::new()),
                }),
            });
        } else if node == **r.borrow() {
            *r.borrow_mut() = Rc::new(SnailfishNumber {
                data: Rc::new(Sn::Number {
                    value: RefCell::new(new_value),
                    parent: RefCell::new(Weak::new()),
                }),
            });
        } else {
            panic!("could not determine if node is left or right child");
        }
    }

    parent.set_children_parent_pointers();
}

fn replace_with_pair(node: SnailfishNumber) {
    let parent = get_parent(&node);

    if let None = parent {
        println!("{:?}", node);
        panic!("Number to split did not have a parent");
    }
    let parent = parent.unwrap();

    let old_node_value;
    if let Sn::Number { value: v, .. } = node.data.deref() {
        old_node_value = *v.borrow();
    } else {
        panic!("Argument to split was not a Number");
    }

    let new_pair = Rc::new(SnailfishNumber {
        data: Rc::new(Sn::Pair {
            left: RefCell::new(Rc::new(SnailfishNumber {
                data: Rc::new(Sn::Number {
                    value: RefCell::new(old_node_value / 2),
                    parent: RefCell::new(Weak::new()),
                }),
            })),
            right: RefCell::new(Rc::new(SnailfishNumber {
                data: Rc::new(Sn::Number {
                    value: RefCell::new((old_node_value + 1) / 2),
                    parent: RefCell::new(Weak::new()),
                }),
            })),
            parent: RefCell::new(Rc::downgrade(&parent.data)),
        }),
    });
    new_pair.set_children_parent_pointers();

    if let Sn::Pair {
        left: l, right: r, ..
    } = parent.data.deref()
    {
        if node == **l.borrow() {
            *l.borrow_mut() = new_pair;
        } else if node == **r.borrow() {
            *r.borrow_mut() = new_pair;
        } else {
            panic!("could not determine if node is left or right child");
        }
    }
}

#[derive(Debug)]
enum Sn {
    Pair {
        left: RefCell<Rc<SnailfishNumber>>,
        right: RefCell<Rc<SnailfishNumber>>,
        parent: RefCell<Weak<Sn>>,
    },
    Number {
        value: RefCell<u32>,
        parent: RefCell<Weak<Sn>>,
    },
}

impl PartialEq for Sn {
    fn eq(&self, other: &Self) -> bool {
        if let Sn::Pair {
            left: l, right: r, ..
        } = &self
        {
            if let Sn::Pair {
                left: l2,
                right: r2,
                ..
            } = other
            {
                l == l2 && r == r2
            } else {
                false
            }
        } else if let Sn::Number { value: v, .. } = &self {
            if let Sn::Number { value: v2, .. } = other {
                v == v2
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Sn {
    fn len(&self) -> usize {
        if let Sn::Pair { left, right, .. } = self {
            1 + max(left.borrow().len(), right.borrow().len())
        } else {
            0
        }
    }
}

fn sfn_pair(input: &str) -> IResult<&str, SnailfishNumber> {
    let (input, (_, left, _, right, _)) = tuple((
        tag("["),
        snailfish_number,
        tag(","),
        snailfish_number,
        tag("]"),
    ))(input)?;

    let retval = SnailfishNumber {
        data: Rc::new(Sn::Pair {
            left: RefCell::new(Rc::new(left)),
            right: RefCell::new(Rc::new(right)),
            parent: RefCell::new(Weak::new()),
        }),
    };

    retval.set_children_parent_pointers();

    Ok((input, retval))
}

fn sfn_number(input: &str) -> IResult<&str, SnailfishNumber> {
    let (input, number) = digit1(input)?;

    Ok((
        input,
        SnailfishNumber {
            data: Rc::new(Sn::Number {
                value: RefCell::new(number.parse().unwrap()),
                parent: RefCell::new(Weak::new()),
            }),
        },
    ))
}

fn snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
    alt((sfn_pair, sfn_number))(input)
}

#[test]
fn test_number() {
    assert_eq!(
        snailfish_number("185"),
        Ok((
            "",
            SnailfishNumber {
                data: Rc::new(Sn::Number {
                    value: RefCell::new(185),
                    parent: RefCell::new(Weak::new()),
                }),
            }
        ))
    );
}

#[test]
fn test_pair() {
    assert_eq!(
        snailfish_number("[3,4]"),
        Ok((
            "",
            SnailfishNumber {
                data: Rc::new(Sn::Pair {
                    left: RefCell::new(Rc::new(SnailfishNumber {
                        data: Rc::new(Sn::Number {
                            value: RefCell::new(3),
                            parent: RefCell::new(Weak::new()),
                        }),
                    })),
                    right: RefCell::new(Rc::new(SnailfishNumber {
                        data: Rc::new(Sn::Number {
                            value: RefCell::new(4),
                            parent: RefCell::new(Weak::new()),
                        }),
                    })),
                    parent: RefCell::new(Weak::new()),
                }),
            },
        ))
    );
}

#[test]
fn test_pair_more_complicated() {
    assert!(
        snailfish_number("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").is_ok()
    );
}

#[test]
fn test_len() {
    let (input, n) =
        SnailfishNumber::from("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")
            .unwrap();
    assert_eq!(input, "");
    assert_eq!(n.len(), 4);
}

#[test]
fn test_leftmost_nested_pair() {
    let (_, n) = snailfish_number("[[6,[5,[4,[3,2]]]],1]").unwrap();

    assert!(n.leftmost_nested_pair(4).is_some());
    assert!(n.leftmost_nested_pair(5).is_none());
}

#[test]
fn test_get_max() {
    let (_, n1) = SnailfishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();

    assert_eq!(
        get_max(&n1),
        SnailfishNumber {
            data: Rc::new(Sn::Number {
                value: RefCell::new(9),
                parent: RefCell::new(Weak::new()),
            }),
        }
    );
}

#[test]
fn test_get_left() {
    let (_, n1) = SnailfishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();

    let max = get_max(&n1);

    println!("get_left first");

    let left = get_left(&max).unwrap();
    assert_eq!(
        left,
        SnailfishNumber {
            data: Rc::new(Sn::Number {
                value: RefCell::new(4),
                parent: RefCell::new(Weak::new()),
            }),
        }
    );

    println!("get_left second");

    let left = get_left(&left).unwrap();
    assert_eq!(
        left,
        SnailfishNumber {
            data: Rc::new(Sn::Number {
                value: RefCell::new(8),
                parent: RefCell::new(Weak::new()),
            }),
        }
    );

    println!("get_left third");

    let left = get_left(&left).unwrap();
    assert_eq!(
        left,
        SnailfishNumber {
            data: Rc::new(Sn::Number {
                value: RefCell::new(7),
                parent: RefCell::new(Weak::new()),
            }),
        }
    );

    ////
    let min = get_min(&n1);
    assert!(get_left(&min).is_none());
}

#[test]
fn test_get_right() {
    let (_, n1) = SnailfishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();

    let max = get_max(&n1);

    assert!(get_right(&max).is_none());
}

#[test]
fn test_add() {
    let (_, n1) = SnailfishNumber::from("[1,2]").unwrap();
    let (_, n2) = SnailfishNumber::from("[3,4]").unwrap();

    let (_, expected) = SnailfishNumber::from("[[1,2],[3,4]]").unwrap();
    assert_eq!(&n1 + &n2, expected);

    let (_, n1_copy) = SnailfishNumber::from("[1,2]").unwrap();
    let (_, n2_copy) = SnailfishNumber::from("[3,4]").unwrap();
    assert_eq!(n1, n1_copy);
    assert_eq!(n2, n2_copy);

    ////////////

    let (_, n1) = SnailfishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
    let (_, n2) = SnailfishNumber::from("[1,1]").unwrap();

    let (_, expected) = SnailfishNumber::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
    assert_eq!(&n1 + &n2, expected);
}
