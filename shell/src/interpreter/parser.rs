#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    EOF,
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Prefix {
        op: Token,
        rhs: Box<Expr>,
    },
    Infix {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' => i += 1,
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            d if d.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let number = input[start..i].parse::<f64>().unwrap();
                tokens.push(Token::Number(number));
            }
            _ => panic!("Unexpected character: {}", chars[i]),
        }
    }

    tokens.push(Token::EOF);
    tokens
}

pub fn precedence(op: &Token) -> u8 {
    match op {
        Token::Plus | Token::Minus => 1,
        Token::Star | Token::Slash => 2,
        _ => 0,
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let tokens = tokenize(input);
        Self { tokens, pos: 0 }
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    pub fn next(&mut self) -> Token {
        let token = self.peek().clone();
        self.pos += 1;
        token
    }

    pub fn parse_expression(&mut self, min_prec: u8) -> Expr {
        let mut lhs = match self.next() {
            Token::Number(n) => Expr::Number(n),
            Token::Minus => {
                let rhs = self.parse_expression(10); // high precedence for unary minus
                Expr::Prefix {
                    op: Token::Minus,
                    rhs: Box::new(rhs),
                }
            }
            Token::LParen => {
                let expr = self.parse_expression(0);
                assert_eq!(self.next(), Token::RParen);
                expr
            }
            t => panic!("Unexpected token: {:?}", t),
        };

        while let Some(op) = match self.peek() {
            t @ Token::Plus | t @ Token::Minus | t @ Token::Star | t @ Token::Slash => Some(t),
            _ => None,
        } {
            let prec = precedence(op);
            if prec < min_prec {
                break;
            }
            let op = self.next();
            let rhs = self.parse_expression(prec + 1); // right associativity
            lhs = Expr::Infix {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        lhs
    }
}

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Prefix { op, rhs } => match op {
            Token::Minus => -eval(rhs),
            _ => panic!("Invalid prefix op"),
        },
        Expr::Infix { lhs, op, rhs } => {
            let l = eval(lhs);
            let r = eval(rhs);
            match op {
                Token::Plus => l + r,
                Token::Minus => l - r,
                Token::Star => l * r,
                Token::Slash => l / r,
                _ => panic!("Invalid infix op"),
            }
        }
    }
}
