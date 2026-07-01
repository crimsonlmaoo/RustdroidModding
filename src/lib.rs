#![allow(static_mut_refs)]

pub mod il2cpp_imports;
pub mod il2cpp;
pub mod detour;
pub mod logging;
pub mod unity;

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
use unity::*;

//init once for android logger
static INIT: Once = Once::new();
static LOADED: AtomicBool = AtomicBool::new(false);

//orig funcs for your method hooks
static mut ORIGINAL_AWAKEMETHOD: Option<extern "C" fn(*mut c_void)> = None;
static mut ORIGINAL_UPDATEMETHOD: Option<extern "C" fn(*mut c_void)> = None;

//your awake hook, runs when the script attached to a gameobject awakes
extern "C" fn my_awake(instance: *mut c_void) {
    unsafe {
        if let Some(orig) = ORIGINAL_AWAKEMETHOD {
            orig(instance);
        }
    }
}

//your update hook, runs every frame
extern "C" fn my_update(instance: *mut c_void) {
    unsafe {
        if let Some(orig) = ORIGINAL_UPDATEMETHOD {
            orig(instance);
        }
    }
}

//find class by namespace and name - returns Option<Class> (you'll do .unwrap())
fn example_find_class() {
    let cls = Class::find("UnityEngine", "GameObject");
}

//call a static method - creates a new GameObject with a name
fn example_create_obj() {
    let name = string_new("my obj").unwrap();
    let obj = GameObject::create_new_object(name);
}

//find a single object by type in the scene
fn example_find_object() {
    let cls = Class::find("UnityEngine", "Camera").unwrap();
    let cam = cls.find_object_of_type();
}

//find all objects of a type
fn example_find_objects() {
    let cls = Class::find("UnityEngine", "Camera").unwrap();
    if let Some(arr) = cls.find_objects_of_type() {
        let cams = arr.as_slice::<Il2CppObject>();
        for &c in cams {
            let name = NamedObject(c).get_name();
        }
    }
}

//read a field from an instance - gets transform of gameobject
fn example_get_field(obj: Il2CppObject) {
    let transform = GameObject(obj).get_transform();
}

//write a field - sets position on a transform
fn example_set_field(transform: Il2CppObject) {
    let t = Transform(transform);
    t.set_local_position(Vector3::new(1.0, 2.0, 3.0));
}

//add a component to a gameobject
fn example_add_component(obj: Il2CppObject) {
    let rb_type = Rigidbody::get_type().unwrap();
    let rb = GameObject::add_component_static(obj, rb_type);
}

//get a component from a gameobject
fn example_get_component(obj: Il2CppObject) {
    let rb_type = Rigidbody::get_type().unwrap();
    let rb = GameObject::get_component_static(obj, rb_type);
}

//call an instance method - Rigidbody::set_velocity
fn example_call_method(rb: Il2CppObject) {
    let rigid = Rigidbody(rb);
    rigid.set_velocity(Vector3::new(0.0, 5.0, 0.0));
}

//work with unity arrays via as_slice
fn example_array() {
    let cls = Class::find("UnityEngine", "Camera").unwrap();
    if let Some(arr) = cls.find_objects_of_type() {
        let slice = arr.as_slice::<Il2CppObject>();
    }
}

//create a new mono/il2cpp string
fn example_string() {
    let s = string_new("hello world");
}

//convert managed string back to rust string
fn example_string_from(obj: Il2CppObject) {
    let s = string_from(obj);
}

//use the List wrapper for List<T>
fn example_list(list_obj: Il2CppObject) {
    let list = List(list_obj);
    let count = list.count();
    for i in 0..count {
        let item = list.get_item(i);
    }
    list.add(list_obj);
    list.remove_at(0);
}

//use the Dictionary wrapper for Dictionary<K,V>
fn example_dict(dict_obj: Il2CppObject) {
    let dict = Dictionary(dict_obj);
    let count = dict.count();
    if dict.contains_key(dict_obj) {
        let val = dict.get_item(dict_obj);
    }
}

//use same() to compare two objects by their m_cachedptr
fn example_same(a: Il2CppObject, b: Il2CppObject) -> bool {
    a.same(b)
}

//get the il2cpp class from an object instance
fn example_class_of(obj: Il2CppObject) {
    let cls = obj.class_of();
}

//get system.type from a class - needed for findobjectsoftype etc
fn example_system_type() {
    let t = Camera::get_type();
}

