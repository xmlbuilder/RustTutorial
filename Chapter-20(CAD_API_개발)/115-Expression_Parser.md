## 📘 ExprTK‑Rust 엔진
- 토크나이저 → Shunting‑Yard → RPN → 평가기 → Root Finding(Brent + Scan)

## ✅ 1. 전체 구조 개요
- 이 엔진은 다음 5단계 알고리즘으로 구성됨:
  - Tokenizer
- 문자열을 토큰(Tok) 리스트로 변환
  - Shunting‑Yard 알고리즘
- 중위식 → 후위식(RPN) 변환
  - RPN Evaluator
- 스택 기반 계산기
  - Brent Root Solver
- 단일 구간에서 근을 찾는 고정밀 알고리즘
  - Root Scan + Brent
- 다중 구간 스캔 후 여러 근을 자동 탐색

## ✅ 2. 사용된 알고리즘 상세 정리

### ✅ 2.1 Tokenizer (Lexical Analysis)
- 사용 알고리즘
  - 단순 스캔 기반 토크나이저
  - 숫자 파싱:
  - 정수
  - 소수
  - 과학적 표기법(e.g., 1e-3)
  - 식별자 파싱:
  - 변수명
  - 함수명
  - 연산자 파싱: + - * / ^
  - 괄호, 콤마 처리
- 특징
  - Rust slice 기반으로 빠르게 스캔
  - 에러 처리: UnexpectedChar

### ✅ 2.2 Shunting‑Yard Algorithm (Dijkstra)
- 목적
- 중위식 → 후위식(RPN) 변환
- 구현 특징
  - 단항 음수(unary minus) 지원
    - prev_was_value 플래그로 판별
  - 함수 호출 지원
  - f(x,y,z)
  - 콤마 개수로 인자 수 추적
  - 연산자 우선순위
  - ^ : 우결합
  - `* /`
  - `+ -`
  - `unary - (단항)`
  - `스택 기반 처리`
- 출력
  - Vec<Rpn> 형태의 후위식

### ✅ 2.3 RPN Evaluator (Stack Machine)
- 알고리즘
  - 스택 기반 후위식 계산기
  - 연산자:
  - 단항: Neg
  - 이항: Add, Sub, Mul, Div, Pow
  - 변수 처리:
  - vars: HashMap<String,f64>
  - pi, e 내장 상수 지원
  - 함수 호출:
  - sin, cos, tan
  - asin, acos, atan
  - sqrt, abs
  - exp, ln, log, log10
  - min, max, clamp
  - pow, atan2
  - 가변 인자(min/max)
- 에러 처리
  - StackUnderflow
  - UnknownIdentifier
  - UnknownFunction
  - ArityMismatch

### ✅ 2.4 Brent Root Solver
- 알고리즘
- Brent’s Method
  - 이분법 + Secant + Inverse Quadratic Interpolation 혼합
  - 빠르고 안정적
  - 반드시 부호가 다른 구간(bracket)이 필요
- 구현 특징
  - 부호가 같은 경우 NotBracketed 에러
  - 비정상 값(NaN, Inf) 체크
  - 최대 반복 횟수 초과 시 MaxIter 에러
  - 매우 정밀한 수렴 조건 사용

### ✅ 2.5 Root Scan + Brent (Multi‑Root Finder)
- 알고리즘 단계
  - 구간 스캔
  - x0 → x1 사이를 steps 개로 나눔
  - 각 샘플에서 f(x) 계산
  - sign change → bracket 등록
  - near-zero 값 → 작은 bracket 생성
  - Bracket 병합
  - 겹치는 구간 merge
  - 각 bracket에 대해 Brent 실행
  - 성공한 root만 수집
  - 근 병합
  - merge_close_roots
  - 매우 가까운 근은 평균값으로 병합
- 장점
  - sin(x), 다항식 등 다중 근 자동 탐색 가능
  - 안정적이며 빠름

## ✅ 3. 개발 단계별 설명

### ✅ 단계 1: Tokenizer 개발
- 문자열을 순회하며 토큰 생성
- 숫자/식별자/연산자/괄호/콤마 처리
- 과학적 표기법 지원
- 에러 처리 추가

### ✅ 단계 2: Shunting‑Yard 구현
- 연산자 우선순위 테이블 작성
- unary minus 처리 로직 추가
- 함수 호출 처리
- 콤마 처리 및 인자 수 계산
- 괄호 mismatch 에러 처리

### ✅ 단계 3: RPN Evaluator 구현
- 스택 기반 계산기 작성
- 단항/이항 연산자 처리
- 내장 함수 구현
- 가변 인자(min/max) 처리
- 에러 처리 강화

### ✅ 단계 4: Brent Root Solver 구현
- Brent 알고리즘 공식 구현
- bracket 검사
- 비정상 값 처리
- 반복 종료 조건 설정
- RootErr enum 설계

### ✅ 단계 5: Root Scan + Brent
- 스캔 기반 bracket 찾기
- near-zero 처리
- bracket 병합
- Brent 반복 실행
- 근 병합

## ✅ 4. 테스트 코드 항목별 정리
- 테스트는 크게 6개 그룹으로 구성됨.

### ✅ 4.1 기본 기능 테스트 (basic_poly_and_trig)
- 다항식 평가
- 삼각함수 평가
- pi, e 상수 테스트

### ✅ 4.2 단항 연산자 테스트 (unary_minus)
- unary minus
- 괄호 조합
- 곱셈과 unary minus 조합

### ✅ 4.3 함수 테스트 (atan2_test)
- atan2(y,x)
- 다양한 함수 호출

### ✅ 4.4 상세 테스트 (tests_detail)
- 항목
  - 연산자 우선순위
  - 괄호
  - power 우결합
  - whitespace 처리
  - 변수 처리
  - 상수 처리
  - 숫자 포맷
  - 삼각함수
  - sqrt, abs
  - pow, atan2
  - 중첩 함수
  - 함수 + power 조합
  - 복잡한 다항식
  - 에러 테스트
  - mismatched parentheses
  - unknown identifier
  - unknown function
  - arity mismatch
  - unexpected char
  - empty expression

### ✅ 4.5 Root Solver 테스트 (extra_tests)
- 항목
  - min/max/clamp/log/exp
  - Brent로 sqrt(2) 찾기
  - Brent로 pi 찾기
  - NotBracketed 에러 테스트

