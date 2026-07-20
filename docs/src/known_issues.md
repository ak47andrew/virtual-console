# Known Issues

There's a bunch of things that doesn't work as expected or have questionable naming/behavior

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
