# Directives
.byte 'A'
.word 10

# Jumps
JMP label         # JMP
JMR reg_1         # JMR
!0  reg_1 label   # BNZ
>0  reg_1 label   # BGT
<0  reg_1 label   # BLT
=0  reg_1 label   # BRZ

# Moves
MOV reg_1 reg_2   # MOV
LDA reg_1 label   # LDA
STW reg_1 label   # STR ("store word")
LDW reg_1 label   # LDR ("load word")
STB reg_1 label   # STB
LDB reg_1 label   # LDB

# Arithmetic
+   reg_1 reg_2   # ADD
+   reg_1 10      # ADI
-   reg_1 reg_2   # SUB
*   reg_1 reg_2   # MUL
/   reg_1 reg_2   # DIV

# Logical
&&  reg_1 reg_2
||  reg_1 reg_2

# Compare
== reg_1 reg_2
