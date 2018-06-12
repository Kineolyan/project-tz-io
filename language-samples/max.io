/// Input  [[1, 2], [2, 3], ...]
/// Output [ 2    ,  3    , ...]

Node #split
===========
IN:1 -> 1, IN:2 -> 2, #left:1 -> 3, #right:1 -> 4
------------
// Split the values
START:
MOV <1, >2
MOV <2, >3
MOV <3, ACC
SUB <4
JLZ LEFT
MOV <4, >1
MOV <3, NIL
JMP START
LEFT:
MOV <3, >1
MOV <4, NIL
------------
1 -> OUT:1, 2 -> #left:1, 3 -> #right:1
===========

Node #left
===========
#split:2 -> 1
-------------
MOV <1, ACC
MOV ACC, >1
MOV ACC, >1
-----------
1 -> #split:3
===========

Node #right
===========
#split:3 -> 1
-------------
MOV <1, ACC
MOV ACC, >1
MOV ACC, >1
-----------
1 -> #split:4
===========
