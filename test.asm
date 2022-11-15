.data
    msg: .ascii "Hello World\n" ; Declare some ascii data

.text

.global _start

_start:
    jsr a             ; Jump to sub routine a
    inc %00001010     ; inc value in address 10
    mov %eax, 1       ; Use 'exit' syscall
    mov %ebx, $5      ; Move return code into ebx
    syscall           ; perform exit syscall
a:
    ret