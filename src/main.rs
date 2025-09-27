use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

/// Token enum represents different types of tokens.
#[derive(Debug, Clone)]
enum Token {
    Print,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Let,
    Assign, // = operator
    Identifier(String),
    Number(i32),
    String(String),
    // Comparison operators
    Equals,      // ==
    NotEquals,   // !=
    LessThan,    // <
    GreaterThan, // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
    // Control flow
    If,
    Else,
    // Block delimiters
    LeftBrace,  // {
    RightBrace, // }
}

/// Value enum represents a number or variable.
#[derive(Debug, Clone)]
enum Value {
    Number(i32),
    Variable(String),
}

/// Condition enum for comparisons.
#[derive(Debug, Clone)]
enum Condition {
    Equals(Value, Value),
    NotEquals(Value, Value),
    LessThan(Value, Value),
    GreaterThan(Value, Value),
    LessThanOrEqual(Value, Value),
    GreaterThanOrEqual(Value, Value),
}

/// ASTNode represents parsed expressions.
#[derive(Debug, Clone)]
enum ASTNode {
    Print(String),
    Let(String, i32),
    Add(Value, Value),
    Subtract(Value, Value),
    Multiply(Value, Value),
    Divide(Value, Value),
    Modulus(Value, Value),
    If(Condition, Vec<ASTNode>, Option<Vec<ASTNode>>), // condition, then_block, else_block
}

/// Lexer: Converts source code into tokens.
fn lexer(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '"' => {
                chars.next(); // consume opening quote
                let mut string_content = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next(); // consume closing quote
                        break;
                    }
                    string_content.push(chars.next().unwrap());
                }
                tokens.push(Token::String(string_content));
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next(); // consume second =
                    tokens.push(Token::Equals);
                } else {
                    tokens.push(Token::Assign);
                }
            }
            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next(); // consume =
                    tokens.push(Token::LessThanOrEqual);
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next(); // consume =
                    tokens.push(Token::GreaterThanOrEqual);
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next(); // consume =
                    tokens.push(Token::NotEquals);
                } else {
                    eprintln!("Unexpected character: !");
                }
            }
            '{' => {
                chars.next();
                tokens.push(Token::LeftBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RightBrace);
            }
            _ => {
                let mut word = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_whitespace() || ch == '"' || ch == '{' || ch == '}' || ch == '=' || ch == '<' || ch == '>' || ch == '!' {
                        break;
                    }
                    word.push(chars.next().unwrap());
                }

                if !word.is_empty() {
                    match word.as_str() {
                        "print" => tokens.push(Token::Print),
                        "add" => tokens.push(Token::Add),
                        "subtract" => tokens.push(Token::Subtract),
                        "multiply" => tokens.push(Token::Multiply),
                        "divide" => tokens.push(Token::Divide),
                        "mod" => tokens.push(Token::Modulus),
                        "let" => tokens.push(Token::Let),
                        "if" => tokens.push(Token::If),
                        "else" => tokens.push(Token::Else),
                        _ => {
                            if let Ok(num) = word.parse::<i32>() {
                                tokens.push(Token::Number(num));
                            } else if word.chars().all(|c| c.is_alphanumeric() || c == '_') {
                                tokens.push(Token::Identifier(word));
                            } else {
                                eprintln!("Unknown token: {}", word);
                            }
                        }
                    }
                }
            }
        }
    }
    tokens
}

/// Helper function to parse a value (number or identifier)
fn parse_value(token: Token) -> Option<Value> {
    match token {
        Token::Number(n) => Some(Value::Number(n)),
        Token::Identifier(name) => Some(Value::Variable(name)),
        _ => None,
    }
}

