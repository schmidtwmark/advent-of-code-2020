fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;
    use std::collections::VecDeque;


    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Operator{
        Plus,
        Multiply,
        LParen,
    }

    impl Operator {
        fn to_char(&self) -> char {
            match self {
                Self::Plus => '+',
                Self::Multiply => '*',
                Self::LParen => '('
            }
        }
    }


    #[derive(Debug, Clone, PartialEq, Eq)]
    enum ExpressionNode {
        Value(i64),
        Operation(Operator, Box<ExpressionNode>, Box<ExpressionNode>),
        Paren(Box<ExpressionNode>)
    }

    impl ExpressionNode {
        fn from_stacks(operators: &mut VecDeque<Operator>, vals: &mut VecDeque<ExpressionNode>) -> ExpressionNode {
            let v = vals.pop_back().unwrap();
            if let Some(op) = operators.pop_back() {
                ExpressionNode::Operation(op, Box::new(v), Box::new(ExpressionNode::from_stacks(operators, vals)))
            } else {
                v
            }
        }

        fn from_line(line: &str) -> (ExpressionNode, usize) {
            // println!("Processing expression {line}");
            let mut operator_stack= VecDeque::new();
            let mut num_queue = VecDeque::new();
            let spaceless : String = line.chars().filter(|c| *c != ' ').collect();
            let mut i = 0;
            let chars = spaceless.chars().collect_vec();
            while i < chars.len() {
                let c= chars[i];
                match c {
                    '0'..='9' => {
                        num_queue.push_back(ExpressionNode::Value(c.to_digit(10).unwrap() as i64));
                    },
                    '+' => operator_stack.push_back(Operator::Plus),
                    '*' => operator_stack.push_back(Operator::Multiply),
                    '(' => {
                        // println!("Entering paren scope");
                        let (expr, chars_consumed )= ExpressionNode::from_line(&spaceless[(i + 1)..]);
                        num_queue.push_back(ExpressionNode::Paren(Box::new(expr)));
                        i += chars_consumed;
                    }
                    ')' => {
                        // println!("Exiting paren scope");
                        // end of the current scope, return what we've got
                        return (ExpressionNode::from_stacks(&mut operator_stack, &mut num_queue), i + 1);

                    }
                    _ => panic!()
                }
                i += 1;
            }
            // println!("Processed line");
            (ExpressionNode::from_stacks(&mut operator_stack, &mut num_queue), i)
        }
        fn from_line2(line: &str) -> Vec<char>{
            // println!("Processing expression {line}");
            
            let mut operator_stack : VecDeque<Operator> = VecDeque::new();
            let mut out = vec![];
            let spaceless : String = line.chars().filter(|c| *c != ' ').collect();

            let mut i = 0;
            let chars = spaceless.chars().collect_vec();
            while i < chars.len() {
                let c= chars[i];
                // println!("Processing {c}, out: {out:?}, stack: {operator_stack:?}");
                match c {
                    '0'..='9' => {
                        out.push(c)
                    },
                    '+' => {
                        // while operator_stack.back().map_or(false, |op: &Operator| *op == Operator::Multiply) {
                        //     let op = operator_stack.pop_back().unwrap();
                        //     out.push(op.to_char());
                        // }
                        operator_stack.push_back(Operator::Plus)
                    },
                    '*' => {
                        // while operator_stack.back().map_or(false, |op: &Operator| *op == Operator::Plus) {
                        //     let op = operator_stack.pop_back().unwrap();
                        //     out.push(op.to_char());
                        // }
                        while let Some(op) = operator_stack.back() {
                            if *op != Operator::LParen {
                                let c = op.to_char();
                                operator_stack.pop_back();
                                out.push(c);
                            } else {
                                break;
                            }
                        }
                        operator_stack.push_back(Operator::Multiply)
                    },
                    '(' => {
                        // println!("Entering paren scope");
                        // let (expr, chars_consumed )= ExpressionNode::from_line2(&spaceless[(i + 1)..]);
                        // num_queue.push_back(ExpressionNode::Paren(Box::new(expr)));
                        // i += chars_consumed;
                        operator_stack.push_back(Operator::LParen)
                    }
                    ')' => {
                        // println!("Exiting paren scope");
                        // end of the current scope, return what we've got
                        // return (ExpressionNode::from_stacks2(&mut operator_stack, &mut num_queue), i + 1);
                        while let Some(op) = operator_stack.pop_back() {
                            if op == Operator::LParen {
                                break;
                            } else {
                                out.push(op.to_char())
                            }
                        }
                    }
                    _ => panic!()
                }
                i += 1;
            }
            // println!("Processed line");
            // (ExpressionNode::from_stacks2(&mut operator_stack, &mut num_queue), i)
            out.extend(operator_stack.into_iter().rev().map(|op| op.to_char()));
            out
        }
        
        fn evaluate(&self) -> i64 {
            match self {
                ExpressionNode::Operation(op, left, right) => {
                    match op {
                        Operator::Multiply=> left.evaluate() * right.evaluate(),
                        Operator::Plus => left.evaluate() + right.evaluate(),
                        _ => panic!()
                    }
                },
                ExpressionNode::Value(v) => *v,
                ExpressionNode::Paren(inner) => inner.evaluate()
            }
        }
    }






    fn part_one(lines: Vec<String>, _param: usize) -> i64 {
        let expressions = lines.iter().map(|l| (l, ExpressionNode::from_line(l).0)).collect_vec();
        expressions.iter().map(|(l, expr)| {
            let eval = expr.evaluate();
            // println!("Line: {l}\nExpression: {expr:?}\nValue: {eval}\n");
            println!("Line: {l}\nValue: {eval}\n");
            eval
        }
        ).sum()
    }

    fn part_two(lines: Vec<String>, _param: usize) -> i64{
        let expressions = lines.iter().map(|l| (l, ExpressionNode::from_line2(l))).collect_vec();
        expressions.iter().map(|(l, expr)| {

            let mut stack = VecDeque::new();
            for c in expr {
                match c {
                    '0'..='9' => {
                        stack.push_back(c.to_digit(10).unwrap() as i64)
                    },
                    '+' => {
                        let a = stack.pop_back().unwrap();
                        let b = stack.pop_back().unwrap();
                        stack.push_back(a + b);
                    }
                    '*' => {
                        let a = stack.pop_back().unwrap();
                        let b = stack.pop_back().unwrap();
                        stack.push_back(a * b);

                    }
                    _ => panic!()

                }
            }
            let value = stack.pop_back().unwrap();
            println!("Line: {l}\nRpn: {}\nValue: {}\n", expr.iter().collect::<String>(), value);
            // println!("Line: {l}\nValue: {eval}\n");
            value
        }
        ).sum()
    }

    fn get_filename(sample: bool) -> &'static str {
        if sample {
            "samples/18.txt"
        } else {
            "inputs/18.txt"
        }
    }

    fn get_lines(filename: &str) -> Vec<String> {
        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
        contents.lines().map(|s| s.to_owned()).collect_vec()
    }

    const SAMPLE_PARAM: usize = 0;
    const REAL_PARAM: usize = 0;

    #[test]
    fn test_part_one_sample() {
        let result = part_one(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part one sample: {:?}", result);
        assert_eq!(result, 26386);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 75592527415659);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 693942);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 0);
    }

}
