use futures_core::Stream;
use rocket::http::ContentType;
use rocket::response::stream::ReaderStream;
use rocket::response::{self, Responder};
use rocket::{Request, Response};

#[derive(Debug, Clone)]
pub struct CameraStream<S>(pub S);

impl<S> From<S> for CameraStream<S> {
    /// Creates a `ByteStream` from any `S: Stream`.
    fn from(stream: S) -> Self {
        CameraStream(stream)
    }
}

impl<'r, S: Stream> Responder<'r, 'r> for CameraStream<S>
where
    S: Send + 'r,
    S::Item: AsRef<[u8]> + Send + Unpin + 'r,
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        Response::build()
            .header(ContentType::Binary)
            .streamed_body(ReaderStream::from(self.0.map(std::io::Cursor::new)))
            .ok()
    }
}
