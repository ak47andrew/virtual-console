### Trunc

Move data from higher-size container to the lower-size. Only lower bytes is gonna be saved (Big-endian), everything else is gonna be **trunc**ated. Usually used to move 64-bit register to 8-bit one

**Operands:**
- `LongImmediate`, `Register`
- `LongerImmediate`, `Register`
- `LongRegister`, `Register`
- `LongerImmediate`, `LongRegister`