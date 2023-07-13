struct Node 
{
    value: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn is_operator(c: char) -> bool 
{
    c == '+' || c == '-' || c == '*' || c == '/'
}

fn get_precedence(op: char) -> i32 
{
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn evaluate_expression_tree(root: &Node) -> f32 
{
    match root.value {
        '+' => evaluate_expression_tree(root.left.as_ref().unwrap()) + evaluate_expression_tree(root.right.as_ref().unwrap()),
        '-' => evaluate_expression_tree(root.left.as_ref().unwrap()) - evaluate_expression_tree(root.right.as_ref().unwrap()),
        '*' => evaluate_expression_tree(root.left.as_ref().unwrap()) * evaluate_expression_tree(root.right.as_ref().unwrap()),
        '/' => evaluate_expression_tree(root.left.as_ref().unwrap()) / evaluate_expression_tree(root.right.as_ref().unwrap()),
        _ => root.value.to_digit(10).unwrap() as f32,
    }
}


fn main() 
{
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    let mut operator_stack: Vec<char> = Vec::new();
    let mut operands_stack: Vec<Box<Node>> = Vec::new();

    for c in expression.chars() 
    {
        if c.is_digit(10) 
        {
            let operand = Box::new(Node {
                value: c,
                left: None,
                right: None,
            });
            operands_stack.push(operand);
        } 
        else if is_operator(c) 
        {
            while !operator_stack.is_empty()&& is_operator(*operator_stack.last().unwrap())&& get_precedence(c) <= get_precedence(*operator_stack.last().unwrap())
            {
                let top_operator = operator_stack.pop().unwrap();
                let right_child = operands_stack.pop().unwrap();
                let left_child = operands_stack.pop().unwrap();
                let top_operator_node = Box::new(Node {
                    value: top_operator,
                    left: Some(left_child),
                    right: Some(right_child),
                });
                operands_stack.push(top_operator_node);
            }
            operator_stack.push(c);
        } 
        else if c == '(' 
        {
            operator_stack.push(c);
        } 
        else if c == ')' 
        {
            while !operator_stack.is_empty() && *operator_stack.last().unwrap() != '(' 
            {
                let top_operator = operator_stack.pop().unwrap();
                let right_child = operands_stack.pop().unwrap();
                let left_child = operands_stack.pop().unwrap();
                let top_operator_node = Box::new(Node {
                    value: top_operator,
                    left: Some(left_child),
                    right: Some(right_child),
                });
                operands_stack.push(top_operator_node);
            }
            operator_stack.pop(); 
        }
    }

    while let Some(top_operator) = operator_stack.pop() 
    {
        let right_child = operands_stack.pop().unwrap();
        let left_child = operands_stack.pop().unwrap();
        let top_operator_node = Box::new(Node {
            value: top_operator,
            left: Some(left_child),
            right: Some(right_child),
        });
        operands_stack.push(top_operator_node);
    }

    let root = operands_stack.pop().unwrap();
    
    let result = evaluate_expression_tree(&root);
    println!("Result: {}", result);
}
