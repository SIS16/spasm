/**
 * This is a program to test out the assembler and language tools 
 */

@include "lib/std.asm"

@define PI #3

@macro jsr &lbl {
    mov %eax, %pc   ; Push current program counter onto the stack
    add %eax, #10   ; Add 10 bytes (the size of the jsr macro)         
    jmp &lbl        ; Jump to the label passed in
}

@macro ret {
    pop %pc         ; Pop the last return address off of the stack to jump back
}

@org $0000

    bin:
        .word %1010101101011111             ; Declare a binary word constant
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
    reserved:
        .resb 64                            ; Reserve an array of 64 bytes
        .resw 16                            ; reserve an array of 16 words (32 bytes)

_start:
    nop                          ; No Operation
    
    ; Direct 16-bit Addressing
    mov $F354, %ax               ; Copy 16-bit value in %ax to mem address $F354-F355
    mov %ax, $F354               ; Copy 16-bit value in mem addresses $F354-F355 to %ax
    mov %ax, #$F354              ; Copy immediate 16-bit value #$F354 to %ax
    mov %ax, %bx                 ; Copy 16-bit value in %bx to %ax
    mov $F354, #69               ; Copy 16-bit immediate #69 to mem address $F354-F355
    
    ; Indirect 16-bit Addressing
    mov [$F354], %ax             ; Copy 16-bit value in %ax to the mem address stored as a 16-bit pointer in $F354-F355
    mov %ax,   [$F354]           ; Copy 16-bit value at the pointer address stored in mem addresses $F354-F355 to %ax
    mov [%ax], #$F354            ; Copy immediate 16-bit value #$F354 to the mem address stored as a 16-bit pointer %ax
    mov [%ax], %bx               ; Copy 16-bit value in %bx to the mem address stored as a 16-bit pointer in %ax
    mov %ax, [%bx]               ; Copy 16-bit value at the pointer address stored in %bx to %ax
    mov [$F354], #69             ; Copy 16-bit immediate #69 to the mem address stored as a 16-bit pointer in $F354-F355
    mov [label], #$69            ; (Dangerous if label does contain a valid ram address) Copy the 16-bit immediate #$69 into the mem address stored as a 16-bit pointer at the labels address in rom
    mov %ax, label               ; Copy the 16-bit rom address of the label into %ax
    mov %ax, [label]             ; Copy the 16-bit value at the pointer address stored in address of the label into %ax
    mov [%spx + 2], #$F354        ; Copy immediate 16-bit value #$F354 to the mem address computed by adding 2 to the stack pointer (%sp)
    mov [%spx + %ax * 2], #$F354  ; Copy immediate 16-bit value #$F354 to the mem address computed by adding 2 multiplied by the value in %ax to the stack pointer (%sp)

    ; Direct 8-bit Addressing
    movb $F354, %al              ; Copy 8-bit value in %al to mem address $F354
    movb %al, $F354              ; Copy 8-bit value in mem address $F354 to %al
    movb %al, #$69               ; Copy immediate 8-bit value #$69 to %al
    movb %al, %bl                ; Copy 8-bit value in %bl to %al
    movb $F354, #69              ; Copy 8-bit immediate #69 to mem address $F354
    
    ; Indirect 8-bit Addressing
    movb [$F354], %al              ; Copy 8-bit value in %al to the mem address stored as a 16-bit pointer in $F354-F355
    movb %al,   [$F354]            ; Copy 8-bit value at the pointer address stored in mem addresses $F354-F355 to %ax
    movb [%ax], #$F3               ; Copy immediate 8-bit value #$F3 to the mem address stored as a 16-bit pointer %ax
    movb [%ax], %bl                ; Copy 8-bit value in %bl to the mem address stored as a 16-bit pointer in %ax
    movb %al, [%bx]                ; Copy 8-bit value at the pointer address stored in %bx to %al
    movb [$F354], #69              ; Copy 8-bit immediate #69 to the mem address stored as a 16-bit pointer in $F354-F355
    movb [label], #$69             ; (Dangerous if label does contain a valid ram address) Copy the 8-bit immediate #$69 into the mem address stored as a 16-bit pointer at the labels address in rom
    movb %al, [label]              ; Copy the 8-bit value at the pointer address stored in address of the label into %al
    movb [%spx + 2], #$F3          ; Copy immediate 8-bit value #$F3 to the mem address computed by adding 2 to the stack pointer (%sp)
    movb [%spx + %ax * 2], #$F3    ; Copy immediate 8-bit value #$F3 to the mem address computed by adding 2 multiplied by the value in %ax to the stack pointer (%sp)

    ; 16-bit addition
    add %bx, %cx                ; Add the value of %ecx to the value in %bx
    add $F354, %cx              ; 
    add %bx, $F354              ; 
    add %bx, #2                 ; Add 2 to the value in %bx
    add $F354, #2
    add [$F354], #2
    add $F354, [%cx]

    ; 8-bit addition
    add %bl, %cl 

some_sr:
    mov %ax, msg
    jsr print           ; print a message stored at this address
    ret 