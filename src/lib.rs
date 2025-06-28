use std::fs;

// 定义配置结构体
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // 创建一个新的配置实例
    pub fn build(args: &[String]) -> Result<Config, &str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // 读取文件内容
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}