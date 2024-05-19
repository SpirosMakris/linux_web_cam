// #include <sys/ioctl.h>
#include <linux/videodev2.h>
#include <stdio.h>

#define PRINT_DEFINE_LU(n) printf("const %s: u64 = %lu;\n", #n, n)
#define PRINT_DEFINE_U(n) printf("const %s: u32 = %u;\n", #n, n)

int main(void)
{
  PRINT_DEFINE_LU(VIDIOC_QUERYCAP);
  PRINT_DEFINE_LU(VIDIOC_G_FMT);

  /* RGB formats (1 or 2 bytes per pixel) */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB332);   /*  8  RGB-3-3-2     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB444);   /* 16  xxxxrrrr ggggbbbb */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ARGB444);  /* 16  aaaarrrr ggggbbbb */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XRGB444);  /* 16  xxxxrrrr ggggbbbb */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBA444);  /* 16  rrrrgggg bbbbaaaa */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBX444);  /* 16  rrrrgggg bbbbxxxx */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ABGR444);  /* 16  aaaabbbb ggggrrrr */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XBGR444);  /* 16  xxxxbbbb ggggrrrr */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGRA444);  /* 16  bbbbgggg rrrraaaa */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGRX444);  /* 16  bbbbgggg rrrrxxxx */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB555);   /* 16  RGB-5-5-5     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ARGB555);  /* 16  ARGB-1-5-5-5  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XRGB555);  /* 16  XRGB-1-5-5-5  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBA555);  /* 16  RGBA-5-5-5-1  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBX555);  /* 16  RGBX-5-5-5-1  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ABGR555);  /* 16  ABGR-1-5-5-5  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XBGR555);  /* 16  XBGR-1-5-5-5  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGRA555);  /* 16  BGRA-5-5-5-1  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGRX555);  /* 16  BGRX-5-5-5-1  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB565);   /* 16  RGB-5-6-5     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB555X);  /* 16  RGB-5-5-5 BE  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ARGB555X); /* 16  ARGB-5-5-5 BE */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XRGB555X); /* 16  XRGB-5-5-5 BE */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB565X);  /* 16  RGB-5-6-5 BE  */

  /* RGB formats (3 or 4 bytes per pixel) */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGR666);      /* 18  BGR-6-6-6	  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGR24);       /* 24  BGR-8-8-8     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB24);       /* 24  RGB-8-8-8     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGR32);       /* 32  BGR-8-8-8-8   */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ABGR32);      /* 32  BGRA-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XBGR32);      /* 32  BGRX-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGRA32);      /* 32  ABGR-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGRX32);      /* 32  XBGR-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGB32);       /* 32  RGB-8-8-8-8   */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBA32);      /* 32  RGBA-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBX32);      /* 32  RGBX-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ARGB32);      /* 32  ARGB-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XRGB32);      /* 32  XRGB-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBX1010102); /* 32  RGBX-10-10-10-2 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RGBA1010102); /* 32  RGBA-10-10-10-2 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ARGB2101010); /* 32  ARGB-2-10-10-10 */

  /* RGB formats (6 or 8 bytes per pixel) */
  PRINT_DEFINE_U(V4L2_PIX_FMT_BGR48_12);  /* 48  BGR 12-bit per component */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ABGR64_12); /* 64  BGRA 12-bit per component */

  /* Grey formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_GREY);   /*  8  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y4);     /*  4  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y6);     /*  6  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y10);    /* 10  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y12);    /* 12  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y012);   /* 12  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y14);    /* 14  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y16);    /* 16  Greyscale     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y16_BE); /* 16  Greyscale BE  */

  /* Grey bit-packed formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y10BPACK); /* 10  Greyscale bit-packed */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y10P);     /* 10  Greyscale, MIPI RAW10 packed */
  PRINT_DEFINE_U(V4L2_PIX_FMT_IPU3_Y10); /* IPU3 packed 10-bit greyscale */

  /* Palette formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_PAL8); /*  8  8-bit palette */

  /* Chrominance formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_UV8); /*  8  UV 4:4 */

  /* Luminance+Chrominance formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUYV);     /* 16  YUV 4:2:2     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YYUV);     /* 16  YUV 4:2:2     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YVYU);     /* 16 YVU 4:2:2 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_UYVY);     /* 16  YUV 4:2:2     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VYUY);     /* 16  YUV 4:2:2     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y41P);     /* 12  YUV 4:1:1     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV444);   /* 16  xxxxyyyy uuuuvvvv */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV555);   /* 16  YUV-5-5-5     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV565);   /* 16  YUV-5-6-5     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV24);    /* 24  YUV-8-8-8     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV32);    /* 32  YUV-8-8-8-8   */
  PRINT_DEFINE_U(V4L2_PIX_FMT_AYUV32);   /* 32  AYUV-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XYUV32);   /* 32  XYUV-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VUYA32);   /* 32  VUYA-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VUYX32);   /* 32  VUYX-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUVA32);   /* 32  YUVA-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUVX32);   /* 32  YUVX-8-8-8-8  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_M420);     /* 12  YUV 4:2:0 2 lines y, 1 line uv interleaved */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV48_12); /* 48  YUV 4:4:4 12-bit per component */

  /*
   * YCbCr packed format. For each Y2xx format, xx bits of valid data occupy the MSBs
   * of the 16 bit components, and 16-xx bits of zero padding occupy the LSBs.
   */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y210); /* 32  YUYV 4:2:2 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y212); /* 32  YUYV 4:2:2 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y216); /* 32  YUYV 4:2:2 */

  /* two planes -- one Y, one Cr + Cb interleaved  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12); /* 12  Y/CbCr 4:2:0  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV21); /* 12  Y/CrCb 4:2:0  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV16); /* 16  Y/CbCr 4:2:2  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV61); /* 16  Y/CrCb 4:2:2  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV24); /* 24  Y/CbCr 4:4:4  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV42); /* 24  Y/CrCb 4:4:4  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_P010); /* 24  Y/CbCr 4:2:0 10-bit per component */
  PRINT_DEFINE_U(V4L2_PIX_FMT_P012); /* 24  Y/CbCr 4:2:0 12-bit per component */

  /* two non contiguous planes - one Y, one Cr + Cb interleaved  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12M); /* 12  Y/CbCr 4:2:0  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV21M); /* 21  Y/CrCb 4:2:0  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV16M); /* 16  Y/CbCr 4:2:2  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV61M); /* 16  Y/CrCb 4:2:2  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_P012M); /* 24  Y/CbCr 4:2:0 12-bit per component */

  /* three planes - Y Cb, Cr */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV410);  /*  9  YUV 4:1:0     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YVU410);  /*  9  YVU 4:1:0     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV411P); /* 12  YVU411 planar */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV420);  /* 12  YUV 4:2:0     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YVU420);  /* 12  YVU 4:2:0     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV422P); /* 16  YVU422 planar */

  /* three non contiguous planes - Y, Cb, Cr */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV420M); /* 12  YUV420 planar */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YVU420M); /* 12  YVU420 planar */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV422M); /* 16  YUV422 planar */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YVU422M); /* 16  YVU422 planar */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YUV444M); /* 24  YUV444 planar */
  PRINT_DEFINE_U(V4L2_PIX_FMT_YVU444M); /* 24  YVU444 planar */

  /* Tiled YUV formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12_4L4);        /* 12  Y/CbCr 4:2:0  4x4 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12_16L16);      /* 12  Y/CbCr 4:2:0 16x16 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12_32L32);      /* 12  Y/CbCr 4:2:0 32x32 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV15_4L4);        /* 15 Y/CbCr 4:2:0 10-bit 4x4 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_P010_4L4);        /* 12  Y/CbCr 4:2:0 10-bit 4x4 macroblocks */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12_8L128);      /* Y/CbCr 4:2:0 8x128 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12_10BE_8L128); /* Y/CbCr 4:2:0 10-bit 8x128 tiles */

  /* Tiled YUV formats, non contiguous planes */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12MT);           /* 12  Y/CbCr 4:2:0 64x32 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12MT_16X16);     /* 12  Y/CbCr 4:2:0 16x16 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12M_8L128);      /* Y/CbCr 4:2:0 8x128 tiles */
  PRINT_DEFINE_U(V4L2_PIX_FMT_NV12M_10BE_8L128); /* Y/CbCr 4:2:0 10-bit 8x128 tiles */

  /* Bayer formats - see http://www.siliconimaging.com/RGB%20Bayer.htm */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR8);  /*  8  BGBG.. GRGR.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG8);  /*  8  GBGB.. RGRG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG8);  /*  8  GRGR.. BGBG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB8);  /*  8  RGRG.. GBGB.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR10); /* 10  BGBG.. GRGR.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG10); /* 10  GBGB.. RGRG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG10); /* 10  GRGR.. BGBG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB10); /* 10  RGRG.. GBGB.. */
                                        /* 10bit raw bayer packed, 5 bytes for every 4 pixels */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR10P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG10P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG10P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB10P);
  /* 10bit raw bayer a-law compressed to 8 bits */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR10ALAW8);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG10ALAW8);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG10ALAW8);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB10ALAW8);
  /* 10bit raw bayer DPCM compressed to 8 bits */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR10DPCM8);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG10DPCM8);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG10DPCM8);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB10DPCM8);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR12); /* 12  BGBG.. GRGR.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG12); /* 12  GBGB.. RGRG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG12); /* 12  GRGR.. BGBG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB12); /* 12  RGRG.. GBGB.. */
                                        /* 12bit raw bayer packed, 6 bytes for every 4 pixels */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR12P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG12P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG12P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB12P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR14); /* 14  BGBG.. GRGR.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG14); /* 14  GBGB.. RGRG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG14); /* 14  GRGR.. BGBG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB14); /* 14  RGRG.. GBGB.. */
                                        /* 14bit raw bayer packed, 7 bytes for every 4 pixels */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR14P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG14P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG14P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB14P);
  PRINT_DEFINE_U(V4L2_PIX_FMT_SBGGR16); /* 16  BGBG.. GRGR.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGBRG16); /* 16  GBGB.. RGRG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SGRBG16); /* 16  GRGR.. BGBG.. */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SRGGB16); /* 16  RGRG.. GBGB.. */

  /* HSV formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_HSV24);
  PRINT_DEFINE_U(V4L2_PIX_FMT_HSV32);

  /* compressed formats */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MJPEG);       /* Motion-JPEG   */
  PRINT_DEFINE_U(V4L2_PIX_FMT_JPEG);        /* JFIF JPEG     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_DV);          /* 1394          */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MPEG);        /* MPEG-1/2/4 Multiplexed */
  PRINT_DEFINE_U(V4L2_PIX_FMT_H264);        /* H264 with start codes */
  PRINT_DEFINE_U(V4L2_PIX_FMT_H264_NO_SC);  /* H264 without start codes */
  PRINT_DEFINE_U(V4L2_PIX_FMT_H264_MVC);    /* H264 MVC */
  PRINT_DEFINE_U(V4L2_PIX_FMT_H263);        /* H263          */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MPEG1);       /* MPEG-1 ES     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MPEG2);       /* MPEG-2 ES     */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MPEG2_SLICE); /* MPEG-2 parsed slice data */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MPEG4);       /* MPEG-4 part 2 ES */
  PRINT_DEFINE_U(V4L2_PIX_FMT_XVID);        /* Xvid           */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VC1_ANNEX_G); /* SMPTE 421M Annex G compliant stream */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VC1_ANNEX_L); /* SMPTE 421M Annex L compliant stream */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VP8);         /* VP8 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VP8_FRAME);   /* VP8 parsed frame */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VP9);         /* VP9 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_VP9_FRAME);   /* VP9 parsed frame */
  PRINT_DEFINE_U(V4L2_PIX_FMT_HEVC);        /* HEVC aka H.265 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_FWHT);

  PRINT_DEFINE_U(V4L2_PIX_FMT_FWHT_STATELESS);

  PRINT_DEFINE_U(V4L2_PIX_FMT_H264_SLICE); /* H264 parsed slices */
  PRINT_DEFINE_U(V4L2_PIX_FMT_HEVC_SLICE); /* HEVC parsed slices */
  PRINT_DEFINE_U(V4L2_PIX_FMT_AV1_FRAME);  /* AV1 parsed frame */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SPK);        /* Sorenson Spark */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RV30);       /* RealVideo 8 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_RV40);       /* RealVideo 9 & 10 */

  /*  Vendor-specific formats   */
  PRINT_DEFINE_U(V4L2_PIX_FMT_CPIA1);        /* cpia1 YUV */
  PRINT_DEFINE_U(V4L2_PIX_FMT_WNVA);         /* Winnov hw compress */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SN9C10X);      /* SN9C10x compression */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SN9C20X_I420); /* SN9C20x YUV 4:2:0 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_PWC1);         /* pwc older webcam */
  PRINT_DEFINE_U(V4L2_PIX_FMT_PWC2);         /* pwc newer webcam */
  PRINT_DEFINE_U(V4L2_PIX_FMT_ET61X251);     /* ET61X251 compression */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SPCA501);      /* YUYV per line */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SPCA505);      /* YYUV per line */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SPCA508);      /* YUVY per line */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SPCA561);      /* compressed GBRG bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_PAC207);       /* compressed BGGR bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MR97310A);     /* compressed BGGR bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_JL2005BCD);    /* compressed RGGB bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SN9C2028);     /* compressed GBRG bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SQ905C);       /* compressed RGGB bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_PJPG);         /* Pixart 73xx JPEG */
  PRINT_DEFINE_U(V4L2_PIX_FMT_OV511);        /* ov511 JPEG */
  PRINT_DEFINE_U(V4L2_PIX_FMT_OV518);        /* ov518 JPEG */
  PRINT_DEFINE_U(V4L2_PIX_FMT_STV0680);      /* stv0680 bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_TM6000);       /* tm5600/tm60x0 */
  PRINT_DEFINE_U(V4L2_PIX_FMT_CIT_YYVYUY);   /* one line of Y then 1 line of VYUY */
  PRINT_DEFINE_U(V4L2_PIX_FMT_KONICA420);    /* YUV420 planar in blocks of 256 pixels */
  PRINT_DEFINE_U(V4L2_PIX_FMT_JPGL);         /* JPEG-Lite */
  PRINT_DEFINE_U(V4L2_PIX_FMT_SE401);        /* se401 janggu compressed rgb */
  PRINT_DEFINE_U(V4L2_PIX_FMT_S5C_UYVY_JPG); /* S5C73M3 interleaved UYVY/JPEG */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y8I);          /* Greyscale 8-bit L/R interleaved */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Y12I);         /* Greyscale 12-bit L/R interleaved */
  PRINT_DEFINE_U(V4L2_PIX_FMT_Z16);          /* Depth data 16-bit */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MT21C);        /* Mediatek compressed block mode  */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MM21);         /* Mediatek 8-bit block mode, two non-contiguous planes */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MT2110T);      /* Mediatek 10-bit block tile mode */
  PRINT_DEFINE_U(V4L2_PIX_FMT_MT2110R);      /* Mediatek 10-bit block raster mode */
  PRINT_DEFINE_U(V4L2_PIX_FMT_INZI);         /* Intel Planar Greyscale 10-bit and Depth 16-bit */
  PRINT_DEFINE_U(V4L2_PIX_FMT_CNF4);         /* Intel 4-bit packed depth confidence information */
  PRINT_DEFINE_U(V4L2_PIX_FMT_HI240);        /* BTTV 8-bit dithered RGB */
  PRINT_DEFINE_U(V4L2_PIX_FMT_QC08C);        /* Qualcomm 8-bit compressed */
  PRINT_DEFINE_U(V4L2_PIX_FMT_QC10C);        /* Qualcomm 10-bit compressed */
  PRINT_DEFINE_U(V4L2_PIX_FMT_AJPG);         /* Aspeed JPEG */
  PRINT_DEFINE_U(V4L2_PIX_FMT_HEXTILE);      /* Hextile compressed */

  /* 10bit raw packed, 32 bytes for every 25 pixels, last LSB 6 bits unused */
  PRINT_DEFINE_U(V4L2_PIX_FMT_IPU3_SBGGR10); /* IPU3 packed 10-bit BGGR bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_IPU3_SGBRG10); /* IPU3 packed 10-bit GBRG bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_IPU3_SGRBG10); /* IPU3 packed 10-bit GRBG bayer */
  PRINT_DEFINE_U(V4L2_PIX_FMT_IPU3_SRGGB10); /* IPU3 packed 10-bit RGGB bayer */

  /* SDR formats - used only for Software Defined Radio devices */
  PRINT_DEFINE_U(V4L2_SDR_FMT_CU8);     /* IQ u8 */
  PRINT_DEFINE_U(V4L2_SDR_FMT_CU16LE);  /* IQ u16le */
  PRINT_DEFINE_U(V4L2_SDR_FMT_CS8);     /* complex s8 */
  PRINT_DEFINE_U(V4L2_SDR_FMT_CS14LE);  /* complex s14le */
  PRINT_DEFINE_U(V4L2_SDR_FMT_RU12LE);  /* real u12le */
  PRINT_DEFINE_U(V4L2_SDR_FMT_PCU16BE); /* planar complex u16be */
  PRINT_DEFINE_U(V4L2_SDR_FMT_PCU18BE); /* planar complex u18be */
  PRINT_DEFINE_U(V4L2_SDR_FMT_PCU20BE); /* planar complex u20be */

  /* Touch formats - used for Touch devices */
  PRINT_DEFINE_U(V4L2_TCH_FMT_DELTA_TD16); /* 16-bit signed deltas */
  PRINT_DEFINE_U(V4L2_TCH_FMT_DELTA_TD08); /* 8-bit signed deltas */
  PRINT_DEFINE_U(V4L2_TCH_FMT_TU16);       /* 16-bit unsigned touch data */
  PRINT_DEFINE_U(V4L2_TCH_FMT_TU08);       /* 8-bit unsigned touch data */

  /* Meta-data formats */
  PRINT_DEFINE_U(V4L2_META_FMT_VSP1_HGO); /* R-Car VSP1 1-D Histogram */
  PRINT_DEFINE_U(V4L2_META_FMT_VSP1_HGT); /* R-Car VSP1 2-D Histogram */
  PRINT_DEFINE_U(V4L2_META_FMT_UVC);      /* UVC Payload Header metadata */
  PRINT_DEFINE_U(V4L2_META_FMT_D4XX);     /* D4XX Payload Header metadata */
  PRINT_DEFINE_U(V4L2_META_FMT_VIVID);    /* Vivid Metadata */

  /* Vendor specific - used for RK_ISP1 camera sub-system */
  PRINT_DEFINE_U(V4L2_META_FMT_RK_ISP1_PARAMS);  /* Rockchip ISP1 3A Parameters */
  PRINT_DEFINE_U(V4L2_META_FMT_RK_ISP1_STAT_3A); /* Rockchip ISP1 3A Statistics */

  PRINT_DEFINE_LU(VIDIOC_REQBUFS);
  PRINT_DEFINE_LU(VIDIOC_QBUF);

  PRINT_DEFINE_LU(VIDIOC_STREAMON);
}
