use std::io::{BufRead, BufReader, Write};
use std::fs;
use std::path::PathBuf;

use des_asm::*;

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
	/// list of decimal values
	List,
	/// Like list, but side-by-side with the input assembly
	Tagged,
	/// Format ready for insertion into desmos
	Desmos,
}

#[derive(clap::Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// format to output with
	#[arg(value_enum, long, short, default_value_t = OutputFormat::Desmos)]
	format: OutputFormat,
	/// file to read from, '-' is stdin
	#[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
	input_file: PathBuf,
	/// file to write to, '-' is stdout
	#[arg(long, short, value_name = "FILE", value_hint = clap::ValueHint::FilePath, default_value = "-")]
	output_file: PathBuf,
	#[command(flatten)]
	verbosity: clap_verbosity_flag::Verbosity,
}

struct Logger(log::LevelFilter);
impl log::Log for Logger {
	fn enabled(&self, meta: &log::Metadata) -> bool {
		self.0 >= meta.level()
	}
	fn log(&self, record: &log::Record) {
		if self.enabled(record.metadata()) {
			eprint!("{}", record.module_path().unwrap_or(""));
			if let Some(line) = record.line() {
				eprint!(":{line}");
			}
			eprint!(" {:>5}", record.level());
			eprintln!(": {}", record.args());
		}
	}
	fn flush(&self) {}
}

fn main() {
	let args = <Args as clap::Parser>::parse();
	let filter = args.verbosity.log_level_filter();
	log::set_logger(Box::leak(Box::new(Logger(filter))))
		.map(|_| log::set_max_level(filter))
		.unwrap();
	let mut in_file = match if args.input_file.to_str() == Some("-") {
		Ok(Box::new(std::io::stdin().lock()) as Box<dyn BufRead>)
	} else {
		fs::File::open(args.input_file).map(|v| Box::new(BufReader::new(v)) as Box<dyn BufRead>)
	} {
		Ok(v) => v,
		Err(e) => hard_error!("failed to open input file: {e}"),
	};
	let mut out_file = match if args.output_file.to_str() == Some("-") {
		Ok(Box::new(std::io::stdout().lock()) as Box<dyn Write>)
	} else {
		fs::OpenOptions::new()
			.write(true).open(args.output_file).map(|v| Box::new(v) as Box<dyn Write>)
	} {
		Ok(v) => v,
		Err(e) => hard_error!("failed to open input file: {e}"),
	};
	let mut outputter: Box<dyn Outputter> = match args.format {
		OutputFormat::List => Box::new(ListOutputter),
		OutputFormat::Tagged => Box::new(TaggedOutputter),
		OutputFormat::Desmos => Box::new(DesmosOutputter),
	};
	run(&mut in_file, &mut out_file, &mut *outputter);
}
