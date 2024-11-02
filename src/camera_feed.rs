use rocket::futures::stream::Stream;
use rocket::response::stream::{stream, ReaderStream};

use opencv::core::Vector;
use opencv::{imgcodecs, prelude::*, videoio};
use rocket::http::ContentType;

#[macro_use]
extern crate rocket;

// Working http endpoint that publishes camera data
#[get("/camera2")]
async fn camera_feed2<'a>() -> (ContentType, Vec<u8>) {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
    let opened = videoio::set_chunked_content_providerVideoCapture::is_opened(&cam).unwrap();
    if !opened {
        panic!("Unable to open camera");
    }

    let mut frame = Mat::default();
    cam.read(&mut frame);
    let mut output_buff = Vector::<u8>::new();
    imgcodecs::imencode_def(".jpg", &frame, &mut output_buff).unwrap();

    return (ContentType::JPEG, output_buff.to_vec());
}
