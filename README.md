# Ract

<img src="https://img.shields.io/badge/Ract-0.0.1-orange?style=flat-square&logo=rust&logoColor=%23fff&labelColor=%23DEA584&color=%23DEA584"> 
<img src="https://img.shields.io/badge/License-MIT-orange?style=flat-square&logoColor=%23fff&labelColor=%2323B898&color=%2323B898">

**Ract** is a conversational CLI tool written in Rust, designed to simplify the development process by providing an all-in-one solution for:  

- Integrating dependencies  
- Setting up environments  
- Generating project templates  
- Running and packaging projects  

With **minimal arguments** and an **intuitive dialog-based interface**, Ract supports frameworks like **GenUI** and **Makepad**, making your development workflow smooth and efficient. 🚀

---

## 📖 Table of Contents
1. [Usage](#usage)  
   - [init](#init)  
   - [check](#check)  
   - [install](#install)  
   - [config](#config)  
   - [run](#run)  
   - [studio](#studio)  
   - [wasm](#wasm)  
   - [pkg](#pkg)  
2. [Features](#features)

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

### `run` - Run a Project  
Run **Makepad** or **GenUI** projects.  

```bash
ract run
```

---

### `studio` - Launch Makepad Studio  
Start Makepad Studio for GUI or WASM projects.  

```bash
ract studio
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

---

## 🎯 Features  

### Core Features  
- **Initialization**: Automatically generate `.env` and environment templates.  
- **Toolchain Check**: Verify if all dependencies are installed.  
- **Interactive Installation**: Install only the tools you need.  
- **Environment Configuration**: Flexible configuration of environment paths.  
- **Project Running**: Run Makepad or GenUI projects with a single command.  
- **WASM Support**: Build and serve WASM projects in the browser.  
- **Project Packaging**: Streamlined packaging for distribution.  

### Future Features  
- **Watcher**: Monitor changes for automatic reload.  
- **Logger**: Advanced logging system for debugging.  
- **Cross-platform Packaging**: Simplify builds for multiple platforms.  
- **Documentation**: Include a comprehensive book for learning Ract.  

---

Ract makes your Rust-based development with **Makepad** and **GenUI** easier, faster, and more efficient. 🎉  

Feel free to contribute or share feedback to help us improve! 😊 