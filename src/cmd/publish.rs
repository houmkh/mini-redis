use crate::{Connection, Db, Frame, Parse};

use bytes::Bytes;

/// Posts a message to the given channel.
///
/// Send a message into a channel without any knowledge of individual consumers.
/// Consumers may subscribe to channels in order to receive the messages.
///
/// Channel names have no relation to the key-value namespace. Publishing on a
/// channel named "foo" has no relation to setting the "foo" key.
#[derive(Debug)]
pub struct Publish {
    /// 频道名
    channel: String,

    /// 要发送的消息
    message: Bytes,
}

impl Publish {
    /// 创建Publish命令
    pub(crate) fn new(channel: impl ToString, message: Bytes) -> Publish {
        Publish {
            channel: channel.to_string(),
            message,
        }
    }

    /// 解析frame，并获取对应Publish命令
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Publish> {
        // 获取string形式的channel

        // 获取字节形式的message

        // 返回对应Publish
        todo!();
    }

    /// 对数据库执行publish命令
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // 向数据库连接执行publish操作，并获取到订阅channel的subscriber的数量

        // 将subscriber_num转换成整数帧，并将其写入连接
        todo!();

        

    }

    /// 将publish命令转成frame的形式
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        // 将publish命令以["publish",channel,message]的形式写入数组。注意每个元素都要转成字节形式
        todo!();
        frame
    }
}
