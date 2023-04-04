use async_trait::async_trait;
use axum_core::extract::FromRequest;
use bytes::{Buf, BufMut, Bytes};
use h2::RecvStream;
use hyper::{Body, Request, Response};

fn test<T>(t: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", t);
}

// Generic + trait bound 포함하여 제한적인 Generic 제공
type Try<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[async_trait]
pub trait BodyExt {
    type ResType;
    async fn generic_recv(self) -> Try<Self::ResType>;
}

#[async_trait]
impl BodyExt for Request<Body> {
    type ResType = Bytes;
    async fn generic_recv(self) -> Try<Self::ResType> {
        Ok(Bytes::from_request(self, &()).await?)
    }
}

#[async_trait]
impl BodyExt for Response<RecvStream> {
    type ResType = Vec<u8>;

    async fn generic_recv(self) -> Try<Self::ResType> {
        let mut m_self = self;
        let body = m_self.body_mut();
        let mut flow_control = body.flow_control().clone();
        // 2MB 수신.. body, chunk .. end_of_stream 까지 tarilers는 안받음..
        const DEFAULT_LIMIT: usize = 2_097_152; // 2 mb
        let mut used_size: usize = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(DEFAULT_LIMIT);
        while let Some(raw) = body.data().await {
            let data = raw?;
            println!("Receive Body = {:?}", data);
            used_size += data.remaining();
            if used_size >= DEFAULT_LIMIT {
                println!(
                    "receive Buffer need more size. current max({}). next stream data is droped.",
                    DEFAULT_LIMIT
                );
                break;
            }
            let _ = flow_control.release_capacity(data.len());
            buf.put(Bytes::from(data));
        }

        Ok(buf)
    }
}

fn main() {
    let a = Bytes::from("test bytes");
    test(a);
    let b = String::from("test string");
    test(b);
}
