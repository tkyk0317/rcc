pub mod x64;

pub trait Generator {
    fn pop_stack(&self, reg: &str) -> String;
    fn push_stack(&self, reg: &str) -> String;
    fn push(&self, reg: &str) -> String;
    fn pop(&self, reg: &str) -> String;
    fn label(&self, no: usize) -> String;
    fn jmp(&self, no: usize) -> String;
    fn je(&self, no: usize) -> String;
    fn jne(&self, no: usize) -> String;
    fn cmpl(&self, f: usize, r: &str) -> String;
    fn mul(&self, reg: &str) -> String;
    fn multiple(&self) -> String;
    fn plus(&self) -> String;
    fn minus(&self) -> String;
    fn equal(&self) -> String;
    fn not_equal(&self) -> String;
    fn less_than(&self) -> String;
    fn less_than_equal(&self) -> String;
    fn greater_than(&self) -> String;
    fn greater_than_equal(&self) -> String;
    fn left_shift(&self) -> String;
    fn right_shift(&self) -> String;
    fn bit_and(&self) -> String;
    fn bit_or(&self) -> String;
    fn bit_xor(&self) -> String;
    fn bit_division(&self) -> String;
    fn lea(&self, p: i64) -> String;
    fn not(&self, reg: &str) -> String;
    fn set(&self, reg: &str) -> String;
    fn neg(&self, reg: &str) -> String;
    fn add(&self, src: &str, dst: &str) -> String;
    fn add_imm(&self, i: usize, reg: &str) -> String;
    fn sub(&self, src: &str, dst: &str) -> String;
    fn sub_imm(&self, i: usize, reg: &str) -> String;
    fn ret(&self) -> String;
    fn mov(&self, src: &str, dst: &str) -> String;
    fn mov_src(&self, src: &str, dst: &str, n: i64) -> String;
    fn mov_dst(&self, src: &str, dst: &str, n: i64) -> String;
    fn mov_imm(&self, dst: &str, n: i64) -> String;
    fn movz(&self, src: &str, dst: &str) -> String;
    fn movl_imm(&self, n: i64, reg: &str) -> String;
    fn movl_dst(&self, src: &str, dst: &str, n: i64) -> String;
    fn movl_imm_dst(&self, i: i64, dst: &str, n: i64) -> String;
    fn movl_src(&self, src: &str, dst: &str, n: i64) -> String;
    fn call(&self, a: &str) -> String;
}
