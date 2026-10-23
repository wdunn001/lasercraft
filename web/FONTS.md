# Bundled fonts

The editor serves one font asset for filled text, `text-sans.ttf`.

## text-sans.ttf

This is Liberation Sans Regular. It is licensed under the SIL Open Font License,
version 1.1. The copyright holders are Red Hat, Inc. (Reserved Font Name
"Liberation", 2012) and Google Corporation (digitized data, 2010). The full
license text travels with the font package and at the SIL OFL page.

The deploy copies the file from the build host's Liberation font package into the
served directory next to `index.html`. The page fetches it at runtime and parses it
with `ttf-parser` in the engine. The WebAssembly binary carries no embedded font.

The single-stroke "Mark" font is a separate clean-room design in the
`lasercraft-text` crate and is the default for laser marking.
