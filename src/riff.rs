pub mod riff {
use std::fs;
use std::io;

pub struct Riff {
	// Need the reader as the main source for reading the file
	pub reader: io::BufReader<fs::File>,
	pub writer: io::BufWriter<fs::File>,
	pub format: String,
	pub size: usize,
	pub fmt: Fmt,
	pub data: Data,
	pub info_list: InfoList

}



#[derive(Default)]
pub struct Data {
	pub size: usize,
	pub data_buffer: Vec<u8>,
}

#[derive(Default)]
pub struct Fmt {
	pub size: usize, 
	pub audio_format: i8,
	pub channels: i8,
	pub sample_rate: i32,
	pub byte_rate: i64,
	pub block_align: i32,
	pub bits_per_sample: i8,
	pub raw_data: Vec<u8>,
}

#[derive(Default)]
pub struct InfoList {
	pub data: Vec<u8>,
	pub size: usize,
	pub info: Vec<Info>,
	pub raw_data: Vec<u8>,
}

pub enum Info {
	IARL(String),
	IART(String),
	ICMS(String),
	ICMT(String),
	ICOP(String),
	ICRD(String),
	ICRP(String),
	IDIM(String),
	IDPI(String),
	IENG(String),
	IGNR(String),
	IKEY(String),
	ILGT(String),
	IMED(String),
	INAM(String),
	IPLT(String),
	IPRD(String),
	ISBJ(String),
	ISFT(String),
	ISHP(String),
	ISRC(String),
	ISRF(String),
	ITCH(String),
}


}
