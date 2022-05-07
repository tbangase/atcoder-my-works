use cli_test_dir::*;
use std::{time::{Duration, Instant}, collections::HashSet, iter::FromIterator};

const BIN: &'static str = "./main";

#[test]
fn testcase_1() {
    let n = 1000;

    test_function(n, Some(true));
}

fn test_function(n: usize, _success: Option<bool>) {
    let testdir = TestDir::new(BIN, "");

    let start = Instant::now();
    let duration = start.elapsed();

    let mut efficient_nums: HashSet<usize> = HashSet::from_iter(1..=(2*n+1));

    // Build input

    // Start program
    println!("Point 0");
    let input = format!("{}\n", n);
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    let out = output.stdout_str();
    let result: usize = out.replace("\n", "").parse().unwrap();

    efficient_nums.remove(&result);
    println!("Point 1");

    for _i in 0..=n {
        println!("Point 2");
        let kiminikimeta = efficient_nums.iter().next().unwrap_or(&0).to_owned();
        efficient_nums.remove(&kiminikimeta);
        let input = format!("{}\n", kiminikimeta);
        println!("Inputting {}", kiminikimeta);
        let output = testdir
            .cmd()
            .output_with_stdin(input)
            .tee_output()
            .expect_success();

        // Processing outputs
        let outs = output.stdout_str().split(" ").collect::<Vec<&str>>();
        let result: usize = outs[0].replace("\n", "").parse().unwrap();

        efficient_nums.remove(&result);
    }


    assert!(duration <= Duration::new(2, 0));
    // assert!(output.stderr_str().is_empty());
}
