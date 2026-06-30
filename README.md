# Releasing soon. [06/30/26 (sometime today)]

# RustdroidModding
Modding android games with Rust by loading with the JNI bridge of unity.

# How does it work?

Once we load, we attach our lib as a thread. Once attached, we call try_init();

[CODE]
fn try_load() {
    if LOADED.load(Ordering::SeqCst) { return; }
    log::info!("hooking il2cpp_init");

    let init_ptr = match il2cpp::Il2Cpp::find_symbol("il2cpp_init") {
        Some(p) => p,
        None => { try_late_init_fallback(); return; }
    };

    if let Some(tramp) = detour::create_hook(init_ptr, hooked_il2cpp_init as *mut u8) {
        unsafe { ORIGINAL_IL2CPP_INIT = Some(std::mem::transmute(tramp)); }
        log::info!("il2cpp_init hooked, waiting for start");
    } else {
        log::error!("falling back after init hook fail");
        try_late_init_fallback();
    }
}
[/CODE]
