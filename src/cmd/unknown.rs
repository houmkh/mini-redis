use crate::{Connection, Frame};

use tracing::{debug, instrument};

/// Represents an "unknown" command. This is not a real `Redis` command.
#[derive(Debug)]
pub struct Unknown {
    command_name: String,
}

impl Unknown{
    /// 创建unknown命令
    pub(crate) fn new(key: impl ToString) -> Unknown {
        Unknown {
            command_name: key.to_string(),
        }
    }

    /// 获取命令名
    pub(crate) fn get_name(&self) -> &str {
        todo!();
    }

    /// 回应客户端不支持当前命令
    #[instrument(skip(self, dst))]
    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        // 生成错误帧响应
        todo!();
        // 日志记录响应
        todo!();
        // 将帧写入连接
        todo!();
        Ok(())
    }
}
