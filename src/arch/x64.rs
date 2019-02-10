use arch::Generator;

// x64アセンブリ.
pub struct X64;
impl Generator for X64 {
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
        format!("  cmp ${}, %{}\n", f, r)
    }
    fn mul(&self, reg: &str) -> String {
        format!("  mul %{}\n", reg)
    }
    fn multiple(&self) -> String {
        "  imul %rcx\n".to_string()
    }
    fn plus(&self) -> String {
        "  add %rcx, %rax\n".to_string()
    }
    fn minus(&self) -> String {
        "  sub %rcx, %rax\n".to_string()
    }
    fn equal(&self) -> String {
        "  cmp %rcx, %rax\n  sete %al\n  movzb %al, %rax\n".to_string()
    }
    fn not_equal(&self) -> String {
        "  cmp %rcx, %rax\n  setne %al\n  movzb %al, %rax\n".to_string()
    }
    fn less_than(&self) -> String {
        "  cmp %rcx, %rax\n  setl %al\n  movzb %al, %rax\n".to_string()
    }
    fn less_than_equal(&self) -> String {
        "  cmp %rcx, %rax\n  setle %al\n  movzb %al, %rax\n".to_string()
    }
    fn greater_than(&self) -> String {
        "  cmp %rcx, %rax\n  setg %al\n  movzb %al, %rax\n".to_string()
    }
    fn greater_than_equal(&self) -> String {
        "  cmp %rcx, %rax\n  setge %al\n  movzb %al, %rax\n".to_string()
    }
    fn left_shift(&self) -> String {
        "  sal %cl, %rax\n".to_string()
    }
    fn right_shift(&self) -> String {
        "  sar %cl, %rax\n".to_string()
    }
    fn bit_and(&self) -> String {
        "  and %rcx, %rax\n".to_string()
    }
    fn bit_or(&self) -> String {
        "  or %rcx, %rax\n".to_string()
    }
    fn bit_xor(&self) -> String {
        "  xor %rcx, %rax\n".to_string()
    }
    fn bit_division(&self) -> String {
        "  mov $0, %rdx\n  idiv %rcx\n".to_string()
    }
    fn lea(&self, p: i64) -> String {
        format!("  leaq -{}(%rbp), %rax\n", p)
    }
    fn lea_glb(&self, n: &str) -> String {
        format!("  leaq {}(%rip), %rax\n", n)
    }
    fn not(&self, reg: &str) -> String {
        format!("  not %{}\n", reg)
    }
    fn set(&self, reg: &str) -> String {
        format!("  sete %{}\n", reg)
    }
    fn neg(&self, reg: &str) -> String {
        format!("  neg %{}\n", reg)
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
    fn add_src(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  add {}(%{}), %{}\n", n, src, dst)
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
    fn mov_imm_dst(&self, dst: &str, n: i64, offset: i64) -> String {
        format!("  movq ${}, {}(%{})\n", n, offset, dst)
    }
    fn movz(&self, src: &str, dst: &str) -> String {
        format!("  movzb %{}, %{}\n", src, dst)
    }
    // %srcからn(%dst)へ転送
    fn movb_dst(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  movb %{}, {}(%{})\n", src, n, dst)
    }
    fn movb_src(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  movb {}(%{}), %{}\n", n, src, dst)
    }
    // n(%src)から%dstへ転送
    fn movsbl_src(&self, src: &str, dst: &str, n: i64) -> String {
        format!("  mov {}(%{}), %{}\n", n, src, dst)
    }
    // global変数からの代入
    fn mov_from_glb(&self, dst: &str, name: &str) -> String {
        format!("  mov {}(%rip), %{}\n", name, dst)
    }
    fn movb_from_glb(&self, dst: &str, name: &str) -> String {
        format!("  mov {}(%rip), %{}\n", name, dst)
    }
    // global変数の代入
    fn mov_to_glb(&self, src: &str, name: &str) -> String {
        format!("  mov %{}, {}(%rip)\n", src, name)
    }
    fn movb_to_glb(&self, src: &str, name: &str) -> String {
        format!("  movb %{}, {}(%rip)\n", src, name)
    }
    fn call(&self, a: &str) -> String {
        format!("  call {}\n", a)
    }
    fn leave(&self) -> String {
        "leave\n".to_string()
    }
}
