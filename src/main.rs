extern crate nom;
use nom::branch::alt;
use nom::character::complete;
use nom::combinator::opt;
use nom::number::complete::float;
use nom::sequence::delimited;
use nom::IResult;
use rustyline::Editor;

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Id(f32),
}

#[derive(Debug, PartialEq)]
struct Node {
    left: Option<Box<Node>>,
    op: Op,
    right: Option<Box<Node>>,
}

fn formula(i: &str) -> IResult<&str, Node> {
    nom::combinator::all_consuming(expr)(i)
}

fn expr(i: &str) -> IResult<&str, Node> {
    let (i, sub_tree) = term(i)?;
    if let (i, Some(matched_op)) = opt(alt((complete::char('+'), complete::char('-'))))(i)? {
        let matched_op = match matched_op {
            '+' => Op::Add,
            '-' => Op::Sub,
            _ => panic!("term parse error"),
        };
        let (i, sub2_tree) = expr(i)?;
        Ok((
            i,
            Node {
                left: Some(Box::new(sub_tree)),
                right: Some(Box::new(sub2_tree)),
                op: matched_op,
            },
        ))
    } else {
        Ok((i, sub_tree))
    }
}

fn term(i: &str) -> IResult<&str, Node> {
    let (i, sub_tree) = factor(i)?;
    if let (i, Some(matched_op)) = opt(alt((complete::char('*'), complete::char('/'))))(i)? {
        let matched_op = match matched_op {
            '*' => Op::Mul,
            '/' => Op::Div,
            _ => panic!("term parse error"),
        };
        let (i, sub2_tree) = term(i)?;
        Ok((
            i,
            Node {
                left: Some(Box::new(sub_tree)),
                right: Some(Box::new(sub2_tree)),
                op: matched_op,
            },
        ))
    } else {
        Ok((i, sub_tree))
    }
}

fn factor(i: &str) -> IResult<&str, Node> {
    let (i, node) = alt((
        delimited(complete::char('('), expr, complete::char(')')),
        atom,
    ))(i)?;
    Ok((i, node))
}

fn atom(i: &str) -> IResult<&str, Node> {
    let (i, id) = float(i)?;
    Ok((
        i,
        Node {
            left: None,
            right: None,
            op: Op::Id(id),
        },
    ))
}

fn eval(input: &str) -> Option<f32> {
    fn traverse(stack: &mut Vec<f32>, node: &Node) {
        if let Some(left_node) = &node.left {
            traverse(stack, left_node);
        }
        if let Some(right_node) = &node.right {
            traverse(stack, right_node);
        }
        match node.op {
            Op::Add => {
                let v1 = stack.pop().expect("invalid formula");
                let v2 = stack.pop().expect("invalid formula");
                stack.push(v2 + v1);
            }
            Op::Sub => {
                let v1 = stack.pop().expect("invalid formula");
                let v2 = stack.pop().expect("invalid formula");
                stack.push(v2 - v1);
            }
            Op::Mul => {
                let v1 = stack.pop().expect("invalid formula");
                let v2 = stack.pop().expect("invalid formula");
                stack.push(v2 * v1);
            }
            Op::Div => {
                let v1 = stack.pop().expect("invalid formula");
                let v2 = stack.pop().expect("invalid formula");
                stack.push(v2 / v1);
            }
            Op::Id(x) => stack.push(x),
        }
    }

    let input = &input.split_whitespace().collect::<String>();
    if let Ok((_, node)) = formula(input) {
        let mut stack = Vec::<f32>::new();
        traverse(&mut stack, &node);
        stack.pop()
    } else {
        None
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Some(x) = eval(&line) {
                    println!("{}", x);
                } else {
                    eprintln!("invalid formula: {}", &line);
                }
            }
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::combinator::all_consuming;
    #[test]
    fn factor_test() {
        let (_, node) = all_consuming(factor)("(3.2)").unwrap();
        assert_eq!(
            node,
            Node {
                left: None,
                right: None,
                op: Op::Id(3.2)
            }
        );
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
}
