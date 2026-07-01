use crate::il2cpp_imports::Il2CppImports;
use std::ffi::CString;
use std::os::raw::c_void;

type UnityMethod = *mut c_void;

pub struct Il2Cpp;

impl Il2Cpp {
    pub fn resolve_icall(name: &str) -> Option<*mut c_void> {
        let cname = CString::new(name).ok()?;
        let ptr = unsafe { (Il2CppImports::get().resolve_icall)(cname.as_ptr()) };
        if ptr.is_null() { None } else { Some(ptr) }
    }

    pub fn domain_get() -> Option<*mut c_void> {
        let domain = unsafe { (Il2CppImports::get().domain_get)() };
        if domain.is_null() { None } else { Some(domain) }
    }

    pub fn find_symbol(name: &str) -> Option<*mut u8> {
        let c_name = CString::new(name).ok()?;
        let ptr = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c_name.as_ptr()) };
        if !ptr.is_null() {
            return Some(ptr as *mut u8);
        }
        let lib_name = CString::new("libil2cpp.so").ok()?;
        let handle = unsafe { libc::dlopen(lib_name.as_ptr(), libc::RTLD_NOW | libc::RTLD_GLOBAL) };
        if handle.is_null() {
            return None;
        }
        let ptr = unsafe { libc::dlsym(handle, c_name.as_ptr()) };
        if ptr.is_null() { None } else { Some(ptr as *mut u8) }
    }

    pub fn class_from_name(namespace: &str, name: &str) -> Option<*mut c_void> {
        let c_namespace = CString::new(namespace).ok()?;
        let c_name = CString::new(name).ok()?;
        let domain = Self::domain_get()?;
        let mut size: usize = 0;
        let assemblies = unsafe { (Il2CppImports::get().domain_get_assemblies)(domain, &mut size) };
        if assemblies.is_null() {
            return None;
        }
        for i in 0..size {
            let assembly = unsafe { *assemblies.add(i) };
            let image = unsafe { (Il2CppImports::get().assembly_get_image)(assembly) };
            if image.is_null() { continue; }
            let cls = unsafe { (Il2CppImports::get().class_from_name)(image, c_namespace.as_ptr(), c_name.as_ptr()) };
            if !cls.is_null() {
                return Some(cls);
            }
        }
        None
    }

    pub fn class_from_il2cpp_type(type_: *mut c_void) -> Option<*mut c_void> {
        let cls = unsafe { (Il2CppImports::get().class_from_il2cpp_type)(type_) };
        if cls.is_null() { None } else { Some(cls) }
    }

    pub fn get_method_from_name(cls: *mut c_void, method_name: &str, param_count: i32) -> Option<*mut c_void> {
        let c_name = CString::new(method_name).ok()?;
        let method = unsafe { (Il2CppImports::get().class_get_method_from_name)(cls, c_name.as_ptr(), param_count) };
        if method.is_null() { None } else { Some(method) }
    }

    pub fn find_method(namespace: &str, class_name: &str, method_name: &str, param_count: i32) -> Option<*mut c_void> {
        let cls = Self::class_from_name(namespace, class_name)?;
        Self::get_method_from_name(cls, method_name, param_count)
    }

    pub fn get_method_pointer(method: *mut c_void) -> Option<UnityMethod> {
        if method.is_null() { return None; }
        let ptr = unsafe { *(method as *const *mut c_void) };
        if ptr.is_null() { None } else { Some(ptr) }
    }

    pub fn string_new(s: &str) -> Option<*mut c_void> {
        let c_s = CString::new(s).ok()?;
        let result = unsafe { (Il2CppImports::get().string_new)(c_s.as_ptr()) };
        if result.is_null() { None } else { Some(result) }
    }

    pub fn new_object(cls: *mut c_void) -> *mut c_void {
        unsafe { (Il2CppImports::get().object_new)(cls) }
    }

    pub fn get_class_from_object(obj: *mut c_void) -> Option<*mut c_void> {
        if obj.is_null() { return None; }
        let cls = unsafe { (Il2CppImports::get().object_get_class)(obj) };
        if cls.is_null() { None } else { Some(cls) }
    }

    pub fn value_box(klass: *mut c_void, value: *mut c_void) -> Option<*mut c_void> {
        let result = unsafe { (Il2CppImports::get().value_box)(klass, value) };
        if result.is_null() { None } else { Some(result) }
    }

    pub fn field_get_flags(field: *mut c_void) -> u32 {
        unsafe { (Il2CppImports::get().field_get_flags)(field) }
    }

    pub fn class_get_field_from_name(cls: *mut c_void, name: &str) -> Option<*mut c_void> {
        let c_name = CString::new(name).ok()?;
        let field = unsafe { (Il2CppImports::get().class_get_field_from_name)(cls, c_name.as_ptr()) };
        if field.is_null() { None } else { Some(field) }
    }

    pub fn field_get_value(obj: *mut c_void, field: *mut c_void, value: *mut c_void) {
        unsafe { (Il2CppImports::get().field_get_value)(obj, field, value) }
    }

    pub fn field_set_value(obj: *mut c_void, field: *mut c_void, value: *mut c_void) {
        unsafe { (Il2CppImports::get().field_set_value)(obj, field, value) }
    }

    pub fn class_get_type(cls: *mut c_void) -> Option<*mut c_void> {
        let t = unsafe { (Il2CppImports::get().class_get_type)(cls) };
        if t.is_null() { None } else { Some(t) }
    }

    pub fn class_get_name(cls: *mut c_void) -> Option<String> {
        let ptr = unsafe { (Il2CppImports::get().class_get_name)(cls) };
        if ptr.is_null() { return None; }
        unsafe { Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().to_string()) }
    }

    pub fn type_get_object(type_: *mut c_void) -> Option<*mut c_void> {
        let obj = unsafe { (Il2CppImports::get().type_get_object)(type_) };
        if !obj.is_null() { return Some(obj); }
        Self::type_get_object_fallback(type_)
    }

    fn type_get_object_fallback(type_: *mut c_void) -> Option<*mut c_void> {
        #[repr(C)]
        struct RuntimeTypeHandle { value: *mut c_void }
        let handle = RuntimeTypeHandle { value: type_ };
        let type_cls = Self::class_from_name("System", "Type")?;
        let ptr = Self::get_method_func(type_cls, "GetTypeFromHandle", 1)?;
        let func: unsafe extern "C" fn(RuntimeTypeHandle) -> *mut c_void = unsafe { std::mem::transmute(ptr) };
        let obj = unsafe { func(handle) };
        if obj.is_null() { None } else { Some(obj) }
    }

    pub fn get_method_func(cls: *mut c_void, name: &str, param_count: i32) -> Option<*mut c_void> {
        let m = Self::get_method_from_name(cls, name, param_count)?;
        Self::get_method_pointer(m)
    }

    pub fn class_get_image(cls: *mut c_void) -> Option<*mut c_void> {
        let img = unsafe { (Il2CppImports::get().class_get_image)(cls) };
        if img.is_null() { None } else { Some(img) }
    }

    pub fn image_get_name(image: *mut c_void) -> Option<String> {
        let ptr = unsafe { (Il2CppImports::get().image_get_name)(image) };
        if ptr.is_null() { return None; }
        unsafe { Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().to_string()) }
    }

    pub fn array_get_length(array: *mut c_void) -> usize {
        unsafe { (Il2CppImports::get().array_get_length)(array) }
    }

    pub fn array_new_specific(cls: *mut c_void, size: usize) -> *mut c_void {
        unsafe { (Il2CppImports::get().array_new_specific)(cls, size) }
    }
}

pub unsafe fn rust_string_from_mono(mono_string: *mut c_void) -> Option<String> {
    if mono_string.is_null() { return None; }
    let len = (Il2CppImports::get().string_length)(mono_string) as usize;
    if len == 0 { return Some(String::new()); }
    let chars = (Il2CppImports::get().string_chars)(mono_string);
    let slice = std::slice::from_raw_parts(chars, len);
    Some(String::from_utf16_lossy(slice))
}
