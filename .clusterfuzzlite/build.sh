#!/bin/bash -eu
cd "$SRC/glyphpress"

# Vendor at build time (not checked into git) so repo fingerprints are first-party code.
cargo vendor vendor
mkdir -p .cargo
cat > .cargo/config.toml <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

cargo fuzz build -O
FUZZ_TARGET_DIR="fuzz/target/x86_64-unknown-linux-gnu/release"
for t in sfnt_fuzzer cmap_fuzzer glyf_fuzzer name_fuzzer subset_fuzzer pipeline_fuzzer; do
  cp "$FUZZ_TARGET_DIR/$t" "$OUT/"
  if [ -d "fuzz/corpus/$t" ]; then
    (cd "fuzz/corpus/$t" && zip -qr "$OUT/${t}_seed_corpus.zip" .)
  fi
done
