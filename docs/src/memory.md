# Memory

There's quite a few places where data is stored and read, we're gonna visit them one at a time. Starting with...

## Main memory

...

I already said that I'm good with naming, right?

So basically, it's your main memory chip, accessible with addresses (prefixed with `$` in assembly). And it's split into multiple logical parts. And since they're only logical it's easy to overflow some addresses and start writing to RAM while thinking you're still in VRAM and writing to ROM at any point would probably cause console to execute inexistent instruction and crash

### VRAM

The `0 - 61439 (240 * 256 - 1)` space is for pixels rendered on the screen. Each byte is corresponded to the palette to get the color value and placed on the screen each `vsync`.

So to, for example, draw a little smile at the top left of the screen you would do something like this:
```
mov 2 $0   ; row 0, column 0
mov 2 $2   ; row 0, column 2
mov 2 $260 ; row 1, column 3
mov 2 $512 ; row 2, column 0
mov 2 $514 ; row 2, column 2
```

So, to translate pixel coordinate (from the top-left, indexed from 0) you take the y coordinate, multiply it by 256 and add x coordinate: `y * 256 + x`

### RAM

It starts after the VRAM and spreads for `ram_size` bytes (check [Manifest](./manifest.md))

This is basically your workspace: you can use it to store variables, manipulate bytes in any way, etc.

For example to store player's score you can decide on address (let's say 61441) and then update when needed:

```
update_score:
    mov $61441 ?LL1
    add ?LL1 &1
    mov ?LL1 $61441
    ret
```

Last two cells of the RAM are reserved for user input: held and pressed buttons. You can learn more about it at [Assembly section](./assembler.md)

### Stack

This is the part where data goes on `push` and `pop` commands as well as calling the function with `call` and returning with `ret`. For example

```
; Pushing values onto the stack
push 2
push 3
push 4
push 5

; Popping into !A and drawing at the screen
pop  ; moves 5 to !A
mov !A $0

pop  ; moves 4 to !A
mov !A $1

pop
mov !A $2

pop
mov !A $3

vsync
```

It's especially useful if you're working with functions (this example is a bit complex, but I believe in ya <3)

```
jmp $start

factorial:  ; factorial of number N is N * (N - 1) * ... * 2 * 1
    mov !A !G1          ; save n
    sub !A &1           ; n-1, sets !Z to 1 on underflow
    je !Z $base_case    ; n was 0 -> return 1
    je !A $base_case    ; n-1 == 0, so n was 1 -> return 1
    mov !G1 !A          ; restore n into !A for the next check
    sub !A &1           ; !A = n-1 again (we need it as argument)
    push !G1            ; save n on stack before recursing
    call $factorial     ; !A = fact(n-1)
    mov !A !G2          ; save recursive result
    pop                 ; !A = n (restored from stack)
    mov !A !G1          ; put n back in !G1
    mul !G2 !G1         ; !A = fact(n-1) * n
    ret
base_case:  ; this is basically the other branch of the same function. 
    mov &1 !A
    ret  ; Since we jumped here and not called - the last address is still belongs to factorial's call which ret uses

start:
    ; so, to call factorial, we need to set !A as an argument and call
    ; Tho keep in mind that 5 is basically the max this example can take. Since we're using 1 byte (max value it can store is 255) and 6! is already 720

    mov 5 !A
    call factorial
    ; Result is also at !A so we can... idk, put it on the screen
    mov !A $0
    vsync
```

### ROM

Well... ROM is usually stands for `Read-only memory`, but you can actually edit this one so it's more of a `program storage`

So, not much to say here. And yes you can write self-modifying code, but... it's a pain in the ass and there's no reason to tbh

## Registers

Registers are like variables you can use during your runtime. There's two types of them depending on their size: 8-bit (Register) and 64-bit (LongRegister) ones 

### Register

This registers are prefixed with `!` and split in two different categories:
- Program space: `!A`, `!X`, `!Y`, `!Z` - this ones can be touched by different instructions. For example `div` saves the result to the `!A` register and the remainder to the `!Z`
- User space: `!G1` to `!G5` - this one are completely there for your comfort. Drop a thing there and it's gonna wait for ya when you'll need it

### LongRegister

This registers are prefixed with `?` and split follow the same schema:
- Program space: `?LL1`, `?LL2`
- User space: `?GP1` to `?GP3`
- Special registers: there's also two LongRegisters that're used elsewhere
    - `?PC` (Program counter): it points to the address that's gonna be executed at the next step
    - `?SP` (Stack pointer): handles where to read and write stack to

