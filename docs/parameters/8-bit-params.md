# 8-bit Parameter Types

Instructions that accept a source and/or a destination can accept values using several addressing modes.
This document outlines all the possible parameter types and addressing modes for 16-bit instructions.

## Examples

In the following example code, instructions prefixed with `!` are illegal and will not assemble.

```asm
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

```

## Encoding

The parameter types for an instruction are encoded as 2 nibbles of 4 bits.
The high nibble describes the type of the first parameter (dest) and the low nibble describes the type of the second parameter (src).

| Parameter         | Addressing Mode Nibble | Data Bytes                | Data Bytes Description                                                                                                   | Destination | Source |
| ----------------- | ---------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ----------- | ------ |
| `<none>`          | `0b0000`               | `0bxxxxxxxx` `0bxxxxxxxx` | Memory address in LE                                                                                                     | ✅          | ✅     |
| `#$F354`          | `0b0001`               | `0b01010100` `0b11110011` | Immediate in LE                                                                                                          | ❌          | ✅     |
| `$F354`           | `0b0010`               | `0b01010100` `0b11110011` | Memory address in LE                                                                                                     | ✅          | ✅     |
| `%sp`             | `0b0011`               | `0bxxxx0100` `0bxxxxxxxx` | Register index in low 4 bits of first byte                                                                               | ✅          | ✅     |
| `[$F354]`         | `0b0100`               | `0b01010100` `0b11110011` | Memory address in LE                                                                                                     | ✅          | ✅     |
| `[%sp]`           | `0b0101`               | `0bxxxx0100` `0bxxxxxxxx` | Register index in _low_ 4 bits of first byte                                                                             | ✅          | ✅     |
| `[%sp + 2]`       | `0b0110`               | `0bxxxx0100` `0b00000010` | Register index in _low_ 4 bits of first byte, Constant as 8-bit int                                                      | ✅          | ✅     |
| `[%sp + %ax]`     | `0b0111`               | `0b00010100` `0b00000001` | Register index in _low_ 4 bits of first byte , Offset register index in _high_ 4 bits of first byte                      | ✅          | ✅     |
| `[%sp + %ax * 2]` | `0b1000`               | `0b00010100` `0b00000010` | Register index in _low_ 4 bits of first byte , Offset register index in _high_ 4 bits of first byte , Scale as 8-bit int | ✅          | ✅     |

### Examples

```asm
mov [%sp + %ax * 2], #$F354
```

would be encoded as:

```bin
0b00000001 0b10000001 0b00010100 0b00000010 0b01010100 0b11110011
```

| Byte         | Meaning                                                          |
| ------------ | ---------------------------------------------------------------- |
| `0b00000001` | `mov` instruction                                                |
| `0b10000001` | First param is Scaled Register Offset, Second param is immediate |
| `0b00010100` | Base register is `%sp` and index register is `%ax`               |
| `0b00000010` | Index scale is `2`                                               |
| `0b01010100` | Low byte of immediate is `0x54`                                  |
| `0b11110011` | High byte of immediate is `0xF3`                                 |
