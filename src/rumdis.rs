#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mult,
    Div,
    Bit,
    Halt,
    Map,
    Unmap,
    Out,
    Input,
    LoadPro,
    LoadVal,
}

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::actions::memory::{cmove,segload,segstore};
use crate::actions::math::{add,mult,division,bitwise_nand};
use crate::actions::puts::{output,input};
use crate::actions::map::{map,unmap};
use crate::actions::loads::{loadpro,loadval};
use bitpack;
type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
}
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    bitpack::bitpack::getu(instruction as u64, field.width as u64, field.lsb as u64).unwrap().try_into().unwrap()
}
    /// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32(bitpack::bitpack::getu(instruction as u64, OP.width as u64, OP.lsb as u64).unwrap() as u32)
}
pub fn run_instruct(inst: Umi, mem_seg: &mut Vec<Vec<u32>>, register: &mut [u32; 8], index: &mut usize){
    match op(inst) {
        Some(Opcode::CMov) => {
            let c = get(&RC, inst);
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            cmove(register, a, b, c);
            *index += 1;

        },
        Some(Opcode::Load) => {
            let c = get(&RC, inst);
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            segload(register, mem_seg, a, b, c);
            *index += 1;
        },
        Some(Opcode::Store) => {
            let c = get(&RC, inst);
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            segstore(register, mem_seg, a, b, c);
            *index += 1;
        },
        Some(Opcode::Add) => {
            let c = get(&RC, inst);
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            add(register, a, b, c);
            *index += 1;
        },
        Some(Opcode::Mult) => {
            let c = get(&RC, inst);
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            mult(register, a, b, c);
            *index += 1;
        },
        Some(Opcode::Div) => {
            let c = get(&RC, inst);
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            division(register, a, b, c);
            *index += 1;
        },
        Some(Opcode::Bit) => {
            let c = get(&RC, inst);
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            bitwise_nand(register, a, b, c);
            *index += 1;
        },
        Some(Opcode::Halt) => {
            *index = mem_seg[0].len() + 1;
        },
        Some(Opcode::Map) => {
            let c = get(&RC, inst);
            let b = get(&RB, inst);
            map(register, mem_seg, b, c);
            *index += 1;
        },
        Some(Opcode::Unmap) => {
            let c = get(&RC, inst);
            unmap(register, mem_seg, c);
            *index += 1;
        },
        Some(Opcode::Out) => {
            let c = get(&RC, inst);
            output(register,c);
            *index += 1;
        },
        Some(Opcode::Input) => {
            let c = get(&RC, inst);
            input(register,c);
            *index += 1;
        },
        Some(Opcode::LoadPro) => {
            let b = get(&RB, inst);
            let c = get(&RC, inst);
            loadpro(register, mem_seg, b);
            *index = register[c as usize] as usize;

        },
        Some(Opcode::LoadVal) => {
            let rl = get(&RL, inst);
            let rv = get(&VL, inst);

            loadval(register, rl, rv);
            *index += 1;
        },
        None => {
    
        }
    }
}
