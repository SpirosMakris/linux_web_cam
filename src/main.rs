use std::{fs::OpenOptions, io::Write};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

fn main() {
    let device = v4l2::V4l2VideoDevice::new(&DEVICE_NAME);

    let frame = device.get_frame();

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test_2.yuv")
        .unwrap();
    output.write_all(frame.data()).unwrap();
}
