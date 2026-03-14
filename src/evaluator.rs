mod error;
mod symbol_table;
mod value;

use std::io;

use crate::{
    error::Error,
    evaluator::{
        error::EvaluatingError,
        symbol_table::SymbolTable,
        value::{Closure, Value, ValueType},
    },
    parser::ast::{
        AnonymousFunctionCall, BinaryOperation, BinaryOperator, DefineStatement, Expression,
        FunctionCall, IfExpression, Literal, NAryOperation, NAryOperator, NamedFunctionCall,
        PrintStatement, Program, Statement, UnaryOperation, UnaryOperator, Variable,
    },
    scanner::token::Token,
};

pub struct Evaluator {
    program: Program,
    symbol_table: SymbolTable,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn evaluate<W: io::Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        for statement in &self.program.0 {
            let result = match statement {
                Statement::Define(DefineStatement {
                    variable: Variable(name),
                    body,
                }) => {
                    let value = body.evaluate(&self.symbol_table);
                    match value {
                        Ok(value) => {
                            self.symbol_table.insert(name.clone(), value);
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                }
                Statement::Print(print_statement) => {
                    let value = match print_statement {
                        PrintStatement::Integer(expr) => expr.evaluate(&self.symbol_table),
                        PrintStatement::Boolean(expr) => expr.evaluate(&self.symbol_table),
                    };
                    match (print_statement, value) {
                        (PrintStatement::Boolean(_), Ok(Value::Boolean(b))) => {
                            writeln!(writer, "{}", if b { "#t" } else { "#f" })
                                .map_err(|e| Error::IOError(e))?;
                            Ok(())
                        }
                        (PrintStatement::Integer(_), Ok(Value::Integer(n))) => {
                            writeln!(writer, "{}", n).map_err(|e| Error::IOError(e))?;
                            Ok(())
                        }
                        (PrintStatement::Boolean(_), Ok(v)) => Err(EvaluatingError::TypeError(
                            Token::PrintBoolean,
                            ValueType::Boolean,
                            v,
                        )),
                        (PrintStatement::Integer(_), Ok(v)) => Err(EvaluatingError::TypeError(
                            Token::PrintInteger,
                            ValueType::Integer,
                            v,
                        )),
                        (_, Err(e)) => Err(e),
                    }
                }
                Statement::Expression(expression) => {
                    expression.evaluate(&self.symbol_table).map(|_| ())
                }
            };

            result.map_err(|e| Error::SemanticError(e.to_string()))?;
        }

        Ok(())
    }
}

impl Expression {
    fn evaluate(&self, symbol_table: &SymbolTable) -> Result<Value, EvaluatingError> {
        match self {
            Expression::Literal(Literal::Boolean(b)) => Ok(Value::Boolean(*b)),
            Expression::Literal(Literal::Integer(n)) => Ok(Value::Integer(*n)),
            Expression::FunctionExpression(func) => Ok(Value::Function(Closure {
                function: func.clone(),
                env: symbol_table.clone(),
            })),

            Expression::Variable(Variable(name)) => match symbol_table.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(EvaluatingError::UndefinedVariable(name.clone())),
            },
            Expression::UnaryOperation(UnaryOperation { operator, operand }) => {
                let value = operand.evaluate(symbol_table)?;
                match operator {
                    UnaryOperator::Not => match value {
                        Value::Boolean(b) => Ok(Value::Boolean(!b)),
                        _ => Err(EvaluatingError::TypeError(
                            Token::Not,
                            ValueType::Boolean,
                            value,
                        )),
                    },
                }
            }
            Expression::BinaryOperation(BinaryOperation {
                operator,
                left,
                right,
            }) => {
                let value_left = left.evaluate(symbol_table)?;
                let value_right = right.evaluate(symbol_table)?;
                match (value_left, value_right) {
                    (Value::Integer(n1), Value::Integer(n2)) => match operator {
                        BinaryOperator::Subtract => Ok(Value::Integer(n1 - n2)),
                        BinaryOperator::Divide => Ok(Value::Integer(n1 / n2)),
                        BinaryOperator::Modulo => Ok(Value::Integer(n1 % n2)),
                        BinaryOperator::GreaterThan => Ok(Value::Boolean(n1 > n2)),
                        BinaryOperator::LessThan => Ok(Value::Boolean(n1 < n2)),
                    },
                    (v1, v2) => Err(EvaluatingError::TypeError(
                        match operator {
                            BinaryOperator::Subtract => Token::Minus,
                            BinaryOperator::Divide => Token::Divide,
                            BinaryOperator::Modulo => Token::Modulo,
                            BinaryOperator::GreaterThan => Token::Greater,
                            BinaryOperator::LessThan => Token::Less,
                        },
                        ValueType::Integer,
                        if let Value::Boolean(_) = v1 { v1 } else { v2 },
                    )),
                }
            }
            Expression::NAryOperation(NAryOperation { operator, operands }) => {
                let values: Vec<Value> = operands
                    .iter()
                    .map(|operand| operand.evaluate(symbol_table))
                    .collect::<Result<Vec<Value>, EvaluatingError>>()?;

                match operator {
                    NAryOperator::Add | NAryOperator::Multiply => {
                        let mut result = match operator {
                            NAryOperator::Add => 0,
                            NAryOperator::Multiply => 1,
                            _ => unreachable!(),
                        };
                        for value in values {
                            let value = match value {
                                Value::Integer(n) => Ok(n),
                                v => Err(EvaluatingError::TypeError(
                                    match operator {
                                        NAryOperator::Add => Token::Plus,
                                        NAryOperator::Multiply => Token::Multiply,
                                        _ => unreachable!(),
                                    },
                                    ValueType::Integer,
                                    v,
                                )),
                            }?;
                            result = match operator {
                                NAryOperator::Add => result + value,
                                NAryOperator::Multiply => result * value,
                                _ => unreachable!(),
                            };
                        }
                        Ok(Value::Integer(result))
                    }
                    NAryOperator::And | NAryOperator::Or => {
                        let mut result = match operator {
                            NAryOperator::And => true,
                            NAryOperator::Or => false,
                            _ => unreachable!(),
                        };
                        for value in values {
                            let value = match value {
                                Value::Boolean(b) => Ok(b),
                                v => Err(EvaluatingError::TypeError(
                                    match operator {
                                        NAryOperator::And => Token::And,
                                        NAryOperator::Or => Token::Or,
                                        _ => unreachable!(),
                                    },
                                    ValueType::Boolean,
                                    v,
                                )),
                            }?;
                            result = match operator {
                                NAryOperator::And => result && value,
                                NAryOperator::Or => result || value,
                                _ => unreachable!(),
                            };
                        }
                        Ok(Value::Boolean(result))
                    }
                    NAryOperator::EqualTo => {
                        let first_value = &values[0];
                        for value in &values[1..] {
                            if value != first_value {
                                return Ok(Value::Boolean(false));
                            }
                        }
                        Ok(Value::Boolean(true))
                    }
                }
            }

            Expression::IfExpression(IfExpression {
                condition,
                then_branch,
                else_branch,
            }) => {
                let condition_value = condition.evaluate(symbol_table)?;
                match condition_value {
                    Value::Boolean(true) => then_branch.evaluate(symbol_table),
                    Value::Boolean(false) => else_branch.evaluate(symbol_table),
                    v => Err(EvaluatingError::TypeError(Token::If, ValueType::Boolean, v)),
                }
            }

            Expression::FunctionCall(function_call) => {
                let (function, argements, function_name) = match function_call {
                    FunctionCall::Named(NamedFunctionCall {
                        variable: Variable(name),
                        arguments,
                    }) => {
                        let function = match symbol_table.get(&name) {
                            Some(value) => value,
                            None => return Err(EvaluatingError::UndefinedVariable(name.clone())),
                        };
                        (function.clone(), arguments, Some(name.clone()))
                    }
                    FunctionCall::Anonymous(AnonymousFunctionCall {
                        function,
                        arguments,
                    }) => (
                        Value::Function(Closure {
                            function: function.clone(),
                            env: symbol_table.clone(),
                        }),
                        arguments,
                        None,
                    ),
                };

                match function {
                    Value::Function(closure) => {
                        let mut function_symbol_table =
                            SymbolTable::with_parent(closure.env.clone());
                        if let Some(name) = function_name {
                            function_symbol_table.insert(name, Value::Function(closure.clone()));
                        }
                        for (Variable(parameter), argument) in
                            closure.function.parameters.iter().zip(argements.iter())
                        {
                            let argument_value = argument.evaluate(symbol_table)?;
                            function_symbol_table.insert(parameter.clone(), argument_value);
                        }
                        for define in &closure.function.defines {
                            let value = define.body.evaluate(&function_symbol_table)?;
                            function_symbol_table.insert(define.variable.0.clone(), value);
                        }
                        closure.function.body.evaluate(&function_symbol_table)
                    }
                    v => Err(EvaluatingError::TypeError(
                        Token::Function,
                        ValueType::Function,
                        v,
                    )),
                }
            }
        }
    }
}
