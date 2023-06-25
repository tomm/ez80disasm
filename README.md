# ez80disasm

A disassembler for ADL and Z80 eZ80 code

Build with:

```cargo build -r```

To use:

```./target/release/ez80disasm <input.bin>```

The disassembler defaults to ADL=1 (24-bit) disassembly, but
can be put into ADL=0 (Z80) mode with:

```ez80disasm -z80 <input.bin>```
