# Known Issues

There's a bunch of things that doesn't work as expected or have questionable naming/behavior

## Addresses are fucked up hard

```
[2026-07-08T13:14:44Z][INFO]: Compiling project folder "source/"...
[2026-07-08T13:14:44Z][INFO]: Parsed manifest at source/manifest.toml
[2026-07-08T13:14:44Z][INFO]: === SECTIONS ===
[2026-07-08T13:14:44Z][INFO]: VRAM: 0-61440
[2026-07-08T13:14:44Z][INFO]: RAM: 61441-63441
[2026-07-08T13:14:44Z][INFO]: STACK: 63440-64440
[2026-07-08T13:14:44Z][INFO]: ROM: 64441-...
[2026-07-08T13:14:44Z][INFO]: === SPECIAL ADDRESSES ===
[2026-07-08T13:14:44Z][INFO]: INPUT_HELD: $63438
[2026-07-08T13:14:44Z][INFO]: INPUT_PRESSED: $63439
```

Just when adding updating the docs I found out that sections and special addresses it reports are completely messed up and I sense that in the actual runtime it also somewhat messed up. But since fixing it will mean a breaking change (if it's actually incorrect addresses at runtime) and it's not breaking any game - it has to wait for the next major update. For now, when setting up RAM size to what you need, add 50-100 bytes more just in case. Both input addresses are working as intended tho (for some reason, I really need to reread/rewrite parts of my memory management code)

(And yeah, I hand-edited the example in [Assembly language](./assembler.md) just so it's not confusing there)

## Compiler naming

Yes, it's technically not a compiler anymore (and it would be called assembler anyway) because it's doing quite a bunch of things: convert assembly to bytecode, re-encode images and pallet and then pack all of it into a cartridge. So I'm settled on a name `builder`, but it's not a major fix so it also has to wait

## CPU cycles

I never thought about them existing so if we take one `step()` call to be one CPU cycle it would mean that it can read as much as (1) (`mov` opcode) + (1 + 8 + u64::MAX) (`LongerImmediate`) + (1 + 8)(`Address`) and write u64::MAX bytes in a single cycle. Maybe I'll come back to it and fix it, maybe not - I don't really know

Adding to this, it's technically 64-bit CPU due to Addresses being u64s under the hood, but this CPU cycle shenanigans are kinda messing with it

I'm classifying it as "64-bit CPU with 8-bit peripherals", but you do you

## Invalid labels

For now, there's no check for labels so this:

```armasm
0x80:
dbgsec 50

<...>

jmp $0x80
```

instead of jumping to the address `0x80` it's gonna jump to label `0x80`. This is also gonna be fixed in the nearest future, just... don't be stupid and it's not gonna touch you, okay?
