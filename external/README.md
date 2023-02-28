# External libraries (with patches)

Some external libraries that with some custom patches for some features `Festival` needs.

| Library | Patch | Purpose | Merged upstream? |
|---------|-------|---------|------------------|
| `egui`  | Add `trailing_fill()` bool toggle to `Slider` | Adds color to a slider similar to a `ProgressBar` | [Yes](https://github.com/emilk/egui/pull/2660)
| `egui`  | Add `thickness()` to `Slider` | Change thickness (not length/width) of a `Slider` | No