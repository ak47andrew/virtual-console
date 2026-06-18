# Instructions

## Noop

Instruction that does nothing. Useful for padding or removing code at runtime

**Operands:**
- `none`

**Example:**
```armasm
noop
```

## Hlt

Stops the execution forever. Less useful for real games, more useful for simulations and tests

**Operands:**
- `none`

**Example:**
```armasm
; Draw the thing...
mov 4 $0
mov 2 $1
mov 3 $2
vsync
hlt  ; ...and then stop to look at it
```

> In real games you might want to have an endless loop with vsync and checking user input for, let's say, restart

## Vsync

Tell the console to "swap the banks" and update input. After this command is reached, console will display update screen texture from VRAM (see [Memory](./memory.md)) and place pressed and held buttons at the last two bytes of the RAM (compiler reports this addresses)

**Operands:**
- `none`

**Example:**
```armasm
; Fill up the VRAM
mov 4 $0
mov 2 $1
mov 3 $2
; Show it on the screen
vsync
```
> Note: after the vsync, VRAM isn't cleared and stays as it was beforehand

## Mov

Move data between same-size containers. First operand is the source, second - the destination. If you need transfer something between different places - you probably 

**Operands:**
- `Immediate`, `Address`
- `LongImmediate`, `Address`
- `LongerImmediate`, `Address`
- `Register`, `Address`
- `LongRegister`, `Address`
- `Immediate`, `IndirectAddress`
- `LongImmediate`, `IndirectAddress`
- `LongerImmediate`, `IndirectAddress`
- `Register`, `IndirectAddress`
- `LongRegister`, `IndirectAddress`
- `Address`, `Register`
- `Address`, `LongRegister`
- `IndirectAddress`, `Register`
- `IndirectAddress`, `LongRegister`
- `Register`, `Register`
- `LongImmediate`, `LongRegister`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`

**Example:**
```armasm
mov !A $0
mov ?LL1 $1
mov $2 !G1
```

## Trunc

Move data from higher-size container to the lower-size. Only last bytes is gonna be save, everything else is gonna be **trunc**ated. Usually used to move 64-bit register to 8-bit one or if you don't want to truncate your immediates for whatever reason

**Operands:**
- `LongImmediate`, `Register`
- `LongerImmediate`, `Register`
- `LongRegister`, `Register`
- `LongerImmediate`, `LongRegister`

**Example:**
```armasm
trunc ?LL1 !A
```

## Ext
Move data from lower-size container to the higher-size one. The first 7 bits is zeroed out. This can also be used to transfer one byte from main memory to LongRegister instead of the whole 8 when using `mov`

**Operands:**
- `Address`, `LongRegister`
- `IndirectAddress`, `LongRegister`
- `Immediate`, `LongRegister`
- `Register`, `LongRegister`

**Example:**
```armasm
ext $0 ?LL1  ; Just one byte
mov $0 ?LL1  ; whole 0-7 range
ext !A ?LL2
```

## Copy

Your primary way to copy huge chunks of data between different addresses in memory. First operand tells how many bytes to copy, second is the source address, third - destination address

**Operands:**
- `Immediate/LongImmediate/LongerImmediate/Register/LongRegister`, `Address/IndirectAddress`, `Address/IndirectAddress`

**Example:**

*Check* [*examples/copy.vea*](https://github.com/ak47andrew/virtual-console/blob/main/examples/copy.vea)

## Add

Adds two numbers of the same size and stores result in a register. For 8-bit values it's `!A`, for 64-bit - `?LL1`. Also if overflow occurred then `!Z` is set to `1`, else - to `0`

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`

**Example:**
```armasm
add !A 3
add ?LL1 &12
add ?LL1 ?LL2
add !A !X
```

## Sub

Subtract two numbers of the same size and stores result in a register. For 8-bit values it's `!A`, for 64-bit - `?LL1`. Also if underflow occurred then `!Z` is set to `1`, else - to `0`

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`

**Example:**
```armasm
sub !A 3
sub ?LL1 &12
sub ?LL1 ?LL2
sub !A !X
```

## Mul

Multiply two numbers of the same size and stores result in a register. For 8-bit values it's `!A`, for 64-bit - `?LL1`. Also if underflow occurred then `!Z` is set to `1`, else - to `0`

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`

**Example:**
```armasm
mul !A 3
mul ?LL1 &12
mul ?LL1 ?LL2
mul !A !X
```

## Div

Divide two numbers of the same size and stores result in one register and remainder in another. For 8-bit values it's `!A` and `!X`, for 640-bit it's `?LL1` and `?LL2`

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`

**Example:**
```armasm
div !A 3
div ?LL1 &12
div ?LL1 ?LL2
div !A !X
```

## And

Performs `binary ADD (&)` operation on two same-size numbers. Result is stored in `!A` for 8-bit values and `?LL1` for 64-bit values

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`

**Example:**
```armasm
and !A 3
and ?LL1 &12
and ?LL1 ?LL2
and !A !X
```

## Or

Performs `binary OR (|)` operation on two same-size numbers. Result is stored in `!A` for 8-bit values and `?LL1` for 64-bit values

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`

**Example:**
```armasm
or !A 3
or ?LL1 &12
or ?LL1 ?LL2
or !A !X
```

## Xor

Performs `binary XOR (^)` operation on two same-size numbers. Result is stored in `!A` for 8-bit values and `?LL1` for 64-bit values

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`

