use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // 获取命令的2个参数
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

// 定义配置结构体
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // 创建一个新的配置实例
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments. Usage: <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config {
            query,
            file_path
        }
    }
}
