use std::{fs::OpenOptions, mem::MaybeUninit, os::fd::AsRawFd};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

// Generated with `resolve_ioctl.c``
const VIDIOC_QUERYCAP: u64 = 2154321408;
const VIDIOC_G_FMT: u64 = 3234878980;

const V4L2_PIX_FMT_YUYV: u32 = 1448695129;

// For variadic function ioctl
macro_rules! ioctl {
    ($fd: expr, $num: expr, $($args:expr),+) => {
        {

            let ret = v4l2::ioctl($fd, $num, $($args),+);
            let ret: Result<i32, std::io::Error> = if ret == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(ret)
            };

            ret
        }
    };
}

fn main() {
    // Open device file
    let video_handle = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DEVICE_NAME)
        .unwrap();

    // Get device capabilities
    let fd = video_handle.as_raw_fd();

    let capabilities = unsafe {
        let mut capabilities: MaybeUninit<v4l2::v4l2_capability> = MaybeUninit::uninit();
        ioctl!(fd, VIDIOC_QUERYCAP, capabilities.as_mut_ptr()).unwrap();
        capabilities.assume_init()
    };

    // Assert we have correct capabilities from device
    assert!(capabilities.capabilities & v4l2::V4L2_CAP_VIDEO_CAPTURE != 0);
    assert!(capabilities.capabilities & v4l2::V4L2_CAP_STREAMING != 0);

    println!("{capabilities:?}");

    // Get format v4l2 wants to give us
    let format = unsafe {
        let mut format: v4l2::v4l2_format = std::mem::zeroed();
        format.type_ = v4l2::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        ioctl!(fd, VIDIOC_G_FMT, &mut format).unwrap();
        format
    };

    unsafe {
        println!("image size: {:?}", format.fmt.pix.sizeimage);
        println!("width: {:?}", format.fmt.pix.width);
        println!("height: {:?}", format.fmt.pix.height);
        println!("pixelformat: {:?}", format.fmt.pix.pixelformat);

        assert!(format.fmt.pix.pixelformat == V4L2_PIX_FMT_YUYV);
    }

    // Init the userptr
}
