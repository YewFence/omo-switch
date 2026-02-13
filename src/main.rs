use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use colored::Colorize;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "omos")]
#[command(about = "快速切换 oh-my-opencode 插件状态")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 启用 oh-my-opencode 插件
    On,
    /// 禁用 oh-my-opencode 插件
    Off,
    /// 查看当前插件状态
    #[command(visible_alias("s"))]
    Status,
}

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

fn is_plugin_enabled(config: &OpenCodeConfig) -> bool {
    config.plugin.iter().any(|p| p == "oh-my-opencode")
}

fn enable_plugin(config: &mut OpenCodeConfig) -> bool {
    let plugin_name = "oh-my-opencode";

    if config.plugin.iter().any(|p| p == plugin_name) {
        false // 已经启用
    } else {
        config.plugin.push(plugin_name.to_string());
        true // 已添加
    }
}

fn disable_plugin(config: &mut OpenCodeConfig) -> bool {
    let plugin_name = "oh-my-opencode";

    if let Some(pos) = config.plugin.iter().position(|p| p == plugin_name) {
        config.plugin.remove(pos);
        true // 已删除
    } else {
        false // 本来就是禁用状态
    }
}

fn write_config(path: &PathBuf, config: &OpenCodeConfig) -> Result<()> {
    let content = serde_json::to_string_pretty(config)
        .context("序列化配置失败")?;

    let temp_path = path.with_extension("json.tmp");
    fs::write(&temp_path, content)
        .context("写入临时文件失败")?;

    fs::rename(&temp_path, path)
        .context("重命名配置文件失败")?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = get_config_path()?;

    match cli.command {
        Commands::On => {
            let mut config = read_config(&config_path)?;
            if enable_plugin(&mut config) {
                write_config(&config_path, &config)?;
                println!("{} 已启用 oh-my-opencode 插件", "✓".green());
            } else {
                println!("{} oh-my-opencode 插件已经是启用状态", "○".cyan());
            }
        }
        Commands::Off => {
            let mut config = read_config(&config_path)?;
            if disable_plugin(&mut config) {
                write_config(&config_path, &config)?;
                println!("{} 已禁用 oh-my-opencode 插件", "✗".yellow());
            } else {
                println!("{} oh-my-opencode 插件已经是禁用状态", "○".cyan());
            }
        }
        Commands::Status => {
            let config = read_config(&config_path)?;
            let enabled = is_plugin_enabled(&config);
            println!("配置文件: {}", config_path.display().to_string().cyan());
            if enabled {
                println!("状态: {} oh-my-opencode 插件已启用", "●".green());
            } else {
                println!("状态: {} oh-my-opencode 插件已禁用", "○".yellow());
            }
            println!("当前插件列表: {:?}", config.plugin);
        }
    }

    Ok(())
}
