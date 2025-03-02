use crate::{
    lexer::{Lexer, TokenType},
    syntax_tree::{
        Arguments, Assignment, BinaryOperation, BinaryOperator, Call, Expression, Name, Number,
        Program, Statement, Value,
    },
};

pub struct Parser<'source> {
    lexer: Lexer<'source>,
}

impl<'source> Parser<'source> {
    pub const fn new(source: &'source str) -> Self {
        Self {
            lexer: Lexer::new(source),
        }
    }

    fn parse_arguments(&mut self) -> Arguments {
        let mut contents = Vec::new();

        contents.push(self.parse_expression());

        while self.lexer.next_if(TokenType::Comma).is_some() {
            contents.push(self.parse_expression())
        }

        Arguments { contents }
    }

    fn parse_name(&mut self) -> Option<Name> {
        self.lexer
            .next_if(TokenType::Name)
            .map(ToString::to_string)
            .map(|contents| Name { contents })
    }

    fn parse_number(&mut self) -> Option<Number> {
        self.lexer
            .next_if(TokenType::Number)
            .map(ToString::to_string)
            .map(|contents| Number { contents })
    }

    fn parse_call(&mut self) -> Call {
        let function = self.parse_expression();

        self.lexer
            .next_if(TokenType::LeftParenthesis)
            .expect("`(` should be the next token");

        let arguments = self.parse_arguments();

        self.lexer
            .next_if(TokenType::RightParenthesis)
            .expect("`)` should be the next token");

        Call {
            function,
            arguments,
        }
    }

    fn parse_value(&mut self) -> Value {
        if self.lexer.next_if(TokenType::LeftParenthesis).is_some() {
            let expression = self.parse_expression();

            self.lexer.next_if(TokenType::RightParenthesis).unwrap();

            Value::Parenthesized(expression)
        } else if let Some(name) = self.parse_name() {
            Value::Name(name)
        } else if let Some(number) = self.parse_number() {
            Value::Number(number)
        } else {
            panic!("value expected")
        }
    }

    fn parse_sub_expression(&mut self, binding: u8) -> Expression {
        let mut lhs = Expression::Value(self.parse_value().into());

        while let Ok(operator) = BinaryOperator::try_from(self.lexer.peek(0).token_type) {
            let (lhs_binding, rhs_binding) = operator.binding();

            if lhs_binding < binding {
                break;
            }

            self.lexer.next();

            let rhs = self.parse_sub_expression(rhs_binding);

            lhs = Expression::BinaryOperation(BinaryOperation { lhs, rhs, operator }.into())
        }

        lhs
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_sub_expression(0)
    }

    fn parse_assignment(&mut self) -> Assignment {
        let name = self.parse_name().unwrap();

        self.lexer
            .next_if(TokenType::Equal)
            .expect("= should be the next token");

        let expression = self.parse_expression();

        Assignment { name, expression }
    }

    fn parse_statement(&mut self) -> Statement {
        if self.lexer.peek(0).token_type == TokenType::Name
            && self.lexer.peek(1).token_type == TokenType::Equal
        {
            Statement::Assignment(self.parse_assignment())
        } else {
            Statement::Call(self.parse_call())
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.lexer.next_if(TokenType::EndOfFile).is_none() {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }
}
