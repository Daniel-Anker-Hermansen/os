[bits 16]
[org 0x7E00]
_stage1:
    jmp _start

hello: db "This is a factory free OS!"
end_hello:

_start:
    mov si, 0xA000
    mov gs, si
    xor si, si
    mov byte [gs:si], 0x1
    ud2


_start1:
    mov si, hello
    mov di, end_hello
loop:
    mov byte al, [si]
    call print_char
    add si, 0x01
    cmp si, di
    jnz loop
    ud2

print_char:
    mov ah, 0x0E
    mov bh, 0x00
    int 0x10
    ret
