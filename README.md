# omo-switch

一个简单的 Rust CLI 工具，用于管理 OpenCode 配置文件中的 `oh-my-opencode` 插件状态（兼容旧版 `oh-my-opencode` 和新版 `oh-my-opencode@latest`）。

## 功能

- 自动定位 OpenCode 配置文件（跨平台支持）
- 支持启用、禁用、查看状态的子命令
- 安全的原子写入，避免配置文件损坏

## 安装

```bash
cargo install --path .
```

安装后可以在任何位置使用 `omos` 命令。

## 使用

直接运行 `omos` 不带参数，会同时显示帮助信息和当前插件状态：

```
Usage: omos [COMMAND]

Commands:
  on      启用 oh-my-opencode 插件（新安装默认使用 oh-my-opencode@latest）
  off     禁用 oh-my-opencode 插件（同时移除新旧两个版本）
  status  查看当前插件状态 [aliases: s]

Options:
  -h, --help  Print help

当前状态: ● oh-my-opencode 插件已启用（支持 oh-my-opencode 或 oh-my-opencode@latest）
```

```bash
# 启用 oh-my-opencode 插件（新安装默认使用 @latest 版本）
omos on

# 禁用 oh-my-opencode 插件（同时移除新旧两个版本）
omos off

# 查看当前插件状态（短命令: s）
omos status
omos s
```

### 示例输出

启用插件：
```
✓ 已启用 oh-my-opencode 插件
```

禁用插件：
```
✗ 已禁用 oh-my-opencode 插件
```

查看状态：
```
配置文件: C:\Users\username\.config\opencode\opencode.json
状态: ● oh-my-opencode 插件已启用
当前插件列表: ["oh-my-opencode@latest"]
```

重复操作时会提示：
```
○ oh-my-opencode 插件已经是启用状态
```

## 配置文件位置

- **Windows**: `C:\Users\<username>\.config\opencode\opencode.json`
- **macOS**: `~/.config/opencode/opencode.json`
- **Linux**: `~/.config/opencode/opencode.json`

## 构建

```bash
cargo build --release
```

可执行文件位于 `target/release/omos.exe`（Windows）或 `target/release/omos`（Unix）。

## 依赖

- `serde` - JSON 序列化/反序列化
- `serde_json` - JSON 处理
- `anyhow` - 错误处理
- `dirs` - 跨平台配置目录
- `colored` - 彩色终端输出
- `clap` - 命令行参数解析

## 许可

MIT
