A simple RISC-based virtual machine written in Rust.

# Labels

Any line can be prefixed with an identifier and used as a reference point in other instructions.

# Instructions

## Directives

`.byte`: Store an ASCII value as a byte of data

### Example
```asm
.byte 'A' 
```

`.word`: Store a signed integer as a word of data

### Example
```asm
.word 10
```

## Jumps

`JMP`: Jump to a label

### Example
```asm
JMP label
```

`JMR`: Jump to an address stored in a register

### Example
```asm
JMR reg_1
```

`!0`: Jump to a label if the value stored in a register is not 0

### Example
```asm
!0  reg_1 label
```

`>0`: Jump to a label if the value stored in a register is greater than 0

### Example
```asm
>0  reg_1 label
```

`<0`: Jump to a label if the value stored in a register is greater than 0

### Example
```asm
<0  reg_1 label
```

`=0`: Jump to a label if the value stored in a register is equal to 0

### Example
```asm
=0  reg_1 label
```

_More instructions available, will be documented later_
