use std::{fs::OpenOptions, mem::MaybeUninit, os::fd::AsRawFd};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

// Generated with `resolve_ioctl.c``
const VIDIOC_QUERYCAP: u64 = 2154321408;

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

    println!("{capabilities:?}");
}
