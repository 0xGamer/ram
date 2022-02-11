use colored::*;
pub mod operation_test;
pub mod stack_test;

use operation_test::*;
use stack_test::*;

pub fn test() {
    ram_works();
    operation_works();
}

pub fn log_pass(string: &str) {
    println!("{}: {}", "PASSED".green().bold(), string.blue());
}

pub fn log_fail(string: &str) {
    println!("{}: {}", "FAILED".red().bold(), string.yellow());
}

pub fn assert_f64(a: f64, b: f64, statement: &str) {
    if a == b {
        log_pass(statement)
    } else {
        log_fail(statement)
    }
}

pub fn assert_str(a: &str, b: &str, statement: &str) {
    if a == b {
        log_pass(statement)
    } else {
        log_fail(statement)
    }
}

pub fn assert_vec_int(a: Vec<f64>, b: Vec<f64>, statement: &str) {
    let mut i = 0usize;
    for item in a {
        if item != b[i] {
            return log_fail(statement);
        }
        i = i + 1;
    }
    log_pass(statement)
}

pub fn assert_vec_str(a: Vec<String>, b: Vec<&str>, statement: &str) {
    let mut i = 0usize;
    for item in a {
        if item.trim() != b[i] {
            return log_fail(statement);
        }
        i = i + 1;
    }
    log_pass(statement)
}