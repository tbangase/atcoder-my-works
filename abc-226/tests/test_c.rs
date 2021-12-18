use cli_test_dir::*;
use std::time::{Duration, Instant};

const BIN: &'static str = "./main";

#[test]
fn testcase_1() {
    let n = 6;
    let a = vec![
        vec![2, 0],
        vec![3, 0],
        vec![5, 1, 1],
        vec![1, 1, 1],
        vec![3, 2, 3, 4],
        vec![8, 1, 5],
    ];

    test_function(n, a, Some(19));
}

fn test_function(n: usize, a: Vec<Vec<i64>>, success: Option<usize>) {
    let testdir = TestDir::new(BIN, "");

    // Build input
    let mut input = format!("{}\n", n);
    for row in a {
        for val in row {
            input.push_str(&format!("{} ", val));
        }
    }
    input.push_str("\n");

    // Start program
    let start = Instant::now();
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    let duration = start.elapsed();

    // Processing outputs
    let outs = output.stdout_str().split(" ").collect::<Vec<&str>>();

    if let Some(val) = success {
        let result: usize = outs[0].replace("\n", "").parse().unwrap();
        assert_eq!(result, val);
    }
    assert!(duration <= Duration::new(2, 0));
    assert!(output.stderr_str().is_empty());
}
