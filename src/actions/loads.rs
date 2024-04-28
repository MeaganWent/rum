// Load Program, Load Value

pub fn loadpro(register: &mut [u32; 8], mem_seg: &mut Vec<Vec<u32>>, b: u32 ) {
    if register[b as usize] != 0 {
        let duplicate_segment = mem_seg[register[b as usize] as usize].to_vec();

        // Replace the segment $m[0] with the duplicate
        mem_seg[0] = duplicate_segment;
    }
}

pub fn loadval(register: &mut [u32; 8], rl: u32, rv: u32) {
    register[rl as usize] = rv;
}
