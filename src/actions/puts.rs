use std::io::Read;
// Output,Input
pub fn output(register: &mut [u32; 8], c: u32) {
    // if register[c as usize] <= 255 && register[c as usize] > 0 {
        print!("{}", register[c as usize] as u8 as char);
    // }
}

pub fn input(register: &mut [u32; 8], c: u32) {
    let mut buffer = [0;1];
    std::io::stdin().read(&mut buffer).unwrap();
    register[c as usize] = buffer[0].try_into().unwrap();
}