**Example:**
```armasm
xor !A 3
xor ?LL1 &12
xor ?LL1 ?LL2
xor !A !X
```

## Not

Performs `binary NOT (!)` operation on the number. Result is stored in `!A` for 8-bit values and `?LL1` for 64-bit values

**Operands:**
- `Immediate`
- `LongImmediate`
- `Register`
- `LongRegister`

**Example:**
```armasm
not 25
not &25
not !A
not ?LL1
```

## Shr

Performs `binary right shift (>>)` operation. Result is stored in `!A` for 8-bit values and `?LL1` for 64-bit values

**Operands:**
- `Immediate`, `Immediate`
- `Register`, `Immediate`
- `Immediate`, `Register`
- `LongImmediate`, `Immediate`
- `LongImmediate`, `Register`
- `LongRegister`, `Immediate`
- `LongRegister`, `Register`
- `LongRegister`, `LongRegister`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`
- `LongImmediate`, `LongImmediate`

**Example:**
```armasm
shr !A 3
shr ?LL1 10
shr ?LL1 ?LL2
shr !A !X
shr ?LL1 !A
```

## Shl

Performs `binary left shift (<<)` operation. Result is stored in `!A` for 8-bit values and `?LL1` for 64-bit values

**Operands:**
- `Immediate`, `Immediate`
- `Register`, `Immediate`
- `Immediate`, `Register`
- `LongImmediate`, `Immediate`
- `LongImmediate`, `Register`
- `LongRegister`, `Immediate`
- `LongRegister`, `Register`
- `LongRegister`, `LongRegister`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`
- `LongImmediate`, `LongImmediate`

**Example:**
```armasm
shl !A 3
shl ?LL1 10
shl ?LL1 ?LL2
shl !A !X
shl ?LL1 !A
```

## Jmp

Jumps (moves PC) to the specified label or address, allowing to create infinite loops

**Operands:**
- `Address`
- `IndirectAddress`

**Example:**
```armasm
loop:
; <Do something in a loop>

jmp $loop  ; label is gonna be resolved to the address at compile time
```

## Je

Jumps (moves PC) to the specified label or address only if specified register **NOT equal to zero**, allowing to create branching and loops

**Operands:**
- `Register/LongRegister`, `Address/IndirectAddress`

**Example:**
```armasm
mov !G1 30

; Check against 30
sub G1 30
; If it's not zero - then the initial value isn't 30, meaning we should jump to `else` block
jmp $else_block

; <Do something, G1 = 30>

jmp $if_finished  ; So we're not executing the else block

else_block:

; <Do something else, G1 != 30>

; No need to jump here - if_finished is right away

if_finished:
```
This corresponds to this C code:
```C
if (G1 == 30) {
    // <Do something, G1 = 30>
} else {
    // <Do something else, G1 != 30>
}
```

## Jne

Jumps (moves PC) to the specified label or address only if specified register **equal to zero**, allowing to create branching and loops

**Operands:**
- `Register/LongRegister`, `Address/IndirectAddress`

**Example:**
```armasm
mov !G1 0  ; setup the counter
loop:
; Check if !G1 is equal to target value (let's say 10)
sub !G1 10
jne !A $loop_end

; <Do something in a loop>

; Increment the G1
add !G1 1
mov !A !G1
jmp $loop  ; this is gonna be resolved to the address at compile time

loop_end:
```
This corresponds to this C code:
```C
for (int G1 = 0; G1 != 10; G1++) {
    // <Do something in a loop>
}
```

## Push

Pushes 8-bit value onto the stack (see [Memory](./memory.md))

**Operands**
- `Immediate`
- `Register`

**Example:**
```armasm
push !A
push 10
pop
pop
```

## Pop

Pop 8-bit value off of the stack and stores it in the `!A` register (see [Memory](./memory.md))

**Operands**
- `Immediate`
- `Register`

**Example:**
```armasm
push !A
push 10
pop  ; 10 is in the !A
pop  ; Previous state of the !A is back in the !A
```

> There's no way to push or pop 64-bit values from the stack, tho you can do that with loops and binary shifts if you need to and wrap it into a function

## Call

Pushes current address onto the stack and jumps to the specified location. This is useful when creating functions and, especially, recursive functions

**Operands**
- `Address`
- `IndirectAddress`

**Example:**
*Check the example at* [Memory](./memory.md) *in Stack section*

## Ret
Pops value from the stack and jumps to it, continuing execution after `call`

**Operands**
- `none`

**Example:**
*Check the example at* [Memory](./memory.md) *in Stack section*

## BG

Fills up the VRAM with specified background. Number corresponding to the one in Manifest. Check [Images](./images.md) for more info

**Operands**
- `Immediate`
- `LongImmediate`

**Example:**
```
bg 0
vsync
bg &256  ; In case you have more then 255 backgrounds
vsync
```

## IMG

Blits sprite onto the screen at specified coordinates. Number corresponding to the one in Manifest. Coordinates are 0-indexed, counting from the top left corner. Check [Images](./images.md) for more info

**Operands**
- `Immediate/LongImmediate`, `Immediate/Register`, `Immediate/Register`

**Example:**
```
img 0 30 100
vsync
img &256 !G1 !G2
vsync
```