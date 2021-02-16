use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

use spin::Mutex;

static LOGGER: Logger = Logger { cursor: Mutex::new((0, 0)) };

pub struct Logger {
    pub cursor: Mutex<(usize, usize)>,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        //TODO: Implement usage of record.target() so we can do info!(target: "test", "message");
        let level_colour = match record.level() {
            Level::Error => [255,0,0],
            Level::Warn => [196,90,80], //Orange/yellow
            Level::Trace => [0,255,255],
            _ => [0,255,0],
        };

        self.print_colored("[", level_colour);
        self.print_colored(record.level().as_str(), level_colour);
        self.print_colored("] ", level_colour);
        self.print_colored(record.args().as_str().unwrap_or(" "), [255, 255, 255]);
        self.print_colored("\n", [255, 255, 255]);
    }

    fn flush(&self) {}
}

impl Logger {
    fn newline(&self) {
        let mut cursor_pos = self.cursor.lock();
        *cursor_pos = (0, cursor_pos.1 + 1);
        let size = super::FRAMEBUFFER.lock().size();
        if cursor_pos.1 >= size.1 / 8 {
            //Clear screen and start again
            //TODO :)
        }
    }

    fn print_colored(&self, msg: &str, color: [u8; 3]) {
        let size = super::FRAMEBUFFER.lock().size();
        for c in msg.chars() {
            match c {
                '\n' => self.newline(),
                c => {
                    let (x, y) = {
                        let lock = self.cursor.lock();
                        (lock.0, lock.1)
                    };
                    {
                        let mut cursor_pos = self.cursor.lock();
                        super::FRAMEBUFFER.lock().write_char(c, x.wrapping_mul(8),y.wrapping_mul(8), color[0], color[1], color[2]);
                        *cursor_pos = (cursor_pos.0 + 1, cursor_pos.1);
                    }
                    if x + 1 >= size.0 / 8 {
                        self.newline();
                    }
                }
            }
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::max()))
}
