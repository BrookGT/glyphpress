
# glyphpress

glyphpress is an OpenType and TrueType SFNT parser, validator, and glyph subsetter
aimed at embedded devices with tight memory budgets. It parses core TrueType tables
(`head`, `hhea`, `maxp`, `loca`, `glyf`, `hmtx`, `cmap`, `name`, `post`, `OS/2`),
optional layout tables (`GDEF`, `GPOS`, `GSUB`), and can build subset fonts with
recalculated checksums.

## Building

```sh
cargo vendor
cargo check --workspace --offline
```

## Fuzzing

```sh
cargo fuzz run sfnt_fuzzer --offline
```

See [docs/OPENTYPE.md](docs/OPENTYPE.md) for table coverage notes.

ClusterFuzzLite builds use `.clusterfuzzlite/build.sh` with vendored crates for fully offline CI.
The library targets embedded devices with zero-copy table views and scratch outline walks.
Subset emit rebuilds trimmed SFNT tables with recalculated checksums for constrained targets.
