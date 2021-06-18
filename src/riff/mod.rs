pub mod riff {
use std::fs;
use std::io;
use std::error::Error;
use std::fmt;

use crate::ints;
static IARL: &'static[u8] = &[0x49, 0x41, 0x52, 0x4c];
static IART: &'static[u8] = &[0x49, 0x41, 0x52, 0x54];
static ICMS : &'static[u8] = &[0x49, 0x43, 0x4d, 0x53];
static ICMT : &'static[u8] = &[0x49, 0x43, 0x4d, 0x54];
static ICOP : &'static[u8] = &[0x49, 0x43, 0x4f, 0x50];
static ICRD : &'static[u8] = &[0x49, 0x43, 0x52, 0x44];
static ICRP : &'static[u8] = &[0x49, 0x43, 0x52, 0x50];
static IDIM: &'static[u8] = &[0x49, 0x44, 0x49, 0x4d];
static IDPI: &'static[u8] = &[0x49, 0x44, 0x50, 0x49];
static IENG: &'static[u8] = &[0x49, 0x45, 0x4e, 0x47];
static IGNR: &'static[u8] = &[0x49, 0x47, 0x4e, 0x52];
static IKEY: &'static[u8] = &[0x49, 0x4b, 0x45, 0x59];
static ILGT: &'static[u8] = &[0x49, 0x4c, 0x47, 0x54];
static IMED: &'static[u8] = &[0x49, 0x4d, 0x45, 0x44];
static INAM: &'static[u8] = &[0x49, 0x4e, 0x41, 0x4d];
static IPLT: &'static[u8] = &[0x49, 0x50, 0x4c, 0x54];
static IPRD: &'static[u8] = &[0x49, 0x50, 0x52, 0x44];
static ISBJ: &'static[u8] = &[0x49, 0x53, 0x42, 0x4a];
static ISFT: &'static[u8] = &[0x49, 0x53, 0x46, 0x54];
static ISHP: &'static[u8] = &[0x49, 0x53, 0x48, 0x50];
static ISRC: &'static[u8] = &[0x49, 0x53, 0x52, 0x43];
static ISRF: &'static[u8] = &[0x49, 0x53, 0x52, 0x46];
static ITCH: &'static[u8] = &[0x49, 0x54, 0x43, 0x48];


pub struct Wave {
	// Need the reader as the main source for reading the file
	
	
	pub size: usize,
	pub fmt: Fmt,
	pub data: Data,
	pub info_list: InfoList

}

#[derive(Debug)]
struct WaveError(String);

impl fmt::Display for WaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for WaveError {}



impl Wave {
	pub fn new(filename: String) -> Result<Wave, Box<dyn Error>> {
		let buffer = fs::read(filename);

		if let Ok(buf) = buffer {
			let (fmt_data, _) = get_fmt(&buf);
			let fmt = Fmt::parse_fmt(&fmt_data);
			let (size, offset) = get_data(&buf);
			let data = Data::parse_data(&buf[offset..offset + size], size);
			
			 let (list_data, _, list_size) = get_list(&buf, size);
			let info_list = InfoList::parse_info(&list_data, list_size as usize);
			
			

			return Ok(Wave {
				size: ints::ints::as_u32_le(&buf[4..8]) as usize,
				fmt,
				data,
				info_list
			});
		};


		return  Err(Box::new(WaveError("Could not create wave struct".into())));
		
		
		
	}
}



pub fn get_fmt(data: &[u8]) -> (Vec<u8>, usize) {
    for (i, _) in data.iter().enumerate() {
            if i >= data.len() - 3{
                break
            }
            if data[i..i+4] == [102, 109, 116, 32] {
                   let new_i = i+4;
                   let size = ints::ints::as_u32_le(&data[new_i..new_i+4]);
                   
                return (Vec::from(&data[i..i+4+4+(size as usize)]), i);
            }
        
    };
    let vecu:Vec<u8> = vec![0];
        (vecu, 0)
}

pub fn get_list(data: &[u8], data_size: usize) -> (Vec<u8>, usize, u32) {
    let len_data = data.len();
    let mut i = 0; 
    while i < len_data - 3{
        
        
          if data[i..i+4] == [76, 73, 83, 84] {
                let new_i = i+4;
                   let size = ints::ints::as_u32_le(&data[new_i..new_i+4]);
                   
                   return (Vec::from(&data[i..i+8+(size as usize)]), i, size);
            }
            
        else if data[i..i+4] == [100, 97, 116, 97] {
            i += data_size;
        }
        i+= 1;
    };

    return (Vec::from([0]), 0, 0);
}


pub fn get_data(data: &[u8]) -> (usize, usize) {
   for (i, _) in data.iter().enumerate() {
        if data[i..i+4] == [100, 97, 116, 97] {
            let new_i = i+4;
            let size = ints::ints::as_u32_le(&data[new_i..new_i+4]);
            // Must remeber to offset data by 8 bytes!!
            return (size as usize, i)
        }
   };
   return (0,0)
}

pub fn resample(fmt: &mut Fmt, buffer: &mut Vec<u8>, sample_rate: u32, fmt_offset: usize) {
	
    fmt.change_sample_rate(sample_rate);

    buffer.splice(fmt_offset..fmt.size + 8 + fmt_offset, fmt.raw_data.iter().cloned());
   
}



#[derive(Default)]
pub struct Data {
	pub size: usize,
	pub data_buffer: Vec<u8>,
}

impl Data {
	pub fn parse_data(vec: &[u8], size: usize) -> Data {
		Data {
			size,
			data_buffer: Vec::from(&vec[..])
		}
	}

	pub fn remove_list_meta(&mut self, list_offset:usize, list: &InfoList) {
		let mut front_data = Vec::from(&self.data_buffer[..list_offset]);
    	let mut back_data = Vec::from(&self.data_buffer[list_offset+list.raw_data.len()..]);
    	front_data.append(&mut back_data);
	}
}

#[derive(Debug)]
#[derive(Default)]
pub struct Fmt {
	pub size: usize, 
	pub audio_format: u16,
	pub channels: u16,
	pub sample_rate: u32,
	pub byte_rate: u64,
	pub block_align: u16,
	pub bits_per_sample: u16,
	pub raw_data: Vec<u8>,
}

impl Fmt {
	pub fn change_sample_rate(&mut self, new_rate: u32) {
		self.sample_rate = new_rate;
		self.raw_data.splice(12..16, new_rate.to_le_bytes().iter().cloned());

		self.byte_rate = (self.bits_per_sample as u64 * self.channels as u64 * self.sample_rate as u64) / 8;
		let byte_rate : u32 = self.byte_rate as u32;
		self.raw_data.splice(16..20, byte_rate.to_le_bytes().iter().cloned());

	}

	pub fn parse_fmt(vec: &[u8]) -> Fmt {
    Fmt {
        size: ints::ints::as_u32_le(&vec[4..8]) as usize,
        audio_format: ints::ints::as_u16_le(&vec[8..10]),
        channels: ints::ints::as_u16_le(&vec[10..12]),
        sample_rate: ints::ints::as_u32_le(&vec[12..16]),
        byte_rate: ints::ints::as_u32_le(&vec[16..20]) as u64,
        block_align: ints::ints::as_u16_le(&vec[20..22]),
        bits_per_sample: ints::ints::as_u16_le(&vec[22..24]),
        raw_data: Vec::from(&vec[..])
    }
}

}

pub fn write_to_file(data: &[u8], filename: String) -> io::Result<()> {
	fs::write(filename, &data)
}



#[derive(Default)]
pub struct InfoList {
	pub size: usize,
	pub info: Vec<Info>,
	pub raw_data: Vec<u8>,
}


pub fn add_to_info_list(info: &[u8], size: &mut usize) -> String {
	let mut vec = Vec::new();
		*size+=8;
		while info[*size] > 31 && info[*size] < 126 {
			vec.push(info[*size]);
			*size+=1;
			}
		
		let vec_string = String::from_utf8(vec);
		if let Ok(i) = vec_string {
			return i;
	}
	String::from("")
}

impl InfoList {
	pub fn parse_info(data: &[u8], list_size: usize) -> InfoList {
		
		let mut i = 8;

		let mut info_list = InfoList::default();
		info_list.size = list_size;
		info_list.raw_data = Vec::from(data);
		// Between 33 and 126 is acceptable
		while i < data.len() - 3 {
			if Vec::from(&data[i..i+4]) == IARL
				 {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IARL(s));
					
				}
			else if Vec::from(&data[i..i+4]) == IART {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IART(s));
				}
				else if Vec::from(&data[i..i+4]) == ICMS  {
					
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ICMS(s));
				}
				
				else if Vec::from(&data[i..i+4]) == ICMT  {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ICMT(s));
					
	
				}
				else if Vec::from(&data[i..i+4]) == ICOP  {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ICOP(s));
				}
				else if Vec::from(&data[i..i+4]) == ICRD  {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ICRD(s));
				}
				else if Vec::from(&data[i..i+4]) == ICRP  {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ICRP(s));
					
				}
				else if Vec::from(&data[i..i+4]) == IDIM  {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IDIM(s));
				}
				else if Vec::from(&data[i..i+4]) == IDPI  {
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IDPI(s));
				}
				else if Vec::from(&data[i..i+4]) == IENG  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IENG(s));
				}
				else if Vec::from(&data[i..i+4]) == IGNR  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IGNR(s));
				}
				else if Vec::from(&data[i..i+4]) == IKEY  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IKEY(s));
				}
				else if Vec::from(&data[i..i+4]) == ILGT  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ILGT(s));
				}
				else if Vec::from(&data[i..i+4]) == IMED  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IMED(s));
				}
				else if Vec::from(&data[i..i+4]) == INAM  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::INAM(s));
				}
				else if Vec::from(&data[i..i+4]) == IPLT  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IPLT(s));
				}
				else if Vec::from(&data[i..i+4]) == IPRD  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::IPRD(s));
				}
				else if Vec::from(&data[i..i+4]) == ISBJ  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ISBJ(s));
				}
				else if Vec::from(&data[i..i+4]) == ISFT  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ISFT(s));
				}
				else if Vec::from(&data[i..i+4]) == ISHP  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ISHP(s));
				}
				else if Vec::from(&data[i..i+4]) == ISRC  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ISRC(s));
				}
				else if Vec::from(&data[i..i+4]) == ISRF  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ISRF(s));
				}
				else if Vec::from(&data[i..i+4]) == ITCH  { 
					let s = add_to_info_list(data, &mut i);
					info_list.info.push(Info::ITCH(s));
				}
			
			i+=1;
		};

		return info_list;

	}
}


#[derive(Debug)]
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
