use std::task::Poll;

use packet::CameraStream;
use rocket::response::stream::ReaderStream;
use rocket::response::Responder;

use opencv::core::Vector;
use opencv::{imgcodecs, prelude::*, videoio};
use tokio::io::AsyncRead;

mod packet;

#[macro_use]
extern crate rocket;

// // Working http endpoint that publishes camera data
// #[get("/camera2")]
// async fn camera_feed2<'a>() -> (ContentType, Vec<u8>) {
//     let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
//     let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
//     if !opened {
//         panic!("Unable to open camera");
//     }

//     let mut frame = Mat::default();
//     let _ = cam.read(&mut frame);
//     let mut output_buff = Vector::<u8>::new();
//     imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

//     return (ContentType::JPEG, output_buff.to_vec());
// }

#[derive(Responder)]
#[response(content_type = "image/jpeg")]
struct CameraPacket(Vec<u8>);

impl AsyncRead for CameraPacket {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let image_data = &self.0;

        if !image_data.is_empty() {
            buf.put_slice(&self.0);

            return Poll::Ready(Ok(()));
        } else {
            return Poll::Pending;
        }
    }
}

#[get("/camera")]
async fn camera_feed<'a>() -> ReaderStream![CameraPacket] {
    ReaderStream! {
      let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
      let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
      if !opened {
        panic!("Unable to open camera");
      }

      loop {
        let mut frame = Mat::default();
        match cam.read(&mut frame) {
          Ok(_) => (),
          Err(e) => panic!("Camera Read Error Occured! {e:?}"),
        }
        let mut output_buff = Vector::<u8>::new();
        imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

        yield CameraPacket(output_buff.to_vec());
      }
    }
}

#[get("/camera2")]
fn camera_feed2() -> CameraStream![Vec<u8>] {
    CameraStream! {
      let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
      let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
      if !opened {
        panic!("Unable to open camera");
      }

      loop {
        let mut frame = Mat::default();
        match cam.read(&mut frame) {
          Ok(_) => (),
          Err(e) => panic!("Camera Read Error Occured! {e:?}"),
        }
        let mut output_buff = Vector::<u8>::new();
        imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

        yield output_buff.to_vec();
      }
    }
}

#[launch]
fn root() -> _ {
    rocket::build().mount("/", routes![camera_feed, camera_feed2])
}
