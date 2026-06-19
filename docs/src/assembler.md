# Assembly language

Assembly language is something you're gonna be working. A lot. So pay attention, kids, this is really important

Assembly language here is called VEA which stands for... Virtual emulator assembly.
<details>
    <summary>Off-topic about naming and extensions</summary>

First of all... what is `virtual emulator`? Isn't emulator already virtual? Bruh *Skull emoji*

But yeah and VEA it's only because it was the only thing that came to my mind when naming this docs and... now it's there forever ig. So yeah, the thing I poured a lot of work and 1 month of time into is named... "Virtual emulator assembly fantasy console"... I need to get some professional help ASAP

If talking about extensions:
- `.vea` (Virtual emulator assembly). Already talked about it, insane naming
- `.veb` (Virtual emulator binary): the actual executed binary... At least it's got some connection
- `.vec` (Virtual emulator cartridge): whole bundle that console executes. And I swear the ve**a**, ve**b** and ve**c** is the coincidence
- `.rfe` (Random f\*\*king extension). The `.pal` files after compiling turn this into clean byte sequence and it would be called `.act` which apparently it isn't (due to being variable-length and having alpha)

---
</details>

Overall the language is pretty much your usual assembly language. Instructions are split with newlines, first word is the instruction (or opcode) - basically what you need to do, second and later are operands (arguments) that specifies on what things to do that.

For example `mov` will tell console to move the data, first operand `!A` will tell it to pull value from register `A` and second operand `$0` will tell it to put it on the first pixel of the screen (check [Memory](./memory.md))

## Comments

This only supports one-line comments and they start with semicolon (`;`)

No multiline or docstrings for ya, sorry!

```armasm
; This is the comment!
hlt  ; I can also be up here too!
; And you can't break the comment ; vsync - nope, not gonna work
```

## Labels

Actually feasible way of doing addresses! Jumping to `$1238517` is... strange and unreliable. So what can you do instead?

Introducing: Labels! Just drop it at any line of your code with `name:` and you can jump to it instantly - compiler is gonna do the heavy lifting

```armasm
infinite_loop:

jmp $infinite_loop  ; See? Much better then `jmp $1238157`!
```

Oh and two more things:
- For different types of operands that exist check out [Operand types](./operands.md)
- For all instructions that exist see [Instruction description](./opcodes.md)