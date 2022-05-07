use cli_test_dir::*;
use std::time::{Duration, Instant};

const BIN: &'static str = "./main";

#[test]
fn testcase_1() {
    let n = 500000;
    let k = 1;
    let mut p = vec![];

    p.push(2);
    p.push(1);
    for i in 3..500001 {
        p.push(i);
    }

    test_function(n, k, p, Some(true));
}

fn test_function(n: usize, k: usize, p: Vec<i64>, success: Option<bool>) {
    let testdir = TestDir::new(BIN, "");

    // Build input
    let mut input = format!("{} {}\n", n, k);
    for val in p {
        input.push_str(&format!("{} ", val));
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
    // let outs = output.stdout_str().split(" ").collect::<Vec<&str>>();
    // let result: usize = outs[0].replace("\n", "").parse().unwrap();

    // if let Some(val) = success {
    //     assert_eq!(result, if val { 0 } else { 1 });
    // }
    assert!(duration <= Duration::new(2, 0));
    assert!(output.stderr_str().is_empty());
}
