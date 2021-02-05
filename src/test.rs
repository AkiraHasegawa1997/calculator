use super::*;
use nom::combinator::all_consuming;
#[test]
fn atom_test() {
    let (_, node) = all_consuming(atom)("3.2").unwrap();
    assert_eq!(
        node,
        Node {
            left: None,
            right: None,
            op: Op::Id(3.2)
        }
    );
}
#[test]
fn factor_test() {
    let (_, node) = all_consuming(factor)("3.14").unwrap();
    assert_eq!(
        node,
        Node {
            left: None,
            right: None,
            op: Op::Id(3.14)
        }
    );
}

#[test]
fn factor_and_expr() {
    let (_, node) = all_consuming(factor)("(3.2)").unwrap();
    assert_eq!(
        node,
        Node {
            left: None,
            right: None,
            op: Op::Id(3.2)
        }
    );
}

#[test]
fn term_test() {
    let (_, node) = all_consuming(term)("3*2").unwrap();
    assert_eq!(
        node,
        Node {
            left: Some(Box::new(Node {
                left: None,
                right: None,
                op: Op::Id(3.0)
            })),
            right: Some(Box::new(Node {
                left: None,
                right: None,
                op: Op::Id(2.0)
            })),
            op: Op::Mul
        }
    );
}

#[test]
fn expr_test() {
    let (_, node) = formula("3*4+4/3").unwrap();
    let test_node = Node {
        op: Op::Add,
        left: Some(Box::new(Node {
            op: Op::Mul,
            left: Some(Box::new(Node {
                op: Op::Id(3.0),
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                op: Op::Id(4.0),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            op: Op::Div,
            left: Some(Box::new(Node {
                op: Op::Id(4.0),
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                op: Op::Id(3.0),
                left: None,
                right: None,
            })),
        })),
    };
    assert_eq!(node, test_node);
    let (_, node) = formula("(3+4)/3").unwrap();
    let test_node = Node {
        op: Op::Div,
        left: Some(Box::new(Node {
            op: Op::Add,
            left: Some(Box::new(Node {
                op: Op::Id(3.0),
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                op: Op::Id(4.0),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            op: Op::Id(3.0),
            left: None,
            right: None,
        })),
    };
    assert_eq!(node, test_node);
    let (_, node) = formula("((3+4)/(3))").unwrap();
    let test_node = Node {
        op: Op::Div,
        left: Some(Box::new(Node {
            op: Op::Add,
            left: Some(Box::new(Node {
                op: Op::Id(3.0),
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                op: Op::Id(4.0),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            op: Op::Id(3.0),
            left: None,
            right: None,
        })),
    };
    assert_eq!(node, test_node);
}

#[test]
fn eval_test() {
    let ans = eval("(2+2)/2");
    assert_eq!(ans, Some(2.0));
}

#[test]
fn minus_test() {
    let (_, node) = formula("-(100+2)").unwrap();
    println!("{:?}", node);
    assert_eq!(eval("-(100+2)"), Some(-102.0));
    let (_, node) = formula("-3.0+10.0-5.0-100.0").unwrap();
    println!("{:?}", node);
    assert_eq!(eval("-3.0+10.0-5.0-100.0"), Some(-98.0));
    let (_, node) = formula("-122.72-245.44-68.27-27.85-27.48+28.13+28.86").unwrap();
    println!("{:?}", node);
    assert_eq!(
        eval("-122.72-245.44-68.27-27.85-27.48+28.13+28.86"),
        Some(-122.72 - 245.44 - 68.27 - 27.85 - 27.48 + 28.13 + 28.86)
    );
}
