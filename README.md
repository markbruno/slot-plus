# slot.

A bespoke, GBA-centric frontend for the Anbernic RG SP.

Has support for GBA titles only.

A full user guide can be found at [slot.kowalski.io](https://slot.kowalski.io).

## Shaders and cheats (fork additions)

This fork adds two things to slot: shaders for the game screen, and cheats.

### Shaders

Put single-pass RetroArch `.glsl` shaders in `Shaders/` on the card. Tap `MENU` on the
carousel and use the **Shader** row to pick one with Left and Right. It is saved like every
other setting and applies to every game.

- **LCD** is slot's own look and stays the default. **Off** is the plain picture at 3x.
- Only single `.glsl` files work, not `.glslp` presets or `.slang` shaders.
- Shader parameters run at their defaults.
- Add the line `#pragma slot_filter linear` to a shader that expects smooth filtering, such
  as sharp-bilinear. RetroArch ignores it.
- A shader can read the previous frame through `PrevTexture`, for frame blending.
- If a shader fails to compile, slot shows "Shader failed", goes back to LCD, and writes the
  compiler's error to its log.

`examples/shaders/blend-grid.glsl` is a small example to start from.

### Cheats

Put a RetroArch cheat file at `Cheats/GBA/<rom name>.cht`, named like the rom, the same way
labels are. Files from the libretro cheat database work as they are:

```
cheats = 1
cheat0_desc = "Infinite HP"
cheat0_code = "82003B4C+0063"
cheat0_enable = true
```

Only cheats with `cheatN_enable = true` run. Edit the file on a computer to choose them.

In a game, `SELECT` + `X` turns all of that game's enabled cheats off and on again. The
choice is remembered per game in `System/cheats.ini`. Cheats are skipped in link cable
sessions.

The first time cheats run on a game, its battery save is copied to
`Saves/GBA/<rom name>.sav.before-cheats`. Some cheats can break a save for good, so keep that
copy until you're sure.

mGBA supports GameShark, Action Replay and CodeBreaker codes. If a code does nothing on gpSP,
try the game on mGBA.

## AI Disclosure

The Rust frontend was put together by Claude Opus. I reviewed everything that was
produced. All documentation is 100% free-range, meatbag prose.

The project is extremely low stakes. I wanted a bespoke frontend for my RG SP and thought
that something that evokes the feeling of using my GBA SP as a kid would be pretty neat.

Use it, don't use it, I don't care. 

Figured I should share the end result of all the wasted water. ✌🏻