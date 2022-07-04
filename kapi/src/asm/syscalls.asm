format ELF

section '.text' executable

public asm_syscall_a
public asm_syscall_a_a
public asm_syscall_ab
public asm_syscall_ab_a
public asm_syscall_abc
public asm_syscall_abc_a
public asm_syscall_abcdSD



; [esp + 0x04] == eax
asm_syscall_a:
    mov eax, [esp + 04]
    int 0x40
    ret



; [esp + 0x04] == eax
; [esp + 0x08] == out_eax
asm_syscall_a_a:
    mov eax, [esp + 0x04]
    int 0x40
    mov edx, [esp + 0x08]
    mov [edx], eax
    ret



; [esp + 0x04] == eax
; [esp + 0x08] == ebx
asm_syscall_ab:
    mov edx, ebx
    mov eax, [esp + 0x04]
    mov ebx, [esp + 0x08]
    int 0x40
    mov ebx, edx
    ret



; [esp + 0x04] == eax
; [esp + 0x08] == ebx
; [esp + 0x0C] == out_eax
asm_syscall_ab_a:
    mov edx, ebx
    mov eax, [esp + 0x04]
    mov ebx, [esp + 0x08]
    int 0x40
    mov ebx, [esp + 0x0C]
    mov [ebx], eax
    mov ebx, edx
    ret



; [esp + 0x04] == eax
; [esp + 0x08] == ebx
; [esp + 0x0C] == ecx
asm_syscall_abc:
    mov edx, ebx
    mov eax, [esp + 0x04]
    mov ebx, [esp + 0x08]
    mov ecx, [esp + 0x0C]
    int 0x40
    mov ebx, edx
    ret



; [esp + 0x04] == eax
; [esp + 0x08] == ebx
; [esp + 0x0C] == ecx
; [esp + 0x10] == out_eax
asm_syscall_abc_a:
    mov edx, ebx
    mov eax, [esp + 0x04]
    mov ebx, [esp + 0x08]
    mov ecx, [esp + 0x0C]
    int 0x40
    mov ebx, [esp + 0x10]
    mov [ebx], eax
    mov ebx, edx
    ret



; [esp + 0x04] == eax
; [esp + 0x08] == ebx
; [esp + 0x0C] == ecx
; [esp + 0x10] == edx
; [esp + 0x14] == esi
; [esp + 0x18] == edi
asm_syscall_abcdSD:
    push ebx esi edi
    mov eax, [esp + 0x10]
    mov ebx, [esp + 0x14]
    mov ecx, [esp + 0x18]
    mov edx, [esp + 0x1C]
    mov esi, [esp + 0x20]
    mov edi, [esp + 0x24]
    int 0x40
    pop edi esi ebx
    ret

