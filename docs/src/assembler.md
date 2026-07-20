# Assembly language

Assembly language is something you're gonna be working with. A lot. So pay attention, kids, this is really important

Assembly language here is called VEA which stands for... Virtual emulator assembly.
<details>
    <summary>Off-topic about naming and extensions</summary>

First of all... what is `virtual emulator`? Isn't emulator already virtual? Bruh *Skull emoji*

But yeah and VEA it's only because it was the only thing that came to my mind when naming this docs and... now it's there forever ig. So yeah, the thing I poured a lot of work and 1 month of time into is named... "Virtual emulator assembly fantasy console"... I need to get some professional help ASAP

If talking about extensions:
- `.vea` (Virtual emulator assembly): already talked about it, insane naming
- `.veb` (Virtual emulator binary): the actual executed binary... At least it's got some connection
- `.vec` (Virtual emulator cartridge): whole bundle that console executes. And I swear the ve**a**, ve**b** and ve**c** is a coincidence
- `.rfe` (Random f\*\*king extension): my favorite of them all. The `.pal` file, after compiling, is encoded into clean byte sequence and saved in `.rfe` file. And it would be called `.act` which apparently it isn't (due to being variable-length and having alpha)

---
</details>

Overall the language is pretty much your usual assembly language. Instructions are split with newlines, first word is the instruction (or opcode) - basically what you need to do. Other are operands (arguments) that specifies sources, destinations, sizes, dev's mother phone numbers and so on.

For example `mov` will tell console to move the data, first operand `!A` will tell it to pull value from register `A` and second operand `$0` will tell it to put it on the first pixel of the screen (check [Memory](./memory.md))

## Comments

This only supports one-line comments and they start with semicolon (`;`)

No multiline for ya, sorry!

```armasm
; This is the comment!
hlt  ; I can also be up here too!
; And you can't break the comment ; vsync - nope, not gonna work
```

Unofficial docstrings convention for functions (which I personally use) is this:
```armasm
;; ---
;; remove_bullet(n) -> void
;; Takes n in !G1 - index of the bullet
;; Removes bullet at nth index
;; ---
remove_bullet:
    ext !G1 ?LL1
    mul ?LL1 &4
    add ?LL1 $61445
    mov 0 [?LL1]
    ret
```

## Labels

Actually feasible way of doing addresses! Jumping to `$1238517` is... strange and unreliable. So what can you do instead?

Introducing: Labels! Just drop it at any line of your code with `name:` and you can jump to it instantly - compiler is gonna do the heavy lifting

```armasm
infinite_loop:

jmp $infinite_loop  ; See? Much better then `jmp $1238157`!
```

## HELD and PRESSED addresses

The input consists of two bytes at the very end of the RAM section: HELD and PRESSED. So if at previous frame you pressed "space" and "c" and at start of this frame you’re continuing pressing "space", but release "c" and press "x", then HELD byte is going to have "space", "c" and "x" in it, while PRESSED is only gonna have "x" pressed. Makes sense? Here's a little example just in case:

```armasm
mov $63438 !G3
and !G3 0b10000000  ; Checking the "up" button
; If up is pressed, then !A register is gonna be set to 1
; So now you can do jz/jnz from it
```

The exact schema of the byte is as follows:
`[Up][Down][Left][Right][Z][X][C][Space]`

Due to this addresses being dependent on the RAM size you set in the Manifest, you need to look at the compiler's output. When you compile your project it will output something like this:
```
[2026-07-08T13:04:20Z][INFO]: Compiling project folder "source/"...
[2026-07-08T13:04:20Z][INFO]: Parsed manifest at source/manifest.toml
[2026-07-08T13:04:20Z][INFO]: === SECTIONS ===
[2026-07-08T13:04:20Z][INFO]: VRAM: 0-61440
[2026-07-08T13:04:20Z][INFO]: RAM: 61441-63440
[2026-07-08T13:04:20Z][INFO]: STACK: 63440-64439
[2026-07-08T13:04:20Z][INFO]: ROM: 64440-...
[2026-07-08T13:04:20Z][INFO]: === SPECIAL ADDRESSES ===
[2026-07-08T13:04:20Z][INFO]: INPUT_HELD: $63438
[2026-07-08T13:04:20Z][INFO]: INPUT_PRESSED: $63439
```
This will help you in both figuring out where main memory's data is and what addresses to pull input data from

## `new` argument

When running compiler you can write something like this: `vea_compile new CoolProject` and this will create a `CoolProject` folder with full hierarchy you might need for creating a project: filled up Manifest, minimal palette, script and folders for backgrounds and sprites. This is the quick way to create a new project

Oh and two more things:
- For different types of operands that exist check out [Operand types](./operands.md)
- For all instructions that exist see [Instruction description](./opcodes.md)
