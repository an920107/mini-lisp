pub mod ast;
pub mod error;

use std::vec::IntoIter;

use itertools::{Itertools, MultiPeek};

use crate::{error::Error, parser::error::ParsingError, scanner::token::Token};

pub struct Parser {
    tokens: MultiPeek<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().multipeek(),
        }
    }

    pub fn parse(&mut self) -> Result<ast::Program, Error> {
        let mut statements: Vec<ast::Statement> = vec![];

        while self.tokens.peek().is_some() {
            statements.push(
                self.parse_statement()
                    .map_err(|e| Error::SyntaxError(e.to_string()))?,
            );
        }

        Ok(ast::Program(statements))
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, ParsingError> {
        self.tokens.reset_peek();

        match self.tokens.peek() {
            Some(Token::LeftBracket) => match self.tokens.peek() {
                Some(Token::Define) => Ok(ast::Statement::Define(self.parse_define_statement()?)),
                Some(Token::PrintBoolean | Token::PrintInteger) => {
                    Ok(ast::Statement::Print(self.parse_print_statement()?))
                }
                Some(_) => Ok(ast::Statement::Expression(self.parse_expression()?)),
                None => Err(ParsingError::UnexpectedEnd),
            },
            Some(_) => Ok(ast::Statement::Expression(self.parse_expression()?)),
            None => Err(ParsingError::UnexpectedEnd),
        }
    }

    fn parse_define_statement(&mut self) -> Result<ast::DefineStatement, ParsingError> {
        self.tokens.reset_peek();

        self.parse_wrapped_with_brackets(|parser| match parser.tokens.next() {
            Some(Token::Define) => Ok(ast::DefineStatement {
                variable: parser.parse_variable()?,
                body: parser.parse_expression()?,
            }),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_print_statement(&mut self) -> Result<ast::PrintStatement, ParsingError> {
        self.tokens.reset_peek();

        self.parse_wrapped_with_brackets(|parser| match parser.tokens.next() {
            Some(Token::PrintInteger) => {
                Ok(ast::PrintStatement::Integer(parser.parse_expression()?))
            }
            Some(Token::PrintBoolean) => {
                Ok(ast::PrintStatement::Boolean(parser.parse_expression()?))
            }
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, ParsingError> {
        self.tokens.reset_peek();

        match self.tokens.peek() {
            Some(Token::Boolean(_)) | Some(Token::Integer(_)) => {
                Ok(ast::Expression::Literal(self.parse_literal()?))
            }
            Some(Token::Symbol(_)) => Ok(ast::Expression::Variable(self.parse_variable()?)),
            Some(Token::LeftBracket) => match self.tokens.peek() {
                Some(Token::Not) => Ok(ast::Expression::UnaryOperation(
                    self.parse_unary_operation()?,
                )),
                Some(
                    Token::Minus | Token::Divide | Token::Modulo | Token::Greater | Token::Less,
                ) => Ok(ast::Expression::BinaryOperation(
                    self.parse_binary_operation()?,
                )),
                Some(Token::Plus | Token::Multiply | Token::Equal | Token::And | Token::Or) => Ok(
                    ast::Expression::NAryOperation(self.parse_n_ary_operation()?),
                ),
                Some(Token::If) => Ok(ast::Expression::IfExpression(self.parse_if_expression()?)),
                Some(Token::Function) => Ok(ast::Expression::FunctionExpression(
                    self.parse_function_expression()?,
                )),
                Some(Token::LeftBracket | Token::Symbol(_)) => {
                    Ok(ast::Expression::FunctionCall(self.parse_function_call()?))
                }
                Some(token) => Err(ParsingError::UnexpectedToken(token.clone())),
                None => Err(ParsingError::UnexpectedEnd),
            },
            Some(token) => Err(ParsingError::UnexpectedToken(token.clone())),
            None => Err(ParsingError::UnexpectedEnd),
        }
    }

    fn parse_unary_operation(&mut self) -> Result<ast::UnaryOperation, ParsingError> {
        self.tokens.reset_peek();

        self.parse_wrapped_with_brackets(|parser| match parser.tokens.next() {
            Some(Token::Not) => Ok(ast::UnaryOperation {
                operator: ast::UnaryOperator::Not,
                operand: Box::new(parser.parse_expression()?),
            }),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_binary_operation(&mut self) -> Result<ast::BinaryOperation, ParsingError> {
        self.tokens.reset_peek();

        fn parse_operands(
            parser: &mut Parser,
            operator: ast::BinaryOperator,
        ) -> Result<ast::BinaryOperation, ParsingError> {
            Ok(ast::BinaryOperation {
                operator,
                left: Box::new(parser.parse_expression()?),
                right: Box::new(parser.parse_expression()?),
            })
        }

        self.parse_wrapped_with_brackets(|parser| match parser.tokens.next() {
            Some(Token::Minus) => Ok(parse_operands(parser, ast::BinaryOperator::Subtract)?),
            Some(Token::Divide) => Ok(parse_operands(parser, ast::BinaryOperator::Divide)?),
            Some(Token::Modulo) => Ok(parse_operands(parser, ast::BinaryOperator::Modulo)?),
            Some(Token::Greater) => Ok(parse_operands(parser, ast::BinaryOperator::GreaterThan)?),
            Some(Token::Less) => Ok(parse_operands(parser, ast::BinaryOperator::LessThan)?),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_n_ary_operation(&mut self) -> Result<ast::NAryOperation, ParsingError> {
        self.tokens.reset_peek();

        fn parse_operands(
            parser: &mut Parser,
            operator: ast::NAryOperator,
        ) -> Result<ast::NAryOperation, ParsingError> {
            let mut operands: Vec<ast::Expression> =
                vec![parser.parse_expression()?, parser.parse_expression()?];
            while let Some(token) = parser.tokens.peek()
                && token != &Token::RightBracket
            {
                operands.push(parser.parse_expression()?);
            }
            Ok(ast::NAryOperation { operator, operands })
        }

        self.parse_wrapped_with_brackets(|parser| match parser.tokens.next() {
            Some(Token::Plus) => Ok(parse_operands(parser, ast::NAryOperator::Add)?),
            Some(Token::Multiply) => Ok(parse_operands(parser, ast::NAryOperator::Multiply)?),
            Some(Token::Equal) => Ok(parse_operands(parser, ast::NAryOperator::EqualTo)?),
            Some(Token::And) => Ok(parse_operands(parser, ast::NAryOperator::And)?),
            Some(Token::Or) => Ok(parse_operands(parser, ast::NAryOperator::Or)?),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_function_expression(&mut self) -> Result<ast::FunctionExpression, ParsingError> {
        self.tokens.reset_peek();

        self.parse_wrapped_with_brackets(|parser| match parser.tokens.next() {
            Some(Token::Function) => {
                let parameters = parser.parse_wrapped_with_brackets(|parser| {
                    let mut parameters: Vec<ast::Variable> = vec![];
                    while let Some(Token::Symbol(_)) = parser.tokens.peek() {
                        parameters.push(parser.parse_variable()?);
                    }
                    Ok(parameters)
                })?;
                let mut defines: Vec<ast::DefineStatement> = vec![];
                while let Some(Token::LeftBracket) = parser.tokens.peek()
                    && let Some(Token::Define) = parser.tokens.peek()
                {
                    defines.push(parser.parse_define_statement()?);
                }
                Ok(ast::FunctionExpression {
                    parameters,
                    defines,
                    body: Box::new(parser.parse_expression()?),
                })
            }
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_function_call(&mut self) -> Result<ast::FunctionCall, ParsingError> {
        self.tokens.reset_peek();

        fn parse_argements(parser: &mut Parser) -> Result<Vec<ast::Expression>, ParsingError> {
            let mut arguments: Vec<ast::Expression> = vec![];
            while let Some(token) = parser.tokens.peek()
                && token != &Token::RightBracket
            {
                arguments.push(parser.parse_expression()?);
            }
            Ok(arguments)
        }

        self.parse_wrapped_with_brackets(|parser| match parser.tokens.peek() {
            Some(Token::Symbol(_)) => Ok(ast::FunctionCall::Named(ast::NamedFunctionCall {
                variable: parser.parse_variable()?,
                arguments: parse_argements(parser)?,
            })),
            Some(Token::LeftBracket) => {
                Ok(ast::FunctionCall::Anonymous(ast::AnonymousFunctionCall {
                    function: parser.parse_function_expression()?,
                    arguments: parse_argements(parser)?,
                }))
            }
            Some(token) => Err(ParsingError::UnexpectedToken(token.clone())),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_if_expression(&mut self) -> Result<ast::IfExpression, ParsingError> {
        self.tokens.reset_peek();

        match self.tokens.next() {
            Some(Token::LeftBracket) => match self.tokens.next() {
                Some(Token::If) => Ok(ast::IfExpression {
                    condition: Box::new(self.parse_expression()?),
                    then_branch: Box::new(self.parse_expression()?),
                    else_branch: Box::new(self.parse_expression()?),
                }),
                Some(token) => Err(ParsingError::UnexpectedToken(token)),
                None => Err(ParsingError::UnexpectedEnd),
            },
            Some(token) => return Err(ParsingError::UnexpectedToken(token)),
            None => return Err(ParsingError::UnexpectedEnd),
        }
        .and_then(|expr| match self.tokens.next() {
            Some(Token::RightBracket) => Ok(expr),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }

    fn parse_literal(&mut self) -> Result<ast::Literal, ParsingError> {
        self.tokens.reset_peek();

        match self.tokens.next() {
            Some(Token::Boolean(bit)) => Ok(ast::Literal::Boolean(bit)),
            Some(Token::Integer(num)) => Ok(ast::Literal::Integer(num)),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        }
    }

    fn parse_variable(&mut self) -> Result<ast::Variable, ParsingError> {
        self.tokens.reset_peek();

        match self.tokens.next() {
            Some(Token::Symbol(symbol)) => Ok(ast::Variable(symbol)),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        }
    }

    fn parse_wrapped_with_brackets<T>(
        &mut self,
        parsing_fn: impl Fn(&mut Self) -> Result<T, ParsingError>,
    ) -> Result<T, ParsingError> {
        self.tokens.reset_peek();

        match self.tokens.next() {
            Some(Token::LeftBracket) => Ok(()),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        }
        .and_then(|_| parsing_fn(self))
        .and_then(|result| match self.tokens.next() {
            Some(Token::RightBracket) => Ok(result),
            Some(token) => Err(ParsingError::UnexpectedToken(token)),
            None => Err(ParsingError::UnexpectedEnd),
        })
    }
}
