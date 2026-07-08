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

Written like this: `$100`, `$0xAB`, `$0b10001010` or as a label `$loop`

It's the address in main memory (check [Memory](./memory.md)). Usually refers to whatever byte is at this address, but it's not always the case:
- If all other variations looks like 8-bit data - it's whatever value sits in this specific address in main memory
- If all other variations looks like 64-bit data - it's whatever value sits at range of this address and 8 bits forward, big endian style
- If it's a second operand of an arithmetic or binary instruction - it's just a address as number/index so you can create jump tables
- If it's in `call` or `jmp` - it's a target address you need to be executed next

## Immediate


Simple 1 byte (8 bit) value! Used across the table. Written like this: `10`, `0xF`, `0b101`

## LongImmediate

8 bit (64 bit) value. Written like this: `&256`, `&0xFFF`, `&0b1111111111111111111`

## LongerImmediate

However large value you want (uhh... technically u64::MAX bytes at most, but that's 16 Exabytes! so you're covered). Written like this: `^17590867159768927456096789568934698759469810`, `^0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF`, `^0b11111....1111`

## Register

Place where you can put a 8-bit value to just hang out there until overwritten. Prefixed with `!`, for example `!A`, `!G1`. Check [Memory](./memory.md) for info on registers

## LongRegister

Same as register, but 64-bit. Prefixed with `?`, for example `?LL1`, `?GP1`. Check [Memory](./memory.md) for info on registers


## IndirectAddress

It's if LongRegister and Address had a baby. Console just takes whatever value is written at LongRegister and treats it as it's an address. Wherever you can pass an Address, you can pass IndirectAddress as well. Written something like this: `[?LL1]`, `[?PC]`
