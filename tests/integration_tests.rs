use language_parser::compile_and_run;

#[test]
fn test_simple_integer() {
    let result = compile_and_run("let x = 42; return x;");
    assert_eq!(result.unwrap(), Some(42));
}

#[test]
fn test_arithmetic_add() {
    let result = compile_and_run("let x = 2 + 3; return x;");
    assert_eq!(result.unwrap(), Some(5));
}

#[test]
fn test_arithmetic_mul() {
    let result = compile_and_run("let x = 4 * 5; return x;");
    assert_eq!(result.unwrap(), Some(20));
}

#[test]
fn test_operator_precedence() {
    // 2 + 3 * 4 = 2 + 12 = 14
    let result = compile_and_run("let x = 2 + 3 * 4; return x;");
    assert_eq!(result.unwrap(), Some(14));
}

#[test]
fn test_parentheses() {
    // (2 + 3) * 4 = 5 * 4 = 20
    let result = compile_and_run("let x = (2 + 3) * 4; return x;");
    assert_eq!(result.unwrap(), Some(20));
}

#[test]
fn test_multiple_variables() {
    let result = compile_and_run("let x = 10; let y = 20; let z = x + y; return z;");
    assert_eq!(result.unwrap(), Some(30));
}

#[test]
fn test_if_true_branch() {
    let result = compile_and_run("let x = 5; if x > 3 { return 100; } else { return 0; }");
    assert_eq!(result.unwrap(), Some(100));
}

#[test]
fn test_if_false_branch() {
    let result = compile_and_run("let x = 2; if x > 3 { return 100; } else { return 0; }");
    assert_eq!(result.unwrap(), Some(0));
}

#[test]
fn test_comparison_eq() {
    let result = compile_and_run("let x = 5; let y = 5; if x == y { return 1; } else { return 0; }");
    assert_eq!(result.unwrap(), Some(1));
}

#[test]
fn test_comparison_lt() {
    let result = compile_and_run("let x = 3; if x < 5 { return 1; } else { return 0; }");
    assert_eq!(result.unwrap(), Some(1));
}

#[test]
fn test_while_loop_sum() {
    // sum = 0; i = 0; while i < 5 { sum = sum + i; i = i + 1; } return sum; => 10
    let result = compile_and_run(
        "let sum = 0; let i = 0; while i < 5 { sum = sum + i; i = i + 1; } return sum;"
    );
    assert_eq!(result.unwrap(), Some(10));
}

#[test]
fn test_while_loop_factorial() {
    // n = 5; fact = 1; while n > 1 { fact = fact * n; n = n - 1; } return fact; => 120
    let result = compile_and_run(
        "let n = 5; let fact = 1; while n > 1 { fact = fact * n; n = n - 1; } return fact;"
    );
    assert_eq!(result.unwrap(), Some(120));
}

#[test]
fn test_fibonacci_10() {
    // Fibonacci(10) = 89
    let result = compile_and_run(
        "let a = 0; let b = 1; let i = 0; while i < 10 { let temp = a + b; a = b; b = temp; i = i + 1; } return b;"
    );
    assert_eq!(result.unwrap(), Some(89));
}

#[test]
fn test_nested_loops() {
    // Count: 2x3 = 6
    let result = compile_and_run(
        "let i = 0; let j = 0; let count = 0; while i < 2 { j = 0; while j < 3 { count = count + 1; j = j + 1; } i = i + 1; } return count;"
    );
    assert_eq!(result.unwrap(), Some(6));
}

#[test]
fn test_complex_expression() {
    // ((10 + 5) * 2 - 3) / 3 = 9
    let result = compile_and_run("let x = ((10 + 5) * 2 - 3) / 3; return x;");
    assert_eq!(result.unwrap(), Some(9));
}

#[test]
fn test_no_return_value() {
    let result = compile_and_run("let x = 5;");
    assert_eq!(result.unwrap(), None);
}