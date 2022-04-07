---
title: 清理 mac 启动项
date: 2022-04-07 10:37:00
tags:
  - mac
---

# 清理 mac 启动项

有几个死活去不掉的开机启动项，搜索了下解决方法。

macOS 系统的启动项会以 .plist 的文件存在于以下目录中：

- `/Library/LaunchDaemons`：系统启动时运行，用户不登录也会运行。
- `/Library/LaunchAgents`：用户登录后运行。
- `~/Library/LaunchAgents`：用户自定义的用户启动项
- `/System/Library/LaunchDaemons`：系统自带的启动项
- `/System/Library/LaunchAgents`：系统自带的启动项

`RunAtLoad`：开机时是否运行。修改为 false。
