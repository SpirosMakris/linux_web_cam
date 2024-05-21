use std::{
    convert::AsRef,
    fs::{File, OpenOptions},
    marker::PhantomData,
    mem::MaybeUninit,
    os::fd::AsRawFd,
    path::Path,
};

// Generated with `resolve_ioctl.c``
const VIDIOC_QUERYCAP: u64 = 2154321408;
const VIDIOC_G_FMT: u64 = 3234878980;
const _VIDIOC_S_FMT: u64 = 3234878981;

const V4L2_PIX_FMT_YUYV: u32 = 1448695129;
const VIDIOC_REQBUFS: u64 = 3222558216;
const VIDIOC_QBUF: u64 = 3227014671;
const VIDIOC_DQBUF: u64 = 3227014673;
const VIDIOC_STREAMON: u64 = 1074026002;

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/v4l2-bindings.rs"));
}

// For variadic function ioctl
macro_rules! ioctl {
  ($fd: expr, $num: expr, $($args:expr),+) => {
      {

          let ret = sys::ioctl($fd, $num, $($args),+);
          let ret: Result<i32, std::io::Error> = if ret == -1 {
              Err(std::io::Error::last_os_error())
          } else {
              Ok(ret)
          };

          ret
      }
  };
}

pub struct V4l2Frame<'fd> {
    fd: i32,
    buf: sys::v4l2_buffer,
    _phantom: PhantomData<&'fd ()>,
}

impl V4l2Frame<'_> {
    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.buf.m.userptr as *const u8, self.buf.bytesused as usize)
        }
    }
    // @FIXME: Get these from actual device
    pub fn width(&self) -> usize {
        640
    }
    pub fn height(&self) -> usize {
        480
    }
}

impl Drop for V4l2Frame<'_> {
    fn drop(&mut self) {
        unsafe {
            ioctl!(self.fd, VIDIOC_QBUF, &mut self.buf).unwrap();
        }
    }
}

pub struct V4l2VideoDevice {
    handle: File,
    _buffers: Vec<Vec<u8>>,
}

impl V4l2VideoDevice {
    pub fn new<P: AsRef<Path>>(device_path: &P) -> Self {
        // Open device file
        let video_handle = OpenOptions::new()
            .read(true)
            .write(true)
            .open(device_path)
            .unwrap();

        // Get device capabilities
        let fd = video_handle.as_raw_fd();

        let capabilities = unsafe {
            let mut capabilities: MaybeUninit<sys::v4l2_capability> = MaybeUninit::uninit();
            ioctl!(fd, VIDIOC_QUERYCAP, capabilities.as_mut_ptr()).unwrap();
            capabilities.assume_init()
        };

        // Assert we have correct capabilities from device
        assert!(capabilities.capabilities & sys::V4L2_CAP_VIDEO_CAPTURE != 0);
        assert!(capabilities.capabilities & sys::V4L2_CAP_STREAMING != 0);

        println!("{capabilities:?}");

        // Get format v4l2 wants to give us
        let format = unsafe {
            let mut format: sys::v4l2_format = std::mem::zeroed();
            format.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            ioctl!(fd, VIDIOC_G_FMT, &mut format).unwrap();
            format
        };

        unsafe {
            println!("image size: {:?}", format.fmt.pix.sizeimage);
            println!("width: {:?}", format.fmt.pix.width);
            println!("height: {:?}", format.fmt.pix.height);
            println!("pixelformat: {:?}", format.fmt.pix.pixelformat);
            println!("field: {:?}", format.fmt.pix.field);

            assert!(format.fmt.pix.pixelformat == V4L2_PIX_FMT_YUYV);
            assert!(format.fmt.pix.field == sys::v4l2_field_V4L2_FIELD_NONE);
        }

        // Init the buffers, user ptr, etc (init_userp)
        let image_size = unsafe { format.fmt.pix.sizeimage };

        const NUM_BUFFERS: u32 = 4;
        // @TODO @FIXME Unsafe cell around each buf?
        let mut buffers = Vec::new();

        unsafe {
            for _ in 0..NUM_BUFFERS {
                buffers.push(vec![0u8; image_size.try_into().unwrap()]);
            }

            let mut bufreq: sys::v4l2_requestbuffers = std::mem::zeroed();
            bufreq.count = buffers.len().try_into().unwrap();
            bufreq.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            bufreq.memory = sys::v4l2_memory_V4L2_MEMORY_USERPTR;

            ioctl!(fd, VIDIOC_REQBUFS, &mut bufreq).unwrap();
        }

        // Prepare streaming
        (0..buffers.len()).for_each(|i| unsafe {
            let mut v4l2_buf: sys::v4l2_buffer = std::mem::zeroed();
            v4l2_buf.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            v4l2_buf.memory = sys::v4l2_memory_V4L2_MEMORY_USERPTR;
            v4l2_buf.index = i.try_into().unwrap();
            v4l2_buf.m.userptr = buffers[i].as_ptr() as u64;
            v4l2_buf.length = buffers[i].len().try_into().unwrap();

            ioctl!(fd, VIDIOC_QBUF, &mut v4l2_buf).unwrap();
        });

        // Start streaming
        let video_capture_buf_type: sys::v4l2_buf_type =
            sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        unsafe {
            ioctl!(fd, VIDIOC_STREAMON, &video_capture_buf_type).unwrap();
        }

        Self {
            handle: video_handle,
            _buffers: buffers,
        }
    }

    pub fn get_frame(&self) -> V4l2Frame<'_> {
        let fd = self.handle.as_raw_fd();
        let revents: i16 = 0;

        let mut poll_fd: [sys::pollfd; 1] = [sys::pollfd {
            fd,
            events: sys::POLLIN as i16,
            revents,
        }];

        unsafe {
            let infinite_timeout = -1;
            let _ret = sys::poll(poll_fd.as_mut_ptr(), poll_fd.len() as u64, infinite_timeout);
            // println!("{}", ret);

            let mut v4l2_buf: sys::v4l2_buffer = std::mem::zeroed();
            v4l2_buf.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            v4l2_buf.memory = sys::v4l2_memory_V4L2_MEMORY_USERPTR;

            // Read frame

            // Deque buffer. We can use them now and queue them
            // up again after we're done.
            ioctl!(fd, VIDIOC_DQBUF, &v4l2_buf).unwrap();

            V4l2Frame {
                fd,
                buf: v4l2_buf,
                _phantom: PhantomData,
            }
        }
    }
}
