use std::{env, fs, process};

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

// 定义配置结构体
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // 创建一个新的配置实例
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query,
            file_path
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // 读取文件内容
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
