# Registers

The SIS16 architecture has 8 16-bit general purpose register (addressable by the low 8 bits or high 8 bits), 4 16-bit only registers, and 8 8-bit IO registers.

## 16-bit Registers

The 16-bit registers can be represented as a 4-bit index.

| Register        | Name  | Low 8-bit Name | High 8-bit Name | 4-bit Index | Purpose                                                             |
| --------------- | ----- | -------------- | --------------- | ----------- | ------------------------------------------------------------------- |
| A               | `%ax` | `%al`          | `%ah`           | `0000`      |                                                                     |
| C               | `%cx` | `%bl`          | `%bh`           | `0001`      |                                                                     |
| D               | `%dx` | `%cl`          | `%ch`           | `0010`      |                                                                     |
| B               | `%bx` | `%dl`          | `%dh`           | `0011`      |                                                                     |
| E               | `%ex` | `%el`          | `%eh`           | `0100`      |                                                                     |
| F               | `%fx` | `%fl`          | `%fh`           | `0101`      |                                                                     |
| G               | `%gx` | `%gl`          | `%gh`           | `0110`      |                                                                     |
| H               | `%hx` | `%hl`          | `%hh`           | `0111`      |                                                                     |
| Program Counter | `%pc` | _--_           | _--_            | `1000`      | Tracks the pointer to the next instruction to be fetched by the CPU |
| Stack Pointer   | `%sp` | _--_           | _--_            | `1001`      | Tracks the location of the top of the stack                         |
| Frame Pointer   | `%fp` | _--_           | _--_            | `1010`      | Tracks the location of the start of the current frame               |

## 8-bit Registers

The 8-bit registers can be represented as a 5-bit index

| Register | Name   | 5-bit Index | Description  |
| -------- | ------ | ----------- | ------------ |
| AL       | `%al`  | `00000`     | A Low Bytes  |
| AH       | `%ah`  | `00001`     | A High Bytes |
| BL       | `%bl`  | `00010`     | B Low Bytes  |
| BH       | `%bh`  | `00011`     | B High Bytes |
| CL       | `%cl`  | `00100`     | C Low Bytes  |
| CH       | `%ch`  | `00101`     | C High Bytes |
| DL       | `%dl`  | `00110`     | D Low Bytes  |
| DH       | `%dh`  | `00111`     | D High Bytes |
| EL       | `%el`  | `01000`     | E Low Bytes  |
| EH       | `%eh`  | `01001`     | E High Bytes |
| FL       | `%fl`  | `01010`     | F Low Bytes  |
| FH       | `%fh`  | `01011`     | F High Bytes |
| GL       | `%gl`  | `01100`     | G Low Bytes  |
| GH       | `%gh`  | `01101`     | G High Bytes |
| HL       | `%hl`  | `01110`     | H Low Bytes  |
| HH       | `%hh`  | `01111`     | H High Bytes |
| IOA      | `%ioa` | `10000`     | IO Port A    |
| IOB      | `%iob` | `10001`     | IO Port B    |
| IOC      | `%ioc` | `10010`     | IO Port C    |
| IOD      | `%iod` | `10011`     | IO Port D    |
| IOE      | `%ioe` | `10100`     | IO Port E    |
| IOF      | `%iof` | `10101`     | IO Port F    |
| IOG      | `%iog` | `10110`     | IO Port G    |
| IOH      | `%ioh` | `10111`     | IO Port H    |