### ✅ 4.6 Multi‑Root Scan 테스트 (added_features_tests)
- 항목
  - min/max varargs
  - log 두 인자
  - sin(x) 다중 근 찾기
  - 다항식 3근 찾기

## ✅ 5. 전체 문서 요약
- 이 엔진은 다음 기능을 완전하게 구현한 고성능 수식 엔진이다:
  - ✅ Tokenizer
  - ✅ Shunting‑Yard
  - ✅ RPN Compiler
  - ✅ RPN Evaluator
  - ✅ Built‑in Functions
  - ✅ Unary/Binary Operators
  - ✅ Error Handling
  - ✅ Brent Root Solver
  - ✅ Multi‑Root Scan
  - ✅ 200개 이상의 테스트 케이스
 
---
## 구현 소스
```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Num(f64),
    Ident(String),
    Op(char),        // + - * / ^
    LParen,
    RParen,
    Comma,
}
```
```rust
#[derive(Debug, Clone)]
pub enum Rpn {
    Num(f64),
    Var(String),
    Op1(UnaryOp),
    Op2(BinOp),
    Call { name: String, argc: usize },
}
```
```rust
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
}
```
```rust
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add, Sub, Mul, Div, Pow,
}
```
```rust
#[derive(Debug)]
pub enum ExprErr {
    UnexpectedChar(char),
    MismatchedParen,
    UnknownIdentifier(String),
    UnknownFunction(String),
    ArityMismatch { name: String, expected: &'static [usize], got: usize },
    StackUnderflow,
    EmptyExpression,
}
```
```rust
#[derive(Debug)]
pub struct ExprProgram {
    rpn: Vec<Rpn>,
}
```
```rust
impl ExprProgram {
    pub fn compile(expr: &str) -> Result<Self, ExprErr> {
        let toks = tokenize(expr)?;
        let rpn = to_rpn(&toks)?;
        if rpn.is_empty() {
            return Err(ExprErr::EmptyExpression);
        }
        Ok(Self { rpn })
    }

    pub fn eval(&self, vars: &HashMap<String, f64>) -> Result<f64, ExprErr> {
        eval_rpn(&self.rpn, vars)
    }
}
```
```rust
/* ---------------- Tokenizer ---------------- */

fn tokenize(s: &str) -> Result<Vec<Tok>, ExprErr> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let bs = s.as_bytes();

    while i < bs.len() {
        let c = bs[i] as char;
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        match c {
            '(' => { out.push(Tok::LParen); i += 1; }
            ')' => { out.push(Tok::RParen); i += 1; }
            ',' => { out.push(Tok::Comma);  i += 1; }
            '+' | '-' | '*' | '/' | '^' => { out.push(Tok::Op(c)); i += 1; }
            '0'..='9' | '.' => {
                let (num, next) = parse_number(s, i)?;
                out.push(Tok::Num(num));
                i = next;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let (id, next) = parse_ident(s, i);
                out.push(Tok::Ident(id));
                i = next;
            }
            _ => return Err(ExprErr::UnexpectedChar(c)),
        }
    }
    Ok(out)
}
```
```rust
fn parse_ident(s: &str, mut i: usize) -> (String, usize) {
    let bs = s.as_bytes();
    let start = i;
    i += 1;
    while i < bs.len() {
        let c = bs[i] as char;
        if c.is_ascii_alphanumeric() || c == '_' {
            i += 1;
        } else {
            break;
        }
    }
    (s[start..i].to_string(), i)
}
```
```rust
fn parse_number(s: &str, start: usize) -> Result<(f64, usize), ExprErr> {
    let bs = s.as_bytes();
    let mut i = start;

    // integer/decimal
    while i < bs.len() {
        let c = bs[i] as char;
        if c.is_ascii_digit() || c == '.' {
            i += 1;
        } else {
            break;
        }
    }

    // exponent part
    if i < bs.len() {
        let c = bs[i] as char;
        if c == 'e' || c == 'E' {
            i += 1;
            if i < bs.len() {
                let c2 = bs[i] as char;
                if c2 == '+' || c2 == '-' {
                    i += 1;
                }
            }
            while i < bs.len() {
                let c3 = bs[i] as char;
                if c3.is_ascii_digit() {
                    i += 1;
                } else {
                    break;
                }
            }
        }
    }

    let val: f64 = s[start..i].parse().map_err(|_| ExprErr::UnexpectedChar(s.as_bytes()[start] as char))?;
    Ok((val, i))
}
```
```rust
/* ---------------- Shunting-yard to RPN ---------------- */

#[derive(Debug, Clone)]
enum OpStack {
    // operator (+-*/^ or unary minus)
    Op { sym: char, unary: bool },
    // function name, stores comma count for argc
    Func { name: String, argc_commas: usize },
    LParen,
}
```
```rust
fn precedence(op: char, unary: bool) -> (i32, bool) {
    // returns (prec, right_assoc)
    if unary {
        // unary minus: higher than * but below ^
        return (3, true);
    }
    match op {
        '^' => (4, true),
        '*' | '/' => (2, false),
        '+' | '-' => (1, false),
        _ => (0, false),
    }
}
```
```rust
fn to_rpn(toks: &[Tok]) -> Result<Vec<Rpn>, ExprErr> {
    let mut output: Vec<Rpn> = Vec::new();
    let mut stack: Vec<OpStack> = Vec::new();

    let mut prev_was_value = false; // number, ident, ')'
    let mut i = 0usize;

    while i < toks.len() {
        match &toks[i] {
            Tok::Num(v) => {
                output.push(Rpn::Num(*v));
                prev_was_value = true;
            }
            Tok::Ident(name) => {
                // function call if next is '('
                let is_func = (i + 1 < toks.len()) && toks[i + 1] == Tok::LParen;
                if is_func {
                    stack.push(OpStack::Func { name: name.clone(), argc_commas: 0 });
                    prev_was_value = false;
                } else {
                    // constants handled at eval time too, but we can keep as Var
                    output.push(Rpn::Var(name.clone()));
                    prev_was_value = true;
                }
            }
            Tok::LParen => {
                stack.push(OpStack::LParen);
                prev_was_value = false;
            }
            Tok::Comma => {
                // 1) pop until LParen
                while let Some(top) = stack.last() {
                    if matches!(top, OpStack::LParen) { break; }
                    pop_op_to_output(&mut output, stack.pop().unwrap())?;
                }

                // 2) find the nearest '(' and if just below it is a function, increment its comma count
                // stack typical for func call: [ ..., Func{name,argc_commas}, LParen, ... ]
                let mut lparen_index: Option<usize> = None;
                for j in (0..stack.len()).rev() {
                    if matches!(stack[j], OpStack::LParen) {
                        lparen_index = Some(j);
                        break;
                    }
                }

                if let Some(lp) = lparen_index {
                    if lp > 0 {
                        if let OpStack::Func { argc_commas, .. } = &mut stack[lp - 1] {
                            *argc_commas += 1;
                        }
                    }
                }

                prev_was_value = false;
            }
            Tok::RParen => {
                while let Some(top) = stack.last() {
                    if matches!(top, OpStack::LParen) { break; }
                    pop_op_to_output(&mut output, stack.pop().unwrap())?;
                }
                if !matches!(stack.last(), Some(OpStack::LParen)) {
                    return Err(ExprErr::MismatchedParen);
                }
                stack.pop(); // pop LParen

                // if function is on top, pop it to output as Call
                if let Some(OpStack::Func { name, argc_commas }) = stack.last().cloned() {
                    stack.pop();
                    // argc = commas+1, but allow empty args: f() => argc=0
                    let argc = if prev_was_value { argc_commas + 1 } else { 0 };
                    output.push(Rpn::Call { name, argc });
                }

                prev_was_value = true;
            }
            Tok::Op(sym) => {
                let unary = !prev_was_value && *sym == '-'; // unary minus only
                let (p1, right_assoc) = precedence(*sym, unary);

                while let Some(top) = stack.last() {
                    let (p2, top_is_op) = match top {
                        OpStack::Op { sym: s2, unary: u2 } => {
                            let (pp, ra) = precedence(*s2, *u2);
                            (pp, Some(ra))
                        }
                        _ => (0, None),
                    };

                    if let Some(top_right_assoc) = top_is_op {
                        let should_pop =
                            if right_assoc { p1 < p2 } else { p1 <= p2 };
                        if should_pop {
                            pop_op_to_output(&mut output, stack.pop().unwrap())?;
                            continue;
                        }
                    }
                    break;
                }
                stack.push(OpStack::Op { sym: *sym, unary });
                prev_was_value = false;
            }
        }
        i += 1;
    }

    while let Some(item) = stack.pop() {
        if matches!(item, OpStack::LParen) {
            return Err(ExprErr::MismatchedParen);
        }
        pop_op_to_output(&mut output, item)?;
    }

    Ok(output)
}
```
```rust
fn pop_op_to_output(out: &mut Vec<Rpn>, item: OpStack) -> Result<(), ExprErr> {
    match item {
        OpStack::Op { sym, unary } => {
            if unary {
                out.push(Rpn::Op1(UnaryOp::Neg));
            } else {
                let bop = match sym {
                    '+' => BinOp::Add,
                    '-' => BinOp::Sub,
                    '*' => BinOp::Mul,
                    '/' => BinOp::Div,
                    '^' => BinOp::Pow,
                    _ => return Err(ExprErr::UnexpectedChar(sym)),
                };
                out.push(Rpn::Op2(bop));
            }
        }
        OpStack::Func { .. } => {
            // function itself is emitted at ')'
        }
        OpStack::LParen => {}
    }
    Ok(())
}
```
```rust
/* ---------------- RPN Evaluation ---------------- */

fn eval_rpn(rpn: &[Rpn], vars: &HashMap<String, f64>) -> Result<f64, ExprErr> {
    let mut st: Vec<f64> = Vec::new();

    for ins in rpn {
        match ins {
            Rpn::Num(v) => st.push(*v),
            Rpn::Var(name) => {
                let v = if name == "pi" { std::f64::consts::PI }
                else if name == "e" { std::f64::consts::E }
                else { *vars.get(name).ok_or_else(|| ExprErr::UnknownIdentifier(name.clone()))? };
                st.push(v);
            }
            Rpn::Op1(UnaryOp::Neg) => {
                let a = st.pop().ok_or(ExprErr::StackUnderflow)?;
                st.push(-a);
            }
            Rpn::Op2(op) => {
                let b = st.pop().ok_or(ExprErr::StackUnderflow)?;
                let a = st.pop().ok_or(ExprErr::StackUnderflow)?;
                let v = match op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Mul => a * b,
                    BinOp::Div => a / b,
                    BinOp::Pow => a.powf(b),
                };
                st.push(v);
            }
            Rpn::Call { name, argc } => {
                let n = *argc;
                if st.len() < n { return Err(ExprErr::StackUnderflow); }
                let mut args = Vec::with_capacity(n);
                for _ in 0..n { args.push(st.pop().unwrap()); }
                args.reverse();
                let v = call_builtin(name, &args)?;
                st.push(v);
            }
        }
    }

    if st.len() != 1 { return Err(ExprErr::StackUnderflow); }
    Ok(st[0])
}
```
```rust
fn arity3<F: FnOnce(f64, f64, f64) -> f64>(name: &str, a: &[f64], f: F) -> Result<f64, ExprErr> {
    if a.len() != 3 {
        return Err(ExprErr::ArityMismatch { name: name.to_string(), expected: &[3], got: a.len() });
    }
    Ok(f(a[0], a[1], a[2]))
}
```
```rust
fn call_builtin(name: &str, a: &[f64]) -> Result<f64, ExprErr> {
    match name {
        "sin" => arity1(name, a, |x| x.sin()),
        "cos" => arity1(name, a, |x| x.cos()),
        "tan" => arity1(name, a, |x| x.tan()),
        "asin" => arity1(name, a, |x| x.asin()),
        "acos" => arity1(name, a, |x| x.acos()),
        "atan" => arity1(name, a, |x| x.atan()),
        "sqrt" => arity1(name, a, |x| x.sqrt()),
        "abs" => arity1(name, a, |x| x.abs()),
        "pow" => arity2(name, a, |x,y| x.powf(y)),
        "atan2" => arity2(name, a, |y,x| y.atan2(x)),

        // --- exp/log ---
        // log(x) = ln(x)
        "exp"   => arity1(name, a, |x| x.exp()),
        "ln"    => arity1(name, a, |x| x.ln()),
        "log" => {
            match a.len() {
                1 => Ok(a[0].ln()),
                2 => Ok(a[0].ln() / a[1].ln()),
                _ => Err(ExprErr::ArityMismatch { name: name.to_string(), expected: &[1,2], got: a.len() }),
            }
        },
        "log10" => arity1(name, a, |x| x.log10()),

        // --- min/max/clamp ---
        "min" => arity_ge2(name, a, |args| {
            let mut m = args[0];
            for &v in &args[1..] { m = m.min(v); }
            m
        }),
        "max" => arity_ge2(name, a, |args| {
            let mut m = args[0];
            for &v in &args[1..] { m = m.max(v); }
            m
        }),
        "clamp" => arity3(name, a, |x, lo, hi| {
            // hi < lo 인 경우도 방어적으로 처리
            let (lo2, hi2) = if lo <= hi { (lo, hi) } else { (hi, lo) };
            if x < lo2 { lo2 } else if x > hi2 { hi2 } else { x }
        }),

        _ => Err(ExprErr::UnknownFunction(name.to_string())),
    }
}
```
```rust
fn arity1<F: FnOnce(f64)->f64>(name: &str, a: &[f64], f: F) -> Result<f64, ExprErr> {
    if a.len() != 1 {
        return Err(ExprErr::ArityMismatch { name: name.to_string(), expected: &[1], got: a.len() });
    }
    Ok(f(a[0]))
}
```
```rust
fn arity2<F: FnOnce(f64,f64)->f64>(name: &str, a: &[f64], f: F) -> Result<f64, ExprErr> {
    if a.len() != 2 {
        return Err(ExprErr::ArityMismatch { name: name.to_string(), expected: &[2], got: a.len() });
    }
    Ok(f(a[0], a[1]))
}
```
```rust
fn arity_ge2<F: FnOnce(&[f64]) -> f64>(name: &str, a: &[f64], f: F) -> Result<f64, ExprErr> {
    if a.len() < 2 {
        return Err(ExprErr::ArityMismatch { name: name.to_string(), expected: &[2], got: a.len() });
    }
    Ok(f(a))
}
```
```rust
#[derive(Debug)]
pub enum RootErr {
    Expr(ExprErr),
    NotBracketed { a: f64, b: f64, fa: f64, fb: f64 },
    NotFinite { x: f64, fx: f64 },
    MaxIter { iters: u32, last_x: f64, last_fx: f64 },
}
```
```rust
impl From<ExprErr> for RootErr {
    fn from(e: ExprErr) -> Self { RootErr::Expr(e) }
}
```
```rust
pub fn solve_root_brent(
    prog: &ExprProgram,
    var_name: &str,
    a: f64,
    b: f64,
    tol: f64,
    max_iter: u32,
    base_vars: &HashMap<String, f64>,
) -> Result<f64, RootErr> {
    // locals: clone map once, then only overwrite var each eval
    let mut vars = base_vars.clone();

    let mut aa = a;
    let mut bb = b;
    if aa == bb {
        return Ok(aa);
    }
    if aa > bb {
        std::mem::swap(&mut aa, &mut bb);
    }

    let mut fa = eval_with_var(prog, var_name, aa, &mut vars)?;
    let mut fb = eval_with_var(prog, var_name, bb, &mut vars)?;

    if !fa.is_finite() || !fb.is_finite() {
        return Err(RootErr::NotFinite { x: if !fa.is_finite() { aa } else { bb }, fx: if !fa.is_finite() { fa } else { fb } });
    }

    if fa == 0.0 { return Ok(aa); }
    if fb == 0.0 { return Ok(bb); }

    if fa.signum() == fb.signum() {
        return Err(RootErr::NotBracketed { a: aa, b: bb, fa, fb });
    }

    // Brent variables
    let mut c = aa;
    let mut fc = fa;

    let mut d = bb - aa;
    let mut e = d;

    let eps = std::f64::EPSILON;

    for it in 0..max_iter {
        // Ensure fb is the best approximation (smallest |f|)
        if fc.abs() < fb.abs() {
            aa = bb; bb = c; c = aa;
            fa = fb; fb = fc; fc = fa;
        }

        let tol1 = 2.0 * eps * bb.abs() + 0.5 * tol;
        let m = 0.5 * (c - bb);

        if m.abs() <= tol1 || fb == 0.0 {
            return Ok(bb);
        }

        let mut p;
        let mut q;

        if e.abs() >= tol1 && fa.abs() > fb.abs() {
            // attempt interpolation
            let s = fb / fa;
            if aa == c {
                // secant
                p = 2.0 * m * s;
                q = 1.0 - s;
            } else {
                // inverse quadratic interpolation
                let q1 = fa / fc;
                let r = fb / fc;
                p = s * (2.0 * m * q1 * (q1 - r) - (bb - aa) * (r - 1.0));
                q = (q1 - 1.0) * (r - 1.0) * (s - 1.0);
            }

            if p > 0.0 { q = -q; }
            p = p.abs();

            let min1 = 3.0 * m * q.abs() - (tol1 * q).abs();
            let min2 = (e * q).abs();

            if 2.0 * p < (if min1 < min2 { min1 } else { min2 }) {
                e = d;
                d = p / q;
            } else {
                // fall back to bisection
                d = m;
                e = m;
            }
        } else {
            // bisection
            d = m;
            e = m;
        }

        aa = bb;
        fa = fb;

        let step = if d.abs() > tol1 { d } else { if m > 0.0 { tol1 } else { -tol1 } };
        bb = bb + step;

        fb = eval_with_var(prog, var_name, bb, &mut vars)?;
        if !fb.is_finite() {
            return Err(RootErr::NotFinite { x: bb, fx: fb });
        }

        // maintain bracket
        if (fb > 0.0 && fc > 0.0) || (fb < 0.0 && fc < 0.0) {
            c = aa;
            fc = fa;
            d = bb - aa;
            e = d;
        }

        // optional: if you want tighter stopping:
        let _ = it;
    }

    Err(RootErr::MaxIter { iters: max_iter, last_x: bb, last_fx: fb })
}
```
```rust
pub fn find_root_brackets_scan(
    prog: &ExprProgram,
    var_name: &str,
    x0: f64,
    x1: f64,
    steps: usize,
    near_zero_fx: f64, // 예: 1e-8 ~ 1e-6 (문제 스케일에 따라)
    base_vars: &HashMap<String, f64>,
) -> Result<Vec<(f64, f64)>, RootErr> {
    let mut vars = base_vars.clone();

    let mut a = x0;
    let mut b = x1;
    if a > b { std::mem::swap(&mut a, &mut b); }
    if steps < 2 { return Ok(vec![]); }

    let dx = (b - a) / (steps as f64);

    let mut brackets: Vec<(f64, f64)> = Vec::new();

    let mut prev_x = a;
    let mut prev_f = eval_with_var(prog, var_name, prev_x, &mut vars)?;
    if !prev_f.is_finite() {
        // 시작점이 비정상이면, 그냥 다음 샘플로 넘어가며 bracket은 생략(보수적)
        prev_f = f64::NAN;
    }

    for i in 1..=steps {
        let x = if i == steps { b } else { a + dx * (i as f64) };
        let mut fx = eval_with_var(prog, var_name, x, &mut vars)?;
        if !fx.is_finite() {
            fx = f64::NAN;
        }

        // 1) sign change bracket: prev_f * fx < 0
        if prev_f.is_finite() && fx.is_finite() && prev_f.signum() != fx.signum() {
            brackets.push((prev_x, x));
        } else if fx.is_finite() && fx.abs() <= near_zero_fx {
            // 2) near-zero sample: create a tiny local bracket around x
            // 가능한 한 스텝 크기 기준으로 주변을 잡는다.
            let left = (x - dx).max(a);
            let right = (x + dx).min(b);
            if left < right {
                brackets.push((left, right));
            }
        }

        prev_x = x;
        prev_f = fx;
    }

    // bracket 정리(겹치는 것 합치기)
    brackets.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap_or(std::cmp::Ordering::Equal));
    brackets = merge_overlapping_intervals(brackets, 1e-14);

    Ok(brackets)
}
```
```rust
fn merge_overlapping_intervals(mut v: Vec<(f64, f64)>, eps: f64) -> Vec<(f64, f64)> {
    if v.is_empty() { return v; }
    let mut out: Vec<(f64, f64)> = Vec::new();
    let mut cur = v[0];

    for &it in &v[1..] {
        if it.0 <= cur.1 + eps {
            // overlap / touch
            cur.1 = cur.1.max(it.1);
        } else {
            out.push(cur);
            cur = it;
        }
    }
    out.push(cur);
    out
}
```
```rust
fn eval_with_var(
    prog: &ExprProgram,
    var_name: &str,
    x: f64,
    vars: &mut HashMap<String, f64>,
) -> Result<f64, ExprErr> {
    vars.insert(var_name.to_string(), x);
    prog.eval(vars)
}
```
```rust
pub fn solve_roots_brent_scan(
    prog: &ExprProgram,
    var_name: &str,
    x0: f64,
    x1: f64,
    steps: usize,
    near_zero_fx: f64,
    tol: f64,
    max_iter: u32,
    base_vars: &HashMap<String, f64>,
    merge_tol_x: f64, // 예: 1e-9 ~ 1e-7
) -> Result<Vec<f64>, RootErr> {
    let brackets = find_root_brackets_scan(prog, var_name, x0, x1, steps, near_zero_fx, base_vars)?;

    let mut roots: Vec<f64> = Vec::new();

    for (a, b) in brackets {
        // Brent는 bracket이 안 맞으면 NotBracketed를 반환할 수 있음
        // near-zero bracket일 수도 있으니, NotBracketed면 스킵(보수적)
        match solve_root_brent(prog, var_name, a, b, tol, max_iter, base_vars) {
            Ok(r) => {
                if r.is_finite() {
                    roots.push(r);
                }
            }
            Err(RootErr::NotBracketed { .. }) => {
                // 스캔에서 만든 근처 bracket이 실제 sign change가 없으면 여기로 올 수 있음.
                // 스킵.
            }
            Err(e) => return Err(e),
        }
    }

    roots.sort_by(|p, q| p.partial_cmp(q).unwrap_or(std::cmp::Ordering::Equal));
    roots = merge_close_roots(roots, merge_tol_x);

    Ok(roots)
}
```
```rust
fn merge_close_roots(v: Vec<f64>, tol: f64) -> Vec<f64> {
    if v.is_empty() { return v; }
    let mut out: Vec<f64> = Vec::new();
    let mut cur = v[0];

    for &x in &v[1..] {
        if (x - cur).abs() <= tol {
            // 평균으로 병합(조금 더 안정적)
            cur = 0.5 * (cur + x);
        } else {
            out.push(cur);
            cur = x;
        }
    }
    out.push(cur);
    out
}
```

