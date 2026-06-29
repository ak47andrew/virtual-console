# Operand types

Okay, before getting the juicy stuff which is actual instructions you need to know all the different types of operands

Operand is the something you pass to the instruction to specify what specifically you want it to do. For example what value to move to what place

```armasm
mov 12 $0  ; 12 and $0 are operands!
```

Oh and before we start: all numbers can be written with three different forms
- 1, 15, 159 - basic decimal, base 10
- 0xFF, 0x123 - hexadecimal, base 16
- 0b1010, 0b1000001 - binary, base 2

This works everywhere where you need to pass a number

## Address

Written like this: `$100`, `$0xAB`, `$0b10001010`

It's the address in main memory (check [Memory](./memory.md)). Usually refers to whatever byte is at this address, but it's not always the case:
- If all other variations looks like 8-bit data - it's whatever value sits in this specific address in main memory
- If all other variations looks like 64-bit data - it's whatever value sits at range of this address and 8 bits forward, big endian style
- If it's in `add` - it's just a address as number/index and you should only do that with labels to create jump tables and stuff
- If it's in `call` or `jmp` - it's a target address you need to be executed next

## Immediate


Simple 1 byte (8 bit) value! Used across the table. Written like this: `10`, `0xF`, `0b101`

## LongImmediate

8 bit (64 bit) value. Written like this: `&256`, `&0xFFF`, `&0b1111111111111111111`

## LongerImmediate

However large value you want (uhh... not really, but big enough for anything you need). Written like this: `^17590867159768927456096789568934698759469810`, `^0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF`, `^0b11111....1111`

## Register

Place where you can put a 8-bit value to just hang out there until overwritten. Prefixed with `!` at assembly, for example `!A`, `!G1`. Check [Memory](./memory.md) for most info

## LongRegister

Place where you can put a 64-bit value to just hang out there until overwritten. Prefixed with `?` at assembly, for example `?LL1`, `?GP1`. Check [Memory](./memory.md) for most info


## IndirectAddress

It's if LongRegister and Address had a baby. You just take whatever value is written at LongRegister and treat it as it's an address. Wherever you can pass an Address, you can pass IndirectAddress as well. Written something like this: `[?LL1]`, `[?PC]`
