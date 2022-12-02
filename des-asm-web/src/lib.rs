use std::io::Cursor;

use des_asm::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn debug(msg: &str);
	#[wasm_bindgen(js_namespace = console)]
	fn log(msg: &str);
	#[wasm_bindgen(js_namespace = console)]
	fn info(msg: &str);
	#[wasm_bindgen(js_namespace = console)]
	fn warn(msg: &str);
	#[wasm_bindgen(js_namespace = console)]
	fn error(msg: &str);
	#[wasm_bindgen(js_namespace = window)]
	fn asm_error(msg: &str);
}

struct Logger;
impl log::Log for Logger {
	fn enabled(&self, _meta: &log::Metadata) -> bool {
		true
	}
	fn log(&self, record: &log::Record) {
		use std::fmt::Write;
		let mut out = String::new();
		write!(out, "{}", record.module_path().unwrap_or("")).unwrap();
		if let Some(line) = record.line() {
			write!(out, ":{line}").unwrap();
		}
		write!(out, " {:>5}", record.level()).unwrap();
		writeln!(out, ": {}", record.args()).unwrap();
		(match record.level() {
			log::Level::Trace => debug,
			log::Level::Debug => log,
			log::Level::Info => info,
			log::Level::Warn => warn,
			log::Level::Error => {
				asm_error(&out);
				error
			},
		})(&out);
	}
	fn flush(&self) {}
}

#[wasm_bindgen(start)]
pub fn main() {
	log("main called");
	log::set_logger(Box::leak(Box::new(Logger)))
		.map(|_| log::set_max_level(log::LevelFilter::Trace))
		.unwrap();
	log::info!("logger started!");
}

#[wasm_bindgen]
pub fn compile(input: &str, format: &str) -> String {
	log::debug!("compile: {format:?} {input:?}");
	let mut outputter: Box<dyn Outputter> = match format {
		"list" => Box::new(ListOutputter),
		"tagged" => Box::new(TaggedOutputter),
		"desmos" => Box::new(DesmosOutputter),
		_ => hard_error!("invalid output type {format:?}"),
	};
	let mut out = Cursor::new(Vec::new());
	run(&mut Cursor::new(input), &mut out, &mut *outputter);
	match String::from_utf8(out.into_inner()) {
		Ok(v) => v,
		Err(_) => hard_error!("invalid utf-8 output"),
	}
}
