use lazy_static::lazy_static;
use spin::Mutex;

pub struct Framebuffer<'a> {
    buf: Option<&'a mut bootloader::boot_info::FrameBuffer>,
    info: Option<bootloader::boot_info::FrameBufferInfo>,
}

impl<'a> Framebuffer<'a> {
    /// Creates an empty framebuffer
    pub const fn empty() -> Self {
        Self {
            buf: None,
            info: None,
        }
    }

    /// Creates a framebuffer mapped to an actual framebuffer in memory
    pub fn with_buf(buf: &'a mut bootloader::boot_info::FrameBuffer) -> Self {
        let info = buf.info(); //Take info here to prevent ownership issues
        Self {
            buf: Some(buf),
            info: Some(info),
        }
    }

    /// Size of the framebuffer
    pub fn size(&self) -> (usize, usize) {
        match self.info {
            Some(info) => (info.horizontal_resolution, info.vertical_resolution),
            None => (0, 0),
        }
    }

    /// Clears the screen
    pub fn clear(&mut self) {
        if let Some(buf) = &mut self.buf {
            for byte in buf.buffer_mut() {
                *byte = 0x90; //Writes 144 to each byte
            }
        }
    }

    /// Sets a pixel in the framebuffer
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        use bootloader::boot_info::PixelFormat;

        match self.info {
            None => {}, //Nothing can be done
            Some(info) => {
                if let Some(buf) = &mut self.buf {
                    let pixel_stride = info.bytes_per_pixel;
                    let line_stride = info.stride;
                    let size = (info.horizontal_resolution, info.vertical_resolution);
                    if x < size.0 && y < size.1 {
                        let pixel_index = x * pixel_stride + y * line_stride;
                        let buf_mut_ref = buf.buffer_mut();
                        match info.pixel_format {
                            PixelFormat::RGB => {
                                buf_mut_ref[pixel_index  ] = r;
                                buf_mut_ref[pixel_index+1] = g;
                                buf_mut_ref[pixel_index+2] = b;
                            },
                            PixelFormat::BGR => {
                                buf_mut_ref[pixel_index  ] = b;
                                buf_mut_ref[pixel_index+1] = g;
                                buf_mut_ref[pixel_index+2] = r;
                            },
                            _ => {} //TODO
                        }
                    } //Pixels outside the screen get discarded entirely
                }
            }
        }
    }
}

lazy_static! {
    pub static ref FRAMEBUFFER: Mutex<Framebuffer<'static>> = Mutex::new(
        Framebuffer::empty()
    );
}
