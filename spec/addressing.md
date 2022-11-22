# 16-bit Addressing Modes

Instructions that accept a source and/or a destination can accept values using several addressing modes.
This document outlines all the possible addressing modes for 16-bit instructions.

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

The addressing mode for an instruction is encoded as 2 nibbles of 4 bits.
The high nibble describes the addressing mode for the first parameter (dest) and the low nibble describes the addressing mode for the second parameter (src).

| Representation    | Addressing Mode Nibble | Data Bytes                             | Data Bytes Description                                    | Destination | Source |
| ----------------- | ---------------------- | -------------------------------------- | --------------------------------------------------------- | ----------- | ------ |
| `$F354`           | `0b000x`               | `0b01010100` `0b11110011`              | Memory address in LE                                      | ✅          | ✅     |
| `#$F354`          | `0b001x`               | `0b01010100` `0b11110011`              | Immediate in LE                                           | ❌          | ✅     |
| `[$F354]`         | `0b10xx`               | `0b01010100` `0b11110011`              | Memory address in LE                                      | ✅          | ✅     |
| `%sp`             | `0b01xx`               | `0b00010000`                           | Register index                                            | ✅          | ✅     |
| `[%sp]`           | `0b1100`               | `0b00010000`                           | Register index                                            | ✅          | ✅     |
| `[%sp + 2]`       | `0b1101`               | `0b00010000` `0b00000010`              | Register index, Constant as 8-bit int                     | ✅          | ✅     |
| `[%sp + %ax]`     | `0b1110`               | `0b00010000` `0b00000001`              | Register index, Offset register index                     | ✅          | ✅     |
| `[%sp + %ax * 2]` | `0b1111`               | `0b00010000` `0b00000001` `0b00000010` | Register index, Offset register index, Scale as 8-bit int | ✅          | ✅     |

### Bits of addressing mode nibble

| Bit                                                     | Name                     | Description                               |
| ------------------------------------------------------- | ------------------------ | ----------------------------------------- |
| 4                                                       | Addressing Mode          | `0` for direct, `1` for indirect          |
| 3                                                       | Value type               | `0` for constant, `1` for register        |
| 2 (direct and constant)                                 | Constant Type            | `0` for memory address, `1` for immediate |
| 2-1 (indirect constant is assumed to be memory address) | Indirect Addressing Mode | `00` = Only base                          |
|                                                         |                          | `01` = Base and constant offset           |
|                                                         |                          | `10` = Base and register offset           |
|                                                         |                          | `11` = Base and scaled register offset    |