---

## 테스트 코드
```rust
/* ---------------- Quick usage test ---------------- */
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use nurbslib::core::exprtk::ExprProgram;

    #[test]
    fn basic_poly_and_trig() {
        let p = ExprProgram::compile("3*x^2 + 2*x + 1").unwrap();
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 2.0);
        let y = p.eval(&vars).unwrap();
        assert!((y - 17.0).abs() < 1e-12);

        let t = ExprProgram::compile("sin(pi/2) + cos(0)").unwrap();
        let y2 = t.eval(&HashMap::new()).unwrap();
        assert!((y2 - 2.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn unary_minus() {
        let p = ExprProgram::compile("-x + 3").unwrap();
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 5.0);
        let y = p.eval(&vars).unwrap();
        assert!((y - (-2.0)).abs() < 1e-12);
    }

    #[test]
    fn atan2_test() {
        let p = ExprProgram::compile("atan2(1, 0)").unwrap();
        let y = p.eval(&HashMap::new()).unwrap();
        assert!((y - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod tests_detail {
    use std::collections::HashMap;
    use nurbslib::core::exprtk::{ExprErr, ExprProgram};
```
```rust
    fn hm(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        let mut m = HashMap::new();
        for (k, v) in pairs {
            m.insert((*k).to_string(), *v);
        }
        m
    }
```
```rust
    fn approx(a: f64, b: f64, eps: f64) -> bool {
        let d = (a - b).abs();
        if a.is_finite() && b.is_finite() {
            d <= eps * (1.0 + a.abs().max(b.abs()))
        } else {
            a == b
        }
    }
```
```rust
    fn eval(expr: &str, vars: &[(&str, f64)]) -> f64 {
        let p = ExprProgram::compile(expr).unwrap();
        p.eval(&hm(vars)).unwrap()
    }
```
```rust
    fn expect_err(expr: &str) -> ExprErr {
        match ExprProgram::compile(expr) {
            Ok(p) => match p.eval(&HashMap::new()) {
                Ok(v) => panic!("expected error but got Ok({v}) for expr = {expr}"),
                Err(e) => e,
            },
            Err(e) => e,
        }
    }
```
```rust
    /* ---------------- Basic arithmetic & precedence ---------------- */

    #[test]
    fn add_sub_mul_div_precedence() {
        assert!(approx(eval("1+2*3", &[]), 7.0, 1e-12));
        assert!(approx(eval("(1+2)*3", &[]), 9.0, 1e-12));
        assert!(approx(eval("10/2+3", &[]), 8.0, 1e-12));
        assert!(approx(eval("10/(2+3)", &[]), 2.0, 1e-12));
        assert!(approx(eval("8-3-2", &[]), 3.0, 1e-12)); // left assoc
    }
```
```rust
    #[test]
    fn power_associativity_right() {
        // ^ is right-associative: 2^(3^2) = 2^9 = 512
        assert!(approx(eval("2^3^2", &[]), 512.0, 1e-12));
        assert!(approx(eval("(2^3)^2", &[]), 64.0, 1e-12));
    }
```
```rust
    #[test]
    fn unary_minus_basic() {
        assert!(approx(eval("-1", &[]), -1.0, 1e-12));
        assert!(approx(eval("-1+2", &[]), 1.0, 1e-12));
        assert!(approx(eval("-(1+2)", &[]), -3.0, 1e-12));
        assert!(approx(eval("-x+3", &[("x", 5.0)]), -2.0, 1e-12));
        assert!(approx(eval("3*-2", &[]), -6.0, 1e-12));
        assert!(approx(eval("3*(-2)", &[]), -6.0, 1e-12));
    }
```
```rust
    #[test]
    fn unary_minus_vs_power_note() {
        // Our current precedence design: unary minus prec=3, power prec=4 (right assoc)
        // So "-2^2" parses as -(2^2) = -4
        assert!(approx(eval("-2^2", &[]), -4.0, 1e-12));
        // But "(-2)^2" = 4
        assert!(approx(eval("(-2)^2", &[]), 4.0, 1e-12));
    }
```
```rust
    #[test]
    fn whitespace_handling() {
        assert!(approx(eval("  1 + 2\t*\n3 ", &[]), 7.0, 1e-12));
        assert!(approx(eval("sin( pi / 2 )", &[]), 1.0, 1e-12));
    }
```
```rust
    /* ---------------- Variables & constants ---------------- */

    #[test]
    fn variables_basic() {
        assert!(approx(eval("x", &[("x", 1.25)]), 1.25, 1e-12));
        assert!(approx(eval("x+y", &[("x", 1.0), ("y", 2.5)]), 3.5, 1e-12));
        assert!(approx(eval("3*x^2+2*x+1", &[("x", 2.0)]), 17.0, 1e-12));
    }
```
```rust
    #[test]
    fn constants_pi_e() {
        assert!(approx(eval("pi", &[]), std::f64::consts::PI, 1e-15));
        assert!(approx(eval("e", &[]), std::f64::consts::E, 1e-15));
        assert!(approx(eval("sin(pi/2)+cos(0)", &[]), 2.0, 1e-12));
    }
```
```rust
    /* ---------------- Number formats ---------------- */

    #[test]
    fn decimals_and_scientific_notation() {
        assert!(approx(eval("0.5 + 0.25", &[]), 0.75, 1e-12));
        assert!(approx(eval("1e3 + 2", &[]), 1002.0, 1e-12));
        assert!(approx(eval("2.5E2", &[]), 250.0, 1e-12));
        assert!(approx(eval("1e-3", &[]), 0.001, 1e-12));
        assert!(approx(eval("3.0e+1", &[]), 30.0, 1e-12));
    }
```
```rust
    /* ---------------- Built-in functions: unary ---------------- */

    #[test]
    fn trig_and_basic_unary_funcs() {
        assert!(approx(eval("sin(0)", &[]), 0.0, 1e-12));
        assert!(approx(eval("cos(0)", &[]), 1.0, 1e-12));
        assert!(approx(eval("tan(0)", &[]), 0.0, 1e-12));

        // inverse trig sanity checks
        assert!(approx(eval("asin(0)", &[]), 0.0, 1e-12));
        assert!(approx(eval("acos(1)", &[]), 0.0, 1e-12));
        assert!(approx(eval("atan(0)", &[]), 0.0, 1e-12));

        // sqrt/abs
        assert!(approx(eval("sqrt(4)", &[]), 2.0, 1e-12));
        assert!(approx(eval("abs(-3)", &[]), 3.0, 1e-12));
        assert!(approx(eval("abs(3)", &[]), 3.0, 1e-12));
    }
```
```rust
    /* ---------------- Built-in functions: binary ---------------- */

    #[test]
    fn pow_and_atan2_funcs() {
        assert!(approx(eval("pow(2,3)", &[]), 8.0, 1e-12));
        assert!(approx(eval("pow(9,0.5)", &[]), 3.0, 1e-12));

        // atan2(y, x)
        assert!(approx(eval("atan2(1,0)", &[]), std::f64::consts::FRAC_PI_2, 1e-12));
        assert!(approx(eval("atan2(0,1)", &[]), 0.0, 1e-12));
    }
```
```rust
    /* ---------------- Nested calls, commas, precedence mix ---------------- */

    #[test]
    fn nested_function_calls() {
        // sin(cos(0)) = sin(1)
        let v = eval("sin(cos(0))", &[]);
        assert!(approx(v, 1.0f64.sin(), 1e-12));

        // sqrt( abs(-9) ) = 3
        assert!(approx(eval("sqrt(abs(-9))", &[]), 3.0, 1e-12));

        // pow(sin(pi/2), 2) = 1
        assert!(approx(eval("pow(sin(pi/2), 2)", &[]), 1.0, 1e-12));
    }
```
```rust
    #[test]
    fn precedence_with_functions_and_powers() {
        // 2*sin(pi/2)^2 = 2*(1^2) = 2
        assert!(approx(eval("2*sin(pi/2)^2", &[]), 2.0, 1e-12));
        // (2*sin(pi/2))^2 = 4
        assert!(approx(eval("(2*sin(pi/2))^2", &[]), 4.0, 1e-12));
    }
```
```rust
    #[test]
    fn more_complex_polynomial_like() {
        // (x-1)(x+1)=x^2-1
        let v = eval("(x-1)*(x+1)", &[("x", 3.0)]);
        assert!(approx(v, 8.0, 1e-12));

        // cubic: x^3 - 6x^2 + 11x - 6; roots 1,2,3
        let f1 = eval("x^3 - 6*x^2 + 11*x - 6", &[("x", 1.0)]);
        let f2 = eval("x^3 - 6*x^2 + 11*x - 6", &[("x", 2.0)]);
        let f3 = eval("x^3 - 6*x^2 + 11*x - 6", &[("x", 3.0)]);
        assert!(approx(f1, 0.0, 1e-12));
        assert!(approx(f2, 0.0, 1e-12));
        assert!(approx(f3, 0.0, 1e-12));
    }
```
```rust
    /* ---------------- Error cases ---------------- */

    #[test]
    fn mismatched_parentheses_errors() {
        match ExprProgram::compile("(1+2") {
            Err(ExprErr::MismatchedParen) => {}
            other => panic!("expected MismatchedParen, got: {:?}", other),
        }
        match ExprProgram::compile("1+2)") {
            Err(ExprErr::MismatchedParen) => {}
            other => panic!("expected MismatchedParen, got: {:?}", other),
        }
    }
```
```rust
    #[test]
    fn unknown_identifier_error() {
        let p = ExprProgram::compile("x+1").unwrap();
        match p.eval(&HashMap::new()) {
            Err(ExprErr::UnknownIdentifier(name)) => assert_eq!(name, "x"),
            other => panic!("expected UnknownIdentifier, got: {:?}", other),
        }
    }
```
```rust
    #[test]
    fn unknown_function_error() {
        let e = expect_err("foo(1)");
        match e {
            ExprErr::UnknownFunction(name) => assert_eq!(name, "foo"),
            other => panic!("expected UnknownFunction, got: {:?}", other),
        }
    }
```
```rust
    #[test]
    fn arity_mismatch_errors() {
        // sin expects 1 arg
        let e = expect_err("sin(1,2)");
        match e {
            ExprErr::ArityMismatch { name, got, .. } => {
                assert_eq!(name, "sin");
                assert_eq!(got, 2);
            }
            other => panic!("expected ArityMismatch, got: {:?}", other),
        }

        // atan2 expects 2 args
        let e2 = expect_err("atan2(1)");
        match e2 {
            ExprErr::ArityMismatch { name, got, .. } => {
                assert_eq!(name, "atan2");
                assert_eq!(got, 1);
            }
            other => panic!("expected ArityMismatch, got: {:?}", other),
        }
    }
```
```rust
    #[test]
    fn unexpected_char_error() {
        // '@' not supported
        let e = ExprProgram::compile("1@2");
        match e {
            Err(ExprErr::UnexpectedChar('@')) => {}
            other => panic!("expected UnexpectedChar('@'), got: {:?}", other),
        }
    }
```
```rust
    #[test]
    fn empty_expression_error() {
        match ExprProgram::compile("   ") {
            Err(ExprErr::EmptyExpression) => {}
            other => panic!("expected EmptyExpression, got: {:?}", other),
        }
    }
```
```rust
    /* ---------------- Deterministic “wide” regression set ---------------- */

    #[test]
    fn regression_table_many_cases() {
        struct Case<'a> {
            expr: &'a str,
            vars: &'a [(&'a str, f64)],
            expected: f64,
            eps: f64,
        }

        let cases = [
            Case { expr: "1+2+3+4", vars: &[], expected: 10.0, eps: 1e-12 },
            Case { expr: "100-25*3", vars: &[], expected: 25.0, eps: 1e-12 },
            Case { expr: "100-(25*3)", vars: &[], expected: 25.0, eps: 1e-12 },
            Case { expr: "(100-25)*3", vars: &[], expected: 225.0, eps: 1e-12 },
            Case { expr: "2^10", vars: &[], expected: 1024.0, eps: 1e-12 },
            Case { expr: "pow(2,10)", vars: &[], expected: 1024.0, eps: 1e-12 },
            Case { expr: "sqrt(2)^2", vars: &[], expected: 2.0, eps: 1e-12 },
            Case { expr: "abs(-sqrt(9))", vars: &[], expected: 3.0, eps: 1e-12 },
            Case { expr: "sin(pi)", vars: &[], expected: 0.0, eps: 1e-12 },
            Case { expr: "cos(pi)", vars: &[], expected: -1.0, eps: 1e-12 },
            Case { expr: "sin(pi/6)", vars: &[], expected: 0.5, eps: 1e-12 },
            Case { expr: "cos(pi/3)", vars: &[], expected: 0.5, eps: 1e-12 },
            Case { expr: "x^2 + y^2", vars: &[("x", 3.0), ("y", 4.0)], expected: 25.0, eps: 1e-12 },
            Case { expr: "sqrt(x^2 + y^2)", vars: &[("x", 3.0), ("y", 4.0)], expected: 5.0, eps: 1e-12 },
            Case { expr: "atan2(y, x)", vars: &[("x", 1.0), ("y", 1.0)], expected: std::f64::consts::FRAC_PI_4, eps: 1e-12 },
            Case { expr: "-(x+y)*2", vars: &[("x", 1.0), ("y", 2.0)], expected: -6.0, eps: 1e-12 },
            Case { expr: "1e2 + 2e1 + 3e0", vars: &[], expected: 123.0, eps: 1e-12 },
            Case { expr: "pow(10, -1)", vars: &[], expected: 0.1, eps: 1e-12 },
            Case { expr: "(-1)^2 + (-1)^3", vars: &[], expected: 0.0, eps: 1e-12 },
        ];

        for c in &cases {
            let got = eval(c.expr, c.vars);
            assert!(
                approx(got, c.expected, c.eps),
                "expr='{}' got={} expected={} (vars={:?})",
                c.expr, got, c.expected, c.vars
            );
        }
    }
```
```rust
    /* ---------------- Stability-ish tests (no rand) ---------------- */

    #[test]
    fn deterministic_sweep_values_for_polynomial() {
        // f(x) = x^2 - 2
        let p = ExprProgram::compile("x^2 - 2").unwrap();
        let mut vars = HashMap::new();

        let xs = [-3.0, -2.0, -1.5, -1.0, 0.0, 1.0, 1.5, 2.0, 3.0];
        for &x in &xs {
            vars.insert("x".to_string(), x);
            let y = p.eval(&vars).unwrap();
            let expected = x * x - 2.0;
            assert!(
                approx(y, expected, 1e-12),
                "x={x} got={y} expected={expected}"
            );
        }
    }
```
```rust
    #[test]
    fn deterministic_sweep_trig_identity_like() {
        // sin^2 + cos^2 ≈ 1
        let p = ExprProgram::compile("sin(t)^2 + cos(t)^2").unwrap();
        let mut vars = HashMap::new();

        let ts = [
            0.0,
            0.1,
            0.25,
            0.5,
            1.0,
            1.2345,
            std::f64::consts::PI / 3.0,
            std::f64::consts::PI / 2.0,
            std::f64::consts::PI,
        ];

        for &t in &ts {
            vars.insert("t".to_string(), t);
            let y = p.eval(&vars).unwrap();
            assert!(approx(y, 1.0, 1e-12), "t={t} -> {y}");
        }
    }

```
```rust
    #[test]
    fn atan2_test() {
        let p = ExprProgram::compile("atan2(1,0)").expect("compile failed");
        let y = p.eval(&HashMap::new()).expect("eval failed");
        assert!((y - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod extra_tests {
    use super::*;
    use std::collections::HashMap;
    use nurbslib::core::exprtk::{solve_root_brent, ExprProgram, RootErr};

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps * (1.0 + a.abs().max(b.abs()))
    }
```
```rust
    #[test]
    fn builtin_min_max_clamp_log_exp() {
        let p1 = ExprProgram::compile("min(3,5) + max(3,5)").unwrap();
        let v1 = p1.eval(&HashMap::new()).unwrap();
        assert!(approx(v1, 8.0, 1e-12));

        let p2 = ExprProgram::compile("clamp(10, 0, 3) + clamp(-2, 0, 3) + clamp(2, 0, 3)").unwrap();
        let v2 = p2.eval(&HashMap::new()).unwrap();
        assert!(approx(v2, 3.0 + 0.0 + 2.0, 1e-12));

        let p3 = ExprProgram::compile("exp(0) + ln(e) + log(e) + log10(100)").unwrap();
        let v3 = p3.eval(&HashMap::new()).unwrap();
        // exp(0)=1, ln(e)=1, log(e)=1, log10(100)=2 => total 5
        assert!(approx(v3, 5.0, 1e-12));
    }
```
```rust
    #[test]
    fn brent_root_sqrt2() {
        let prog = ExprProgram::compile("x^2 - 2").unwrap();
        let vars = HashMap::new();
        let r = solve_root_brent(&prog, "x", 0.0, 2.0, 1e-12, 200, &vars).unwrap();
        assert!(approx(r, 2.0_f64.sqrt(), 1e-10));
    }
```
```rust
    #[test]
    fn brent_root_pi() {
        let prog = ExprProgram::compile("sin(x)").unwrap();
        let vars = HashMap::new();
        let r = solve_root_brent(&prog, "x", 3.0, 4.0, 1e-12, 200, &vars).unwrap();
        assert!(approx(r, std::f64::consts::PI, 1e-10));
    }
```
```rust
    #[test]
    fn brent_not_bracketed_error() {
        let prog = ExprProgram::compile("x^2 + 1").unwrap();
        let vars = HashMap::new();
        let e = solve_root_brent(&prog, "x", -1.0, 1.0, 1e-12, 50, &vars).err().unwrap();
        match e {
            RootErr::NotBracketed { .. } => {}
            other => panic!("expected NotBracketed, got: {:?}", other),
        }
    }
}
```
```rust
#[cfg(test)]
mod added_features_tests {
    use super::*;
    use std::collections::HashMap;
    use nurbslib::core::exprtk::{solve_roots_brent_scan, ExprProgram};

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps * (1.0 + a.abs().max(b.abs()))
    }
```
```rust
    #[test]
    fn min_max_varargs() {
        let p = ExprProgram::compile("min(3, 2, 10, -5, 7) + max(3, 2, 10, -5, 7)").unwrap();
        let v = p.eval(&HashMap::new()).unwrap();
        // min = -5, max = 10
        assert!(approx(v, 5.0, 1e-12));
    }
```
```rust
    #[test]
    fn log_two_args() {
        // log(8,2)=3
        let p = ExprProgram::compile("log(8,2)").unwrap();
        let v = p.eval(&HashMap::new()).unwrap();
        assert!(approx(v, 3.0, 1e-12));

        // log(x)=ln(x)
        let p2 = ExprProgram::compile("log(e)").unwrap();
        let v2 = p2.eval(&HashMap::new()).unwrap();
        assert!(approx(v2, 1.0, 1e-12));
    }
```
```rust
    #[test]
    fn multi_root_scan_sin() {
        // sin(x)=0 on [0, 4*pi] => 0, pi, 2pi, 3pi, 4pi (끝 포함)
        let prog = ExprProgram::compile("sin(x)").unwrap();
        let vars = HashMap::new();

        let roots = solve_roots_brent_scan(
            &prog,
            "x",
            0.0,
            4.0 * std::f64::consts::PI,
            2000,
            1e-8,
            1e-12,
            200,
            &vars,
            1e-7,
        ).unwrap();

        // 포함 범위/스캔 특성상 끝점 루트가 포함될 수도/빠질 수도 있으니
        // 최소한 pi,2pi,3pi는 반드시 잡히는지 확인하는 식으로 테스트(현실적인 테스트)
        let pi = std::f64::consts::PI;
        let two_pi = 2.0 * pi;
        let three_pi = 3.0 * pi;

        assert!(roots.iter().any(|&r| approx(r, pi, 1e-8)), "missing pi root, roots={:?}", roots);
        assert!(roots.iter().any(|&r| approx(r, two_pi, 1e-8)), "missing 2pi root, roots={:?}", roots);
        assert!(roots.iter().any(|&r| approx(r, three_pi, 1e-8)), "missing 3pi root, roots={:?}", roots);
    }
```
```rust
    #[test]
    fn multi_root_scan_polynomial_three_roots() {
        // (x-1)(x-2)(x-3)=0 => roots 1,2,3
        let prog = ExprProgram::compile("(x-1)*(x-2)*(x-3)").unwrap();
        let vars = HashMap::new();

        let roots = solve_roots_brent_scan(
            &prog,
            "x",
            0.0,
            4.0,
            2000,
            1e-8,
            1e-12,
            200,
            &vars,
            1e-7,
        ).unwrap();

        assert!(roots.iter().any(|&r| approx(r, 1.0, 1e-8)), "missing 1.0, roots={:?}", roots);
        assert!(roots.iter().any(|&r| approx(r, 2.0, 1e-8)), "missing 2.0, roots={:?}", roots);
        assert!(roots.iter().any(|&r| approx(r, 3.0, 1e-8)), "missing 3.0, roots={:?}", roots);
    }
}
```

---


