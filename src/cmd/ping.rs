use crate::{Connection, Frame, Parse, ParseError};
use bytes::Bytes;
use tracing::{debug, instrument};

/// Returns PONG if no argument is provided, otherwise
/// return a copy of the argument as a bulk.
///
/// This command is often used to test if a connection
/// is still alive, or to measure latency.
#[derive(Debug, Default)]
pub struct Ping {
    /// 自定义消息返回
    msg: Option<Bytes>,
}

impl Ping {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(msg: Option<Bytes>) -> Ping {
        Ping { msg }
    }

    /// 解析ping的帧
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Ping> {
        /*
            获取数据，分情况讨论
            1. 数据正常：生成带信息ping类型命令并返回
            2. EndOfStream: 生成默认类型ping命令并返回
            3. 错误信息：返回错误
        */
        todo!();
    }

    /// ping命令执行逻辑
    #[instrument(skip(self, dst))]
    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        /*
           1. 判断是否需要携带信息
               a.不需要携带信息则生成"PONG"响应
               b.携带信息则将信息转成Bulk形式
           2. 日志记录响应
           3. 回复响应
        */
        todo!();
    }

    /// 将命令转换成frame
    pub(crate) fn into_frame(self) -> Frame {
        // 生成["ping",xxxx]形式的bulk frame
        todo!();
    }
}
