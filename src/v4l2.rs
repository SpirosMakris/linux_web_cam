use std::{
    convert::AsRef,
    fs::{File, OpenOptions},
    marker::PhantomData,
    mem::MaybeUninit,
    os::fd::AsRawFd,
    path::Path,
};

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/v4l2-bindings.rs"));
    include!(concat!(env!("OUT_DIR"), "/v4l2_constants.rs"));
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
    width: usize,
    height: usize,
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
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Drop for V4l2Frame<'_> {
    fn drop(&mut self) {
        unsafe {
            ioctl!(self.fd, sys::VIDIOC_QBUF, &mut self.buf).unwrap();
        }
    }
}

pub struct V4l2VideoDevice {
    handle: File,
    width: usize,
    height: usize,
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
            ioctl!(fd, sys::VIDIOC_QUERYCAP, capabilities.as_mut_ptr()).unwrap();
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
            ioctl!(fd, sys::VIDIOC_G_FMT, &mut format).unwrap();
            format
        };

        unsafe {
            println!("image size: {:?}", format.fmt.pix.sizeimage);
            println!("width: {:?}", format.fmt.pix.width);
            println!("height: {:?}", format.fmt.pix.height);
            println!("pixelformat: {:?}", format.fmt.pix.pixelformat);
            println!("field: {:?}", format.fmt.pix.field);

            assert!(format.fmt.pix.pixelformat == sys::V4L2_PIX_FMT_YUYV);
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

            ioctl!(fd, sys::VIDIOC_REQBUFS, &mut bufreq).unwrap();
        }

        // Prepare streaming
        (0..buffers.len()).for_each(|i| unsafe {
            let mut v4l2_buf: sys::v4l2_buffer = std::mem::zeroed();
            v4l2_buf.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            v4l2_buf.memory = sys::v4l2_memory_V4L2_MEMORY_USERPTR;
            v4l2_buf.index = i.try_into().unwrap();
            v4l2_buf.m.userptr = buffers[i].as_ptr() as u64;
            v4l2_buf.length = buffers[i].len().try_into().unwrap();

            ioctl!(fd, sys::VIDIOC_QBUF, &mut v4l2_buf).unwrap();
        });

        // Start streaming
        let video_capture_buf_type: sys::v4l2_buf_type =
            sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        unsafe {
            ioctl!(fd, sys::VIDIOC_STREAMON, &video_capture_buf_type).unwrap();
        }

        let (width, height) = unsafe {
            (
                format.fmt.pix.width as usize,
                format.fmt.pix.height as usize,
            )
        };

        Self {
            handle: video_handle,
            width,
            height,
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
            ioctl!(fd, sys::VIDIOC_DQBUF, &v4l2_buf).unwrap();

            V4l2Frame {
                fd,
                width: self.width,
                height: self.height,
                buf: v4l2_buf,
                _phantom: PhantomData,
            }
        }
    }

    pub fn print_formats(&self) {
        let mut i: u32 = 0;
        let fd = self.handle.as_raw_fd();
        loop {
            unsafe {
                let mut descr: sys::v4l2_fmtdesc = std::mem::zeroed();
                descr.index = i;
                descr.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
                let ret = ioctl!(fd, sys::VIDIOC_ENUM_FMT, &mut descr);

                if let Err(e) = ret {
                    if e.kind() == std::io::ErrorKind::InvalidInput {
                        break;
                    }

                    panic!("Unexpected error: {e}");
                };

                if descr.pixelformat != sys::V4L2_PIX_FMT_YUYV {
                    continue;
                }

                println!("{}", pixel_format_to_string(descr.pixelformat));
            }

            i += 1;
        }
    }
}

