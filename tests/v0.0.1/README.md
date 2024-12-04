# Test-v0.0.1

- date: 2024-12-04
- tester: Will Sheng

## Goal

- init
- check
- install
- config
- create
- run
- studio
- wasm
- pkg

## Platforms

### Macos

- [x] Goal

#### Issues

##### Makepad Wasm

```
Uncaught SyntaxError: unexpected token: '{'

use {WasmWebGL} from './makepad_platform/web_gl.js'
```

### Windows

- [x] Goal

#### Issues

##### Makepad Wasm

```
Uncaught SyntaxError: unexpected token: '{'

use {WasmWebGL} from './makepad_platform/web_gl.js'
```

##### pkg

1. Missing resource files
2. Need to run `cargo build --release` before package (missing target/releases/${project.exe})
3. setup_exe can not execute in other windows computer (link: 1)

### Linux (Ubuntu)

- [x] Goal