
# glyphpress

glyphpress is an OpenType and TrueType SFNT parser, validator, and glyph subsetter
aimed at embedded devices with tight memory budgets. It parses core TrueType tables
(`head`, `hhea`, `maxp`, `loca`, `glyf`, `hmtx`, `cmap`, `name`, `post`, `OS/2`),
optional layout tables (`GDEF`, `GPOS`, `GSUB`), and can build subset fonts with
recalculated checksums.

## Building

```sh
cargo vendor vendor
cargo check --workspace --offline
```

ClusterFuzzLite runs `cargo vendor` during the build (dependencies are not committed).

## Fuzzing

```sh
cargo fuzz run sfnt_fuzzer
```

See [docs/OPENTYPE.md](docs/OPENTYPE.md) for table coverage notes.

## License

MIT
