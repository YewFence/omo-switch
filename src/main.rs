use anyhow::{Context, Result, bail};
use colored::Colorize;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OpenCodeConfig {
    #[serde(default)]
    plugin: Vec<String>,
    #[serde(flatten)]
    other: serde_json::Value,
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = config_dir()
        .context("无法确定配置目录")?;
    Ok(config_dir.join("opencode").join("opencode.json"))
}

fn read_config(path: &PathBuf) -> Result<OpenCodeConfig> {
    if !path.exists() {
        bail!("配置文件不存在: {}", path.display());
    }

    let content = fs::read_to_string(path)
        .context("读取配置文件失败")?;

    let config: OpenCodeConfig = serde_json::from_str(&content)
        .context("解析配置文件失败，JSON 格式可能不正确")?;

    Ok(config)
}

fn toggle_plugin(config: &mut OpenCodeConfig) -> bool {
    let plugin_name = "oh-my-opencode";

    if let Some(pos) = config.plugin.iter().position(|p| p == plugin_name) {
        config.plugin.remove(pos);
        false // 已删除
    } else {
        config.plugin.push(plugin_name.to_string());
        true // 已添加
    }
}

fn write_config(path: &PathBuf, config: &OpenCodeConfig) -> Result<()> {
    // 序列化为格式化的 JSON
    let content = serde_json::to_string_pretty(config)
        .context("序列化配置失败")?;

    // 原子写入：先写临时文件
    let temp_path = path.with_extension("json.tmp");
    fs::write(&temp_path, content)
        .context("写入临时文件失败")?;

    // 重命名为目标文件
    fs::rename(&temp_path, path)
        .context("重命名配置文件失败")?;

    Ok(())
}

fn main() -> Result<()> {
    let config_path = get_config_path()?;

    println!("配置文件路径: {}", config_path.display().to_string().cyan());

    let mut config = read_config(&config_path)?;

    let added = toggle_plugin(&mut config);

    write_config(&config_path, &config)?;

    if added {
        println!("{} 已启用 oh-my-opencode 插件", "✓".green());
    } else {
        println!("{} 已禁用 oh-my-opencode 插件", "✗".yellow());
    }

    println!("当前插件列表: {:?}", config.plugin);

    Ok(())
}
