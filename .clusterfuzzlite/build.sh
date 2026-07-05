#!/bin/bash -eu
# Fenrir sets $SRC to the extracted repository root.
cd "$SRC"

export CARGO_TARGET_DIR="$SRC/target"

# Vendor workspace deps (fuzz crate pulls libfuzzer-sys from crates.io).
cargo vendor vendor
mkdir -p .cargo
cat > .cargo/config.toml <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

TARGETS=(
  sfnt_fuzzer
  cmap_fuzzer
  glyf_fuzzer
  name_fuzzer
  subset_fuzzer
  pipeline_fuzzer
)

BIN_ARGS=()
for t in "${TARGETS[@]}"; do
  BIN_ARGS+=(--bin "$t")
done

cargo build \
  --release \
  --package glyphpress-fuzz \
  --features libfuzzer \
  "${BIN_ARGS[@]}"

for t in "${TARGETS[@]}"; do
  cp "target/release/$t" "$OUT/$t"
  corpus_dir="fuzz/corpus/${t}"
  if [ -d "$corpus_dir" ]; then
    (cd "$corpus_dir" && zip -q -r "$OUT/${t}_seed_corpus.zip" .)
  fi
done
