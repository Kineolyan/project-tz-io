// Computes the differences between two input values
// X, Y -> X - Y, Y - X
/// [1, 2] -> [-1, 1]
/// [3, -4] -> [7, -7]

Node #double
===========
IN:1 -> 1, IN:2 -> 2
------
MOV <1, ACC
SUB <2
MOV ACC, >1
NEG
MOV ACC, >2
------
1 -> OUT:1, 2 -> OUT:2
===========
