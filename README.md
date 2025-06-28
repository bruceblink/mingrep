# mingrep

一个用 Rust 实现的简化版 grep 命令行工具。

## 功能简介

- 在指定文件中查找包含指定模式（字符串）的所有行，并输出这些行。

## 用法

```bash
cargo run -- <pattern> <filename>
```

- `<pattern>`：要查找的字符串模式。
- `<filename>`：要搜索的目标文件名。

示例：

```bash
cargo run -- fn main.rs
```

将会输出 `main.rs` 文件中所有包含 `fn` 的行。

## 依赖

- Rust 1.86 及以上

## 构建与运行

1. 构建项目：

   ```bash
   cargo build
   ```

2. 运行示例：

   ```bash
   cargo run -- hello src/main.rs
   ```

## 许可证

MIT
