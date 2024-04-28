use std::env;
use rum::rumload;
use rum::rumdis;
fn main() {

    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let mut registers = [0_u32; 8];
    // println!("{} instructions", instructions.len());
    let mut index = 0;
    let mut mem_seg: Vec<Vec<u32>> = vec![instructions.clone()];
    let mut count = 0;
    while index < mem_seg[0].len(){
        // println!("{} instructions", index);
        count += 1;
        rumdis::run_instruct(mem_seg[0][index], &mut mem_seg, &mut registers, &mut index);

        if count > 50000000 {
            break;
        }
    }
}