use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::iter::Peekable;
use std::fmt;
use std::ops::Range;

#[macro_export]
macro_rules! hard_error_ {
	($($tt:tt)*) => {{
		::log::error!($($tt)*);
		// ::std::process::exit(69);
		panic!("hard error");
	}};
}

pub use hard_error_ as hard_error;

pub trait Outputter {
	fn start_lit(&mut self, f: &mut dyn Write) -> io::Result<()>;
	fn write_lit(&mut self, idx: u16, value: f64, f: &mut dyn Write) -> io::Result<()>;
	fn start_expr(&mut self, f: &mut dyn Write) -> io::Result<()>;
	fn write_label(&mut self, addr: u16, name: &str, f: &mut dyn Write) -> io::Result<()>;
	fn write_expr(&mut self, addr: u16, expr: Ir1, f: &mut dyn Write) -> io::Result<()>;
	fn end(&mut self, f: &mut dyn Write) -> io::Result<()>;
}

pub struct ListOutputter;
pub struct TaggedOutputter;
pub struct DesmosOutputter;
impl Outputter for ListOutputter {
	fn start_lit(&mut self, _f: &mut dyn Write) -> io::Result<()> {
		Ok(())
	}
	fn write_lit(&mut self, idx: u16, value: f64, f: &mut dyn Write) -> io::Result<()> {
		if idx > 0 {
			write!(f, " ")?;
		}
		write!(f, "{value}")
	}
	fn start_expr(&mut self, f: &mut dyn Write) -> io::Result<()> {
		writeln!(f, "\n")
	}
	fn write_label(&mut self, _addr: u16, _name: &str, _f: &mut dyn Write) -> io::Result<()> {
		Ok(())
	}
	fn write_expr(&mut self, addr: u16, expr: Ir1, f: &mut dyn Write) -> io::Result<()> {
		if addr > 0 {
			write!(f, " ")?;
		}
		write!(f, "{}", expr.to_num())
	}
	fn end(&mut self, f: &mut dyn Write) -> io::Result<()> {
		writeln!(f)
	}
}
impl Outputter for TaggedOutputter {
	fn start_lit(&mut self, _f: &mut dyn Write) -> io::Result<()> {
		Ok(())
	}
	fn write_lit(&mut self, idx: u16, value: f64, f: &mut dyn Write) -> io::Result<()> {
		writeln!(f, "{idx:>5}: {value:?}")
	}
	fn start_expr(&mut self, _f: &mut dyn Write) -> io::Result<()> {
		Ok(())
	}
	fn write_label(&mut self, _addr: u16, name: &str, f: &mut dyn Write) -> io::Result<()> {
		writeln!(f, "              | {name}:")
	}
	fn write_expr(&mut self, addr: u16, expr: Ir1, f: &mut dyn Write) -> io::Result<()> {
		writeln!(f, "{addr:>5} {:>5}   |   {expr}", expr.to_num())
	}
	fn end(&mut self, _f: &mut dyn Write) -> io::Result<()> {
		Ok(())
	}
}
impl Outputter for DesmosOutputter {
	fn start_lit(&mut self, f: &mut dyn Write) -> io::Result<()> {
		write!(f, "s_{{pV}}=\\left[")
	}
	fn write_lit(&mut self, idx: u16, value: f64, f: &mut dyn Write) -> io::Result<()> {
		if idx > 0 {
			write!(f, ",")?;
		}
		write!(f, "{value}")
	}
	fn start_expr(&mut self, f: &mut dyn Write) -> io::Result<()> {
		write!(f, "\\right]\n\ns_{{pC}}=\\left[")
	}
	fn write_label(&mut self, _addr: u16, _name: &str, _f: &mut dyn Write) -> io::Result<()> {
		Ok(())
	}
	fn write_expr(&mut self, addr: u16, expr: Ir1, f: &mut dyn Write) -> io::Result<()> {
		if addr > 0 {
			write!(f, ",")?;
		}
		write!(f, "{}", expr.to_num())
	}
	fn end(&mut self, f: &mut dyn Write) -> io::Result<()> {
		writeln!(f, "\\right]")
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Op {
	UInc = 100,
	UDec = 101,
	ULog10 = 102,
	ULog2 = 103,
	ULn = 104,
	UExp10 = 105,
	UExp2 = 106,
	UExp = 107,
	UNeg = 108,
	USin = 110,
	UCos = 111,
	UTan = 112,
	USec = 113,
	UCsc = 114,
	UCot = 115,
	UCeil = 116,
	UFloor = 117,
	URound = 118,
	USign = 119,
	USinH = 120,
	UCosH = 121,
	UTanH = 122,
	USecH = 123,
	UCscH = 124,
	UCotH = 125,
	USqr = 126,
	USqrt = 127,
	UASin = 130,
	UACos = 131,
	UATan = 132,
	UASec = 133,
	UACsc = 134,
	UACot = 135,
	UASinH = 140,
	UACosH = 141,
	UATanH = 142,
	UASecH = 143,
	UACscH = 144,
	UACotH = 145,
	BAdd = 200,
	BSub = 201,
	BMul = 202,
	BDiv = 203,
	BMod = 204,
	BExp = 205,
	BLog = 206,
	BNrt = 207,
	BAtan2 = 208,
}
impl fmt::Display for Op {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Op::UInc => write!(f, "++"),
			Op::UDec => write!(f, "--"),
			Op::ULog10 => write!(f, "log10"),
			Op::ULog2 => write!(f, "log2"),
			Op::ULn => write!(f, "ln"),
			Op::UExp10 => write!(f, "10^"),
			Op::UExp2 => write!(f, "2^"),
			Op::UExp => write!(f, "e^"),
			Op::UNeg => write!(f, "1-"),
			Op::USin => write!(f, "sin"),
			Op::UCos => write!(f, "cos"),
			Op::UTan => write!(f, "tan"),
			Op::USec => write!(f, "sec"),
			Op::UCsc => write!(f, "csc"),
			Op::UCot => write!(f, "cot"),
			Op::UCeil => write!(f, "ceil"),
			Op::UFloor => write!(f, "floor"),
			Op::URound => write!(f, "round"),
			Op::USign => write!(f, "sign"),
			Op::USinH => write!(f, "sinH"),
			Op::UCosH => write!(f, "cosH"),
			Op::UTanH => write!(f, "tanH"),
			Op::USecH => write!(f, "secH"),
			Op::UCscH => write!(f, "cscH"),
			Op::UCotH => write!(f, "cotH"),
			Op::USqr => write!(f, "sqr"),
			Op::USqrt => write!(f, "sqrt"),
			Op::UASin => write!(f, "asin"),
			Op::UACos => write!(f, "acos"),
			Op::UATan => write!(f, "atan"),
			Op::UASec => write!(f, "asec"),
			Op::UACsc => write!(f, "acsc"),
			Op::UACot => write!(f, "acot"),
			Op::UASinH => write!(f, "asinh"),
			Op::UACosH => write!(f, "acosh"),
			Op::UATanH => write!(f, "atanh"),
			Op::UASecH => write!(f, "asech"),
			Op::UACscH => write!(f, "acsch"),
			Op::UACotH => write!(f, "acoth"),
			Op::BAdd => write!(f, "+"),
			Op::BSub => write!(f, "-"),
			Op::BMul => write!(f, "*"),
			Op::BDiv => write!(f, "/"),
			Op::BMod => write!(f, "%"),
			Op::BExp => write!(f, "^"),
			Op::BLog => write!(f, "log"),
			Op::BNrt => write!(f, "nrt"),
			Op::BAtan2 => write!(f, "atan2"),
		}
	}
}
#[derive(Debug)]
enum Jmp {
	Label(Vec<String>),
	Addr(u16),
}
#[derive(Debug)]
enum Ir0 {
	Nop,
	Dup(u16),
	Push(Lit),
	Jmp(Jmp),
	Call(Jmp),
	Op(Op),
	Pop(u16),
	Cmp(u8),
	Ret,
	In,
	Out,
	MemR,
	MemW,
	ProgR,
	ProgW,
	ValR,
	ValW,
	OutS,
}
#[derive(Debug)]
enum Lit {
	Label(Vec<String>),
	Num(f64),
	Id(u16),
}
#[derive(Debug)]
#[repr(u16)]
pub enum Ir1 {
	Nop,
	Dup(u16),
	Push(u16, f64),
	Jmp(u16, String),
	Call(u16, String),
	Op(Op),
	Pop(u16),
	Cmp(u8),
	Ret,
	In,
	Out,
	MemR,
	MemW,
	ProgR,
	ProgW,
	ValR,
	ValW,
	OutS,
}
impl Ir1 {
	fn to_num(&self) -> u16 {
		match self {
			Ir1::Nop => 0,
			Ir1::Dup(n) => 40000 + n,
			Ir1::Push(i, _) => 30000 + i,
			Ir1::Jmp(a, _) => 10000 + a,
			Ir1::Call(a, _) => 20000 + a,
			Ir1::Op(o) => *o as u16,
			Ir1::Pop(n) => 999 + n,
			Ir1::Cmp(c) => *c as u16,
			Ir1::Ret => 8,
			Ir1::In => 9,
			Ir1::Out => 10,
			Ir1::MemR => 11,
			Ir1::MemW => 12,
			Ir1::ProgR => 13,
			Ir1::ProgW => 14,
			Ir1::ValR => 15,
			Ir1::ValW => 16,
			Ir1::OutS => 20,
		}
	}
}
impl fmt::Display for Ir1 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Ir1::Nop => write!(f, "nop"),
			Ir1::Dup(n) => write!(f, "dup {n}"),
			Ir1::Push(i, v) => write!(f, "push {v} ;{i}"),
			Ir1::Jmp(a, l) => write!(f, "jmp {l} ;{a}"),
			Ir1::Call(a, l) => write!(f, "call {l} ;{a}"),
			Ir1::Op(o) => write!(f, "op {o}"),
			Ir1::Pop(n) => write!(f, "pop {n}"),
			Ir1::Cmp(c) => write!(f, "cmp {}", match c {
				1 => ">",
				2 => "=",
				3 => ">=",
				4 => "<",
				5 => "!=",
				6 => "<=",
				7 => "!",
				_ => unreachable!(),
			}),
			Ir1::Ret => write!(f, "ret"),
			Ir1::In => write!(f, "in"),
			Ir1::Out => write!(f, "out"),
			Ir1::MemR => write!(f, "memr"),
			Ir1::MemW => write!(f, "memw"),
			Ir1::ProgR => write!(f, "progr"),
			Ir1::ProgW => write!(f, "progw"),
			Ir1::ValR => write!(f, "valr"),
			Ir1::ValW => write!(f, "valw"),
			Ir1::OutS => write!(f, "outs"),
		}
	}
}
struct Lexer<T> {
	iter: T,
	line: usize,
}
impl<T: Iterator<Item = char>> Lexer<Peekable<T>> {
	fn new(line: usize, v: T) -> Self {
		log::trace!("lex new");
		Self {
			iter: v.peekable(),
			line
		}
	}
	fn peek_char(&mut self) -> char {
		let res = match self.iter.peek() {
			Some('\0') => ' ',
			Some(v) => *v,
			None => '\0',
		};
		log::trace!("lex peek_char {res:?}");
		res
	}
	fn peek(&mut self) -> char {
		let res = match self.iter.peek() {
			Some(';') => '\0',
			Some('\0') => ' ',
			Some(v) => *v,
			None => '\0',
		};
		log::trace!("lex peek {res:?}");
		res
	}
	fn adv(&mut self) -> &mut Self {
		log::trace!("lex adv");
		self.iter.next();
		self
	}
	fn space(&mut self) -> &mut Self {
		log::trace!("lex space");
		while matches!(self.peek(), '\x01'..=' ' | '\u{A0}' | '\u{1680}' | '\u{180E}' | '\u{2000}'..='\u{200B}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}') {
			self.adv();
		}
		self
	}
	fn eol(&mut self) {
		log::trace!("lex eol");
		let c = self.peek();
		if c != '\0' {
			hard_error!("line {} expected eol, got {:?}", self.line, c);
		}
	}
	fn id(&mut self, op: bool) -> String {
		log::trace!("lex id");
		let mut out = String::new();
		loop {
			match self.peek() {
				'\0' | '\'' | ':' | '\x01'..=' ' | '@' | '\u{A0}' | '\u{1680}' | '\u{180E}' | '\u{2000}'..='\u{200B}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}' => break,
				'>' | '<' | '=' | '!' | '+' | '-' | '*' | '/' | '%' | '^' if !op => break,
				c => {
					out.push(c);
					self.adv();
				},
			}
		}
		if out.is_empty() {
			hard_error!("line {} missing id", self.line);
		}
		out
	}
	fn int(&mut self, range: Range<u16>) -> u16 {
		log::trace!("lex int");
		let id = self.id(false);
		match id.parse().ok().filter(|v| range.contains(v)) {
			Some(v) => v,
			None => hard_error!("line {} invalid number {id:?}", self.line),
		}
	}
	fn lit(&mut self, last: &[String]) -> Lit {
		log::trace!("lex lit");
		match self.peek() {
			'@' => {
				Lit::Label(expand_id(self.line, last, &self.id(false)))
			}
			'\'' => {
				let r = self.adv().peek_char();
				if self.adv().peek() != '\'' {
					hard_error!("line {} invalid char", self.line);
				}
				self.adv();
				Lit::Num(r as u32 as f64)
			}
			'0'..='9' | '.' | '+' | '-' => {
				let id = self.id(true);
				match id.parse() {
					Ok(v) => Lit::Num(v),
					Err(_) => hard_error!("line {} invalid number {id:?}", self.line),
				}
			}
			c => hard_error!("line {} invalid literal {c}..", self.line),
		}
	}
}
fn expand_id(line: usize, last: &[String], mut id: &str) -> Vec<String> {
	let mut dot = 0;
	while id.as_bytes()[0] == b'.' {
		dot += 1;
		id = &id[1..];
	}
	if dot > last.len() {
		hard_error!("line {line} too many dots");
	}
	let mut v = last[0..dot].to_vec();
	v.push(id.to_string());
	v
}
pub fn run(in_file: &mut dyn BufRead, out_file: &mut dyn Write, out: &mut dyn Outputter) {
	let mut ir0 = Vec::new();
	let mut labels = HashMap::new();
	let mut lit = Vec::new();
	let mut last = Vec::new();
	for (line, l) in in_file.lines().enumerate() {
		let line = line + 1;
		log::trace!("line {line}");
		let l = match l {
			Ok(l) => l,
			Err(e) => hard_error!("line {line} failed in reading: {e}"),
		};
		let mut c = Lexer::new(line, l.chars().peekable());
		loop {
			c.space();
			if c.peek() == '\0' {
				break;
			}
			let id = c.id(false);
			if c.peek() == ':' {
				c.adv();
				let exp = expand_id(line, &last, &id);
				last = exp.clone();
				labels.insert(exp, ir0.len() as u16);
				continue;
			}
			match &id[..] {
				"nop" => {
					ir0.push(Ir0::Nop);
					c.space().eol();
				},
				"dup" => {
					ir0.push(Ir0::Dup(c.space().int(0..9999)));
					c.space().eol();
				},
				"push" => {
					ir0.push(Ir0::Push(c.space().lit(&last)));
					c.space().eol();
				},
				"jmp" => {
					if c.space().peek() == '@' {
						ir0.push(Ir0::Jmp(Jmp::Addr(c.int(0..10000))));
					} else {
						ir0.push(Ir0::Jmp(Jmp::Label(expand_id(line, &last, &c.id(false)))));
					}
					c.space().eol();
				},
				"call" => {
					if c.space().peek() == '@' {
						ir0.push(Ir0::Call(Jmp::Addr(c.int(0..10000))));
					} else {
						ir0.push(Ir0::Call(Jmp::Label(expand_id(line, &last, &c.id(false)))));
					}
					c.space().eol();
				},
				"op" => {
					let id = c.space().id(true);
					c.space().eol();
					ir0.push(Ir0::Op(match &id[..] {
						"++" => Op::UInc,
						"--" => Op::UDec,
						"log10" => Op::ULog10,
						"log2" => Op::ULog2,
						"ln" => Op::ULn,
						"10^" => Op::UExp10,
						"2^" => Op::UExp2,
						"e^" => Op::UExp,
						"1-" => Op::UNeg,
						"sin" => Op::USin,
						"cos" => Op::UCos,
						"tan" => Op::UTan,
						"sec" => Op::USec,
						"csc" => Op::UCsc,
						"cot" => Op::UCot,
						"ceil" => Op::UCeil,
						"floor" => Op::UFloor,
						"round" => Op::URound,
						"sign" => Op::USign,
						"sinh" => Op::USinH,
						"cosh" => Op::UCosH,
						"tanh" => Op::UTanH,
						"sech" => Op::USecH,
						"csch" => Op::UCscH,
						"coth" => Op::UCotH,
						"sqr" => Op::USqr,
						"sqrt" => Op::USqrt,
						"asin" => Op::UASin,
						"acos" => Op::UACos,
						"atan" => Op::UATan,
						"asec" => Op::UASec,
						"acsc" => Op::UACsc,
						"acot" => Op::UACot,
						"asinh" => Op::UASinH,
						"acosh" => Op::UACosH,
						"atanh" => Op::UATanH,
						"asech" => Op::UASecH,
						"acsch" => Op::UACscH,
						"acoth" => Op::UACotH,
						"+" => Op::BAdd,
						"-" => Op::BSub,
						"*" => Op::BMul,
						"/" => Op::BDiv,
						"%" => Op::BMod,
						"^" => Op::BExp,
						"log" => Op::BLog,
						"nrt" => Op::BNrt,
						"atan2" => Op::BAtan2,
						_ => hard_error!("line {line} invalid op {id:?}"),
					}))
				},
				"pop" => {
					ir0.push(Ir0::Pop(c.space().int(1..1001)));
					c.space().eol();
				},
				"cmp" => {
					let id = c.space().id(true);
					c.space().eol();
					let mut out = 0;
					for c in id.chars() {
						out ^= match c {
							'>' => 1,
							'=' => 2,
							'<' => 4,
							'!' => 7,
							_ => hard_error!("line {line} invalid cmp {c:?}"),
						};
					}
					if out == 0 {
						hard_error!("line {line} always-false cmp cannot be encoded");
					}
					ir0.push(Ir0::Cmp(out));
				},
				"ret" => {
					ir0.push(Ir0::Ret);
					c.space().eol();
				},
				"in" => {
					ir0.push(Ir0::In);
					c.space().eol();
				},
				"out" => {
					ir0.push(Ir0::Out);
					c.space().eol();
				},
				"memr" => {
					ir0.push(Ir0::MemR);
					c.space().eol();
				},
				"memw" => {
					ir0.push(Ir0::MemW);
					c.space().eol();
				},
				"progr" => {
					ir0.push(Ir0::ProgR);
					c.space().eol();
				},
				"progw" => {
					ir0.push(Ir0::ProgW);
					c.space().eol();
				},
				"valr" => {
					ir0.push(Ir0::ValR);
					c.space().eol();
				},
				"valw" => {
					ir0.push(Ir0::ValW);
					c.space().eol();
				},
				"outs" => {
					ir0.push(Ir0::OutS);
					c.space().eol();
				},
				c => hard_error!("line {line} invalid cmd {c:?}"),
			}
			break;
		}
	}
	if ir0.len() > 10000 {
		hard_error!("too many expressions");
	}
	let h = |v| if let Err(e) = v { hard_error!("failed to write: {e}"); };
	let mut labels_rev = HashMap::new();
	for (k, v) in &labels {
		let mut out = String::new();
		for _ in 1..k.len() {
			out.push('.');
		}
		out.push_str(k.last().unwrap());
		labels_rev.entry(*v)
			.or_insert_with(Vec::new)
			.push(out);
	}
	for expr in &mut ir0 {
		if let Ir0::Push(l) = expr {
			let v = match l {
				Lit::Label(k) => {
					*labels.get(&*k).unwrap_or_else(|| hard_error!("no key {k:?}")) as f64
				}
				Lit::Num(n) => *n,
				_ => hard_error!("pre-resolved lit??"),
			};
			let i = match lit.iter().enumerate().find(|(_, i)| i == &&v) {
				Some((v, _)) => v,
				None => {
					lit.push(v);
					lit.len() - 1
				}
			};
			if i >= 10000 {
				hard_error!("too many literals");
			}
			*l = Lit::Id(i as u16);
		}
	}
	h(out.start_lit(out_file));
	for (id, lit) in lit.iter().enumerate() {
		h(out.write_lit(id as u16, *lit, out_file));
	}
	h(out.start_expr(out_file));
	for (addr, expr) in ir0.into_iter().enumerate() {
		for l in labels_rev.remove(&(addr as u16)).unwrap_or_default() {
			h(out.write_label(addr as u16, &l, out_file));
		}
		h(out.write_expr(addr as u16, match expr {
			Ir0::Nop => Ir1::Nop,
			Ir0::Dup(v) => Ir1::Dup(v),
			Ir0::Push(Lit::Id(v)) => Ir1::Push(v, lit[v as usize]),
			Ir0::Push(v) => hard_error!("unresolved literal {v:?}"),
			Ir0::Jmp(Jmp::Label(k)) => {
				let mut out = String::new();
				for _ in 1..k.len() {
					out.push('.');
				}
				out.push_str(k.last().unwrap());
				let l = labels.get(&k).unwrap_or_else(|| hard_error!("no key {k:?}"));
				Ir1::Jmp(*l, out)
			}
			Ir0::Jmp(Jmp::Addr(a)) => Ir1::Jmp(a, format!("@{a}")),
			Ir0::Call(Jmp::Label(k)) => {
				let mut out = String::new();
				for _ in 1..k.len() {
					out.push('.');
				}
				out.push_str(k.last().unwrap());
				let l = labels.get(&k).unwrap_or_else(|| hard_error!("no key {k:?}"));
				Ir1::Call(*l, out)
			}
			Ir0::Call(Jmp::Addr(a)) => Ir1::Call(a, format!("@{a}")),
			Ir0::Op(v) => Ir1::Op(v),
			Ir0::Pop(v) => Ir1::Pop(v),
			Ir0::Cmp(v) => Ir1::Cmp(v),
			Ir0::Ret => Ir1::Ret,
			Ir0::In => Ir1::In,
			Ir0::Out => Ir1::Out,
			Ir0::MemR => Ir1::MemR,
			Ir0::MemW => Ir1::MemW,
			Ir0::ProgR => Ir1::ProgR,
			Ir0::ProgW => Ir1::ProgW,
			Ir0::ValR => Ir1::ValR,
			Ir0::ValW => Ir1::ValW,
			Ir0::OutS => Ir1::OutS,
		}, out_file));
	}
	h(out.end(out_file));
}