fn pixel_format_to_string(format: u32) -> &'static str {
    match format {
        sys::V4L2_PIX_FMT_RGB332 => "V4L2_PIX_FMT_RGB332",
        sys::V4L2_PIX_FMT_RGB444 => "V4L2_PIX_FMT_RGB444",
        sys::V4L2_PIX_FMT_ARGB444 => "V4L2_PIX_FMT_ARGB444",
        sys::V4L2_PIX_FMT_XRGB444 => "V4L2_PIX_FMT_XRGB444",
        sys::V4L2_PIX_FMT_RGBA444 => "V4L2_PIX_FMT_RGBA444",
        sys::V4L2_PIX_FMT_RGBX444 => "V4L2_PIX_FMT_RGBX444",
        sys::V4L2_PIX_FMT_ABGR444 => "V4L2_PIX_FMT_ABGR444",
        sys::V4L2_PIX_FMT_XBGR444 => "V4L2_PIX_FMT_XBGR444",
        sys::V4L2_PIX_FMT_BGRA444 => "V4L2_PIX_FMT_BGRA444",
        sys::V4L2_PIX_FMT_BGRX444 => "V4L2_PIX_FMT_BGRX444",
        sys::V4L2_PIX_FMT_RGB555 => "V4L2_PIX_FMT_RGB555",
        sys::V4L2_PIX_FMT_ARGB555 => "V4L2_PIX_FMT_ARGB555",
        sys::V4L2_PIX_FMT_XRGB555 => "V4L2_PIX_FMT_XRGB555",
        sys::V4L2_PIX_FMT_RGBA555 => "V4L2_PIX_FMT_RGBA555",
        sys::V4L2_PIX_FMT_RGBX555 => "V4L2_PIX_FMT_RGBX555",
        sys::V4L2_PIX_FMT_ABGR555 => "V4L2_PIX_FMT_ABGR555",
        sys::V4L2_PIX_FMT_XBGR555 => "V4L2_PIX_FMT_XBGR555",
        sys::V4L2_PIX_FMT_BGRA555 => "V4L2_PIX_FMT_BGRA555",
        sys::V4L2_PIX_FMT_BGRX555 => "V4L2_PIX_FMT_BGRX555",
        sys::V4L2_PIX_FMT_RGB565 => "V4L2_PIX_FMT_RGB565",
        sys::V4L2_PIX_FMT_RGB555X => "V4L2_PIX_FMT_RGB555X",
        sys::V4L2_PIX_FMT_ARGB555X => "V4L2_PIX_FMT_ARGB555X",
        sys::V4L2_PIX_FMT_XRGB555X => "V4L2_PIX_FMT_XRGB555X",
        sys::V4L2_PIX_FMT_RGB565X => "V4L2_PIX_FMT_RGB565X",

        /* RGB formats (3 or 4 bytes per pixel) */
        sys::V4L2_PIX_FMT_BGR666 => "V4L2_PIX_FMT_BGR666",
        sys::V4L2_PIX_FMT_BGR24 => "V4L2_PIX_FMT_BGR24",
        sys::V4L2_PIX_FMT_RGB24 => "V4L2_PIX_FMT_RGB24",
        sys::V4L2_PIX_FMT_BGR32 => "V4L2_PIX_FMT_BGR32",
        sys::V4L2_PIX_FMT_ABGR32 => "V4L2_PIX_FMT_ABGR32",
        sys::V4L2_PIX_FMT_XBGR32 => "V4L2_PIX_FMT_XBGR32",
        sys::V4L2_PIX_FMT_BGRA32 => "V4L2_PIX_FMT_BGRA32",
        sys::V4L2_PIX_FMT_BGRX32 => "V4L2_PIX_FMT_BGRX32",
        sys::V4L2_PIX_FMT_RGB32 => "V4L2_PIX_FMT_RGB32",
        sys::V4L2_PIX_FMT_RGBA32 => "V4L2_PIX_FMT_RGBA32",
        sys::V4L2_PIX_FMT_RGBX32 => "V4L2_PIX_FMT_RGBX32",
        sys::V4L2_PIX_FMT_ARGB32 => "V4L2_PIX_FMT_ARGB32",
        sys::V4L2_PIX_FMT_XRGB32 => "V4L2_PIX_FMT_XRGB32",
        sys::V4L2_PIX_FMT_RGBX1010102 => "V4L2_PIX_FMT_RGBX1010102",
        sys::V4L2_PIX_FMT_RGBA1010102 => "V4L2_PIX_FMT_RGBA1010102",
        sys::V4L2_PIX_FMT_ARGB2101010 => "V4L2_PIX_FMT_ARGB2101010",

        /* RGB formats (6 or 8 bytes per pixel) */
        sys::V4L2_PIX_FMT_BGR48_12 => "V4L2_PIX_FMT_BGR48_12",
        sys::V4L2_PIX_FMT_ABGR64_12 => "V4L2_PIX_FMT_ABGR64_12",

        /* Grey formats */
        sys::V4L2_PIX_FMT_GREY => "V4L2_PIX_FMT_GREY",
        sys::V4L2_PIX_FMT_Y4 => "V4L2_PIX_FMT_Y4",
        sys::V4L2_PIX_FMT_Y6 => "V4L2_PIX_FMT_Y6",
        sys::V4L2_PIX_FMT_Y10 => "V4L2_PIX_FMT_Y10",
        sys::V4L2_PIX_FMT_Y12 => "V4L2_PIX_FMT_Y12",
        sys::V4L2_PIX_FMT_Y012 => "V4L2_PIX_FMT_Y012",
        sys::V4L2_PIX_FMT_Y14 => "V4L2_PIX_FMT_Y14",
        sys::V4L2_PIX_FMT_Y16 => "V4L2_PIX_FMT_Y16",
        sys::V4L2_PIX_FMT_Y16_BE => "V4L2_PIX_FMT_Y16_BE",

        /* Grey bit-packed formats */
        sys::V4L2_PIX_FMT_Y10BPACK => "V4L2_PIX_FMT_Y10BPACK",
        sys::V4L2_PIX_FMT_Y10P => "V4L2_PIX_FMT_Y10P",
        sys::V4L2_PIX_FMT_IPU3_Y10 => "V4L2_PIX_FMT_IPU3_Y10",

        /* Palette formats */
        sys::V4L2_PIX_FMT_PAL8 => "V4L2_PIX_FMT_PAL8",

        /* Chrominance formats */
        sys::V4L2_PIX_FMT_UV8 => "V4L2_PIX_FMT_UV8",

        /* Luminance+Chrominance formats */
        sys::V4L2_PIX_FMT_YUYV => "V4L2_PIX_FMT_YUYV",
        sys::V4L2_PIX_FMT_YYUV => "V4L2_PIX_FMT_YYUV",
        sys::V4L2_PIX_FMT_YVYU => "V4L2_PIX_FMT_YVYU",
        sys::V4L2_PIX_FMT_UYVY => "V4L2_PIX_FMT_UYVY",
        sys::V4L2_PIX_FMT_VYUY => "V4L2_PIX_FMT_VYUY",
        sys::V4L2_PIX_FMT_Y41P => "V4L2_PIX_FMT_Y41P",
        sys::V4L2_PIX_FMT_YUV444 => "V4L2_PIX_FMT_YUV444",
        sys::V4L2_PIX_FMT_YUV555 => "V4L2_PIX_FMT_YUV555",
        sys::V4L2_PIX_FMT_YUV565 => "V4L2_PIX_FMT_YUV565",
        sys::V4L2_PIX_FMT_YUV24 => "V4L2_PIX_FMT_YUV24",
        sys::V4L2_PIX_FMT_YUV32 => "V4L2_PIX_FMT_YUV32",
        sys::V4L2_PIX_FMT_AYUV32 => "V4L2_PIX_FMT_AYUV32",
        sys::V4L2_PIX_FMT_XYUV32 => "V4L2_PIX_FMT_XYUV32",
        sys::V4L2_PIX_FMT_VUYA32 => "V4L2_PIX_FMT_VUYA32",
        sys::V4L2_PIX_FMT_VUYX32 => "V4L2_PIX_FMT_VUYX32",
        sys::V4L2_PIX_FMT_YUVA32 => "V4L2_PIX_FMT_YUVA32",
        sys::V4L2_PIX_FMT_YUVX32 => "V4L2_PIX_FMT_YUVX32",
        sys::V4L2_PIX_FMT_M420 => "V4L2_PIX_FMT_M420",
        sys::V4L2_PIX_FMT_YUV48_12 => "V4L2_PIX_FMT_YUV48_12",

        /*
         * YCbCr packed format. For each Y2xx format, xx bits of valid data occupy the MSBs
         * of the 16 bit components, and 16-xx bits of zero padding occupy the LSBs.
         */
        sys::V4L2_PIX_FMT_Y210 => "V4L2_PIX_FMT_Y210",
        sys::V4L2_PIX_FMT_Y212 => "V4L2_PIX_FMT_Y212",
        sys::V4L2_PIX_FMT_Y216 => "V4L2_PIX_FMT_Y216",

        /* two planes -- one Y, one Cr + Cb interleaved  */
        sys::V4L2_PIX_FMT_NV12 => "V4L2_PIX_FMT_NV12",
        sys::V4L2_PIX_FMT_NV21 => "V4L2_PIX_FMT_NV21",
        sys::V4L2_PIX_FMT_NV16 => "V4L2_PIX_FMT_NV16",
        sys::V4L2_PIX_FMT_NV61 => "V4L2_PIX_FMT_NV61",
        sys::V4L2_PIX_FMT_NV24 => "V4L2_PIX_FMT_NV24",
        sys::V4L2_PIX_FMT_NV42 => "V4L2_PIX_FMT_NV42",
        sys::V4L2_PIX_FMT_P010 => "V4L2_PIX_FMT_P010",
        sys::V4L2_PIX_FMT_P012 => "V4L2_PIX_FMT_P012",

        /* two non contiguous planes - one Y, one Cr + Cb interleaved  */
        sys::V4L2_PIX_FMT_NV12M => "V4L2_PIX_FMT_NV12M",
        sys::V4L2_PIX_FMT_NV21M => "V4L2_PIX_FMT_NV21M",
        sys::V4L2_PIX_FMT_NV16M => "V4L2_PIX_FMT_NV16M",
        sys::V4L2_PIX_FMT_NV61M => "V4L2_PIX_FMT_NV61M",
        sys::V4L2_PIX_FMT_P012M => "V4L2_PIX_FMT_P012M",

        /* three planes - Y Cb, Cr */
        sys::V4L2_PIX_FMT_YUV410 => "V4L2_PIX_FMT_YUV410",
        sys::V4L2_PIX_FMT_YVU410 => "V4L2_PIX_FMT_YVU410",
        sys::V4L2_PIX_FMT_YUV411P => "V4L2_PIX_FMT_YUV411P",
        sys::V4L2_PIX_FMT_YUV420 => "V4L2_PIX_FMT_YUV420",
        sys::V4L2_PIX_FMT_YVU420 => "V4L2_PIX_FMT_YVU420",
        sys::V4L2_PIX_FMT_YUV422P => "V4L2_PIX_FMT_YUV422P",

        /* three non contiguous planes - Y, Cb, Cr */
        sys::V4L2_PIX_FMT_YUV420M => "V4L2_PIX_FMT_YUV420M",
        sys::V4L2_PIX_FMT_YVU420M => "V4L2_PIX_FMT_YVU420M",
        sys::V4L2_PIX_FMT_YUV422M => "V4L2_PIX_FMT_YUV422M",
        sys::V4L2_PIX_FMT_YVU422M => "V4L2_PIX_FMT_YVU422M",
        sys::V4L2_PIX_FMT_YUV444M => "V4L2_PIX_FMT_YUV444M",
        sys::V4L2_PIX_FMT_YVU444M => "V4L2_PIX_FMT_YVU444M",

        /* Tiled YUV formats */
        sys::V4L2_PIX_FMT_NV12_4L4 => "V4L2_PIX_FMT_NV12_4L4",
        sys::V4L2_PIX_FMT_NV12_16L16 => "V4L2_PIX_FMT_NV12_16L16",
        sys::V4L2_PIX_FMT_NV12_32L32 => "V4L2_PIX_FMT_NV12_32L32",
        sys::V4L2_PIX_FMT_NV15_4L4 => "V4L2_PIX_FMT_NV15_4L4",
        sys::V4L2_PIX_FMT_P010_4L4 => "V4L2_PIX_FMT_P010_4L4",
        sys::V4L2_PIX_FMT_NV12_8L128 => "V4L2_PIX_FMT_NV12_8L128",
        sys::V4L2_PIX_FMT_NV12_10BE_8L128 => "V4L2_PIX_FMT_NV12_10BE_8L128",

        /* Tiled YUV formats, non contiguous planes */
        sys::V4L2_PIX_FMT_NV12MT => "V4L2_PIX_FMT_NV12MT",
        sys::V4L2_PIX_FMT_NV12MT_16X16 => "V4L2_PIX_FMT_NV12MT_16X16",
        sys::V4L2_PIX_FMT_NV12M_8L128 => "V4L2_PIX_FMT_NV12M_8L128",
        sys::V4L2_PIX_FMT_NV12M_10BE_8L128 => "V4L2_PIX_FMT_NV12M_10BE_8L128",

        /* Bayer formats - see http://www.siliconimaging.com/RGB%20Bayer.htm */
        sys::V4L2_PIX_FMT_SBGGR8 => "V4L2_PIX_FMT_SBGGR8",
        sys::V4L2_PIX_FMT_SGBRG8 => "V4L2_PIX_FMT_SGBRG8",
        sys::V4L2_PIX_FMT_SGRBG8 => "V4L2_PIX_FMT_SGRBG8",
        sys::V4L2_PIX_FMT_SRGGB8 => "V4L2_PIX_FMT_SRGGB8",
        sys::V4L2_PIX_FMT_SBGGR10 => "V4L2_PIX_FMT_SBGGR10",
        sys::V4L2_PIX_FMT_SGBRG10 => "V4L2_PIX_FMT_SGBRG10",
        sys::V4L2_PIX_FMT_SGRBG10 => "V4L2_PIX_FMT_SGRBG10",
        sys::V4L2_PIX_FMT_SRGGB10 => "V4L2_PIX_FMT_SRGGB10",
        /* 10bit raw bayer packed, 5 bytes for every 4 pixels */
        sys::V4L2_PIX_FMT_SBGGR10P => "V4L2_PIX_FMT_SBGGR10P",
        sys::V4L2_PIX_FMT_SGBRG10P => "V4L2_PIX_FMT_SGBRG10P",
        sys::V4L2_PIX_FMT_SGRBG10P => "V4L2_PIX_FMT_SGRBG10P",
        sys::V4L2_PIX_FMT_SRGGB10P => "V4L2_PIX_FMT_SRGGB10P",
        /* 10bit raw bayer a-law compressed to 8 bits */
        sys::V4L2_PIX_FMT_SBGGR10ALAW8 => "V4L2_PIX_FMT_SBGGR10ALAW8",
        sys::V4L2_PIX_FMT_SGBRG10ALAW8 => "V4L2_PIX_FMT_SGBRG10ALAW8",
        sys::V4L2_PIX_FMT_SGRBG10ALAW8 => "V4L2_PIX_FMT_SGRBG10ALAW8",
        sys::V4L2_PIX_FMT_SRGGB10ALAW8 => "V4L2_PIX_FMT_SRGGB10ALAW8",
        /* 10bit raw bayer DPCM compressed to 8 bits */
        sys::V4L2_PIX_FMT_SBGGR10DPCM8 => "V4L2_PIX_FMT_SBGGR10DPCM8",
        sys::V4L2_PIX_FMT_SGBRG10DPCM8 => "V4L2_PIX_FMT_SGBRG10DPCM8",
        sys::V4L2_PIX_FMT_SGRBG10DPCM8 => "V4L2_PIX_FMT_SGRBG10DPCM8",
        sys::V4L2_PIX_FMT_SRGGB10DPCM8 => "V4L2_PIX_FMT_SRGGB10DPCM8",
        sys::V4L2_PIX_FMT_SBGGR12 => "V4L2_PIX_FMT_SBGGR12",
        sys::V4L2_PIX_FMT_SGBRG12 => "V4L2_PIX_FMT_SGBRG12",
        sys::V4L2_PIX_FMT_SGRBG12 => "V4L2_PIX_FMT_SGRBG12",
        sys::V4L2_PIX_FMT_SRGGB12 => "V4L2_PIX_FMT_SRGGB12",
        /* 12bit raw bayer packed, 6 bytes for every 4 pixels */
        sys::V4L2_PIX_FMT_SBGGR12P => "V4L2_PIX_FMT_SBGGR12P",
        sys::V4L2_PIX_FMT_SGBRG12P => "V4L2_PIX_FMT_SGBRG12P",
        sys::V4L2_PIX_FMT_SGRBG12P => "V4L2_PIX_FMT_SGRBG12P",
        sys::V4L2_PIX_FMT_SRGGB12P => "V4L2_PIX_FMT_SRGGB12P",
        sys::V4L2_PIX_FMT_SBGGR14 => "V4L2_PIX_FMT_SBGGR14",
        sys::V4L2_PIX_FMT_SGBRG14 => "V4L2_PIX_FMT_SGBRG14",
        sys::V4L2_PIX_FMT_SGRBG14 => "V4L2_PIX_FMT_SGRBG14",
        sys::V4L2_PIX_FMT_SRGGB14 => "V4L2_PIX_FMT_SRGGB14",
        /* 14bit raw bayer packed, 7 bytes for every 4 pixels */
        sys::V4L2_PIX_FMT_SBGGR14P => "V4L2_PIX_FMT_SBGGR14P",
        sys::V4L2_PIX_FMT_SGBRG14P => "V4L2_PIX_FMT_SGBRG14P",
        sys::V4L2_PIX_FMT_SGRBG14P => "V4L2_PIX_FMT_SGRBG14P",
        sys::V4L2_PIX_FMT_SRGGB14P => "V4L2_PIX_FMT_SRGGB14P",
        sys::V4L2_PIX_FMT_SBGGR16 => "V4L2_PIX_FMT_SBGGR16",
        sys::V4L2_PIX_FMT_SGBRG16 => "V4L2_PIX_FMT_SGBRG16",
        sys::V4L2_PIX_FMT_SGRBG16 => "V4L2_PIX_FMT_SGRBG16",
        sys::V4L2_PIX_FMT_SRGGB16 => "V4L2_PIX_FMT_SRGGB16",

        /* HSV formats */
        sys::V4L2_PIX_FMT_HSV24 => "V4L2_PIX_FMT_HSV24",
        sys::V4L2_PIX_FMT_HSV32 => "V4L2_PIX_FMT_HSV32",

        /* compressed formats */
        sys::V4L2_PIX_FMT_MJPEG => "V4L2_PIX_FMT_MJPEG",
        sys::V4L2_PIX_FMT_JPEG => "V4L2_PIX_FMT_JPEG",
        sys::V4L2_PIX_FMT_DV => "V4L2_PIX_FMT_DV",
        sys::V4L2_PIX_FMT_MPEG => "V4L2_PIX_FMT_MPEG",
        sys::V4L2_PIX_FMT_H264 => "V4L2_PIX_FMT_H264",
        sys::V4L2_PIX_FMT_H264_NO_SC => "V4L2_PIX_FMT_H264_NO_SC",
        sys::V4L2_PIX_FMT_H264_MVC => "V4L2_PIX_FMT_H264_MVC",
        sys::V4L2_PIX_FMT_H263 => "V4L2_PIX_FMT_H263",
        sys::V4L2_PIX_FMT_MPEG1 => "V4L2_PIX_FMT_MPEG1",
        sys::V4L2_PIX_FMT_MPEG2 => "V4L2_PIX_FMT_MPEG2",
        sys::V4L2_PIX_FMT_MPEG2_SLICE => "V4L2_PIX_FMT_MPEG2_SLICE",
        sys::V4L2_PIX_FMT_MPEG4 => "V4L2_PIX_FMT_MPEG4",
        sys::V4L2_PIX_FMT_XVID => "V4L2_PIX_FMT_XVID",
        sys::V4L2_PIX_FMT_VC1_ANNEX_G => "V4L2_PIX_FMT_VC1_ANNEX_G",
        sys::V4L2_PIX_FMT_VC1_ANNEX_L => "V4L2_PIX_FMT_VC1_ANNEX_L",
        sys::V4L2_PIX_FMT_VP8 => "V4L2_PIX_FMT_VP8",
        sys::V4L2_PIX_FMT_VP8_FRAME => "V4L2_PIX_FMT_VP8_FRAME",
        sys::V4L2_PIX_FMT_VP9 => "V4L2_PIX_FMT_VP9",
        sys::V4L2_PIX_FMT_VP9_FRAME => "V4L2_PIX_FMT_VP9_FRAME",
        sys::V4L2_PIX_FMT_HEVC => "V4L2_PIX_FMT_HEVC",
        sys::V4L2_PIX_FMT_FWHT => "V4L2_PIX_FMT_FWHT",

        sys::V4L2_PIX_FMT_FWHT_STATELESS => "V4L2_PIX_FMT_FWHT_STATELESS",

        sys::V4L2_PIX_FMT_H264_SLICE => "V4L2_PIX_FMT_H264_SLICE",
        sys::V4L2_PIX_FMT_HEVC_SLICE => "V4L2_PIX_FMT_HEVC_SLICE",
        sys::V4L2_PIX_FMT_AV1_FRAME => "V4L2_PIX_FMT_AV1_FRAME",
        sys::V4L2_PIX_FMT_SPK => "V4L2_PIX_FMT_SPK",
        sys::V4L2_PIX_FMT_RV30 => "V4L2_PIX_FMT_RV30",
        sys::V4L2_PIX_FMT_RV40 => "V4L2_PIX_FMT_RV40",

        /*  Vendor-specific formats   */
        sys::V4L2_PIX_FMT_CPIA1 => "V4L2_PIX_FMT_CPIA1",
        sys::V4L2_PIX_FMT_WNVA => "V4L2_PIX_FMT_WNVA",
        sys::V4L2_PIX_FMT_SN9C10X => "V4L2_PIX_FMT_SN9C10X",
        sys::V4L2_PIX_FMT_SN9C20X_I420 => "V4L2_PIX_FMT_SN9C20X_I420",
        sys::V4L2_PIX_FMT_PWC1 => "V4L2_PIX_FMT_PWC1",
        sys::V4L2_PIX_FMT_PWC2 => "V4L2_PIX_FMT_PWC2",
        sys::V4L2_PIX_FMT_ET61X251 => "V4L2_PIX_FMT_ET61X251",
        sys::V4L2_PIX_FMT_SPCA501 => "V4L2_PIX_FMT_SPCA501",
        sys::V4L2_PIX_FMT_SPCA505 => "V4L2_PIX_FMT_SPCA505",
        sys::V4L2_PIX_FMT_SPCA508 => "V4L2_PIX_FMT_SPCA508",
        sys::V4L2_PIX_FMT_SPCA561 => "V4L2_PIX_FMT_SPCA561",
        sys::V4L2_PIX_FMT_PAC207 => "V4L2_PIX_FMT_PAC207",
        sys::V4L2_PIX_FMT_MR97310A => "V4L2_PIX_FMT_MR97310A",
        sys::V4L2_PIX_FMT_JL2005BCD => "V4L2_PIX_FMT_JL2005BCD",
        sys::V4L2_PIX_FMT_SN9C2028 => "V4L2_PIX_FMT_SN9C2028",
        sys::V4L2_PIX_FMT_SQ905C => "V4L2_PIX_FMT_SQ905C",
        sys::V4L2_PIX_FMT_PJPG => "V4L2_PIX_FMT_PJPG",
        sys::V4L2_PIX_FMT_OV511 => "V4L2_PIX_FMT_OV511",
        sys::V4L2_PIX_FMT_OV518 => "V4L2_PIX_FMT_OV518",
        sys::V4L2_PIX_FMT_STV0680 => "V4L2_PIX_FMT_STV0680",
        sys::V4L2_PIX_FMT_TM6000 => "V4L2_PIX_FMT_TM6000",
        sys::V4L2_PIX_FMT_CIT_YYVYUY => "V4L2_PIX_FMT_CIT_YYVYUY",
        sys::V4L2_PIX_FMT_KONICA420 => "V4L2_PIX_FMT_KONICA420",
        sys::V4L2_PIX_FMT_JPGL => "V4L2_PIX_FMT_JPGL",
        sys::V4L2_PIX_FMT_SE401 => "V4L2_PIX_FMT_SE401",
        sys::V4L2_PIX_FMT_S5C_UYVY_JPG => "V4L2_PIX_FMT_S5C_UYVY_JPG",
        sys::V4L2_PIX_FMT_Y8I => "V4L2_PIX_FMT_Y8I",
        sys::V4L2_PIX_FMT_Y12I => "V4L2_PIX_FMT_Y12I",
        sys::V4L2_PIX_FMT_Z16 => "V4L2_PIX_FMT_Z16",
        sys::V4L2_PIX_FMT_MT21C => "V4L2_PIX_FMT_MT21C",
        sys::V4L2_PIX_FMT_MM21 => "V4L2_PIX_FMT_MM21",
        sys::V4L2_PIX_FMT_MT2110T => "V4L2_PIX_FMT_MT2110T",
        sys::V4L2_PIX_FMT_MT2110R => "V4L2_PIX_FMT_MT2110R",
        sys::V4L2_PIX_FMT_INZI => "V4L2_PIX_FMT_INZI",
        sys::V4L2_PIX_FMT_CNF4 => "V4L2_PIX_FMT_CNF4",
        sys::V4L2_PIX_FMT_HI240 => "V4L2_PIX_FMT_HI240",
        sys::V4L2_PIX_FMT_QC08C => "V4L2_PIX_FMT_QC08C",
        sys::V4L2_PIX_FMT_QC10C => "V4L2_PIX_FMT_QC10C",
        sys::V4L2_PIX_FMT_AJPG => "V4L2_PIX_FMT_AJPG",
        sys::V4L2_PIX_FMT_HEXTILE => "V4L2_PIX_FMT_HEXTILE",

        /* 10bit raw packed, 32 bytes for every 25 pixels, last LSB 6 bits unused */
        sys::V4L2_PIX_FMT_IPU3_SBGGR10 => "V4L2_PIX_FMT_IPU3_SBGGR10",
        sys::V4L2_PIX_FMT_IPU3_SGBRG10 => "V4L2_PIX_FMT_IPU3_SGBRG10",
        sys::V4L2_PIX_FMT_IPU3_SGRBG10 => "V4L2_PIX_FMT_IPU3_SGRBG10",
        sys::V4L2_PIX_FMT_IPU3_SRGGB10 => "V4L2_PIX_FMT_IPU3_SRGGB10",

        _ => "unknown",
    }
}
