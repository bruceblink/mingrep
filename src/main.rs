use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // 获取命令的2个参数
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.1)
    .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}


// 解析命令行参数
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
