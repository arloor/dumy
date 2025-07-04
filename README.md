# DumyCmd

一个用于在后台静默执行 Windows 命令的工具程序。个人用于替换下面的 vbs 脚本以实现隐藏 windows 控制台窗口，用于开启自启动或任务计划程序

```powershell
Set WshShell = CreateObject("WScript.Shell")
WshShell.Run "sslocal --local-addr xxxxxx -k xxxxx -v -m aes-256-gcm -s xxxxxxx", 0
```

直接替换为

```bash
dumycmd.exe sslocal --local-addr xxxxxx -k xxxxx -v -m aes-256-gcm -s xxxxxxx
```

## 功能特点

### 🔇 隐藏控制台窗口

- 使用 `windows_subsystem = "windows"` 编译选项，执行时**不会显示控制台窗口**
- 特别适合在**Windows 任务计划程序**中使用，避免弹出黑色命令行窗口影响用户体验
- 可以用于自动化脚本和后台任务执行

### 📝 支持 GBK 编码转换

- 自动将 Windows 命令输出的 GBK 编码转换为 UTF-8
- 确保中文字符正确显示

### 📤 实时输出流

- 增量式读取命令输出，而不是等待命令完全执行完毕
- 避免因输出过多而填满系统缓冲区

## 使用方法

```bash
dumycmd.exe <command> [args...]
```

### 示例

```powershell
# 列出目录内容
cargo run -- dir
# 运行其他程序
cargo run -- ping baidu.com
cargo run --features cmd -- mkdir "a b" # 参数中带目录时，需要使用cmd feature
cargo run -- "Get-NETIPV4Protocol; GET-NETIPV6protocol" # 默认为powershell模式，支持powershell的命令
```

## Feature 特性说明

本项目支持多个可选特性（features），可以根据不同需求进行编译：

### `no-console`

- **作用**：隐藏控制台窗口，程序运行时不显示黑色命令行窗口
- **实现原理**：启用 `windows_subsystem = "windows"` 编译选项
- **使用场景**：
  - Windows 任务计划程序中的后台任务
  - 自动化脚本执行
  - 需要静默运行的程序
- **启用方法**：
  ```bash
  cargo build --features no-console
  cargo run --features no-console -- <command>
  ```

### `cmd`

- **作用**：使用 Windows CMD 命令行解释器执行命令
- **默认行为**：不启用时使用 PowerShell 执行命令
- **使用场景**：
  - 需要执行传统的 DOS/CMD 命令
  - 参数中包含空格的路径或文件名
  - 与旧版 Windows 批处理脚本兼容
- **启用方法**：
  ```bash
  cargo build --features cmd
  cargo run --features cmd -- mkdir "a b"
  ```

### 组合使用

可以同时启用多个特性：

```bash
# 同时启用无控制台和CMD模式
cargo build --features "no-console,cmd"
cargo run --features "no-console,cmd" -- <command>
```

## 编译方法

确保已安装 Rust，然后根据需要选择不同的编译方式：

### 基础编译

```bash
# 默认编译（显示控制台，使用PowerShell）
cargo build --release

# 编译并安装到系统
cargo install --path .
```

### 静默模式编译（推荐用于生产环境）

```bash
# 编译无控制台版本（适用于任务计划程序）
cargo build --release --features no-console

# 安装无控制台版本
cargo install --path . --features no-console
```

### CMD 模式编译

```bash
# 编译CMD版本
cargo build --release --features cmd

# 编译无控制台的CMD版本
cargo build --release --features "no-console,cmd"
```

编译后的可执行文件位于 `target/release/dumycmd.exe`

## 参考文档：

[Process Creation Flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
