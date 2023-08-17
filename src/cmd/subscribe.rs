use crate::cmd::{Parse, ParseError, Unknown};
use crate::{Command, Connection, Db, Frame, Shutdown};

use bytes::Bytes;
use std::pin::Pin;
use tokio::select;
use tokio::sync::broadcast;
use tokio_stream::{Stream, StreamExt, StreamMap};

/// 订阅命令
///
/// 一旦客户端进入订阅状态，它只能提出
/// SUBSCRIBE, PSUBSCRIBE, UNSUBSCRIBE,
/// PUNSUBSCRIBE, PING 和 QUIT命令
#[derive(Debug)]
pub struct Subscribe {
    channels: Vec<String>,
}

/// Unsubscribes the client from one or more channels.
///
/// When no channels are specified, the client is unsubscribed from all the
/// previously subscribed channels.
#[derive(Clone, Debug)]
pub struct Unsubscribe {
    channels: Vec<String>,
}

/// 信息流
type Messages = Pin<Box<dyn Stream<Item = Bytes> + Send>>;

impl Subscribe {
    /// 创建一个新的订阅者以监听频道
    pub(crate) fn new(channels: &[String]) -> Subscribe {
        todo!();
    }
    /// 解析subscribe帧
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Subscribe> {
        use ParseError::EndOfStream;
        // 获取第一个channel，并判断是否为none，不为none则写入数组
        todo!();
        // 循环解析剩下的channel，并将其写入数组
        todo!();
        // 返回结果
        todo!();
    }

    /// 数据库执行subscribe命令
    pub(crate) async fn apply(
        mut self,
        db: &Db,
        dst: &mut Connection,
        shutdown: &mut Shutdown,
    ) -> crate::Result<()> {
        // TODO 创建一个 StreamMap 对象,用于跟踪活动的订阅。使用map可以防止订阅重复"
        todo!();

        //loop {
        // TODO 遍历channels，drain(..)会将管道从channels中移除
        // subscribe_to_channel会订阅传入的管道
        // await等待管道订阅成功

        // TODO 使用 select! 宏等待以下几种情况中的任何一种发生：
        // 1. 从已订阅的通道接收到消息。
        // 2. 从客户端接收到订阅或取消订阅命令。
        // 3. 收到服务器关闭信号。
        // select! {

        // TODO 1. 接收来自已订阅通道的消息。
        // subscriptions.next() 返回一个异步迭代器
        // 当有新的消息到达时，会返回 Some((channel_name, msg)),并将将收到的消息构造成消息帧写给客户端
        // 其中 channel_name 是通道名称，msg 是消息内容。

        // TODO 2. dst.read_frame() 返回一个异步操作结果，
        //当有新的帧可用时，会返回 Some(frame)，其中 frame 是读取到的帧。如果客户端断开连接，会返回 None。
        // 读取到帧后，处理从客户端接收到的命令帧

        // TODO 3. 接收服务器关闭信号。

        // };
        //}
    }

    /// 将命令转为frame类型
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        // 将命令以["subscribe",channel1,channel2...]的形
        //式放入数组
        todo!();
        frame
    }
}

async fn subscribe_to_channel(
    channel_name: String,
    subscriptions: &mut StreamMap<String, Messages>,
    db: &Db,
    dst: &mut Connection,
) -> crate::Result<()> {
    // 向数据发送订阅请求
    let mut rx = db.subscribe(channel_name.clone());

    // 接受订阅回复
    let rx = Box::pin(async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(msg) => yield msg,
                // If we lagged in consuming messages, just resume.
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(_) => break,
            }
        }
    });

    // 将订阅成功的管道加入到订阅map中
    subscriptions.insert(channel_name.clone(), rx);

    // 响应成功订阅报文
    let response = make_subscribe_frame(channel_name, subscriptions.len());
    dst.write_frame(&response).await?;

    Ok(())
}

/// Handle a command received while inside `Subscribe::apply`. Only subscribe
/// and unsubscribe commands are permitted in this context.
///
/// Any new subscriptions are appended to `subscribe_to` instead of modifying
/// `subscriptions`.
async fn handle_command(
    frame: Frame,
    subscribe_to: &mut Vec<String>,
    subscriptions: &mut StreamMap<String, Messages>,
    dst: &mut Connection,
) -> crate::Result<()> {
    // match Command::from_frame(frame)? {
    // 分别处理Subscribe和Unsubscribe
    /*
        Subscribe：将要订阅的频道加入到订阅map

        Unsubscribe:
        1. 判断是否指定了要取消订阅的频道,如果没有则需要订阅所有channels
        2. 将需要取消订阅的频道从map中移除,同时需要发送unsubscribe响应

        其余命令以unknown命令处理
    */
    // }
    todo!();
}

/// Creates the response to a subcribe request.
///
/// All of these functions take the `channel_name` as a `String` instead of
/// a `&str` since `Bytes::from` can reuse the allocation in the `String`, and
/// taking a `&str` would require copying the data. This allows the caller to
/// decide whether to clone the channel name or not.
fn make_subscribe_frame(channel_name: String, num_subs: usize) -> Frame {
    let mut response = Frame::array();
    response.push_bulk(Bytes::from_static(b"subscribe"));
    response.push_bulk(Bytes::from(channel_name));
    response.push_int(num_subs as u64);
    response
}

/// Creates the response to an unsubcribe request.
fn make_unsubscribe_frame(channel_name: String, num_subs: usize) -> Frame {
    let mut response = Frame::array();
    response.push_bulk(Bytes::from_static(b"unsubscribe"));
    response.push_bulk(Bytes::from(channel_name));
    response.push_int(num_subs as u64);
    response
}

/// Creates a message informing the client about a new message on a channel that
/// the client subscribes to.
fn make_message_frame(channel_name: String, msg: Bytes) -> Frame {
    let mut response = Frame::array();
    response.push_bulk(Bytes::from_static(b"message"));
    response.push_bulk(Bytes::from(channel_name));
    response.push_bulk(msg);
    response
}

impl Unsubscribe {
    pub(crate) fn new(channels: &[String]) -> Unsubscribe {
        Unsubscribe {
            channels: channels.to_vec(),
        }
    }
    /// 解析unsubscribe帧
    pub(crate) fn parse_frames(parse: &mut Parse) -> Result<Unsubscribe, ParseError> {
        use ParseError::EndOfStream;

        // 创建数组，用于存储channels
        todo!();

        // 循环解析数据，将channel放入数组

        // 返回命令
    }
    /// 将unsubscribe命令转成帧
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        // 将命令以["unsubscribe",channel1,channel2...]形
        // 式放入数组
        todo!();
        frame
    }
}
