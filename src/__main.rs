use rocket::futures::io::Cursor;
use rocket::response::stream::ReaderStream;

use opencv::core::Vector;
use opencv::{imgcodecs, prelude::*, videoio};
use rocket::response::{self, Responder};
use rocket::{Error, Response};

#[macro_use]
extern crate rocket;

// // https://rocket.rs/guide/v0.5/responses/#async-streams
// #[get("/camera")]
// fn camera_feed<'a, 'b>() -> ReaderStream![&'a [u8]] {
//     ReaderStream! {
//       let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
//       let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
//       if !opened {
//           panic!("Unable to open camera");
//       }

//       let mut frame = Mat::default();
//       let mut output_buff: &'a Vector<&'b u8> = &'a Vector::<&'b u8>::new();

//       loop {
//         cam.read(&mut frame);
//         imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();
//         let e: &'a [u8] = &mut output_buff.as_slice();

//         yield e;
//       }
//     }
// }

// #[get("/")]
// fn index() -> &'static str {
//     return "hello";
// }

// #[get("/camera1")]
// async fn camera_feed1<'a>() -> (ContentType, Vec<u8>) {
//     let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
//     let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
//     if !opened {
//         panic!("Unable to open camera");
//     }

//     let mut frame = Mat::default();
//     match cam.read(&mut frame) {
//         Ok(_) => (),
//         Err(e) => panic!("Camera Read Error Occured! {e:?}"),
//     }
//     let mut output_buff = Vector::<u8>::new();
//     imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

//     return (ContentType::JPEG, output_buff.to_vec());
// }

#[derive(Responder)]
#[response(content_type = "image/jpeg")]
struct CameraPacket(Vec<u8>);

// https://api.rocket.rs/v0.4/rocket/response/trait.Responder
impl<'r> Responder<'r> for CameraPacket {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'o> {
        Response::build().streamed_body(Cursor::from(self.0));
    }
}

#[get("/camera2")]
async fn camera_feed2<'a>() -> ReaderStream![CameraPacket] {
    ReaderStream! {
      let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
      let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
      if !opened {
        panic!("Unable to open camera");
      }

      while true {
        let mut frame = Mat::default();
        match cam.read(&mut frame) {
          Ok(_) => (),
          Err(e) => panic!("Camera Read Error Occured! {e:?}"),
        }
        let mut output_buff = Vector::<u8>::new();
        imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

        return CameraPacket(output_buff.to_vec());
      }
    }
}

#[launch]
fn root() -> _ {
    rocket::build().mount("/", routes![camera_feed2])
}

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

//     let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
//     let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
//     if !opened {
//         panic!("Unable to open camera");
//     }

//     let mut frame = Mat::default();
//     let mut output_buff: Vector<u8> = Vector::<u8>::new();

//     loop {
//         let (mut stream, _) = listener.accept().expect("Failed to accept connection");
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n"
//         );
//         stream.write_all(response.as_bytes()).unwrap();

//         loop {
//             cam.read(&mut frame).unwrap();
//             output_buff.clear();
//             imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

//             let image_data = format!(
//                 "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
//                 output_buff.len(),
//             );
//             stream.write_all(image_data.as_bytes()).unwrap();
//             stream.write_all(output_buff.as_slice()).unwrap();
//             stream.write_all(b"\r\n").unwrap();
//             stream.flush().unwrap();
//         }
//     }
// }
