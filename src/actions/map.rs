// Map segment, Unmap segment
pub fn map(register: &mut [u32; 8], mem_seg: &mut Vec<Vec<u32>>, b: u32, c: u32) {
    let new = vec![0_u32; register[c as usize] as usize];
    mem_seg.push(new);

    register[b as usize] = (mem_seg.len() - 1) as u32;
}

pub fn unmap(_register: &mut [u32; 8], _mem_seg: &mut Vec<Vec<u32>>, _c: u32) {
    // mem_seg.remove(register[c as usize] as usize);
}