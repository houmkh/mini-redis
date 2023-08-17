use crate::cmd::{Parse, ParseError};
use crate::{Connection, Db, Frame};

use bytes::Bytes;
use std::time::Duration;
use tracing::{debug, instrument};

/// Set `key` to hold the string `value`.
///
/// If `key` already holds a value, it is overwritten, regardless of its type.
/// Any previous time to live associated with the key is discarded on successful
/// SET operation.
///
/// # Options
///
/// Currently, the following options are supported:
///
/// * EX `seconds` -- Set the specified expire time, in seconds.
/// * PX `milliseconds` -- Set the specified expire time, in milliseconds.
#[derive(Debug)]
pub struct Set {
    /// the lookup key
    key: String,

    /// the value to be stored
    value: Bytes,

    /// When to expire the key
    expire: Option<Duration>,
}

impl Set {
    /// 创建一个set命令
    ///
    /// 如果不需要过期时间则expire为None
    pub fn new(key: impl ToString, value: Bytes, expire: Option<Duration>) -> Set {
        Set {
            key: key.to_string(),
            value,
            expire,
        }
    }

    /// Get the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the value
    pub fn value(&self) -> &Bytes {
        &self.value
    }

    /// Get the expire
    pub fn expire(&self) -> Option<Duration> {
        self.expire
    }

    /// 解析frame
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Set> {
        use ParseError::EndOfStream;

        // 使用 parse.next_string() 方法读取下一个字符串字段作为键（key）。
        // 如果解析成功，则返回键的值；否则，返回一个错误。


        // 使用 parse.next_bytes() 方法读取下一个字节字段作为值（value）。
        // 如果解析成功，则返回值的字节切片；否则，返回一个错误。


        // 创建一个mut expire，用于存储过期时间。初始值为 None。


        /*
        使用 match 匹配下一个字符串字段的情况：
        - 如果下一个字符串字段是 "EX"（不区分大小写），则表示设置了过期时间且单位是secs，下一个值应该是一个整数，并转成Duration类型。
        - 如果下一个字符串字段是 "PX"（不区分大小写），则表示设置了过期时间且单位是millis，下一个值应该是一个整数，并转成Duration类型。
        - 如果下一个字符串字段不是 "EX" 或 "PX"，则表示设置了不支持的选项，将返回一个错误，指示 SET 目前仅支持过期时间选项。
        - 如果解析到达流的末尾，表示没有进一步的数据需要解析，这是正常的运行时情况，表示没有指定 SET 的选项。
        - 其他所有错误都会返回一个错误，导致连接被终止。
        */

        // 返回set类型
        todo!();
    }

    /// 对数据库连接执行set命令
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // 执行set

        // 创建执行成功响应，并写给连接
        todo!();
    }

    /// 将set命令转成frame形式
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        // 将命令以["set",key,value]的形式写入frame
        todo!();
        // 判断是否有过期时间，如果有过期时间以["px",ms]的格式写入frame
        todo!();
        frame
    }
}
