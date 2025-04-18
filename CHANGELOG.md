# CHANGELOG

## Release V0.2.0

### General

- Update UI (ratatui)

### Fixes

- fix makepad_widgts, gen_components check

## Release V0.1.8

### General

- Update deps:
  - makepad_gen_plugin
  - gen_utils

## Release V0.1.7

### Fixes

- fix install_name_tool path point error
- fix check xcode-select
- fix package resources path error


## Release V0.1.6

### Fixes

- Windows packaging resources missing
- fix makepad template `set_text_and_redraw` -> `set_text`
- fix unlinked `gen_components` dep
- fix makepad-widgets dynamic configuration loss

### General

- better env.toml
- config struct for `.env`
- config struct for `env.toml`
- commands
  - uninstall
  - update
- support package `gen_ui`, `makepad` project
- new `.dmg` packing brackground picture
- support package usuall project (not `gen_ui` or `makeapd`)
- under normal circumstances, `ract` can be packaged in multiple ways on the current platform, such as macos (dmg and app)
- integrate `robius-packaging-commands` in ract
  - use goblin crate instead of reading ldd output for parsing (only linux)
- remove `before-each-package-command` and `before-packaging-commands` and use internal processing instead


---

## Release V0.1.1(2)

### Fixes

- Linux build
- warning in macos

## Release V0.1.0 

### General

- add command: `ract add` (add GenUI plugin)
- new `.ract` file
- better toml
  - write
  - parse
- tiny code
- makepad workspace
- For GenUI
  - Watcher
  - Runner
  - Packager
  - Compiler
  - Logger
- Recompile and delete the original compilation results

### Fixes

- Macos `Rustc` toolchain install 
- fix `.env` set missing
- Optimize Macos install Bag

---

## Release V0.0.1 (2024-12-04)

### General

Support following commands:

- init
- check
- install
- config
- create
- run
- studio
- wasm
- pkg