//hook a method by name - returns trampoline to original
fn example_hook(cls: &Class, method_name: &str, detour: *mut u8) {
    match cls.hook(method_name, detour) {
        Some(tramp) => {}
        None => {}
    }
}

//set parent of a transform - xf.child.parent = xf.parent
fn example_set_parent(child: Il2CppObject, parent: Il2CppObject) {
    RectTransform::set_parent_static(child, parent, false);
}

//get the main camera
fn example_main_camera() {
    let cam = Camera::get_main();
}

//use the logging system (tag set in logging.rs)
fn example_logging() {
    log::info!("info message");
    log::warn!("warning message");
    log::error!("error message");
}

//call a method with multiple params
fn example_call_complex() {
    let obj = GameObject::create_new_object(string_new("test").unwrap());
    let text_type = Text::get_type().unwrap();
    let com = GameObject::add_component_static(obj, text_type);
}

//check if two objects are the same
fn example_implicit() {
    let a = GameObject::create_new_object(string_new("a").unwrap());
    let b = GameObject::create_new_object(string_new("b").unwrap());
    let same = a.same(b);
}

//find a method on a class by name with param count
fn example_find_method(cls: &Class) {
    let m = cls.method("get_transform", 0);
    let m2 = cls.find_method("SomeMethod");
}

//create a new managed array of a specific type
fn example_new_array() {
    let cls = Class::find("UnityEngine", "Vector3").unwrap();
    let arr = Array::new_specific(cls, 5);
}

fn on_loaded() {
    if LOADED.swap(true, Ordering::SeqCst) {
        return;
    }
    log::info!("RustdroidModding loaded");

    //find your target class - returns Option<Class> (.unwrap() for just Class)
    let cls = Class::find("UnityEngine", "GameObject");
    let cls = match cls {
        Some(c) => c,
        None => { log::error!("target class not found"); return; }
    };

    //hook class methods - returns trampoline pointer or none
    match cls.hook("Awake", my_awake as *mut u8) {
        Some(t) => unsafe { ORIGINAL_AWAKEMETHOD = Some(std::mem::transmute(t)); },
        None => log::error!("hook failed for Awake"),
    }
    match cls.hook("Update", my_update as *mut u8) {
        Some(t) => unsafe { ORIGINAL_UPDATEMETHOD = Some(std::mem::transmute(t)); },
        None => log::error!("hook failed for Update"),
    }
}

//initialization below - not much to change here

static mut ORIGINAL_IL2CPP_INIT: Option<unsafe extern "C" fn(*const libc::c_char)> = None;

unsafe extern "C" fn hooked_il2cpp_init(domain_name: *const libc::c_char) {
    if let Some(orig) = ORIGINAL_IL2CPP_INIT {
        orig(domain_name);
    }
    on_loaded();
}

static mut ORIGINAL_CLASS_FROM_TYPE: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void> = None;

unsafe extern "C" fn hooked_class_from_il2cpp_type(type_: *mut c_void) -> *mut c_void {
    let orig = ORIGINAL_CLASS_FROM_TYPE.expect("original class_from_type pointer");
    let result = orig(type_);
    if !LOADED.load(Ordering::SeqCst) && il2cpp::Il2Cpp::domain_get().is_some() {
        on_loaded();
    }
    result
}

fn try_late_init_fallback() {
    log::warn!("fallback via il2cpp_class_from_il2cpp_type");
    let hook_ptr = match il2cpp::Il2Cpp::find_symbol("il2cpp_class_from_il2cpp_type") {
        Some(p) => p,
        None => { log::error!("symbol not found, calling on_loaded directly"); on_loaded(); return; }
    };
    if let Some(tramp) = detour::create_hook(hook_ptr, hooked_class_from_il2cpp_type as *mut u8) {
        unsafe { ORIGINAL_CLASS_FROM_TYPE = Some(std::mem::transmute(tramp)); }
        log::info!("il2cpp_class_from_il2cpp_type waiting for init");
    } else {
        log::error!("calling on_loaded directly");
        on_loaded();
    }
}

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

//entry point - JNI_OnLoad hook for android
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: *mut jni::sys::JavaVM, _reserved: *mut c_void) -> jni::sys::jint {
    INIT.call_once(|| logging::init());
    log::info!("RustdroidModding loading via JNI_OnLoad");
    unsafe {
        let vm = jni::JavaVM::from_raw(vm).expect("bad JVM pointer");
        let _env = vm.attach_current_thread_as_daemon().expect("failed on attach for the jni thread");
    }
    try_load();
    jni::sys::JNI_VERSION_1_6
}
