#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Define(DefineStatement),
    Print(PrintStatement),
}

#[derive(Debug)]
pub enum PrintStatement {
    Integer(Expression),
    Boolean(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(Variable),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    NAryOperation(NAryOperation),
    FunctionExpression(FunctionExpression),
    FunctionCall(FunctionCall),
    IfExpression(IfExpression),
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Integer(i128),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Subtract,
    Divide,
    Modulo,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, Copy)]
pub enum NAryOperator {
    Add,
    Multiply,
    EqualTo,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum FunctionCall {
    Named(NamedFunctionCall),
    Anonymous(AnonymousFunctionCall),
}

#[derive(Debug, Clone)]
pub struct DefineStatement {
    pub variable: Variable,
    pub body: Expression,
}

#[derive(Debug, Clone)]
pub struct Variable(pub String);

#[derive(Debug, Clone)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct BinaryOperation {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct NAryOperation {
    pub operator: NAryOperator,
    pub operands: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct FunctionExpression {
    pub parameters: Vec<Variable>,
    pub defines: Vec<DefineStatement>,
    pub body: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct NamedFunctionCall {
    pub variable: Variable,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct AnonymousFunctionCall {
    pub function: FunctionExpression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub then_branch: Box<Expression>,
    pub else_branch: Box<Expression>,
}
