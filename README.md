# RustdroidModding
Modding android games with Rust by loading with the JNI bridge of unity.

## How it loads

1. The game loads the `.so` via JNI ->`JNI_OnLoad` runs.
2. Logging is set up (`android_logger`, tag `ModTemplate-Rust` (You can change)).
3. `try_load()` looks up `il2cpp_init` via `dlsym` and hooks it with Dobby.
4. When the game actually calls `il2cpp_init`, the hook fires -> calls the original -> then calls `on_loaded()`.
5. `on_loaded()` is where your actual mod logic goes -> finding classes, hooking methods, etc.

If `il2cpp_init` can't be found (already called, stripped, or exported differently), it'll fall back to hooking `il2cpp_class_from_il2cpp_type` instead as that function gets called a lot during normal startup, so the hook polls `domain_get()` until it succeeds and triggers `on_loaded()` from there. If that also fails to even install, it just calls `on_loaded()` directly with no way of knowing the game is actually ready.

## File layout

| File | Purpose |
|---|---|
| `lib.rs` | Entry, init, mod code and stuff |
| `il2cpp.rs` | Helps entirely with the IL2CPP api using `dlsym` (classes, methods, fields, strings, etc etc) |
| `unity.rs` | Wrapper for unity (`GameObject`, `Class`, `Vector3`, etc.), referenced but not shown in this readme |
| `detour.rs` | Tracks, installs, and removes hooks. Useful, and uses dobby. |
| `logging.rs` | `android_logger` setup |

## Building

```bash
cargo build --target aarch64-linux-android --release
```

You need the Android NDK installed and configured (linker path in `.cargo/config.toml` or `CARGO_NDK_*` env vars depending on your setup). The output `.so` goes wherever your decompiled apk wants it in the lib folder, or in nativemods (from livku's mod loader).

## Writing a mod

Your mod code would go into `on_loaded()`. How you would follow as a pattern is given:

```rust
let cls = Class::find("Namespace", "ClassName")?;
match cls.hook("MethodName", your_hook_fn as *mut u8) {
    Some(tramp) => unsafe { ORIGINAL = Some(std::mem::transmute(tramp)) },
    None => log::error!("hook failed"),
}
```

Always call the original through the original hook func inside your hook unless you want to block the original behavior.
