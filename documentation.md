# Fantaset

My fantasy computer instruction set for 16-bit computers.

# CPU specifications

The CPU has 255 memory stores, each of which contain 255 addresses which can store 16 bit values.

The CPU switches memory stores in-between instructions to access the values stored inside them.
One memory store may have `00` as an address, and another may also do so, but only one can be
accessed at a time by changing memory stores.

# Formatting

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

# Instructions

All variables are in Little Endian format

Write to memory store:

`E1` `vv` (1) `vv` (2) `vv` (3)

Writes (2, 3) to memory under the address (1)

Remove from memory store:

`E2` `vv` (1)

Drops (1)

Change memory store:

`E3` `vv` (1)

Changes the memory store, so that more values can be written/read.
Changes the store to store number (1).

Routine marker:

Used for Jumps.

`E4` `vv` (1)

Sets (1) as an address that can be jumped to. Enter (1) at any jump block to jump to that point.

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

If:

If (1) evaluates to 1 or above, it will count as true. Otherwise, it is false.

Anything after (1) will be taken as an instruction and executed until the next `1F`. For nested if statements, use Jump.

Jump:

`F1` `vv` (1) `vv` (2)

Jumps to byte number (1, 2)

Return:

`FF` `vv` (1)

Returns (1)

Pseudo-random number:

`0C` `vv` (1)

Write a random 16-bit number to `vv`

Display (numerical):

`D1` `vv` (1)

Writes (1)'s number to stdout, without a newline or any form of padding.

Display (UTF-16):

`D2` `vv` (1)

Writes (1)'s UTF-16 character to stdout, without a newline or any form of padding

Display line (UTF-16):

For this one, you can write in (n) amount of addresses.

`D3` -- Start

`vv` -- Any number of addresses in-between.

`00` -- An escape character. Can be used in combination with another byte to become `\t`, `\n`, or to escape another character (e.g, hex `00` `D3` or `00` `00`)

`D3` -- End.

This may be more space-efficient than writing `D2` `vv` n amount of times.