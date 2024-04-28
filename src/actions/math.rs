// addition, multiplication, division, bitwise NAND
pub fn add(register: &mut [u32; 8], a: u32, b: u32, c: u32) {
    let sum = ((register[b as usize]) as usize + (register[c as usize]) as usize) as u64;
    register[a as usize] = (sum % 4294967296) as u32;
}

pub fn mult(register: &mut [u32; 8], a: u32, b: u32, c: u32) {
    let product = ((register[b as usize] ) as usize * (register[c as usize]) as usize) as u64;
    register[a as usize] = (product % 4294967296) as u32;
}

pub fn division(register: &mut [u32; 8], a: u32, b: u32, c: u32) {
    register[a as usize] = ((register[b as usize]) as usize / (register[c as usize]) as usize) as u32;
}

pub fn bitwise_nand(register: &mut [u32; 8], a: u32, b: u32, c: u32) {
    register[a as usize] = !(register[b as usize] & register[c as usize]);
}
