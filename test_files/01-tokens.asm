# Directives
label  .byte 'A'
label2 .word 10

# Jumps
JMP label
JMR reg_1
!0  reg_1 label
>0  reg_1 label
<0  reg_1 label
=0  reg_1 label

# Moves
MOV reg_1 reg_2
LDA reg_1 label
STW reg_1 label
LDW reg_1 label
STB reg_1 label
LDB reg_1 label

# Arithmetic
+   reg_1 reg_2
+   reg_1 10
-   reg_1 reg_2
*   reg_1 reg_2
/   reg_1 reg_2

# Logical
&&  reg_1 reg_2
||  reg_1 reg_2

# Compare
== reg_1 reg_2

# Commands
OUT
IN
ASCO
ASCI
A2I
I2A
END
