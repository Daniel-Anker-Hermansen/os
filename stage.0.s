[bits 16]
[org 0x7C00]

_start:
    ; Set up the stack
    xor ax, ax
    mov ss, ax
    mov bp, 0x7C00
    mov sp, 0x7C00
    
    ; Set video mode (320 x 200, 256 colours)
    mov ah, 0x00
    mov al, 0x13
    int 0x10
    
    ; Read sectors from disk
    ; change this to be dynamically sized based
    ; on binary size of stage1
    xor ax, ax
    mov ds, ax
    mov ah, 0x02
    mov al, 0x40 ; 64 sectors
    mov ch, 0x00
    mov cl, 0x02
    mov dh, 0x00
    xor bx, bx
    mov es, bx
    mov bx, 0x7E00
    int 0x13

    ; Enable A20 line through keyboard controller
    ; Disable interrupts while enabling A20
    cli

    ; Disable keyboard
    call a20wait1
    mov al, 0xAD
    out 0x64, al

    ; Read from input
    call a20wait1
    mov al, 0xD0
    out 0x64, al

    call a20wait2
    in al, 0x60
    push eax

    ; Write to output
    call a20wait1
    mov al, 0xD1
    out 0x64, al

    call a20wait1
    pop eax
    or al, 2
    out 0x60, al
   
    ; Enable keyboard
    call a20wait1
    mov al, 0xAE
    out 0x64, al

    call a20wait1
    
    ; Set up paging tables
    mov edi, 0x00
    mov cr3, edi ; Pointer to the paging table
    xor eax, eax
    mov ecx, 2048 
    rep stosd ; Zero four pages 
    mov edi, cr3

    ; PGD
    mov dword [edi + 0x0000], 0x00001003 ; PUD: 0x1000 + Superuser + Read + Write + Present
    ; PUD
    mov dword [edi + 0x1000], 0x00000083 ; Page: 0x00 + Superuser + Read + Write + Present + Page Size

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    mov eax, cr0
    or eax, 1 << 31 | 1 << 0 ; Protected + Paging (Long mode)
    mov cr0, eax

    mov edi, 0x2000
    mov dword [edi + 0], 0x00000000
    mov dword [edi + 4], 0x00000000
    mov dword [edi + 8], 0x00000000
    mov dword [edi + 12], 0x00209B00
    mov dword [edi + 16], 0x00000000
    mov dword [edi + 20], 0x00409300

    ; Load gdt
    lgdt [gdt]

    jmp 0x8:pe_start ; Jump to 64 bit code

gdt:
    db word (8 * 3 - 1)
    db qword 0x2000 

a20wait1:
    in al, 0x64
    test al, 2
    jnz a20wait1
    ret

a20wait2:
    in al, 0x64
    test al, 1
    jz a20wait2
    ret

[bits 64]
pe_start:
    ; I think I should reload the data segment registers here.
    ; Relocate the stack to a 14 MiB segment for plenty of RAM.
    mov rsp, 0x00EFFFF0
    mov rax, 0x7E00

    ; Finally jump to rust
    jmp rax

times 512 - 2 - ($ - $$) db 0
dw 0xAA55
