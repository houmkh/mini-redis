//! Minimal Redis client implementation
//!
//! Provides an async connect and methods for issuing the supported commands.

use crate::cmd::{Get, Ping, Publish, Set, Subscribe, Unsubscribe};
use crate::{Connection, Frame};

use async_stream::try_stream;
use bytes::Bytes;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_stream::Stream;
use tracing::{debug, instrument};

/// 用于建立和存储客户端连接
pub struct Client {
    /// 客户端连接
    connection: Connection,
}
/// A client that has entered pub/sub mode.
///
/// 订阅者类型，在客户端订阅若干频道后会转变成订阅者，此时只能执行publish/subscribe相关的命令
pub struct Subscriber {
    /// 客户端
    client: Client,
    /// 所订阅的频道集合
    subscribed_channels: Vec<String>,
}

/// A message received on a subscribed channel.
#[derive(Debug, Clone)]
pub struct Message {
    pub channel: String,
    pub content: Bytes,
}

impl Client {
    /// 建立连接
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
        // 与addr建立tcp连接并获取套接字
        todo!();
        // 使用套接字建立连接
        todo!();
        // 返回客户端
        todo!();
    }

    /// ping客户端
    #[instrument(skip(self))]
    pub async fn ping(&mut self, msg: Option<Bytes>) -> crate::Result<Bytes> {
        // 构建ping消息

        // 日志记录ping

        // 发送ping消息

        // 读取响应，对Frame::Simple、Frame::Bulk、error类型进行处理
        todo!();
    }
    /// 获取key对应的value
    #[instrument(skip(self))]
    pub async fn get(&mut self, key: &str) -> crate::Result<Option<Bytes>> {
        // 创建获取'key'的值的'Get'命令,并将其转换成frame格式
        todo!();
        // 日志记录frme
        todo!();

        // 将frame通过连接异步写给套接字
        todo!();

        // 异步读响应,并匹配Simple、Bulk、NUll的情况
        todo!();
    }

    /// 设置key对应的value
    #[instrument(skip(self))]
    pub async fn set(&mut self, key: &str, value: Bytes) -> crate::Result<()> {
        // 通过set_cmd异步创建过期时间为None的Set命令
        todo!();
    }

    /// 设置key对应的value，value在expiration后过期
    #[instrument(skip(self))]
    pub async fn set_expires(
        &mut self,
        key: &str,
        value: Bytes,
        expiration: Duration,
    ) -> crate::Result<()> {
        // 通过set_cmd异步创建过期时间为expiration的Set命令
        todo!();
    }

    /// Set命令的主要逻辑
    async fn set_cmd(&mut self, cmd: Set) -> crate::Result<()> {
        // 将cmd转化为frame的形式
        todo!();
        // 将frame写入连接
        todo!();
        // 读取对frame的响应，并匹配响应状态
        // 执行成功响应为"OK"
        // 其他响应均为失败
        todo!();
    }

    /// 将信息推送给指定的频道
    #[instrument(skip(self))]
    pub async fn publish(&mut self, channel: &str, message: Bytes) -> crate::Result<u64> {
        // 将Publish命令转成frame的格式
        todo!();

        // 日志记录frme
        // debug!(request = ?frame);

        // 将frame通过连接异步写给套接字
        todo!();

        // 异步读请求，并匹配请求类型
        // 如果是整数帧，则返回OK(response)
        // 否则返回错误
        todo!();
    }

    /// SUBSCRIBE的主要逻辑
    async fn subscribe_cmd(&mut self, channels: &[String]) -> crate::Result<()> {
        // 将Subscribe命令转成frame的格式
        todo!();

        // 日志记录frme
        todo!();

        // 将frame通过连接异步写给套接字
        todo!();

        // 遍历channels，服务端通过响应确认订阅每个频道
        todo!();

        Ok(())
    }

    /// 监听若干个指定的频道
    #[instrument(skip(self))]
    pub async fn subscribe(mut self, channels: Vec<String>) -> crate::Result<Subscriber> {
        // 异步调用subscribe_cmd，客户端状态将会转为subscriber
        todo!();

        // 返回Subscriber类型对象
        Ok(Subscriber {
            client: self,
            subscribed_channels: channels,
        })
    }

    /// 读取响应
    async fn read_response(&mut self) -> crate::Result<Frame> {
        // 异步读取连接的frame
        todo!();
        // 日志记录读取信息
        todo!();

        // 解析响应，判断响应的类型
        // Some(Frame::Error(msg))
        // Some(frame)
        // None：响应为None意味着服务端关闭
        todo!();
    }
}

impl Subscriber {
    /// 获取订阅的频道集合
    pub fn get_subscribed(&self) -> &[String] {
        &self.subscribed_channels
    }

    /// 获取客户端收到的下一条消息（可能需要等待）
    #[warn(unused_doc_comments)]
    pub async fn next_message(&mut self) -> crate::Result<Option<Message>> {
        match self.client.connection.read_frame().await? {
            Some(mframe) => {
                // 使用日志记录mframe
                todo!();
                /*
                 * 匹配mframe的格式
                 * a. 如果是数组就分片
                 * - frame分片后格式为[message,channel,content]
                 * - 当接收到信息时，message == 'message'成立则将信息转化为Message的形式返回
                 * - 否则返回错误
                 * b. 否则返回错误
                 */
                todo!();
            }
            None => Ok(None),
        }
    }

    /// 将收到的message转换成stream形式
    pub fn into_stream(mut self) -> impl Stream<Item = crate::Result<Message>> {
        try_stream! {
            while let Some(message) = self.next_message().await? {
                yield message;
            }
        }
    }

    /// 订阅新的频道集合
    #[instrument(skip(self))]
    pub async fn subscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        // 调用subscribe_cmd，订阅新的频道集合
        todo!();

        // 更新当前的已订阅频道
        todo!();

        Ok(())
    }

    /// 取消订阅指定的频道
    #[instrument(skip(self))]
    pub async fn unsubscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        // 创建Unsubcribe命令，并转为frame形式
        todo!();
        // 日志记录请求
        todo!();

        // 将请求写入连接
        todo!();

        // 如果channels为空，那么取消订阅所有频道
        // 否则只取消channels中指定的频道
        let num = if channels.is_empty() {
            self.subscribed_channels.len()
        } else {
            channels.len()
        };

        // 解析响应
        for _ in 0..num {
            // 读取响应
            let response = self.client.read_response().await?;
            // 判断响应类型
            match response {
                Frame::Array(ref frame) => match frame.as_slice() {
                    // frame分片后格式为['unsubscribe',channel,..]
                    // 判断是否取消订阅
                    [unsubscribe, channel, ..] if *unsubscribe == "unsubscribe" => {
                        // 获取当前订阅数组的长度
                        todo!();
                        // 长度为0，返回错误
                        todo!();
                        // 当channel存在于subscribed_channels时将其删除
                        todo!();

                        // 删除数大于1则返回错误
                        todo!();
                    }
                    _ => return Err(response.to_error()),
                },
                frame => return Err(frame.to_error()),
            };
        }

        Ok(())
    }
}
