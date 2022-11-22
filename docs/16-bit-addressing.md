# 16-bit Parameter Types

Instructions that accept a source and/or a destination can accept values using several addressing modes.
This document outlines all the possible parameter types and addressing modes for 16-bit instructions.

## Examples

In the following example code, instructions prefixed with `!` are illegal and will not assemble.

```asm
    ; Raw moves
    mov $F354, %ax               ; Copy 16-bit value in %ax to mem address $F354-F355
    mov %ax, $F354               ; Copy 16-bit value in mem addresses $F354-F355 to %ax
    mov %ax, #$F354              ; Copy immediate 16-bit value #$F354 to %ax
    mov %ax, %bx                 ; Copy 16-bit value in %bx to %ax
    mov $F354, #69               ; Copy 16-bit immediate #69 to mem address $F354-F355

    ; Dereferencing pointers
    mov [$F354], %ax             ; Copy 16-bit value in %ax to the mem address stored as a 16-bit pointer in $F354-F355
  ! mov $F354,   [%ax]           ; (Illegal Mem->Mem) Copy 16-bit value at the pointer address stored in %ax to the mem address $F354-F355
  ! mov [$F354], [%ax]           ; (Illegal Mem->Mem) Copy 16-bit value at the pointer address stored in %ax to the mem address stored as a 16-bit pointer in $F354-F355

  ! mov [%ax], $F354             ; (Illegal Mem->Mem) Copy 16-bit value in mem addresses $F354-F355 to the mem address stored as a 16-bit pointer in %ax
    mov %ax,   [$F354]           ; Copy 16-bit value at the pointer address stored in mem addresses $F354-F355 to %ax
  ! mov [%ax], [$F354]           ; (Illegal Mem->Mem) Copy 16-bit value at the pointer address stored in mem addresses $F354-F355 to the mem address stored as a 16-bit pointer in %ax

    mov [%ax], #$F354            ; Copy immediate 16-bit value #$F354 to the mem address stored as a 16-bit pointer %ax
  ! mov %ax, [#$F354]            ; (Illegal - Cannot dereference an immediate)
  ! mov [%ax], [#$F354]          ; (Illegal - Cannot dereference an immediate)

    mov [%ax], %bx               ; Copy 16-bit value in %bx to the mem address stored as a 16-bit pointer in %ax
    mov %ax, [%bx]               ; Copy 16-bit value at the pointer address stored in %bx to %ax
  ! mov [%ax], [%bx]             ; (Illegal Mem->Mem) Copy 16-bit value at the pointer address stored in %bx to the mem address stored as a 16-bit pointer in %ax

    mov [$F354], #69             ; Copy 16-bit immediate #69 to the mem address stored as a 16-bit pointer in $F354-F355
  ! mov $F354, [#69]             ; (Illegal - Cannot dereference an immediate)
  ! mov [$F354], [#69]           ; (Illegal - Cannot dereference an immediate)

  ! mov label, #$69              ; (Illegal - Write to ROM address) Copy the 16-bit immediate #$69 into the address of the label
    mov [label], #$69            ; (Dangerous if label does contain a valid ram address) Copy the 16-bit immediate #$69 into the mem address stored as a 16-bit pointer at the labels address in rom
    mov %ax, label               ; Copy the 16-bit rom address of the label into %ax
    mov %ax, [label]             ; Copy the 16-bit value at the pointer address stored in address of the label into %ax

    mov [%sp + 2], #$F354        ; Copy immediate 16-bit value #$F354 to the mem address computed by adding 2 to the stack pointer (%sp)
    mov [%sp + %ax * 2], #$F354  ; Copy immediate 16-bit value #$F354 to the mem address computed by adding 2 multiplied by the value in %ax to the stack pointer (%sp)
```

## Encoding

The parameter types for an instruction is encoded as 2 nibbles of 4 bits.
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
