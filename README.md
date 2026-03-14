# mini-lisp

[繁體中文 / Traditional Chinese](README.zh-TW.md)

A small Lisp interpreter written in Rust.

It implements a compact Lisp-like language with:

- integers and booleans
- arithmetic and logical operations
- `if` expressions
- variable definition (`define`)
- anonymous and named functions
- closures and lexical scoping
- recursion

The project includes a scanner, parser (AST), and evaluator, with integration tests that cover core language behavior.

## Background

During a chat with a friend, I suddenly realized that Rust's `match` style is a great fit for expressing grammar rules.
That reminded me of my compiler course final project. Back then, I had already used tools like lex and yacc, but because the implementation was in C, handling grammar and pointers was still inconvenient.
So I spent a bit of time rewriting the project in Rust.

## Requirements

- Rust toolchain (Cargo)

## Quick Start

### Build

```bash
cargo build
```

### Run

The interpreter reads source code from **stdin** until EOF:

```bash
echo "(print-num (+ 1 2 3))" | cargo run --release
```

Or with a multi-line program:

```bash
cat <<'EOF' | cargo run --release
(define fact
  (fun (n)
    (if (< n 3)
        n
        (* n (fact (- n 1))))))
(print-num (fact 10))
EOF
```

Expected output:

```text
6
```

### Run Tests

```bash
cargo test
```

## Language Guide

### Literals

- Integer: `0`, `42`, `-123`
- Boolean: `#t`, `#f`

### Program Form

A program is a sequence of statements/expressions:

```lisp
(define x 10)
(print-num x)
(print-bool (< x 20))
```

## Grammar (EBNF)

```ebnf
program        = { statement } ;

statement      = define_stmt
               | print_stmt
               | expression ;

define_stmt    = "(" "define" symbol expression ")" ;

print_stmt     = "(" "print-num" expression ")"
               | "(" "print-bool" expression ")" ;

expression     = literal
               | symbol
               | unary_expr
               | binary_expr
               | nary_expr
               | if_expr
               | function_expr
               | function_call ;

unary_expr     = "(" "not" expression ")" ;

binary_expr    = "(" ( "-" | "/" | "mod" | ">" | "<" ) expression expression ")" ;

nary_expr      = "(" ( "+" | "*" | "=" | "and" | "or" ) expression expression { expression } ")" ;

if_expr        = "(" "if" expression expression expression ")" ;

function_expr  = "(" "fun" "(" { symbol } ")" { define_stmt } expression ")" ;

function_call  = "(" symbol { expression } ")"
               | "(" function_expr { expression } ")" ;

literal        = integer | boolean ;
integer        = [ "+" | "-" ] digit { digit } ;
boolean        = "#t" | "#f" ;
symbol         = lowercase_letter { lowercase_letter | "-" } ;
```

Notes:

- `+`, `*`, `=`, `and`, `or` require at least 2 operands.
- `-`, `/`, `mod`, `>`, `<` require exactly 2 operands.
- Function parameters are positional.

### Printing

- `(print-num <expr>)` → prints integer result
- `(print-bool <expr>)` → prints `#t` or `#f`

### Variable Definition

```lisp
(define x 1)
(define y (+ x 2))
(print-num y)
```

### Arithmetic

- N-ary: `+`, `*`
- Binary: `-`, `/`, `mod`, `>`, `<`
- Equality (n-ary): `=`

Examples:

```lisp
(print-num (+ 1 2 3 4))
(print-num (* 2 3 4))
(print-num (- 10 3))
(print-num (/ 8 3))
(print-num (mod 10 4))
(print-bool (= 5 5 5))
(print-bool (> 3 2))
```

### Logic

- N-ary: `and`, `or`
- Unary: `not`

```lisp
(print-bool (and #t #t #f))
(print-bool (or #f #f #t))
(print-bool (not #f))
```

### Conditionals

```lisp
(print-num (if (< 1 2) 100 200))
```

### Functions

#### Anonymous function call

```lisp
(print-num ((fun (x) (+ x 1)) 3))
```

#### Named function

```lisp
(define add (fun (a b) (+ a b)))
(print-num (add 4 5))
```

#### Nested function and lexical scope

```lisp
(define dist-square
  (fun (x y)
    (define square (fun (n) (* n n)))
    (+ (square x) (square y))))

(print-num (dist-square 3 4))
```

#### First-class functions / closure

```lisp
(define add-x
  (fun (x)
    (fun (y) (+ x y))))

(define add10 (add-x 10))
(print-num (add10 1))
```

#### Recursion

```lisp
(define fact
  (fun (n)
    (if (< n 3)
        n
        (* n (fact (- n 1))))))

(print-num (fact 10))
```

## Tokens and Identifiers

- Parentheses: `(`, `)`
- Symbols (identifiers): lowercase letters and `-`, starting with a letter
  - valid: `foo`, `bar-z`, `x`
  - invalid: `Foo`, `x1`, `_tmp`

## Error Reporting

The interpreter reports three categories of errors:

- **Lexical error**: invalid character/token during scanning
- **Syntax error**: malformed expression during parsing
- **Semantic error**: runtime issues (e.g., undefined variable, type mismatch)

Examples:

- using boolean where integer is required
- referencing undefined variable

## Development Notes

- In debug builds (`cargo run`), internal token and AST dumps are printed (for easier debugging).
- In release builds (`cargo run --release`), only program output is printed.

## Project Layout

```text
src/
  scanner.rs            # input scanning pipeline
  scanner/token.rs      # token definitions (logos lexer)
  parser.rs             # recursive-descent parser
  parser/ast.rs         # AST definitions
  evaluator.rs          # evaluator/interpreter
  evaluator/value.rs    # runtime value and closure types
  evaluator/symbol_table.rs
  error.rs              # top-level error types
  main.rs               # CLI entrypoint (stdin -> scanner -> parser -> evaluator)

tests/
  ...                   # syntax, ops, if, define, function, recursion, typing tests
```

## License

See [LICENSE](LICENSE).
