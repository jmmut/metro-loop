# metro-loop

## Play

Play in your browser (or download the game) at https://jmmut.itch.io/metro-loop.

## Compiling and running this project

Clone this repo, then [Install rust](https://www.rust-lang.org/tools/install), then do `cargo run --release`.

If that fails, you might need some system packages. If you're on linux, see how [the CI code](.github/workflows/release.yml) installs the dependencies. Look for `apt-get install`.

## Ideas for improvements

### Major

- [/] options screen
  - [x] should add a slider for the volume?
  - [-] a checkbox for enabling disabling sound?
- [x] campaign visualization screen.
  - [-] button for previous level? redundant?
- [x] add button for resetting level
- [ ] diagonal constraint
- [x] rail user constraint (clickable corners)
- [ ] animation
  - [ ] lightness = sin(t) or circling movement for unsatisfied constraints when hovering on the right panel
    - [ ] stations and bridges
    - [ ] unlinked rails
    - can't animate missing blocks, can I?
  - [ ] lightness = sin(t) or lightness wave when puzzle is solved

### Minor

- [/] Select difficulty (should have easy/medium/hard?)
- [ ] rendering details
  - [x] make constraint icons scale smoothly
  - [-] triangles should not have direction when inside constraints on unconnected rails. (decided to keep same icon for incorrect rails)
  - [ ] textures for + and -
- [/] sound
  - [ ] better sounds
  - [ ] longer music
  - [ ] better sound balance. Higher music, lower effects.
- [ ] contrasting font for the title?

- [ ] fix hover
- [ ] draw ghosts for right click dragging
- [ ] move NEXT below SOLVED and above RESTART, move CAMPAIGN to bottom center
- [ ] music off by default
- [ ] name for bridges (ui is not clear, icons don't explain anything)
- [ ] didn't see level was solved,

## Bugs

- when pressing "options", the button in the same position stays highlighted in phones



