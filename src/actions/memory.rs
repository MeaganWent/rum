pub fn cmove(register: &mut [u32; 8], a: u32, b: u32, c: u32) {
    if register[c as usize] != 0 {
        register[a as usize] = register[b as usize];
    }
}

pub fn segload(register: &mut [u32; 8], mem_seg: &mut Vec<Vec<u32>>, a: u32, b: u32, c: u32) {
    register[a as usize] = mem_seg[register[b as usize] as usize][register[c as usize] as usize];
}

pub fn segstore(register: &mut [u32; 8], mem_seg: &mut Vec<Vec<u32>>, a: u32, b: u32, c: u32) {
    mem_seg[register[a as usize] as usize][register[b as usize] as usize] = register[c as usize];    
}