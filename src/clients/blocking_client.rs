//! Minimal blocking Redis client implementation
//!
//! Provides a blocking connect and methods for issuing the supported commands.

use bytes::Bytes;
use std::time::Duration;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

pub use crate::clients::Message;

/// Established connection with a Redis server.
pub struct BlockingClient {
    /// 异步的客户端
    inner: crate::clients::Client,
    /// 运行时环境，负责调度和管理异步任务的执行
    rt: Runtime,
}

/// A client that has entered pub/sub mode.
pub struct BlockingSubscriber {
    /// 异步的订阅者
    inner: crate::clients::Subscriber,
    /// 运行时环境，负责调度和管理异步任务的执行
    rt: Runtime,
}
/// The iterator returned by `Subscriber::into_iter`./// BlockingSubscriber的迭代器
/// 通过`Subscriber::into_iter`可以获取
struct SubscriberIterator {
    /// BlockingSubscriber中的Subscriber
    inner: crate::clients::Subscriber,

    /// BlockingSubscriber中的Runtime
    rt: Runtime,
}
impl BlockingClient {
    /// 建立连接
    pub fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<BlockingClient> {
        // 创建一个基于当前线程的运行时环境
        todo!();
        // 异步创建链接
        todo!();
        // 返回客户端
        todo!();
    }
    /// 异步获取指定值
    pub fn get(&mut self, key: &str) -> crate::Result<Option<Bytes>> {
        // 异步调用get获取key对应的值
        todo!();
    }

    /// 异步给key赋值
    pub fn set(&mut self, key: &str, value: Bytes) -> crate::Result<()> {
        // 异步调用set给key赋值
        todo!();
    }

    /// 异步给key赋值，并指定过期时间
    pub fn set_expires(
        &mut self,
        key: &str,
        value: Bytes,
        expiration: Duration,
    ) -> crate::Result<()> {
        // 异步调用set_expires给key赋值
        todo!();
    }

    /// 异步推送消息
    pub fn publish(&mut self, channel: &str, message: Bytes) -> crate::Result<u64> {
        // 异步调用publish给指定channel推送消息
        todo!();
    }

    /// 异步执行订阅指定频道操作，并将Client转换为BlockingSubcriber状态
    pub fn subscribe(self, channels: Vec<String>) -> crate::Result<BlockingSubscriber> {
        // 异步调用subscribe函数，转换client状态
        todo!();
        // 返回BlockingSubscriber
        todo!();
    }
}

impl BlockingSubscriber {
    /// 获取订阅的channel集合
    pub fn get_subscribed(&self) -> &[String] {
        // 获取订阅的channel集合，并返回
        todo!();
    }

    /// 异步接收下一条信息
    pub fn next_message(&mut self) -> crate::Result<Option<Message>> {
        // 异步执行获取下一条信息的操作
        todo!();
    }

    /// 异步订阅新的频道集合
    pub fn subscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        // 异步订阅新的频道集合
        todo!();
    }

    /// 异步执行取消订阅指定频道集操作
    pub fn unsubscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        // 异步执行取消订阅指定频道集操作
        todo!();
    }

    /// 获取BlockingSubscriber的迭代器
    pub fn into_iter(self) -> impl Iterator<Item = crate::Result<Message>> {
        SubscriberIterator {
            inner: self.inner,
            rt: self.rt,
        }
    }
}

impl Iterator for SubscriberIterator {
    /// 定义迭代器的元素类型
    type Item = crate::Result<Message>;

    /// 获取迭代器的下一个元素
    ///
    /// 返回Some(Ok(message)) 表示存在下一个元素
    ///
    /// 返回Some(Err(error)) 表示获取下一个元素出错
    ///
    /// 返回None 表示没有下一个元素
    fn next(&mut self) -> Option<crate::Result<Message>> {
        self.rt.block_on(self.inner.next_message()).transpose()
    }
}
