extern crate nom;
use nom::branch::alt;
use nom::character::complete::char;
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::delimited;
use nom::sequence::pair;
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
    let (i, mut s) = many0(pair(term, alt((char('+'), char('-')))))(i)?;
    if s.is_empty() {
        term(i)
    } else {
        let mut node = match s.remove(0) {
            (sub_node, '+') => Node {
                left: Some(Box::new(sub_node)),
                right: None,
                op: Op::Add,
            },
            (sub_node, '-') => Node {
                left: Some(Box::new(sub_node)),
                right: None,
                op: Op::Sub,
            },
            _ => panic!("failed to parse term"),
        };

        for sn_op in s {
            let (sub_node, op) = sn_op;
            node.right = Some(Box::new(sub_node));
            node = match op {
                '+' => Node {
                    left: Some(Box::new(node)),
                    right: None,
                    op: Op::Add,
                },
                '-' => Node {
                    left: Some(Box::new(node)),
                    right: None,
                    op: Op::Sub,
                },
                _ => panic!("failed to parse term"),
            }
        }
        let (i, s) = term(i)?;
        node.right = Some(Box::new(s));
        Ok((i, node))
    }
}

fn term(i: &str) -> IResult<&str, Node> {
    let (i, mut s) = many0(pair(factor, alt((char('*'), char('/')))))(i)?;
    if s.is_empty() {
        factor(i)
    } else {
        let mut node = match s.remove(0) {
            (sub_node, '*') => Node {
                left: Some(Box::new(sub_node)),
                right: None,
                op: Op::Mul,
            },
            (sub_node, '/') => Node {
                left: Some(Box::new(sub_node)),
                right: None,
                op: Op::Div,
            },
            _ => panic!("failed to parse term"),
        };

        for sn_op in s {
            let (sub_node, op) = sn_op;
            node.right = Some(Box::new(sub_node));
            node = match op {
                '*' => Node {
                    left: Some(Box::new(node)),
                    right: None,
                    op: Op::Mul,
                },
                '/' => Node {
                    left: Some(Box::new(node)),
                    right: None,
                    op: Op::Div,
                },
                _ => panic!("failed to parse term"),
            }
        }

        let (i, s) = factor(i)?;
        node.right = Some(Box::new(s));
        Ok((i, node))
    }
}

fn factor(i: &str) -> IResult<&str, Node> {
    if let Ok((i, node_right)) = delimited(pair(char('-'), char('(')), expr, char(')'))(i) {
        let node = Node {
            left: Some(Box::new(Node {
                left: None,
                right: None,
                op: Op::Id(-1.0),
            })),
            right: Some(Box::new(node_right)),
            op: Op::Mul,
        };
        Ok((i, node))
    } else {
        let (i, node) = alt((delimited(char('('), expr, char(')')), atom))(i)?;
        Ok((i, node))
    }
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
mod test;
