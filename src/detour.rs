use std::ffi::c_void;
use std::sync::Mutex;

type DobbyDummyFunc = unsafe extern "C" fn();

extern "C" {
    fn DobbyHook(
        address: *mut c_void,
        replace_func: DobbyDummyFunc,
        origin_func: *mut DobbyDummyFunc,
    ) -> i32;
    fn DobbyDestroy(address: *mut c_void) -> i32;
}

unsafe impl Send for Hook {}
unsafe impl Sync for Hook {}

static HOOKS: Mutex<Vec<Hook>> = Mutex::new(Vec::new());

struct Hook {
    target: *mut u8,
}

pub fn create_hook(target: *mut u8, replacement: *mut u8) -> Option<*mut u8> {
    if target.is_null() || replacement.is_null() {
        return None;
    }

    let mut trampoline: std::mem::MaybeUninit<DobbyDummyFunc> = std::mem::MaybeUninit::uninit();
    let result = unsafe {
        DobbyHook(
            target as *mut c_void,
            std::mem::transmute::<*mut u8, DobbyDummyFunc>(replacement),
            trampoline.as_mut_ptr(),
        )
    };

    if result == 0 {
        let trampoline = unsafe { trampoline.assume_init() };
        if let Ok(mut guards) = HOOKS.lock() {
            guards.push(Hook { target });
        }
        Some(trampoline as *mut u8)
    } else {
        log::error!("DobbyHook failed with error: {}", result);
        None
    }
}

pub fn remove_hooks() {
    if let Ok(guards) = HOOKS.lock() {
        for hook in guards.iter() {
            unsafe { DobbyDestroy(hook.target as *mut c_void); }
        }
    }
    if let Ok(mut guards) = HOOKS.lock() {
        guards.clear();
    }
}
