### Mov

Move data between same-size containers. First operand is the source, second - destination. If you need transfer something between different sizes - you probably need to use `trunc` or `ext`

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