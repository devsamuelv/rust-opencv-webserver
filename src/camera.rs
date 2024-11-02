use std::io::Cursor;

use rocket::futures::stream::{Stream, StreamExt};
use rocket::http::ContentType;
use rocket::response::{self, stream::ReaderStream, Responder};
use rocket::{Request, Response};

pub struct CameraStream<S>(S);

impl<'r, S: Stream<Item = Vec<u8>>> Responder<'r, 'r> for CameraStream<S>
where
    S: Send + 'r,
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        Response::build()
            .header(ContentType::JPEG)
            .streamed_body(ReaderStream::from(self.0.map(Cursor::new)))
            .ok()
    }
}
