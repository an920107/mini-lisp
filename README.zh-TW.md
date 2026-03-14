# mini-lisp

一個使用 Rust 撰寫的小型 Lisp 直譯器。

它實作了一個精簡的 Lisp 風格語言，包含：

- 整數與布林值
- 算術與邏輯運算
- `if` 條件表達式
- 變數定義（`define`）
- 匿名函式與具名函式
- 閉包與詞彙作用域（lexical scoping）
- 遞迴

此專案包含 scanner、parser (AST)、和 evaluator，並提供整合測試以涵蓋核心語言行為。

## 背景

有一次和朋友聊天時，我突然意識到 Rust 的 `match` 寫法非常適合拿來表達語法規則。這讓我想到編譯器課程的期末專案。當時雖然已經使用了 lex 與 yacc 等工具，但因為是用 C 實作，處理語法與指標仍然不太方便。

因此我花了一點時間用 Rust 重新寫了這個專案。

## 需求

- Rust 工具鏈（Cargo）

## 快速開始

### 建置

```bash
cargo build
```

### 執行

此直譯器會從 **stdin** 讀取原始碼，直到 EOF：

```bash
echo "(print-num (+ 1 2 3))" | cargo run --release
```

或使用多行程式：

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

預期輸出：

```text
6
```

### 執行測試

```bash
cargo test
```

## 語言指南

### 常值（Literals）

- 整數：`0`、`42`、`-123`
- 布林：`#t`、`#f`

### 程式形式

一個程式由多個陳述式／表達式依序組成：

```lisp
(define x 10)
(print-num x)
(print-bool (< x 20))
```

## 文法（EBNF）

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

說明：

- `+`、`*`、`=`、`and`、`or` 至少需要 2 個運算元。
- `-`、`/`、`mod`、`>`、`<` 必須剛好 2 個運算元。
- 函式參數採位置對應。

### 輸出

- `(print-num <expr>)` → 輸出整數結果
- `(print-bool <expr>)` → 輸出 `#t` 或 `#f`

### 變數定義

```lisp
(define x 1)
(define y (+ x 2))
(print-num y)
```

### 算術運算

- N 元運算：`+`、`*`
- 二元運算：`-`、`/`、`mod`、`>`、`<`
- 相等比較（N 元）：`=`

範例：

```lisp
(print-num (+ 1 2 3 4))
(print-num (* 2 3 4))
(print-num (- 10 3))
(print-num (/ 8 3))
(print-num (mod 10 4))
(print-bool (= 5 5 5))
(print-bool (> 3 2))
```

### 邏輯運算

- N 元運算：`and`、`or`
- 一元運算：`not`

```lisp
(print-bool (and #t #t #f))
(print-bool (or #f #f #t))
(print-bool (not #f))
```

### 條件式

```lisp
(print-num (if (< 1 2) 100 200))
```

### 函式

#### 匿名函式呼叫

```lisp
(print-num ((fun (x) (+ x 1)) 3))
```

#### 具名函式

```lisp
(define add (fun (a b) (+ a b)))
(print-num (add 4 5))
```

#### 巢狀函式與詞彙作用域

```lisp
(define dist-square
  (fun (x y)
    (define square (fun (n) (* n n)))
    (+ (square x) (square y))))

(print-num (dist-square 3 4))
```

#### 一等函式／閉包

```lisp
(define add-x
  (fun (x)
    (fun (y) (+ x y))))

(define add10 (add-x 10))
(print-num (add10 1))
```

#### 遞迴

```lisp
(define fact
  (fun (n)
    (if (< n 3)
        n
        (* n (fact (- n 1))))))

(print-num (fact 10))
```

## Token 與識別字

- 括號：`(`、`)`
- 符號（識別字）：由小寫英文字母與 `-` 組成，且必須以字母開頭
  - 合法：`foo`、`bar-z`、`x`
  - 不合法：`Foo`、`x1`、`_tmp`

## 錯誤回報

直譯器會回報三種類型的錯誤：

- **詞彙錯誤（Lexical error）**：掃描階段出現非法字元／token
- **語法錯誤（Syntax error）**：剖析階段表達式格式不正確
- **語意錯誤（Semantic error）**：執行階段問題（例如：未定義變數、型別不符）

範例：

- 需要整數的地方卻使用布林值
- 參照未定義變數

## 開發備註

- 在 debug 模式（`cargo run`）下，會輸出內部 token 與 AST（方便除錯）。
- 在 release 模式（`cargo run --release`）下，只會輸出程式執行結果。

## 專案結構

```text
src/
  scanner.rs            # 輸入掃描流程
  scanner/token.rs      # token 定義（logos lexer）
  parser.rs             # 遞迴下降剖析器
  parser/ast.rs         # AST 定義
  evaluator.rs          # 求值器／直譯器
  evaluator/value.rs    # 執行期值與閉包型別
  evaluator/symbol_table.rs
  error.rs              # 最上層錯誤型別
  main.rs               # CLI 入口（stdin -> scanner -> parser -> evaluator）

tests/
  ...                   # 語法、運算、if、define、函式、遞迴、型別測試
```

## 授權

請參考 [LICENSE](LICENSE)。
