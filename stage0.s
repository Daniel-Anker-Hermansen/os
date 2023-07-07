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
    
    ; GDT adress
    mov si, 0x800

    ; NULL segment
    mov dword [si], 0x00
    mov dword [si + 4], 0x00

    ; Limit
    mov word [si + 8  + 0], 0xFFFF
    ; Base of 1st segment
    mov word [si + 8 + 2], 0x00
    mov byte [si + 8 + 4], 0x00
    mov byte [si + 8 + 7], 0x00
    ; Limit and flags
    mov byte [si + 8 + 6], 0xCF
    ; Access byte. Present, Not system, ReadWrite
    mov byte [si + 8 + 5], 0x92

    ; Limit
    mov word [si + 16 + 0], 0xFFFF
    ; Base of 2nd segment
    mov word [si + 16 + 2], 0x00
    mov byte [si + 16 + 4], 0x00
    mov byte [si + 16 + 7], 0x00
    ; Limit and flags
    mov byte [si + 16 + 6], 0xCF
    ; Access byte. Present, Not system, ExecRead
    mov byte [si + 16 + 5], 0x9A

    ; Declare gdt
    lgdt [gdt]

    ; Switch to protected mode.
    mov eax, cr0
    or al, 1
    mov cr0, eax

    ; Jump to next sector.
    jmp 0x10:pe_start

gdt:
    db word (8 * 3 - 1)
    db dword 0x800

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

[bits 32]
pe_start:
    ; Relocate the stack to a 14 MiB segment for plenty of RAM.
    mov esp, 0x00EFFFF0

    mov esi, 0x7E00
    add esi, 0x01B0

    ; Finally jump to rust
    jmp esi

times 512 - 2 - ($ - $$) db 0
dw 0xAA55
