use std::io::{Write};

fn run_shadowing_example(initial_x: i32) -> (String, String) {
    let mut output = Vec::new();

    let x = initial_x;
    let x = x + 1;

    {
        let x = x * 2;
        writeln!(output, "The value of x in the inner scope is: {x}").unwrap();
    }

    writeln!(output, "The value of x is: {x}").unwrap();

    // Split output into lines for easier assertions
    let output_str = String::from_utf8(output).unwrap();
    let mut lines = output_str.lines();
    let inner = lines.next().unwrap_or("").to_string();
    let outer = lines.next().unwrap_or("").to_string();
    (inner, outer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_shadowing_scopes() {
        let initial_x = 5;
        let (inner, outer) = run_shadowing_example(initial_x);
        // x = 5 -> x = 6 (outer), inner: x = 6*2 = 12
        assert_eq!(inner, "The value of x in the inner scope is: 12");
        assert_eq!(outer, "The value of x is: 6");
    }

    #[test]
    fn test_inner_scope_print_value() {
        let initial_x = 10;
        let (inner, _) = run_shadowing_example(initial_x);
        // x = 10 -> x = 11, inner: x = 11*2 = 22
        assert_eq!(inner, "The value of x in the inner scope is: 22");
    }

    #[test]
    fn test_outer_scope_print_value() {
        let initial_x = 3;
        let (_, outer) = run_shadowing_example(initial_x);
        // x = 3 -> x = 4, inner: x = 8, outer: x = 4
        assert_eq!(outer, "The value of x is: 4");
    }

    #[test]
    fn test_negative_initial_x() {
        let initial_x = -7;
        let (inner, outer) = run_shadowing_example(initial_x);
        // x = -7 -> x = -6, inner: x = -12, outer: x = -6
        assert_eq!(inner, "The value of x in the inner scope is: -12");
        assert_eq!(outer, "The value of x is: -6");
    }

    #[test]
    fn test_zero_initial_x() {
        let initial_x = 0;
        let (inner, outer) = run_shadowing_example(initial_x);
        // x = 0 -> x = 1, inner: x = 2, outer: x = 1
        assert_eq!(inner, "The value of x in the inner scope is: 2");
        assert_eq!(outer, "The value of x is: 1");
    }

    #[test]
    fn test_no_inner_scope() {
        // Simulate the code with no inner scope
        fn run_no_inner_scope(initial_x: i32) -> String {
            let mut output = Vec::new();
            let x = initial_x;
            let x = x + 1;
            writeln!(output, "The value of x is: {x}").unwrap();
            String::from_utf8(output).unwrap().trim().to_string()
        }
        let result = run_no_inner_scope(5);
        // x = 5 -> x = 6, only outer print
        assert_eq!(result, "The value of x is: 6");
    }
}