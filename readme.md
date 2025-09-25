# metro-loop

## Play

Play in your browser (or download the game) at https://jmmut.itch.io/metro-loop.

## Compiling and running this project

Clone this repo, then [Install rust](https://www.rust-lang.org/tools/install), then do `cargo run --release`.

If that fails, you might need some system packages. If you're on linux, see how [the CI code](.github/workflows/release.yml) installs the dependencies. Look for `apt-get install`.

## Ideas for improvements

### Major

- [/] options screen
  - [ ] should add a slider for the volume?
  - [ ] a checkbox for enabling disabling sound?
- [ ] campaign visualization screen
- [ ] add button for resetting level
- [ ] diagonal constraint
- [ ] rail user constraint (clickable corners)

### Minor

- [/] Select difficulty (should have easy/medium/hard?)
- [ ] rendering details
  - [ ] constraint animations
  - [ ] make constraint icons scale smoothly
  - [ ] triangles should not have direction when inside constraints on unconnected rails
  - [ ] sin(color lightness) on hover satisfaction failures
- [/] sound
  - [ ] better sounds
  - [ ] longer music
  - [ ] better sound balance. Higher music, lower effects.
- [ ] contrasting font for the title?

## Bugs

- when pressing "options", the button in the same position stays highlighted in phones
- remember procedural map when going to main menu



