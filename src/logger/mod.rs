use std::io;

pub struct LoggerConfig {
    log_format: &'static str,
    log_level: log::LevelFilter,
    log_file: String,
}

impl LoggerConfig {
    pub fn new(
        log_format: &'static str,
        log_level: log::LevelFilter,
        log_file: String,
    ) -> LoggerConfig {
        LoggerConfig {
            log_format,
            log_level,
            log_file,
        }
    }
    pub fn set_logger(self) {
        let logger = self.dispatch();
        if self.log_file.is_empty() {
            logger.chain(io::stdout()).apply().unwrap();
        } else {
            logger
                .chain(io::stdout())
                .chain(fern::log_file(self.log_file).unwrap())
                .apply()
                .unwrap();
        }
    }

    fn dispatch(&self) -> fern::Dispatch {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format(self.log_format),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(self.log_level)
    }
}
