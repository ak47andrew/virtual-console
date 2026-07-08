# Introduction

Hello! And welcome to this documentation of... "VEA Fantasy console" ig... *(I'm bad at naming things, sorry ¯\\\_(ツ)_/¯)*

According to Wikipedia:
> A fantasy video game console (or simply fantasy console) is an emulator for a fictitious video game console. It aims to create the experience of retro gaming without the need to emulate a real console, allowing the developer to freely decide what specifications their fictional hardware will have.

And this one was created because I watched a bit too much 100th Coin's videos about Mario TASing and figured out that I wanted to write a NES-like game console. Add 1 month to this and you get what's in front of you

---

Technically this console has a 64-bit(-ish, check [Known issues](./known_issues.md)) CPU with 8-bit display and inputs

It has 256 color palette, 8 buttons with HELD/PRESSED distinction captured each frame, 256x240 screen size (that's upscaled to 1024x960), 256x240 backgrounds and 8x8 sprites. From internal perspective it has monolithic memory logically split into VRAM, RAM, Stack and ROM, 9 8-bit registers, 7 64-bit registers, 28 instruction and 7 prefixed operand types. And you can changes sizes of RAM and Stack to whatever you need it to be

---

This is in lesser sense a thoughtful documentation, but rather somewhat chaotic stream of thoughts and knowledge on how write games on this fantasy console as well as few design choices I made when making this 

Let me outline what this thing contains:
- [Introduction](./intro.md) - You're currently reading this. Shocking, right?
- [Memory](./memory.md) - You're gonna move data. A LOT. So probably it's better to figure out what and where, right?
- [Manifest](./manifest.md) - Shows the structure of the manifest file and how to properly set it up
- [Assembler](./assembler.md) - Information about `.vea` files and assembler language this console understands
    - [Operand types](./operands.md) - Existing operand (argument) types
    - [Instruction description](./opcodes.md) - List of all instructions/opcodes, their signatures and examples
- [Palette](./palette.md) - Explains what palette is, how to create and use one
- [Images](./images.md) - Explains how to make images to make the compiler shut up
- [Known Issues](./known_issues.md) - Dirty sheets: something that's not working and explanation about edge-cases and odd behavior of different parts of the console

---

