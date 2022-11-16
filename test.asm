
.data

    testing:
        .word %1111111111111111
    msg:                            
        .ascii "Hello World\n"  ; Declare some ascii data
        .ascii "Your Mom\n"     ; Declare some ascii data
    pi:  
        .word  65535               ; Declare a word constant
        .word  $FFFF               ; Declare a word constant

.text

_start:
    jsr a, b c,            ; Jump to sub routine a
    inc %00001010     ; inc value in address 10
    mov %eax, 1       ; Use 'exit' syscall
    mov %ebx, $5      ; Move return code into ebx
    syscall           ; perform exit syscall
a:
    ret
