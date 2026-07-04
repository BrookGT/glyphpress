
# OpenType coverage in glyphpress

glyphpress implements pragmatic parsing for TrueType outlines shipped in the `glyf`/`loca`
pair and Unicode mapping via `cmap` format 4 and format 12 subtables.

## Supported tables

| Tag | Purpose |
|-----|---------|
| head | Font header, loca index format |
| hhea | Horizontal typographic metrics |
| maxp | Glyph count and outline limits |
| loca | Glyph byte ranges in glyf |
| glyf | Simple and composite outlines |
| hmtx | Advance widths and LSBs |
| cmap | Character to glyph mapping |
| name | Localized name strings |
| post | PostScript glyph names |
| OS/2 | Weight/width and Unicode ranges |
| kern | Legacy kerning pairs |
| GDEF/GPOS/GSUB | Layout table headers |

## Subsetting

The subset pipeline closes glyph sets over composite references, remaps glyph indices,
and emits a new SFNT with a rebuilt `cmap` format 4 subtable.