/// Helper function to parse a condition
fn parse_condition(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Condition, String> {
    let lhs_token = iter.next().ok_or("Expected left operand in condition")?;
    let lhs = parse_value(lhs_token).ok_or("Invalid left operand in condition")?;

    let op_token = iter.next().ok_or("Expected comparison operator")?;

    let rhs_token = iter.next().ok_or("Expected right operand in condition")?;
    let rhs = parse_value(rhs_token).ok_or("Invalid right operand in condition")?;

    match op_token {
        Token::Equals => Ok(Condition::Equals(lhs, rhs)),
        Token::NotEquals => Ok(Condition::NotEquals(lhs, rhs)),
        Token::LessThan => Ok(Condition::LessThan(lhs, rhs)),
        Token::GreaterThan => Ok(Condition::GreaterThan(lhs, rhs)),
        Token::LessThanOrEqual => Ok(Condition::LessThanOrEqual(lhs, rhs)),
        Token::GreaterThanOrEqual => Ok(Condition::GreaterThanOrEqual(lhs, rhs)),
        _ => Err(format!("Invalid comparison operator: {:?}", op_token)),
    }
}

/// Helper function to parse a block of statements
fn parse_block(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Vec<ASTNode>, String> {
    // Expect opening brace
    if !matches!(iter.next(), Some(Token::LeftBrace)) {
        return Err("Expected '{' to start block".to_string());
    }

    let mut block = Vec::new();

    while let Some(token) = iter.peek() {
        if matches!(token, Token::RightBrace) {
            iter.next(); // consume closing brace
            break;
        }

        // Parse statement in block
        let node = parse_statement(iter)?;
        block.push(node);
    }

    Ok(block)
}

/// Helper function to parse a single statement
fn parse_statement(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<ASTNode, String> {
    let token = iter.next().ok_or("Expected statement")?;

    match token {
        Token::Print => {
            if let Some(Token::String(text)) = iter.next() {
                Ok(ASTNode::Print(text))
            } else {
                Err("Expected string after 'print'".to_string())
            }
        }
        Token::Let => {
            if let Some(Token::Identifier(var)) = iter.next() {
                if let Some(Token::Assign) = iter.next() {
                    if let Some(Token::Number(value)) = iter.next() {
                        Ok(ASTNode::Let(var, value))
                    } else {
                        Err("Expected number after '=' in let statement".to_string())
                    }
                } else {
                    Err("Expected '=' after variable name in let statement".to_string())
                }
            } else {
                Err("Expected variable name after 'let'".to_string())
            }
        }
        Token::If => {
            let condition = parse_condition(iter)?;
            let then_block = parse_block(iter)?;

            // Check for optional else block
            let else_block = if matches!(iter.peek(), Some(Token::Else)) {
                iter.next(); // consume 'else'
                Some(parse_block(iter)?)
            } else {
                None
            };

            Ok(ASTNode::If(condition, then_block, else_block))
        }
        Token::Add | Token::Subtract | Token::Multiply | Token::Divide | Token::Modulus => {
            let lhs_token = iter.next().ok_or("Expected first operand")?;
            let rhs_token = iter.next().ok_or("Expected second operand")?;

            let lhs = parse_value(lhs_token).ok_or("Invalid first operand")?;
            let rhs = parse_value(rhs_token).ok_or("Invalid second operand")?;

            match token {
                Token::Add => Ok(ASTNode::Add(lhs, rhs)),
                Token::Subtract => Ok(ASTNode::Subtract(lhs, rhs)),
                Token::Multiply => Ok(ASTNode::Multiply(lhs, rhs)),
                Token::Divide => Ok(ASTNode::Divide(lhs, rhs)),
                Token::Modulus => Ok(ASTNode::Modulus(lhs, rhs)),
                _ => unreachable!(),
            }
        }
        _ => Err(format!("Unexpected token: {:?}", token)),
    }
}

/// Parser: Converts tokens into an AST.
fn parser(tokens: Vec<Token>) -> Result<Vec<ASTNode>, String> {
    let mut ast = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while iter.peek().is_some() {
        let node = parse_statement(&mut iter)?;
        ast.push(node);
    }

    Ok(ast)
}

/// Helper function to resolve a value
fn resolve_value(value: &Value, variables: &HashMap<String, i32>) -> Result<i32, String> {
    match value {
        Value::Number(n) => Ok(*n),
        Value::Variable(var) => {
            variables.get(var)
                .copied()
                .ok_or_else(|| format!("Undefined variable: {}", var))
        }
    }
}

/// Helper function to evaluate a condition
fn evaluate_condition(condition: &Condition, variables: &HashMap<String, i32>) -> Result<bool, String> {
    match condition {
        Condition::Equals(lhs, rhs) => {
            let lhs_val = resolve_value(lhs, variables)?;
            let rhs_val = resolve_value(rhs, variables)?;
            Ok(lhs_val == rhs_val)
        }
        Condition::NotEquals(lhs, rhs) => {
            let lhs_val = resolve_value(lhs, variables)?;
            let rhs_val = resolve_value(rhs, variables)?;
            Ok(lhs_val != rhs_val)
        }
        Condition::LessThan(lhs, rhs) => {
            let lhs_val = resolve_value(lhs, variables)?;
            let rhs_val = resolve_value(rhs, variables)?;
            Ok(lhs_val < rhs_val)
        }
        Condition::GreaterThan(lhs, rhs) => {
            let lhs_val = resolve_value(lhs, variables)?;
            let rhs_val = resolve_value(rhs, variables)?;
            Ok(lhs_val > rhs_val)
        }
        Condition::LessThanOrEqual(lhs, rhs) => {
            let lhs_val = resolve_value(lhs, variables)?;
            let rhs_val = resolve_value(rhs, variables)?;
            Ok(lhs_val <= rhs_val)
        }
        Condition::GreaterThanOrEqual(lhs, rhs) => {
            let lhs_val = resolve_value(lhs, variables)?;
            let rhs_val = resolve_value(rhs, variables)?;
            Ok(lhs_val >= rhs_val)
        }
    }
}

/// Executor: Processes AST nodes.
fn execute(ast: &[ASTNode]) -> Result<(), String> {
    let mut variables: HashMap<String, i32> = HashMap::new();

    for node in ast {
        match node {
            ASTNode::Print(text) => println!("{}", text),
            ASTNode::Let(var, value) => {
                variables.insert(var.clone(), *value);
            }
            ASTNode::Add(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, &variables)?;
                let rhs_val = resolve_value(rhs, &variables)?;
                let result = lhs_val + rhs_val;
                println!("{} + {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Subtract(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, &variables)?;
                let rhs_val = resolve_value(rhs, &variables)?;
                let result = lhs_val - rhs_val;
                println!("{} - {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Multiply(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, &variables)?;
                let rhs_val = resolve_value(rhs, &variables)?;
                let result = lhs_val * rhs_val;
                println!("{} * {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Divide(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, &variables)?;
                let rhs_val = resolve_value(rhs, &variables)?;
                if rhs_val == 0 {
                    return Err("Division by zero".to_string());
                }
                let result = lhs_val / rhs_val;
                println!("{} / {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Modulus(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, &variables)?;
                let rhs_val = resolve_value(rhs, &variables)?;
                if rhs_val == 0 {
                    return Err("Modulus by zero".to_string());
                }
                let result = lhs_val % rhs_val;
                println!("{} % {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::If(condition, then_block, else_block) => {
                let condition_result = evaluate_condition(condition, &variables)?;
                if condition_result {
                    execute_block(then_block, &mut variables)?;
                } else if let Some(else_block) = else_block {
                    execute_block(else_block, &mut variables)?;
                }
            }
        }
    }
    Ok(())
}

/// Helper function to execute a block of statements
fn execute_block(block: &[ASTNode], variables: &mut HashMap<String, i32>) -> Result<(), String> {
    for node in block {
        match node {
            ASTNode::Print(text) => println!("{}", text),
            ASTNode::Let(var, value) => {
                variables.insert(var.clone(), *value);
            }
            ASTNode::Add(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, variables)?;
                let rhs_val = resolve_value(rhs, variables)?;
                let result = lhs_val + rhs_val;
                println!("{} + {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Subtract(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, variables)?;
                let rhs_val = resolve_value(rhs, variables)?;
                let result = lhs_val - rhs_val;
                println!("{} - {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Multiply(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, variables)?;
                let rhs_val = resolve_value(rhs, variables)?;
                let result = lhs_val * rhs_val;
                println!("{} * {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Divide(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, variables)?;
                let rhs_val = resolve_value(rhs, variables)?;
                if rhs_val == 0 {
                    return Err("Division by zero".to_string());
                }
                let result = lhs_val / rhs_val;
                println!("{} / {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::Modulus(lhs, rhs) => {
                let lhs_val = resolve_value(lhs, variables)?;
                let rhs_val = resolve_value(rhs, variables)?;
                if rhs_val == 0 {
                    return Err("Modulus by zero".to_string());
                }
                let result = lhs_val % rhs_val;
                println!("{} % {} = {}", lhs_val, rhs_val, result);
            }
            ASTNode::If(condition, then_block, else_block) => {
                let condition_result = evaluate_condition(condition, variables)?;
                if condition_result {
                    execute_block(then_block, variables)?;
                } else if let Some(else_block) = else_block {
                    execute_block(else_block, variables)?;
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let tokens = lexer(&code);
    let ast = match parser(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = execute(&ast) {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }
}
