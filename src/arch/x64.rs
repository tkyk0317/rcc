use arch::Generator;

// x64アセンブリ.
pub struct X64;
impl Generator for X64 {
    fn pop_stack(&self, reg: &str) -> String {
        format!("  movl 0(%rsp), %{}\n  add $8, %rsp\n", reg)
    }
    fn push_stack(&self, reg: &str) -> String {
        format!("  sub $8, %rsp\n  movl %{}, 0(%rsp)\n", reg)
    }
    fn push(&self, reg: &str) -> String {
        format!("  push %{}\n", reg)
    }
    fn pop(&self, reg: &str) -> String {
        format!("  pop %{}\n", reg)
    }
    fn label(&self, no: usize) -> String {
        format!(".L{}:\n", no)
    }
    fn jmp(&self, no: usize) -> String {
        format!("  jmp .L{}\n", no)
    }
    fn je(&self, no: usize) -> String {
        format!("  je .L{}\n", no)
    }
    fn jne(&self, no: usize) -> String {
        format!("  jne .L{}\n", no)
    }
    fn cmpl(&self, f: usize, r: &str) -> String {
        format!("  cmpl ${}, %{}\n", f, r)
    }
    fn mul(&self, reg: &str) -> String {
        format!("  mul %{}\n", reg)
    }
    fn multiple(&self) -> String {
        "  imull %ecx\n".to_string()
    }
    fn plus(&self) -> String {
        "  addl %ecx, %eax\n".to_string()
    }
    fn minus(&self) -> String {
        "  subl %ecx, %eax\n".to_string()
    }
    fn equal(&self) -> String {
        "  cmpl %ecx, %eax\n  sete %al\n  movzbl %al, %eax\n".to_string()
    }
    fn not_equal(&self) -> String {
        "  cmpl %ecx, %eax\n  setne %al\n  movzbl %al, %eax\n".to_string()
    }
    fn less_than(&self) -> String {
        "  cmpl %ecx, %eax\n  setl %al\n  movzbl %al, %eax\n".to_string()
    }
    fn less_than_equal(&self) -> String {
        "  cmpl %ecx, %eax\n  setle %al\n  movzbl %al, %eax\n".to_string()
    }
    fn greater_than(&self) -> String {
        "  cmpl %ecx, %eax\n  setg %al\n  movzbl %al, %eax\n".to_string()
    }
    fn greater_than_equal(&self) -> String {
        "  cmpl %ecx, %eax\n  setge %al\n  movzbl %al, %eax\n".to_string()
    }
    fn left_shift(&self) -> String {
        "  sall %cl, %eax\n".to_string()
    }
    fn right_shift(&self) -> String {
        "  sarl %cl, %eax\n".to_string()
    }
    fn bit_and(&self) -> String {
        "  andl %ecx, %eax\n".to_string()
    }
    fn bit_or(&self) -> String {
        "  orl %ecx, %eax\n".to_string()
    }
    fn bit_xor(&self) -> String {
        "  xorl %ecx, %eax\n".to_string()
    }
    fn bit_division(&self) -> String {
        "  movl $0, %edx\n  idivl %ecx\n".to_string()
    }
    fn lea(&self, p: i64) -> String {
        format!("  lea -{}(%rbp), %rax\n", p)
    }
    fn not(&self, reg: &str) -> String {
        format!("  notl %{}\n", reg)
    }
    fn set(&self, reg: &str) -> String {
        format!("  sete %{}\n", reg)
    }
    fn neg(&self, reg: &str) -> String {
        format!("  negl %{}\n", reg)
    }
    fn sub(&self, src: &str, dst: &str) -> String {
        format!("  sub %{}, %{}\n", src, dst)
    }
    fn sub_imm(&self, i: usize, reg: &str) -> String {
        format!("  sub ${}, %{}\n", i, reg)
    }
    fn add(&self, src: &str, dst: &str) -> String {
        format!("  add %{}, %{}\n", src, dst)
    }
    fn add_imm(&self, i: usize, reg: &str) -> String {
        format!("  add ${}, %{}\n", i, reg)
    }
    fn ret(&self) -> String {
        "  ret\n".to_string()
    }
    fn mov(&self, src: &str, dst: &str) -> String {
        format!("  mov %{}, %{}\n", src, dst)
    }
    fn mov_src(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  mov {}(%{}), %{}\n", n, src, dst)
    }
    fn mov_dst(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  mov %{}, {}(%{})\n", src, n, dst)
    }
    fn mov_imm(&self, dst: &str, n: i64) -> String {
        format!("  mov ${}, %{}\n", n, dst)
    }
    fn movz(&self, src: &str, dst: &str) -> String {
        format!("  movzbl %{}, %{}\n", src, dst)
    }
    fn movl_imm(&self, n: i64, reg: &str) -> String {
        format!("  movl ${}, %{}\n", n, reg)
    }
    // %srcからn(%dst)へ転送
    fn movl_dst(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  movl %{}, {}(%{})\n", src, n, dst)
    }
    // 即値をn(%dst)へ転送
    fn movl_imm_dst(&self, i: i64, dst: &str, n: i64) -> String {
        format!("  movl ${}, {}(%{})\n", i, n, dst)
    }
    // n(%src)から%dstへ転送
    fn movl_src(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  movl {}(%{}), %{}\n", n, src, dst)
    }
    fn call(&self, a: &str) -> String {
        format!("  call {}\n", a)
    }
}
