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

Cheats marked `cheatN_enable = true` turn on when the game starts.

To choose cheats on the device, press `SELECT` + `X` in a game. The game pauses and a list of
that game's cheats appears:

| Input          | Action                                   |
| -------------- | ---------------------------------------- |
| `Up` `Down`    | Move through the list                    |
| `L1` `R1`      | Jump a page                              |
| `A`            | Turn the highlighted cheat on or off     |
| `Left` `Right` | Turn it off / on                         |
| `B`            | Close the list and go back to the game   |

Your choices are written back into the `.cht` file, so they're kept for next time and the file
still works in RetroArch. Cheats are skipped in link cable sessions.

The first time cheats run on a game, its battery save is copied to
`Saves/GBA/<rom name>.sav.before-cheats`. Some cheats can break a save for good, so keep that
copy until you're sure.

mGBA supports GameShark, Action Replay and CodeBreaker codes. If a code does nothing on gpSP,
try the game on mGBA.

## 12-hour clock

Tap `MENU` on the carousel and turn on **12-Hour Clock** to show times as 3:07 PM rather
than 15:07: on the shelf, in the menu's Date & Time, and on older save states. The screen for
setting the clock still uses 24-hour time.

## AI Disclosure

The Rust frontend was put together by Claude Opus. I reviewed everything that was
produced. All documentation is 100% free-range, meatbag prose.

The project is extremely low stakes. I wanted a bespoke frontend for my RG SP and thought
that something that evokes the feeling of using my GBA SP as a kid would be pretty neat.

Use it, don't use it, I don't care. 

Figured I should share the end result of all the wasted water. ✌🏻