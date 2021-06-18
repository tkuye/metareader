use std::env;

mod riff;
mod ints;

use riff::riff as rf;
fn main() {
 let cmd_args:Vec<String> = env::args().collect();
 if cmd_args.len() != 2 {
     return;
 }   
 let filename = &cmd_args[1];

 let wave = rf::Wave::new(filename.clone());

 if let Ok(wave) = wave {
     

     

     

    println!("{:?}", wave.info_list.info)
 }

}


