use std::cell::RefCell;

use futures::future::Fuse;
use opencv::core::Vector;
use opencv::videoio::VideoCapture;
use opencv::{imgcodecs, prelude::*, videoio};

use packet::CameraStream;
use rocket::futures::stream::Stream;
use rocket::response::stream::{stream, ByteStream, TextStream};

mod packet;

#[macro_use]
extern crate rocket;

struct CameraManager {
    camera: RefCell<Option<VideoCapture>>,
}

impl CameraManager {
    // fn configure_camera<'b>(self) {
    //     if self.camera.borrow().is_none() == true {
    //         let result = Some(videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap());

    //         self.camera.replace(result);
    //     }
    // }

    pub fn new() -> CameraManager {
        let mut _cam = RefCell::new(Some(
            videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap(),
        ));

        CameraManager { camera: _cam }
    }

    fn read_camera(self, frame: Mat) {
        let cam = self.camera.borrow().unwrap();
        let opened = videoio::VideoCapture::is_opened(&cam).unwrap();

        if !opened {
            panic!("Unable to open camera");
        }

        match cam.read(&mut frame) {
            Ok(_) => (),
            Err(e) => panic!("Camera Read Error Occured! {e:?}"),
        }
    }
}

fn st() -> impl Stream<Item = Vec<u8>> {
    stream! {



        loop {
          let mut frame = Mat::default();

          let mut output_buff = Vector::<u8>::new();
          imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

          yield output_buff.to_vec();
        }
    }
}

#[get("/camera")]
fn camera_feed2() -> CameraStream<impl Stream<Item = Vec<u8>>> {
    return CameraStream::from(st());
}

#[launch]
fn root() -> _ {
    rocket::build().mount("/", routes![camera_feed2])
}
