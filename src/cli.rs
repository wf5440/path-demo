use clap::{Parser, Subcommand};

/// 命令行参数结构体
#[derive(Parser)]
#[command(name = "m")]
#[command(about = "用于记忆和快速跳转常用路径的工具")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// 子命令枚举
#[derive(Subcommand)]
pub enum Commands {
    /// 添加路径别名
    Add {
        /// 别名
        alias: String,
        /// 实际路径
        path: String,
    },
    /// 删除别名
    Remove {
        /// 要删除的别名
        alias: String,
    },
    /// 列出所有别名
    List,
    /// 跳转到路径
    To {
        /// 要跳转的别名
        alias: String,
    },
}
