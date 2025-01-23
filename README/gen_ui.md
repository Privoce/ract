# Ract for GenUI

> ![NOTE]
> 
> Whether it is Makepad or GenUI, the usage of `init`, `check`, `install`, `config`, `run`, `wasm` commands is the same
>
> You only need to focus on: `create`, `add`, `pkg`
>
> What's more `init`, `check`, `config` are automatic

## 📖 Table of Contents
- [Ract for GenUI](#ract-for-genui)
  - [📖 Table of Contents](#-table-of-contents)
  - [🚀 Usage](#-usage)
    - [`init` - Initialize Ract](#init---initialize-ract)
    - [`check` - Check Toolchain](#check---check-toolchain)
    - [`install` - Install Toolchain](#install---install-toolchain)
    - [`config` - Configure CLI](#config---configure-cli)
    - [`create` - Create a GenUI or Makepad Project](#create---create-a-genui-or-makepad-project)
    - [`run` - Run a Project](#run---run-a-project)
    - [`add` - add dev plugin](#add---add-dev-plugin)
    - [`wasm` - Run WASM Project in Browser](#wasm---run-wasm-project-in-browser)
    - [`pkg` - Package a Project](#pkg---package-a-project)
---

## 🚀 Usage

### `init` - Initialize Ract  
Initialize or reset the CLI. Ract will generate:  
1. `.env`  
2. `chain/`  
   - `chain/env.toml`  

```bash
ract init
```

Output:   
```plaintext
🚀 Start to init ract...
✅ Chain init successfully!
🎉 Init ract successfully!
```

---

### `check` - Check Toolchain  
Check if required tools and dependencies are installed. Options include:  
- **Basic**: [cargo, rustc, git]  
- **Underlayer**: [makepad (gen_ui, makepad)]  
- **All**: Combines both basic and underlayer tools.  

```bash
ract check
```

Interactive dialog example:  
```plaintext
🥳 Welcome to use ract checker!
? Which you need to check?
> Basic
  Underlayer
  All
```

---

### `install` - Install Toolchain  
Install required tools and dependencies for development. Available options:  
- **Rust tools**: `rustc`, `cargo`  
- **Version control**: `git`  
- **Makepad-specific tools**: Includes components like `gen_components`, `wasm_build`, and more.  

```bash
ract install
```

Interactive dialog example:  
```plaintext
🥳 Welcome to use ract Install!

🔸 Select the tools to install:
  - rustc
  - cargo
  - git
  - makepad tools (default or custom options)
? What tools you want to (re)install?
> [ ] rustc|cargo
  [ ] git
  [x] makepad
```

---

### `config` - Configure CLI 
 
Set or update environment variables and CLI configurations.  

> [!TIP]
> 
> In most cases, you don't need to configure anything unless you already have Ract dependencies that require special pointers.

```bash
ract config
```

Interactive dialog example: 
```plaintext
🥳 Welcome to use ract config!

🔸 env: Set the `path` for the chain env.toml file
🔸 chain_env_toml: Set the rust dependency for GenUI toolchain

> Which env file do you want to config? chain_env_toml
> Get or Set Config? set
> Which one do you want to config? makepad-widgets
> Path: /Users/shengyifei/projects/makepad/makepad
🎉 Config finish!
```

---

### `create` - Create a GenUI or Makepad Project

Ract will build a **Makepad** or **GenUI** project template based on the configuration entered by the user

**create will help you check current env states**

```bash
ract create
```
Interactive dialog example:  

```plaintext
❤️ WELOCME TO GENUI, ract is a build tool for you!

> Which project you want to create? gen_ui
> Project name: test1
> Authors name: John
> ...
🎉 Your project has been created successfully!
```

---

### `run` - Run a Project  

Run **Makepad** or **GenUI** projects.  

```bash
ract run
```

---

### `add` - add dev plugin

```bash
ract add gen_makepad_http
```

---

### `wasm` - Run WASM Project in Browser  
Build and run a WASM project directly from the CLI.  

```bash
ract wasm
```

Interactive dialog example:  
```plaintext
🥳 Welcome to use ract wasm!

🔸 Port for the web studio 8888
📦 wasm is being packaged
🚀 wasm is being started...
Starting webserver on 127.0.0.1:8888
```

---

### `pkg` - Package a Project  
Package a project using `cargo-packager`.  

```bash
# cd to compiled project package
cd src_gen_0

ract pkg
```

Interactive dialog example:  
```plaintext
🥳 Welcome to use ract packager!

🔸 gpiler will check and install `cargo-packager` if not present.
🔸 Basic packaging configuration is auto-generated.
? Select how to package the project: init
🎉 Package resources have been generated!
```