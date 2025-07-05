# Dumy

一个用于在后台静默执行 Windows 命令的工具程序。

有两个可执行文件，分别是 `dumycmd.exe` 和 `dumypwsh.exe`，前者用于执行传统的 cmd 命令，后者用于执行 PowerShell 命令。

均可以做到完全隐藏 windows 控制台窗口，可以替换以下传统的 VBS 脚本：

```powershell
Set WshShell = CreateObject("WScript.Shell")
WshShell.Run "sslocal --local-addr xxxxxx -k xxxxx -v -m aes-256-gcm -s xxxxxxx", 0
```

特别适合用于在 Windows 的自启动或任务计划程序中执行命令，直接设置为下面的命令即可

```bash
dumycmd.exe sslocal --local-addr xxxxxx -k xxxxx -v -m aes-256-gcm -s xxxxxxx
```

## 测试

```powershell
cargo run --bin dumycmd -- netsh interface ipv6 show global
cargo run --bin dumypwsh -- "Get-netIPV4Protocol;Get-netIPV6Protocol"
```

dumycmd支持参数中包含空格的情况，只需要使用双引号包裹即可:

```powershell
cargo run --bin dumycmd -- mkdir "a b c"
```

dumypwsh 不支持参数中包含双引号。

## 安装

```bash
cargo install --path . --features no-console --bin dumycmd
cargo install --path . --features no-console --bin dumypwsh
```

## 参考文档：

[Process Creation Flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
