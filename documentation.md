# Fantaset

My fantasy computer instruction set!

255 memory addresses

# Instructions

Anything above the hexadecimal digit `f` should be considered a variable.

Anything formatted in `codeblocks` is the hexadecimal data you would write to 
perform the instruction, everything not in code blocks should act merely as a 
comment.

The `0x` prefix is not used to denote hexadecimal numbers. Instead, anything inside of a codeblock is a hexadecimal number

(1), (2), (3) and so on refer to memory addresses

(1, 2) means:

`00000001` = 1
`10000000` = 2
(1, 2) = `0000000110000000`

Write to memory:

`E1` `vv` (1) `vv` (2) `vv` (3)

Writes (2, 3) to memory under the address (1)

Remove from memory:

`E2` `vv` (1)

Drops (1)

Add:

`A1` `vv` (1) `vv` (2)

(1) + (2)

Subtract:

`A2` `vv` (1) `vv` (2)

(1) - (2)

Multiply:

`A3` `vv` (1) `vv` (2)

(1) * (2)

Divide:

`A4` `vv` (1) `vv` (2)

(1) ÷ (2)

Jump

`F1` `vv` (1) `vv` (2)

Jumps to byte number (1, 2)

Return:

`FF` `vv` (1)

Returns (1)