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
                *byte = 0x90;
            }
        }
    }

    /// Sets a pixel in the framebuffer
    pub fn set_pixel(&self, x: usize, y: usize) {

    }
}

lazy_static! {
    pub static ref FRAMEBUFFER: Mutex<Framebuffer<'static>> = Mutex::new(
        Framebuffer::empty()
    );
}
