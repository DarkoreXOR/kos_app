extern "cdecl" {
    fn asm_syscall_a(eax: u32);
    fn asm_syscall_a_a(eax: u32, out_eax: *mut u32);
    fn asm_syscall_ab(eax: u32, ebx: u32);
    fn asm_syscall_ab_a(eax: u32, ebx: u32, out_eax: *mut u32);
    fn asm_syscall_abc(eax: u32, ebx: u32, ecx: u32);
    fn asm_syscall_abc_a(eax: u32, ebx: u32, ecx: u32, out_eax: *mut u32);
    fn asm_syscall_abcdSD(eax: u32, ebx: u32, ecx: u32, edx: u32, esi: u32, edi: u32);
}

pub(crate) unsafe fn syscall_a(eax: u32) {
    asm_syscall_a(eax);
}

pub(crate) unsafe fn syscall_a_a(eax: u32) -> (u32,) {
    let mut out_eax: u32 = 0;

    asm_syscall_a_a(eax, &mut out_eax);

    (out_eax,)
}

pub(crate) unsafe fn syscall_ab(eax: u32, ebx: u32) {
    asm_syscall_ab(eax, ebx);
}

pub(crate) unsafe fn syscall_ab_a(eax: u32, ebx: u32) -> (u32,) {
    let mut out_eax: u32 = 0;

    asm_syscall_ab_a(eax, ebx, &mut out_eax);

    (out_eax,)
}

pub(crate) unsafe fn syscall_abc(eax: u32, ebx: u32, ecx: u32) {
    asm_syscall_abc(eax, ebx, ecx);
}

pub(crate) unsafe fn syscall_abc_a(eax: u32, ebx: u32, ecx: u32) -> (u32,) {
    let mut out_eax: u32 = 0;

    asm_syscall_abc_a(eax, ebx, ecx, &mut out_eax);

    (out_eax,)
}

#[allow(non_snake_case)]
pub(crate) unsafe fn syscall_abcdSD(eax: u32, ebx: u32, ecx: u32, edx: u32, esi: u32, edi: u32) {
    asm_syscall_abcdSD(eax, ebx, ecx, edx, esi, edi);
}
