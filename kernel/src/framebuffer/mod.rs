use lazy_static::lazy_static;
use spin::Mutex;

#[macro_use] pub mod logger;

pub struct Framebuffer<'a> {
    raw_buf: Option<&'a mut bootloader::boot_info::FrameBuffer>,
    info: Option<bootloader::boot_info::FrameBufferInfo>,

    // buffer: &'a mut [u8]
}

impl<'a> Framebuffer<'a> {
    /// Creates an empty framebuffer
    pub const fn empty() -> Self {
        Self {
            raw_buf: None,
            info: None,

            // buffer: &mut [] //Empty slice
        }
    }

    /// Creates a framebuffer mapped to an actual framebuffer in memory
    pub fn with_buf(raw_buf: &'a mut bootloader::boot_info::FrameBuffer) -> Self {
        let info = raw_buf.info(); //Take info here to prevent ownership issues
        Self {
            raw_buf: Some(raw_buf),
            info: Some(info),

            // buffer: &mut vec![0; info.byte_len()],
        }
    }

    /// Size of the framebuffer
    pub fn size(&self) -> (usize, usize) {
        match self.info {
            Some(info) => (info.horizontal_resolution, info.vertical_resolution),
            None => (0, 0),
        }
    }

    /*
    /// Flushes the internal buffer to the raw framebuffer
    pub fn flush(&mut self) {
        if let Some(raw_buf) = &mut self.raw_buf {
            raw_buf.buffer_mut() = self.buffer;
        }
    }
    */

    /// Clears the screen
    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        use bootloader::boot_info::PixelFormat;
        if let Some(raw_buf) = &mut self.raw_buf {
            let value = [r, g, b];
            let mut rgb_index = 0;
            for byte in raw_buf.buffer_mut() {
                match self.info.unwrap().pixel_format { //We can unwrap here, because if we have a buffer set, we'll have info about the buffer too
                    PixelFormat::RGB => *byte = value[2 - rgb_index],
                    PixelFormat::BGR => *byte = value[rgb_index],
                    _ => {}
                }
                rgb_index = (rgb_index + 1) % 3;
            }
        }
    }

    /// Sets a pixel in the framebuffer
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        use bootloader::boot_info::PixelFormat;

        match self.info {
            None => {}, //Nothing can be done
            Some(info) => {
                if let Some(raw_buf) = &mut self.raw_buf {
                    let pixel_stride = info.bytes_per_pixel;
                    let line_stride = info.stride;
                    let size = (info.horizontal_resolution, info.vertical_resolution);

                    let pixel_offset = y * line_stride + x;

                    let color = match info.pixel_format {
                        PixelFormat::RGB => [b,g,r],
                        PixelFormat::BGR => [r,g,b],
                        // PixelFormat::U8
                        _ => [r,g,b],
                    };

                    let byte_offset = pixel_offset * pixel_stride;
                    raw_buf.buffer_mut()[byte_offset..(byte_offset+pixel_stride)].copy_from_slice(&color[..pixel_stride]);
                }
            }
        }
    }

    pub fn write_char(&mut self, c: char, x: usize, y: usize, r: u8, g: u8, b: u8) {
        use font8x8::UnicodeFonts;
        let font_c = font8x8::BASIC_FONTS.get(c).expect("Font does not include this character!");
        for (iy, byte) in font_c.iter().enumerate() {
            for (ix, bit) in (0..8).enumerate() {
                let alpha = if *byte & (1 << bit) == 0 { 0 } else { 1 };
                if alpha > 0 {
                    self.set_pixel(x + ix, y + iy, r, g, b);
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
