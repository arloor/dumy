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
cargo run --bin dumycmd -- mkdir "a b"
cargo run --bin dumypwsh -- "Get-netIPV4Protocol;Get-netIPV6Protocol"
# 将ipv6地址改成slaac分配
cargo run --bin dumypwsh -- "Set-NetIPv6Protocol -UseTemporaryAddresses Disabled;Set-NetIPv6Protocol -RandomizeIdentifiers Disabled;Get-NetIPv6Protocol;Restart-NetAdapter -Name '以太网 6'"
cargo run --bin dumypwsh -- Get-netIPV4Protocol
cargo run --bin dumypwsh -- "mkdir 'a b c'"
```

## 安装

```bash
cargo install --path . --features no-console --bin dumycmd
cargo install --path . --features no-console --bin dumypwsh
```

## 日志文件

如果在执行命令时遇到问题，可以查看日志文件 `dumy.log`，该文件位于当前工作目录下。

## 快捷方式

![alt text](image.png)

| 起始位置下的 log 目录中存放日志文件

| 可以把快捷方式移动到 `C:\ProgramData\Microsoft\Windows\Start Menu\Programs` 来添加到开始菜单

可以代替 powershell 的快捷程序，来达成隐藏窗口，并且突破程序最长 260 个字符的限制，例如：

```pwsh
C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe -ExecutionPolicy Bypass -Command "Stop-VM -Name 'rhel9'"
```

## 任务计划程序使用

![alt text](image-1.png)

或者使用 powershell 脚本设定本用户登录时启动任务

```pwsh
# 1. 定义触发器：当任何用户登录时触发
$trigger = New-ScheduledTaskTrigger -AtLogOn

# 2. 定义操作：要运行的程序和参数
# 请将下面的路径替换为你实际的程序路径
$action = New-ScheduledTaskAction -Execute "C:\Users\arloor\.cargo\bin\dumypwsh.exe" -Argument "C:\Users\arloor\mihomo\mihomo.exe -d C:\Users\arloor\mihomo -f C:\Users\arloor\mihomo\clash.yaml"

# 3. 定义设置（可选）：例如允许按需运行，或者如果任务失败则重新启动
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable

# 4. 注册（创建）任务
# -TaskName: 任务名称
# -User: 指定运行任务的用户账户。
#        "Users" 表示所有用户组（通常用于交互式程序）。
#        "SYSTEM" 表示以系统权限运行（通常用于后台服务）。
# -RunLevel: Highest 表示以最高权限（管理员）运行
Register-ScheduledTask -TaskName "MyAutoStartTask" -Action $action -Trigger $trigger -Settings $settings -User $env:USERNAME -RunLevel Highest
```

| 起始位置下的 log 目录中存放日志文件

## 参考文档：

[Process Creation Flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
