# omo-switch

一个简单的 Rust CLI 工具，用于快速切换 OpenCode 配置文件中的 `oh-my-opencode` 插件状态。

## 功能

- 自动定位 OpenCode 配置文件（跨平台支持）
- 一键切换 `oh-my-opencode` 插件的启用/禁用状态
- 安全的原子写入，避免配置文件损坏

## 安装

```bash
cargo install --path .
```

安装后可以在任何位置使用 `omos` 命令。

## 使用

直接运行命令即可切换插件状态：

```bash
omos
```

### 示例输出

启用插件时：
```
配置文件路径: C:\Users\username\AppData\Roaming\opencode\opencode.json
✓ 已启用 oh-my-opencode 插件
当前插件列表: ["oh-my-opencode"]
```

禁用插件时：
```
配置文件路径: C:\Users\username\AppData\Roaming\opencode\opencode.json
✗ 已禁用 oh-my-opencode 插件
当前插件列表: []
```

## 配置文件位置

- **Windows**: `C:\Users\<username>\AppData\Roaming\opencode\opencode.json`
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

## 许可

MIT
