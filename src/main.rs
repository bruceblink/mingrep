use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // 获取命令的2个参数
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    // 读取文件内容
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


