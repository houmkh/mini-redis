use crate::{Connection, Db, Frame, Parse};

use bytes::Bytes;
use tracing::{debug, instrument};

/// Get the value of key.
///
/// If the key does not exist the special value nil is returned. An error is
/// returned if the value stored at key is not a string, because GET only
/// handles string values.
#[derive(Debug)]
pub struct Get {
    /// Name of the key to get
    key: String,
}

impl Get {
    /// 创建一个Get命令对象
    pub fn new(key: impl ToString) -> Get {
        Get {
            key: key.to_string(),
        }
    }

    /// Get the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// 解析frame，并获取对应Get命令
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Get> {
        let key = parse.next_string()?;
        Ok(Get { key })
    }

    /// 对数据库执行Get命令
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // 从数据库中获取value
        // - 如果值存在，则将其转成Frame::Bulk形式
        // - 否则转成Frame::Null


        // 日志记录响应

        
        // 将响应写入连接
        todo!();
        Ok(())
    }

    /// 将命令转成Frame格式，在客户端将格式化的Get
    ///命令发给服务端时会调用。
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        // 将命令以["get",key]的形式放入frame。注意每个元素都要转成字节形式
        todo!();
        frame
    }
}
