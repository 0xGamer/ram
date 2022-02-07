// Errors

pub fn parse_float(arg: &str, block: usize, line: u32) -> f64 {
    let num: f64;
    num = arg.trim().parse().expect(
        format!(
            "Input a number at block:{}:line{}",
            block.to_string(),
            line.to_string()
        )
        .trim(),
    );
    num
}

pub fn parse_usize(arg: &str, block: usize, line: u32) -> usize {
    let num: usize;
    num = arg.parse().expect(
        format!(
            "Invalid jmp statement, block {} is invalid block:{}::line:{}",
            arg,
            block.to_string(),
            line.to_string()
        )
        .trim(),
    );
    num
}

pub fn stack_len_error(block: usize, line: u32) {
    panic!(
        "Not enough items in the stack, block:{}::line:{}",
        block.to_string(),
        line.to_string()
    );
}

pub fn args_error(block: usize, line: u32) {
    panic!(
        "Invalid arguments at block:{}::line:{}",
        block.to_string(),
        line.to_string()
    );
}

pub fn invalid_jmp(block: usize, line: u32, inv_block: usize) {
    panic!(
        "Invalid jmp arguments at block:{}::line:{}, block {} does not exist",
        block.to_string(),
        line.to_string(),
        inv_block.to_string()
    );
}

pub fn invalid_index(block: usize, line: u32, inv_index: usize) {
    panic!(
        "Invalid vector index, position {} does not exist in vec at block:{}::line:{}",
        inv_index.to_string(),
        block.to_string(),
        line.to_string()
    );
}
