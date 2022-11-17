.data
    bin:
        .word %1111111111111111             ; Declare a binary word constant
    dec:  
        .word  65535                        ; Declare a decimal word constant
    hex:
        .word  $FFFF                        ; Declare a hexadecimal word constant
    multi_word:
        .word  $420                         ; Declare a hexadecimal word constant
        .word  69                           ; Declare a decimal word constant
    msg:
        .ascii "Hello World\n"              ; Declare some ascii data
    multi_msg:
        .ascii "Your Mom\n"                 ; Declare some more ascii data
        .ascii "Never Gonna Give You Up\n"  ; Declare even more ascii data

.text

_start:
    nop                  ; No Operation
    mov $F354, %eax      ; Copy value in %eax to mem address $F354
    mov %eax, $F354      ; Copy value in mem address $F354 to %eax
    mov %eax, #$F354     ; Copy immediate value #$F354 to %eax
    mov %eax, %ebx       ; Copy value in %ebx to %eax
    mov $F354, #69       ; Copy 8 bit immediate #69 to mem address $F354
    mov $F354, #420      ; Copy 16 bit immediate #420 to mem addresses $F354-F355
    add %ebx             ; Add the value of %ebx to the accumulator register
    add #2               ; Add 2 to the accumulator register
    add %ebx, %ecx       ; Add the value of %ecx to the value in %ebx
    add %ebx, #2         ; Add 2 to the value in %ebx