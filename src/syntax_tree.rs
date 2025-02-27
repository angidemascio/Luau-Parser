use crate::lexer::TokenType;

pub struct Name {
    pub contents: String,
}

pub struct Number {
    pub contents: String,
}

pub enum Value {
    Parenthesized(Expression),
    Name(Name),
    Number(Number),
}

#[derive(Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
}

impl BinaryOperator {
    pub const fn binding(self) -> (u8, u8) {
        match self {
            Self::Plus | Self::Minus => (1, 2),
        }
    }
}

impl TryFrom<TokenType> for BinaryOperator {
    type Error = ();

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        let result = match value {
            TokenType::Plus => Self::Plus,
            TokenType::Minus => Self::Minus,

            _ => return Err(()),
        };

        Ok(result)
    }
}

pub struct BinaryOperation {
    pub lhs: Expression,
    pub rhs: Expression,
    pub operator: BinaryOperator,
}

pub enum Expression {
    BinaryOperation(Box<BinaryOperation>),
    Value(Box<Value>),
}

pub struct Assignment {
    pub name: Name,
    pub expression: Expression,
}

pub struct Arguments {
    pub contents: Vec<Expression>,
}

pub struct Call {
    pub function: Expression,
    pub arguments: Arguments,
}

pub enum Statement {
    Assignment(Assignment),
    Call(Call),
}

pub struct Program {
    statements: Vec<Statement>,
}
