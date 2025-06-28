# DumyCmd

一个用于在后台静默执行 Windows 命令的工具程序。

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

```bash
# 列出目录内容
dumycmd.exe dir

# 执行批处理文件
dumycmd.exe script.bat

# 运行其他程序
dumycmd.exe ping google.com
```

## ⚠️ 注意事项

### 空格处理限制

**对原始命令中包含空格的情况支持不是很好**。

### 推荐用法

- ✅ 简单命令：`dumycmd.exe dir`
- ✅ 不含空格的路径：`dumycmd.exe ping google.com`
- ✅ 批处理文件：`dumycmd.exe mybatch.bat`

## 编译方法

确保已安装 Rust，然后执行：

```bash
cargo build --release
```

编译后的可执行文件位于 `target/release/dumycmd.exe`


## 参考文档：

[Process Creation Flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
