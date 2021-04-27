use std::time::Instant;
use std::fmt::{Display, Formatter, Result};
 

pub enum LogLevel {
	DEBUG,
	INFO,
	WARN,
	VERBOSE,
	ERROR
}
impl LogLevel {
	fn to_string(&self) -> String {
		match &self {
			LogLevel::DEBUG => "d".to_string(),
			LogLevel::INFO => "i".to_string(),
			LogLevel::WARN => "w".to_string(),
			LogLevel::VERBOSE => "v".to_string(),
			LogLevel::ERROR => "e".to_string(),
		}
	}
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Logger {
	start_time_instant: Instant,
	prefix: Option<String>
}

impl Logger {
	pub fn new(prefix: Option<String>) -> Self {
		Logger {
			start_time_instant: Instant::now(),
			prefix: prefix
		}
	}
	
	pub fn new_from_logger(logger: &Logger, prefix: Option<String>) -> Self {
		Logger {
			start_time_instant: logger.start_time_instant.clone(),
			prefix
		}
	}

	pub fn sub(&self, prefix: String) -> Logger {
		Logger {
			start_time_instant: self.start_time_instant.clone(),
			prefix: Some(prefix)
		}
	}

	pub fn _log(&self, log_level: LogLevel, message: String) {
		println!("{:.2} [{}] [{}]: {}",
			self.start_time_instant.elapsed().as_secs_f32(),
			log_level,
			self.prefix.as_ref().unwrap_or(&"".to_string()),
			message
		);
	}
	
	pub fn debug(&self, message: String) {
		self._log(LogLevel::DEBUG, message);
	}

	pub fn info(&self, message: String) {
		self._log(LogLevel::INFO, message);
	}

	pub fn warn(&self, message: String) {
		self._log(LogLevel::WARN, message);
	}

	pub fn verbose(&self, message: String) {
		self._log(LogLevel::VERBOSE, message);
	}
	
	pub fn error(&self, message: String) {
		self._log(LogLevel::ERROR, message);
	}
	
} 