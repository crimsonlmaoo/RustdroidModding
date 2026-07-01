use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::sync::OnceLock;
use libc::{dlopen, dlsym, RTLD_GLOBAL, RTLD_NOW};

pub struct Il2CppImports {
    pub resolve_icall: unsafe extern "C" fn(*const c_char) -> *mut c_void,
    pub domain_get: unsafe extern "C" fn() -> *mut c_void,
    pub domain_get_assemblies: unsafe extern "C" fn(*mut c_void, *mut usize) -> *mut *mut c_void,
    pub assembly_get_image: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    pub class_from_name: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void,
    pub class_from_il2cpp_type: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    pub class_get_method_from_name: unsafe extern "C" fn(*mut c_void, *const c_char, i32) -> *mut c_void,
    pub string_new: unsafe extern "C" fn(*const c_char) -> *mut c_void,
    pub object_new: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    pub object_get_class: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    pub value_box: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
    pub field_get_flags: unsafe extern "C" fn(*mut c_void) -> u32,
    pub class_get_field_from_name: unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void,
    pub field_get_value: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    pub field_set_value: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    pub class_get_type: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    pub class_get_name: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    pub type_get_object: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    pub class_get_image: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    pub image_get_name: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    pub string_length: unsafe extern "C" fn(*mut c_void) -> i32,
    pub string_chars: unsafe extern "C" fn(*mut c_void) -> *const u16,
    pub array_get_length: unsafe extern "C" fn(*mut c_void) -> usize,
    pub array_new_specific: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void,
    pub runtime_invoke: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *mut c_void, *mut *mut c_void) -> *mut c_void,
    pub object_unbox: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
}

unsafe fn resolve(lib: *mut c_void, name: &str) -> *mut c_void {
    let sym = CString::new(name).unwrap();
    dlsym(lib, sym.as_ptr())
}

macro_rules! check {
    ($ptr:expr, $name:expr) => {
        if $ptr.is_null() {
            log::error!("symbol not found: {}", $name);
            return None;
        }
    };
}

impl Il2CppImports {
    pub fn get() -> &'static Self {
        static IMPORTS: OnceLock<Il2CppImports> = OnceLock::new();
        IMPORTS.get_or_init(|| Self::init().expect("Il2CppImports::init failed"))
    }

    fn init() -> Option<Self> {
        let lib_name = CString::new("libil2cpp.so").ok()?;
        let lib = unsafe { dlopen(lib_name.as_ptr(), RTLD_NOW | RTLD_GLOBAL) };
        if lib.is_null() {
            log::error!("dlopen libil2cpp.so failed");
            return None;
        }

        macro_rules! load {
            ($name:expr) => {{
                let p = unsafe { resolve(lib, $name) };
                check!(p, $name);
                unsafe { std::mem::transmute::<*mut c_void, _>(p) }
            }};
        }

        Some(Self {
            resolve_icall: load!("il2cpp_resolve_icall"),
            domain_get: load!("il2cpp_domain_get"),
            domain_get_assemblies: load!("il2cpp_domain_get_assemblies"),
            assembly_get_image: load!("il2cpp_assembly_get_image"),
            class_from_name: load!("il2cpp_class_from_name"),
            class_from_il2cpp_type: load!("il2cpp_class_from_il2cpp_type"),
            class_get_method_from_name: load!("il2cpp_class_get_method_from_name"),
            string_new: load!("il2cpp_string_new"),
            object_new: load!("il2cpp_object_new"),
            object_get_class: load!("il2cpp_object_get_class"),
            value_box: load!("il2cpp_value_box"),
            field_get_flags: load!("il2cpp_field_get_flags"),
            class_get_field_from_name: load!("il2cpp_class_get_field_from_name"),
            field_get_value: load!("il2cpp_field_get_value"),
            field_set_value: load!("il2cpp_field_set_value"),
            class_get_type: load!("il2cpp_class_get_type"),
            class_get_name: load!("il2cpp_class_get_name"),
            type_get_object: load!("il2cpp_type_get_object"),
            class_get_image: load!("il2cpp_class_get_image"),
            image_get_name: load!("il2cpp_image_get_name"),
            string_length: load!("il2cpp_string_length"),
            string_chars: load!("il2cpp_string_chars"),
            array_get_length: load!("il2cpp_array_length"),
            array_new_specific: load!("il2cpp_array_new_specific"),
            runtime_invoke: load!("il2cpp_runtime_invoke"),
            object_unbox: load!("il2cpp_object_unbox"),
        })
    }
}
