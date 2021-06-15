use std::env;

use std::fs;
use std::vec;
pub mod riff;


fn main() {
 let cmd_args:Vec<String> = env::args().collect();
 if cmd_args.len() != 2 {
     return;
 }   
 let filename = &cmd_args[1];
   
 

 let buffer = fs::read(filename);

 if let Ok(buf) = buffer {
    let file_size = as_u32_le(&buf[4..8]);

   // println!("{}, {}", file_size, buf.len());
    let fmt_data = get_fmt(&buf);
    let (size, offset) = get_data(&buf);
    let (list_data, list_offset) = get_list(&buf, size);

    if list_data == [0] {
        return
    }

    //println!("FMT: \n{:?}, {}", fmt_data, size);
    //println!("LIST: \n{:?}, {} {}", list_data, offset, &buf[offset]);

    let mut front_data = Vec::from(&buf[..list_offset]);
    let mut back_data = Vec::from(&buf[list_offset+list_data.len()..]);
    front_data.append(&mut back_data);

    //println!("DATA: \n{}", front_data.len());
    
    fs::write(filename, front_data).unwrap();
 }

}

fn get_fmt(data: &[u8]) -> Vec<u8> {
    for (i, _) in data.iter().enumerate() {
            if i >= data.len() - 3{
                break
            }
            if data[i..i+4] == [102, 109, 116, 32] {
                   let new_i = i+4;
                   let size = as_u32_le(&data[new_i..new_i+4]);
                   
                return Vec::from(&data[i..i+4+4+(size as usize)]);
            }
        
    };
    let vecu:Vec<u8> = vec![0];
        vecu
}

fn get_list(data: &[u8], data_size: usize) -> (Vec<u8>, usize) {
    let len_data = data.len();
    let mut i = 0; 
    while i < len_data - 3{
        
        
          if data[i..i+4] == [76, 73, 83, 84] {
                let new_i = i+4;
                   let size = as_u32_le(&data[new_i..new_i+4]);
                   println!("{}", size);
                   return (Vec::from(&data[i..i+4+4+(size as usize)]), i);
            }
            
        else if data[i..i+4] == [100, 97, 116, 97] {
            i += data_size;
        }
        i+= 1;
    };

    return (Vec::from([0]), 0);
}


fn get_data(data: &[u8]) -> (usize, usize) {
   for (i, _) in data.iter().enumerate() {
        if data[i..i+4] == [100, 97, 116, 97] {
            let new_i = i+4;
            let size = as_u32_le(&data[new_i..new_i+4]);
            // Must remeber to offset data by 8 bytes!!
            return (size as usize, i)
        }
   };
   return (0,0)
}


fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}