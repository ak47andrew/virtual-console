### Sub

Subtract two numbers of the same size and stores result in a register. For 8-bit values it's `!A`, for 64-bit - `?LL1`. If underflow occurred, `!Z` is set to `1`, otherwise it's set to `0`

**Operands:**
- `Immediate`, `Immediate`
- `LongImmediate`, `LongImmediate`
- `Register`, `Register`
- `LongRegister`, `LongRegister`
- `Immediate`, `Register`
- `Register`, `Immediate`
- `LongRegister`, `LongImmediate`
- `LongImmediate`, `LongRegister`