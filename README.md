<p align="center">
  <img width="120" src="./images/Skiff.png" alt="Skiff">
  <h1 align="center">Skiff Tauri</h1>
  <p align="center">Skiff Tauri Desktop Application (Windows)</p>
</p>

---

# Installing

- ~~Grab a download from the [Release Artifacts](https://github.com/realAliDevex/skiff-app/releases)~~
- [Compile from source](#dependencies)

---

### What is Skiff?

Skiff is a secure, decentralized platform for collaborating with your team. Create documents, notes, wikis, and more. Skiff also offers features like comments, markdown, tables, file storage, and embeds. Skiff prioritizes privacy, but also provides all the features you need in a collaboration platform.

### What is Skiff Tauri?

It's an unoffical wrapper for [Skiff](https://skiff.com/) that uses the lighweight library [Tauri](https://tauri.app/).

### Is it safe?

It's Safe because it is simply a wrapper for the Skiff website, and no other data transfer occurs. You can verify this by reviewing the source code.

---

### How do I build it?

#### Dependencies

- [Rust](https://www.rust-lang.org/)
- [Yarn](https://yarnpkg.com/)
- [VS Code](https://code.visualstudio.com/)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)

#### Build from Code

```bash
# Step 1:
git clone https://github.com/realAliDevex/skiff-app.git

# Step 2:
cd skiff-app

# Step 3:
yarn

# Step 4:
yarn tauri dev

# Step 5:
# bundle path: src-tauri/target/release/bundle
yarn tauri build
```
