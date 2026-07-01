use crate::il2cpp::Il2Cpp;
use std::os::raw::c_void;

pub type Il2CppObject = *mut c_void;
pub type Il2CppClass = *mut c_void;
pub type Il2CppMethod = *mut c_void;
pub type Il2CppField  = *mut c_void;
pub type Il2CppType   = *mut c_void;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector2 { pub x: f32, pub y: f32 }
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self { Vector2 { x, y } }
    pub fn zero() -> Self { Vector2 { x: 0.0, y: 0.0 } }
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector3 { pub x: f32, pub y: f32, pub z: f32 }
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Vector3 { x, y, z } }
    pub fn zero() -> Self { Vector3 { x: 0.0, y: 0.0, z: 0.0 } }
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector4 { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Quaternion { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Rect { pub x: f32, pub y: f32, pub width: f32, pub height: f32 }
#[repr(C)]
pub struct RaycastHit { _data: [u8; 64] }
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct LayerMask { pub m_mask: i32 }
impl LayerMask {
    pub fn get_value(&self) -> i32 { self.m_mask }
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct GradientColorKey { pub color: Color, pub time: f32 }
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct GradientAlphaKey { pub alpha: f32, pub time: f32 }

#[derive(Clone, Copy)]
pub enum PrimitiveType { Sphere = 0, Capsule = 1, Cylinder = 2, Cube = 3, Plane = 4, Quad = 5 }
#[derive(Clone, Copy)]
pub enum RenderMode { ScreenSpaceOverlay = 0, ScreenSpaceCamera = 1, WorldSpace = 2 }
#[derive(Clone, Copy)]
pub enum ForceMode { Force = 0, Acceleration = 5, Impulse = 1, VelocityChange = 2 }
#[derive(Clone, Copy)]
pub enum GradientMode { Blend, Fixed, PerceptualBlend }
#[derive(Clone, Copy)]
pub enum TextAnchor { UpperLeft=0, UpperCenter=1, UpperRight=2, MiddleLeft=3, MiddleCenter=4, MiddleRight=5, LowerLeft=6, LowerCenter=7, LowerRight=8 }
#[derive(Clone, Copy)]
pub enum FontStyle { Normal, Bold, Italic, BoldAndItalic }
#[derive(Clone, Copy)]
pub enum SpriteMeshType { FullRect, Tight }
#[derive(Clone, Copy)]
pub enum TextAlignmentOptions {
    TopLeft = 257, Top = 258, TopRight = 260, TopJustified = 264, TopFlush = 272, TopGeoAligned = 288,
    LLeft = 513, Center = 514, RRight = 516, Justified = 520, Flush = 528, CenterGeoAligned = 544,
    BottomLeft = 1025, Bottom = 1026, BottomRight = 1028, BottomJustified = 1032, BottomFlush = 1040, BottomGeoAligned = 1056,
    BaselineLeft = 2049, Baseline = 2050, BaselineRight = 2052, BaselineJustified = 2056, BaselineFlush = 2064, BaselineGeoAligned = 2080,
    MidlineLeft = 4097, Midline = 4098, MidlineRight = 4100, MidlineJustified = 4104, MidlineFlush = 4112, MidlineGeoAligned = 4128,
    CaplineLeft = 8193, Capline = 8194, CaplineRight = 8196, CaplineJustified = 8200, CaplineFlush = 8208, CaplineGeoAligned = 8224,
    Converted = 65535
}
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TextureFormat {
    Alpha8 = 1, ARGB4444 = 2, RGB24 = 3, RGBA32 = 4, ARGB32 = 5, RGB565 = 7, R16 = 9, DXT1 = 10, DXT5 = 12,
    RGBA4444 = 13, BGRA32 = 14, RHalf = 15, RGHalf = 16, RGBAHalf = 17, RFloat = 18, RGFloat = 19, RGBAFloat = 20,
    YUY2 = 21, RGB9e5Float = 22, BC4 = 26, BC5 = 27, BC6H = 24, BC7 = 25,
    DXT1Crunched = 28, DXT5Crunched = 29, PVRTC_RGB2 = 30, PVRTC_RGBA2 = 31, PVRTC_RGB4 = 32, PVRTC_RGBA4 = 33,
    ETC_RGB4 = 34, ATC_RGB4 = 35, ATC_RGBA8 = 36, EAC_R = 41, EAC_R_SIGNED = 42, EAC_RG = 43, EAC_RG_SIGNED = 44,
    ETC2_RGB4 = 45, ETC2_RGB4_PUNCHTHROUGH_ALPHA = 46, ETC2_RGBA8 = 47,
    ASTC_4x4 = 48, ASTC_5x5 = 49, ASTC_6x6 = 50, ASTC_8x8 = 51, ASTC_10x10 = 52, ASTC_12x12 = 53,
    RG16 = 62, R8 = 63, ETC_RGB4Crunched = 64, ETC2_RGBA8Crunched = 65,
    ASTC_HDR_4x4 = 66, ASTC_HDR_5x5 = 67, ASTC_HDR_6x6 = 68, ASTC_HDR_8x8 = 69, ASTC_HDR_10x10 = 70, ASTC_HDR_12x12 = 71,
    PBGRA32 = 84,
    PVRTC_2BPP_RGB = 96, PVRTC_2BPP_RGBA = 97, PVRTC_4BPP_RGB = 98, PVRTC_4BPP_RGBA = 99,
    RGBA5551 = 123,
}

pub struct Class(pub Il2CppClass);

impl Class {
    pub fn find(ns: &str, name: &str) -> Option<Self> {
        let r = Il2Cpp::class_from_name(ns, name).map(Class);
        if r.is_none() {
            log::error!("RustdroidModding | Dead class!! '{}::{}' not found", ns, name);
        }
        r
    }
    pub fn method(&self, name: &str, params: i32) -> Option<Method> {
        let r = Il2Cpp::get_method_from_name(self.0, name, params).map(Method::new);
        if r.is_none() {
            log::error!("RustdroidModding | Dead method!! '{}' ({}) not found on class", name, params);
        }
        r
    }
    pub fn find_method(&self, name: &str) -> Option<Method> {
        if let Some(m) = Il2Cpp::get_method_from_name(self.0, name, -1) {
            return Some(Method::new(m));
        }
        for i in 0..=10 {
            if let Some(m) = Il2Cpp::get_method_from_name(self.0, name, i) {
                return Some(Method::new(m));
            }
        }
        log::error!("RustdroidModding | Dead method!! '{}' (any param count) not found", name);
        None
    }
    pub fn field(&self, name: &str) -> Option<Field> {
        let r = Il2Cpp::class_get_field_from_name(self.0, name).map(Field::new);
        if r.is_none() {
            log::error!("RustdroidModding | Dead field!! '{}' not found on class", name);
        }
        r
    }
    pub fn type_ptr(&self) -> Option<Il2CppType> {
        Il2Cpp::class_get_type(self.0)
    }
    pub fn system_type(&self) -> Option<Il2CppObject> {
        Il2Cpp::type_get_object(self.type_ptr()?)
    }
    pub fn create(&self) -> Option<Il2CppObject> {
        let o = Il2Cpp::new_object(self.0);
        if o.is_null() { None } else { Some(o) }
    }
    pub fn hook(&self, method_name: &str, detour: *mut u8) -> Option<*mut u8> {
        self.find_method(method_name)?.hook(detour)
    }
    pub fn raw(&self) -> Il2CppClass { self.0 }
    pub fn find_objects_of_type(&self) -> Option<Array> {
        let st = self.system_type()?;
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Object::FindObjectsOfType") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(st);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let obj_cls = Class::find("UnityEngine", "Object")?;
        if let Some(m) = obj_cls.method("FindObjectsOfType", 1) {
            let p = m.ptr()?;
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(st);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = obj_cls.method("FindObjectsOfType", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(st, false);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn find_object_of_type(&self) -> Option<Il2CppObject> {
        let st = self.system_type()?;
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Object::FindObjectOfType") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(st);
            return if r.is_null() { None } else { Some(r) };
        }
        let obj_cls = Class::find("UnityEngine", "Object")?;
        if let Some(m) = obj_cls.method("FindObjectOfType", 1) {
            let p = m.ptr()?;
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(st);
            return if r.is_null() { None } else { Some(r) };
        }
        let m = obj_cls.method("FindObjectOfType", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(st, false);
        if r.is_null() { None } else { Some(r) }
    }
    pub fn find_objects_of_type_all(&self) -> Option<Array> {
        let st = self.system_type()?;
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Resources::FindObjectsOfTypeAll") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(st);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = Class::find("UnityEngine", "Resources")?.method("FindObjectsOfTypeAll", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(st);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn find_objects_of_type_inactive(&self) -> Option<Array> {
        let st = self.system_type()?;
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Object::FindObjectsOfType") {
            let f: extern "C" fn(Il2CppObject, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(st, true);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = Class::find("UnityEngine", "Object")?.method("FindObjectsOfType", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(st, true);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn find_object_of_type_inactive(&self) -> Option<Il2CppObject> {
        let st = self.system_type()?;
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Object::FindObjectOfType") {
            let f: extern "C" fn(Il2CppObject, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(st, true);
            return if r.is_null() { None } else { Some(r) };
        }
        let m = Class::find("UnityEngine", "Object")?.method("FindObjectOfType", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(st, true);
        if r.is_null() { None } else { Some(r) }
    }
}

pub struct Method {
    pub raw: Il2CppMethod,
    pub inst: Il2CppObject,
}

impl Method {
    pub fn new(raw: Il2CppMethod) -> Self {
        Method { raw, inst: std::ptr::null_mut() }
    }
    pub fn set_instance(&mut self, obj: Il2CppObject) -> &mut Self {
        if obj.is_null() {
            log::error!("RustdroidModding | Bad instance!! Setting Method instance to null");
        }
        self.inst = obj;
        self
    }
    pub fn ptr(&self) -> Option<*mut c_void> {
        let r = Il2Cpp::get_method_pointer(self.raw);
        if r.is_none() {
            log::error!("RustdroidModding | Dead method!! get_method_pointer returned null");
        }
        r
    }
    pub fn hook(&self, detour: *mut u8) -> Option<*mut u8> {
        let target = self.ptr()? as *mut u8;
        crate::detour::create_hook(target, detour)
    }
    pub fn call_0(&self, obj: Il2CppObject) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn(Il2CppObject) = unsafe { std::mem::transmute(ptr) };
        f(obj); Some(())
    }
    pub fn call_1<A: Copy>(&self, obj: Il2CppObject, a: A) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn(Il2CppObject, A) = unsafe { std::mem::transmute(ptr) };
        f(obj, a); Some(())
    }
    pub fn call_2<A: Copy, B: Copy>(&self, obj: Il2CppObject, a: A, b: B) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn(Il2CppObject, A, B) = unsafe { std::mem::transmute(ptr) };
        f(obj, a, b); Some(())
    }
    pub fn call_3<A: Copy, B: Copy, C: Copy>(&self, obj: Il2CppObject, a: A, b: B, c: C) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn(Il2CppObject, A, B, C) = unsafe { std::mem::transmute(ptr) };
        f(obj, a, b, c); Some(())
    }
    pub fn invoke_0(&self) -> Option<()> { self.call_0(self.inst) }
    pub fn invoke_1<A: Copy>(&self, a: A) -> Option<()> { self.call_1(self.inst, a) }
    pub fn invoke_2<A: Copy, B: Copy>(&self, a: A, b: B) -> Option<()> { self.call_2(self.inst, a, b) }
    pub fn invoke_3<A: Copy, B: Copy, C: Copy>(&self, a: A, b: B, c: C) -> Option<()> { self.call_3(self.inst, a, b, c) }
    pub fn call_static_0(&self) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn() = unsafe { std::mem::transmute(ptr) };
        f(); Some(())
    }
    pub fn call_static_1<A: Copy>(&self, a: A) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn(A) = unsafe { std::mem::transmute(ptr) };
        f(a); Some(())
    }
    pub fn call_static_2<A: Copy, B: Copy>(&self, a: A, b: B) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn(A, B) = unsafe { std::mem::transmute(ptr) };
        f(a, b); Some(())
    }
    pub fn call_static_3<A: Copy, B: Copy, C: Copy>(&self, a: A, b: B, c: C) -> Option<()> {
        let ptr = self.ptr()?;
        let f: extern "C" fn(A, B, C) = unsafe { std::mem::transmute(ptr) };
        f(a, b, c); Some(())
    }
}

pub struct Field {
    pub raw: Il2CppField,
    pub inst: Il2CppObject,
}

impl Field {
    pub fn new(raw: Il2CppField) -> Self {
        Field { raw, inst: std::ptr::null_mut() }
    }
    pub fn set_instance(&mut self, obj: Il2CppObject) -> &mut Self {
        if obj.is_null() {
            log::error!("RustdroidModding | Bad instance!! Setting Field instance to null");
        }
        self.inst = obj;
        self
    }
    pub fn get<T: Copy + Default>(&self, obj: Il2CppObject) -> T {
        let mut val: T = unsafe { std::mem::zeroed() };
        Il2Cpp::field_get_value(obj, self.raw, &mut val as *mut _ as *mut c_void);
        val
    }
    pub fn set<T>(&self, obj: Il2CppObject, val: &T) {
        Il2Cpp::field_set_value(obj, self.raw, val as *const _ as *mut c_void);
    }
    pub fn get_obj(&self, obj: Il2CppObject) -> Il2CppObject {
        let mut val: Il2CppObject = std::ptr::null_mut();
        Il2Cpp::field_get_value(obj, self.raw, &mut val as *mut _ as *mut c_void);
        val
    }
    pub fn set_obj(&self, obj: Il2CppObject, val: Il2CppObject) {
        Il2Cpp::field_set_value(obj, self.raw, &val as *const _ as *mut c_void);
    }
    pub fn getv<T: Copy + Default>(&self) -> T { self.get(self.inst) }
    pub fn setv<T>(&self, val: &T) { self.set(self.inst, val); }
    pub fn get_objv(&self) -> Il2CppObject { self.get_obj(self.inst) }
    pub fn set_objv(&self, val: Il2CppObject) { self.set_obj(self.inst, val); }
}

pub trait ObjectExt: Sized {
    fn as_ptr(&self) -> Il2CppObject;
    fn class_of(&self) -> Option<Class> {
        Il2Cpp::get_class_from_object(self.as_ptr()).map(Class)
    }
    fn find_method(&self, name: &str) -> Option<Method> {
        self.class_of()?.find_method(name)
    }
    fn field(&self, name: &str) -> Option<Field> {
        self.class_of()?.field(name)
    }
    fn get_field<T: Copy + Default>(&self, name: &str) -> T {
        self.class_of().and_then(|c| c.field(name))
            .map(|f| f.get(self.as_ptr()))
            .unwrap_or_default()
    }
    fn set_field<T>(&self, name: &str, val: &T) {
        if let Some(f) = self.class_of().and_then(|c| c.field(name)) {
            f.set(self.as_ptr(), val);
        }
    }

    fn same(&self, other: Il2CppObject) -> bool {
        let s = self.as_ptr();
        if s.is_null() && other.is_null() { return true; }
        if s.is_null() || other.is_null() { return false; }
        let a: Il2CppObject = self.get_field("m_CachedPtr");
        let b: Il2CppObject = other.get_field("m_CachedPtr");
        a == b
    }
}

impl ObjectExt for Il2CppObject {
    fn as_ptr(&self) -> Il2CppObject { *self }
}

pub fn string_new(s: &str) -> Option<Il2CppObject> {
    Il2Cpp::string_new(s)
}
pub fn string_from(s: Il2CppObject) -> Option<String> {
    unsafe { crate::il2cpp::rust_string_from_mono(s) }
}
pub fn find_class(ns: &str, name: &str) -> Option<Class> {
    Class::find(ns, name)
}
pub fn find_method(ns: &str, class: &str, method: &str, params: i32) -> Option<Method> {
    Class::find(ns, class)?.method(method, params)
}

macro_rules! icall_or_method0_void {
    ($icall:expr, $obj:expr, $class:expr, $method:expr) => {{
        let ptr = Il2Cpp::resolve_icall($icall);
        if let Some(p) = ptr {
            let f: extern "C" fn(Il2CppObject) = unsafe { std::mem::transmute(p) };
            f($obj);
            return;
        }
        if let Some(m) = $class.method($method, 0) {
            m.call_0($obj);
        }
    }};
}

macro_rules! icall_or_method1_void {
    ($icall:expr, $obj:expr, $a:expr, $class:expr, $method:expr) => {{
        let ptr = Il2Cpp::resolve_icall($icall);
        if let Some(p) = ptr {
            let f: extern "C" fn(Il2CppObject, _) = unsafe { std::mem::transmute(p) };
            f($obj, $a);
            return;
        }
        if let Some(m) = $class.method($method, 1) {
            m.call_1($obj, $a);
        }
    }};
}

macro_rules! icall_or_method2_void {
    ($icall:expr, $obj:expr, $a:expr, $b:expr, $class:expr, $method:expr) => {{
        let ptr = Il2Cpp::resolve_icall($icall);
        if let Some(p) = ptr {
            let f: extern "C" fn(Il2CppObject, _, _) = unsafe { std::mem::transmute(p) };
            f($obj, $a, $b);
            return;
        }
        if let Some(m) = $class.method($method, 2) {
            m.call_2($obj, $a, $b);
        }
    }};
}

macro_rules! icall_or_method0_static_void {
    ($icall:expr, $class:expr, $method:expr) => {{
        let ptr = Il2Cpp::resolve_icall($icall);
        if let Some(p) = ptr {
            let f: extern "C" fn() = unsafe { std::mem::transmute(p) };
            f();
            return;
        }
        if let Some(m) = $class.method($method, 0) {
            m.call_static_0();
        }
    }};
}

macro_rules! icall_or_method1_static_void {
    ($icall:expr, $a:expr, $class:expr, $method:expr) => {{
        let ptr = Il2Cpp::resolve_icall($icall);
        if let Some(p) = ptr {
            let f: extern "C" fn(_) = unsafe { std::mem::transmute(p) };
            f($a);
            return;
        }
        if let Some(m) = $class.method($method, 1) {
            m.call_static_1($a);
        }
    }};
}

macro_rules! icall_or_method2_static_void {
    ($icall:expr, $a:expr, $b:expr, $class:expr, $method:expr) => {{
        let ptr = Il2Cpp::resolve_icall($icall);
        if let Some(p) = ptr {
            let f: extern "C" fn(_, _) = unsafe { std::mem::transmute(p) };
            f($a, $b);
            return;
        }
        if let Some(m) = $class.method($method, 2) {
            m.call_static_2($a, $b);
        }
    }};
}

macro_rules! icall_or_method3_static_void {
    ($icall:expr, $a:expr, $b:expr, $c:expr, $class:expr, $method:expr) => {{
        let ptr = Il2Cpp::resolve_icall($icall);
        if let Some(p) = ptr {
            let f: extern "C" fn(_, _, _) = unsafe { std::mem::transmute(p) };
            f($a, $b, $c);
            return;
        }
        if let Some(m) = $class.method($method, 3) {
            m.call_static_3($a, $b, $c);
        }
    }};
}

macro_rules! extern_method {
    ($sig:expr) => {{ Il2Cpp::resolve_icall($sig) }};
}

pub struct Array(pub Il2CppObject);
impl Array {
    pub fn len(&self) -> usize {
        Il2Cpp::array_get_length(self.0)
    }
    pub fn as_slice<T: Copy>(&self) -> &[T] {
        if self.0.is_null() { return &[]; }
        let len = self.len();
        unsafe {
            let ptr = (self.0 as *mut u8).add(16) as *const T;
            std::slice::from_raw_parts(ptr, len)
        }
    }
    pub fn as_mut_slice<T: Copy>(&mut self) -> &mut [T] {
        if self.0.is_null() { return &mut []; }
        let len = self.len();
        unsafe {
            let ptr = (self.0 as *mut u8).add(16) as *mut T;
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }
    pub fn get_class() -> Option<Class> { Class::find("System", "Array") }
    pub fn new_specific(cls: Class, size: usize) -> Option<Array> {
        let arr = Il2Cpp::array_new_specific(cls.0, size);
        if arr.is_null() { None } else { Some(Array(arr)) }
    }
    pub fn new(ns: &str, name: &str, size: usize) -> Option<Array> {
        let cls = Class::find(ns, name)?;
        Self::new_specific(cls, size)
    }
}

pub struct Object(pub Il2CppObject);
impl Object {
    pub fn find(ns: &str, class: &str) -> Option<Class> { Class::find(ns, class) }
}

pub struct NamedObject(pub Il2CppObject);
impl NamedObject {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Object") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_name(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.Object::get_name") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_name", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_name(&self, name: &str) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method1_void!("UnityEngine.Object::set_name", self.0, s, Self::get_class().unwrap(), "set_name")
    }
}

pub struct Component(pub Il2CppObject);
impl Component {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Component") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_transform(&self) -> Option<Transform> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Component::get_transform") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_transform", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Transform(r)) }
    }
    pub fn get_game_object(&self) -> Option<GameObject> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Component::get_gameObject") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_gameObject", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(GameObject(r)) }
    }
    pub fn get_tag(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.Component::get_tag") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_tag", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_tag(&self, tag: &str) {
        let s = string_new(tag).unwrap_or(std::ptr::null_mut());
        icall_or_method1_void!("UnityEngine.Component::set_tag", self.0, s, Self::get_class().unwrap(), "set_tag")
    }
    pub fn get_component(&self, type_: Il2CppType) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Component::GetComponent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, type_));
        }
        let m = Self::get_class()?.method("GetComponent", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0, type_))
    }
    pub fn get_component_in_children(&self, type_: Il2CppType, include_inactive: bool) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Component::GetComponentInChildren") {
            let f: extern "C" fn(Il2CppObject, Il2CppType, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, type_, include_inactive));
        }
        let m = Self::get_class()?.method("GetComponentInChildren", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0, type_, include_inactive))
    }
    pub fn get_component_in_parent(&self, type_: Il2CppType, include_inactive: bool) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Component::GetComponentInParent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, type_, include_inactive));
        }
        let m = Self::get_class()?.method("GetComponentInParent", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0, type_, include_inactive))
    }
    pub fn find(&self, n: &str) -> Option<Transform> {
        let s = string_new(n).unwrap_or(std::ptr::null_mut());
        if let Some(p) = extern_method!("UnityEngine.Transform::Find") {
            let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0, s);
            return if r.is_null() { None } else { Some(Transform(r)) };
        }
        let m = Self::get_class()?.method("Find", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0, s);
        if r.is_null() { None } else { Some(Transform(r)) }
    }
    pub fn find_child(&self, index: i32) -> Option<Transform> {
        if let Some(p) = extern_method!("UnityEngine.Transform::FindChild") {
            let f: extern "C" fn(Il2CppObject, i32) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0, index);
            return if r.is_null() { None } else { Some(Transform(r)) };
        }
        let m = Self::get_class()?.method("FindChild", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, i32) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0, index);
        if r.is_null() { None } else { Some(Transform(r)) }
    }
}

pub struct GameObject(pub Il2CppObject);
impl GameObject {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "GameObject") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn destroy(obj: Il2CppObject) {
        icall_or_method1_static_void!("UnityEngine.Object::Destroy", obj, Class::find("UnityEngine", "Object").unwrap(), "Destroy")
    }
    pub fn destroy_delayed(obj: Il2CppObject, t: f32) {
        let m = Class::find("UnityEngine", "Object").unwrap().method("Destroy", 2).unwrap();
        m.call_static_2(obj, t);
    }
    pub fn create_primitive(typ: PrimitiveType) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::CreatePrimitive") {
            let f: extern "C" fn(PrimitiveType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(typ));
        }
        let m = Self::get_class()?.method("CreatePrimitive", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(PrimitiveType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(typ))
    }
    pub fn instantiate(original: Il2CppObject) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Object::Instantiate") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(original));
        }
        let m = Self::get_class()?.method("Instantiate", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(original))
    }
    pub fn instantiate_pos_rot(original: Il2CppObject, pos: Vector3, rot: Quaternion) -> Option<Il2CppObject> {
        let cls = Class::find("UnityEngine", "Object").unwrap_or_else(|| Self::get_class().unwrap());
        let m = cls.method("Instantiate", 3)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Vector3, Quaternion) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(original, pos, rot))
    }
    pub fn instantiate_parent(original: Il2CppObject, parent: Il2CppObject, world_space: bool) -> Option<Il2CppObject> {
        let cls = Class::find("UnityEngine", "Object").unwrap_or_else(|| Self::get_class().unwrap());
        let m = cls.method("Instantiate", 3)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppObject, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(original, parent, world_space))
    }
    pub fn dont_destroy_on_load(object: Il2CppObject) {
        icall_or_method1_static_void!("UnityEngine.Object::DontDestroyOnLoad", object, Class::find("UnityEngine", "Object").unwrap(), "DontDestroyOnLoad")
    }
    pub fn find(name: &str) -> Option<GameObject> {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        if let Some(p) = extern_method!("UnityEngine.GameObject::Find") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(s);
            return if r.is_null() { None } else { Some(GameObject(r)) };
        }
        let m = Self::get_class()?.method("Find", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(s);
        if r.is_null() { None } else { Some(GameObject(r)) }
    }
    pub fn get_transform(&self) -> Option<Transform> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.GameObject::get_transform") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_transform", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Transform(r)) }
    }
    pub fn set_active(&self, active: bool) {
        icall_or_method1_void!("UnityEngine.GameObject::SetActive", self.0, active, Self::get_class().unwrap(), "SetActive")
    }
    pub fn get_active_self(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::get_activeSelf") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_activeSelf", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn get_layer(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::get_layer") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_layer", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_layer(&self, layer: i32) {
        icall_or_method1_void!("UnityEngine.GameObject::set_layer", self.0, layer, Self::get_class().unwrap(), "set_layer")
    }
    pub fn get_tag(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::get_tag") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_tag", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_tag(&self, tag: &str) {
        let s = string_new(tag).unwrap_or(std::ptr::null_mut());
        icall_or_method1_void!("UnityEngine.GameObject::set_tag", self.0, s, Self::get_class().unwrap(), "set_tag")
    }
    pub fn get_name(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.Object::get_name") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = NamedObject::get_class()?.method("get_name", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_name(&self, name: &str) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method1_void!("UnityEngine.Object::set_name", self.0, s, NamedObject::get_class().unwrap(), "set_name")
    }
    pub fn get_component(&self, type_: Il2CppType) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::GetComponent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, type_));
        }
        let m = Self::get_class()?.method("GetComponent", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0, type_))
    }
    pub fn add_component(&self, type_: Il2CppType) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::AddComponent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, type_));
        }
        let m = Self::get_class()?.method("AddComponent", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0, type_))
    }
    pub fn new(name: &str) -> Option<GameObject> {
        let cls = Self::get_class()?;
        let obj = cls.create()?;
        if let Some(s) = string_new(name) {
            if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.GameObject::Internal_CreateGameObject") {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(obj, s);
            } else if let Some(m) = cls.find_method(".ctor") {
                m.call_1(obj, s);
            }
        }
        Some(GameObject(obj))
    }
    pub fn get_components_in_children(&self, type_: Il2CppType) -> Option<Array> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::GetComponentsInChildren") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0, type_);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = Self::get_class()?.method("GetComponentsInChildren", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0, type_);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn get_components_in_parent(&self, type_: Il2CppType) -> Option<Array> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::GetComponentsInParent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0, type_);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = Self::get_class()?.method("GetComponentsInParent", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0, type_);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn get_components(&self, type_: Il2CppType) -> Option<Array> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::GetComponents") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0, type_);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = Self::get_class()?.method("GetComponents", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0, type_);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn get_component_in_parent(&self, type_: Il2CppType, include_inactive: bool) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.GameObject::GetComponentInParent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, type_, include_inactive));
        }
        let m = Self::get_class()?.method("GetComponentInParent", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppType, bool) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0, type_, include_inactive))
    }
}

impl GameObject {
    pub fn create_new_object(name: Il2CppObject) -> Il2CppObject {
        let cls = match Self::get_class() {
            Some(c) => c,
            None => { log::error!("GameObject class not found"); return std::ptr::null_mut(); }
        };
        let obj = match cls.create() {
            Some(o) => o,
            None => { log::error!("GameObject create failed"); return std::ptr::null_mut(); }
        };
        if !name.is_null() {
            if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.GameObject::Internal_CreateGameObject") {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(obj, name);
            }
        }
        obj
    }
    pub fn get_component_static(instance: Il2CppObject, type_: Il2CppType) -> Il2CppObject {
        if instance.is_null() { return std::ptr::null_mut(); }
        if let Some(p) = extern_method!("UnityEngine.GameObject::GetComponent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return f(instance, type_);
        }
        let cls = match Self::get_class() {
            Some(c) => c,
            None => { log::error!("GameObject class not found in get_component_static"); return std::ptr::null_mut(); }
        };
        let m = match cls.method("GetComponent", 1) {
            Some(m) => m,
            None => { log::error!("GetComponent method not found"); return std::ptr::null_mut(); }
        };
        let p = match m.ptr() {
            Some(p) => p,
            None => { log::error!("GetComponent method pointer not found"); return std::ptr::null_mut(); }
        };
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        f(instance, type_)
    }
    pub fn add_component_static(instance: Il2CppObject, type_: Il2CppType) -> Il2CppObject {
        if instance.is_null() { return std::ptr::null_mut(); }
        if let Some(p) = extern_method!("UnityEngine.GameObject::AddComponent") {
            let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return f(instance, type_);
        }
        let cls = match Self::get_class() {
            Some(c) => c,
            None => { log::error!("GameObject class not found in add_component_static"); return std::ptr::null_mut(); }
        };
        let m = match cls.method("AddComponent", 1) {
            Some(m) => m,
            None => { log::error!("AddComponent method not found"); return std::ptr::null_mut(); }
        };
        let p = match m.ptr() {
            Some(p) => p,
            None => { log::error!("AddComponent method pointer not found"); return std::ptr::null_mut(); }
        };
        let f: extern "C" fn(Il2CppObject, Il2CppType) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        f(instance, type_)
    }
}

pub struct Transform(pub Il2CppObject);
impl Transform {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Transform") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_position(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_position") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_position", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_position(&self, pos: Vector3) {
        icall_or_method1_void!("UnityEngine.Transform::set_position", self.0, pos, Self::get_class().unwrap(), "set_position")
    }
    pub fn get_local_position(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_localPosition") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_localPosition", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_local_position(&self, pos: Vector3) {
        icall_or_method1_void!("UnityEngine.Transform::set_localPosition", self.0, pos, Self::get_class().unwrap(), "set_localPosition")
    }
    pub fn get_rotation(&self) -> Option<Quaternion> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_rotation") {
            let f: extern "C" fn(Il2CppObject) -> Quaternion = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_rotation", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Quaternion = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_rotation(&self, rot: Quaternion) {
        icall_or_method1_void!("UnityEngine.Transform::set_rotation", self.0, rot, Self::get_class().unwrap(), "set_rotation")
    }
    pub fn get_local_rotation(&self) -> Option<Quaternion> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_localRotation") {
            let f: extern "C" fn(Il2CppObject) -> Quaternion = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_localRotation", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Quaternion = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_local_rotation(&self, rot: Quaternion) {
        icall_or_method1_void!("UnityEngine.Transform::set_localRotation", self.0, rot, Self::get_class().unwrap(), "set_localRotation")
    }
    pub fn get_local_scale(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_localScale") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_localScale", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_local_scale(&self, scale: Vector3) {
        icall_or_method1_void!("UnityEngine.Transform::set_localScale", self.0, scale, Self::get_class().unwrap(), "set_localScale")
    }
    pub fn get_forward(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_forward") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_forward", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_forward(&self, forward: Vector3) {
        icall_or_method1_void!("UnityEngine.Transform::set_forward", self.0, forward, Self::get_class().unwrap(), "set_forward")
    }
    pub fn get_right(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_right") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_right", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn get_up(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_up") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_up", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn get_euler_angles(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_eulerAngles") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_eulerAngles", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_euler_angles(&self, val: Vector3) {
        icall_or_method1_void!("UnityEngine.Transform::set_eulerAngles", self.0, val, Self::get_class().unwrap(), "set_eulerAngles")
    }
    pub fn get_local_euler_angles(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::get_localEulerAngles") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_localEulerAngles", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_local_euler_angles(&self, val: Vector3) {
        icall_or_method1_void!("UnityEngine.Transform::set_localEulerAngles", self.0, val, Self::get_class().unwrap(), "set_localEulerAngles")
    }
    pub fn get_parent(&self) -> Option<Transform> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Transform::get_parent") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_parent", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Transform(r)) }
    }
    pub fn set_parent(&self, parent: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.Transform::set_parent", self.0, parent, Self::get_class().unwrap(), "set_parent")
    }
    pub fn set_parent_ex(&self, parent: &Transform, world_stays: bool) {
        icall_or_method2_void!("UnityEngine.Transform::SetParent", self.0, parent.0, world_stays, Self::get_class().unwrap(), "SetParent")
    }
    pub fn look_at_target(&self, target: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.Transform::LookAt", self.0, target, Self::get_class().unwrap(), "LookAt")
    }
    pub fn look_at_pos(&self, pos: Vector3) {
        icall_or_method1_void!("UnityEngine.Transform::LookAt", self.0, pos, Self::get_class().unwrap(), "LookAt")
    }
    pub fn inverse_transform_point(&self, pos: Vector3) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Transform::InverseTransformPoint") {
            let f: extern "C" fn(Il2CppObject, Vector3) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, pos));
        }
        let m = Self::get_class()?.method("InverseTransformPoint", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Vector3) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0, pos))
    }
}

pub struct RectTransform(pub Il2CppObject);
impl RectTransform {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "RectTransform") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_size_delta(&self, size: Vector2) {
        icall_or_method1_void!("UnityEngine.RectTransform::set_sizeDelta", self.0, size, Self::get_class().unwrap(), "set_sizeDelta")
    }
    pub fn get_size_delta(&self) -> Option<Vector2> {
        if let Some(p) = extern_method!("UnityEngine.RectTransform::get_sizeDelta") {
            let f: extern "C" fn(Il2CppObject) -> Vector2 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_sizeDelta", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector2 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_anchored_position3d(&self, pos: Vector3) {
        icall_or_method1_void!("UnityEngine.RectTransform::set_anchoredPosition3D", self.0, pos, Self::get_class().unwrap(), "set_anchoredPosition3D")
    }
    pub fn get_anchored_position3d(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.RectTransform::get_anchoredPosition3D") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_anchoredPosition3D", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_anchor_min(&self, val: Vector2) {
        icall_or_method1_void!("UnityEngine.RectTransform::set_anchorMin", self.0, val, Self::get_class().unwrap(), "set_anchorMin")
    }
    pub fn set_anchor_max(&self, val: Vector2) {
        icall_or_method1_void!("UnityEngine.RectTransform::set_anchorMax", self.0, val, Self::get_class().unwrap(), "set_anchorMax")
    }
    pub fn set_pivot(&self, val: Vector2) {
        icall_or_method1_void!("UnityEngine.RectTransform::set_pivot", self.0, val, Self::get_class().unwrap(), "set_pivot")
    }
    pub fn set_offset_min(&self, val: Vector2) {
        icall_or_method1_void!("UnityEngine.RectTransform::set_offsetMin", self.0, val, Self::get_class().unwrap(), "set_offsetMin")
    }
    pub fn set_offset_max(&self, val: Vector2) {
        icall_or_method1_void!("UnityEngine.RectTransform::set_offsetMax", self.0, val, Self::get_class().unwrap(), "set_offsetMax")
    }
    pub fn get_rect(&self) -> Option<Rect> {
        if let Some(p) = extern_method!("UnityEngine.RectTransform::get_rect") {
            let f: extern "C" fn(Il2CppObject) -> Rect = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_rect", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Rect = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
}

impl RectTransform {
    pub fn set_parent_static(transform: Il2CppObject, parent: Il2CppObject, world: bool) {
        icall_or_method2_void!("UnityEngine.Transform::SetParent", transform, parent, world, Transform::get_class().unwrap(), "SetParent")
    }
    pub fn set_local_position_static(transform: Il2CppObject, pos: Vector3) {
        Transform(transform).set_local_position(pos);
    }
    pub fn set_local_euler_angles_static(transform: Il2CppObject, angles: Vector3) {
        Transform(transform).set_local_euler_angles(angles);
    }
    pub fn set_local_scale_static(transform: Il2CppObject, scale: Vector3) {
        Transform(transform).set_local_scale(scale);
    }
    pub fn set_size_delta_static(transform: Il2CppObject, size: Vector2) {
        RectTransform(transform).set_size_delta(size);
    }
}

pub struct Behaviour(pub Il2CppObject);
impl Behaviour {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Behaviour") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_enabled(&self, enabled: bool) {
        icall_or_method1_void!("UnityEngine.Behaviour::set_enabled", self.0, enabled, Self::get_class().unwrap(), "set_enabled")
    }
    pub fn get_enabled(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.Behaviour::get_enabled") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_enabled", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
}

#[derive(Clone, Copy)]
pub struct Camera(pub Il2CppObject);
impl Camera {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Camera") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_main() -> Option<Camera> {
        if let Some(p) = extern_method!("UnityEngine.Camera::get_main") {
            let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f();
            return if r.is_null() { None } else { Some(Camera(r)) };
        }
        let m = Self::get_class()?.method("get_main", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f();
        if r.is_null() { None } else { Some(Camera(r)) }
    }
    pub fn get_far_clip_plane(&self) -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.Camera::get_farClipPlane") {
            let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_farClipPlane", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_far_clip_plane(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.Camera::set_farClipPlane", self.0, val, Self::get_class().unwrap(), "set_farClipPlane")
    }
}

pub struct Canvas(pub Il2CppObject);
impl Canvas {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Canvas") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_render_mode(&self, mode: RenderMode) {
        icall_or_method1_void!("UnityEngine.Canvas::set_renderMode", self.0, mode as i32, Self::get_class().unwrap(), "set_renderMode")
    }
    pub fn get_render_mode(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.Canvas::get_renderMode") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_renderMode", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_world_camera(&self, camera: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.Canvas::set_worldCamera", self.0, camera, Self::get_class().unwrap(), "set_worldCamera")
    }
}
impl Canvas {
    pub fn set_render_mode_static(canvas: Il2CppObject, mode: RenderMode) {
        Canvas(canvas).set_render_mode(mode);
    }
    pub fn get_type_static() -> Il2CppObject {
        Self::get_type().expect("Canvas type not found")
    }
}

pub struct Shader(pub Il2CppObject);
impl Shader {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Shader") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn find_s(name: &str) -> Option<Shader> {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        if let Some(p) = extern_method!("UnityEngine.Shader::Find") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(s);
            return if r.is_null() { None } else { Some(Shader(r)) };
        }
        let m = Self::get_class()?.method("Find", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(s);
        if r.is_null() { None } else { Some(Shader(r)) }
    }
    pub fn find(mono_string: Il2CppObject) -> Il2CppObject {
        if let Some(p) = extern_method!("UnityEngine.Shader::Find") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return f(mono_string);
        }
        let cls = match Self::get_class() {
            Some(c) => c,
            None => { log::error!("Shader class not found"); return std::ptr::null_mut(); }
        };
        let m = match cls.method("Find", 1) {
            Some(m) => m,
            None => { log::error!("Shader.Find method not found"); return std::ptr::null_mut(); }
        };
        let p = match m.ptr() {
            Some(p) => p,
            None => { log::error!("Shader.Find method pointer not found"); return std::ptr::null_mut(); }
        };
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        f(mono_string)
    }
}

pub struct Material(pub Il2CppObject);
impl Material {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Material") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_shader(&self, shader: &Shader) {
        icall_or_method1_void!("UnityEngine.Material::set_shader", self.0, shader.0, Self::get_class().unwrap(), "set_shader")
    }
    pub fn get_shader(&self) -> Option<Shader> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Material::get_shader") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_shader", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Shader(r)) }
    }
    pub fn set_color(&self, color: Color) {
        icall_or_method1_void!("UnityEngine.Material::set_color", self.0, color, Self::get_class().unwrap(), "set_color")
    }
    pub fn get_color(&self) -> Option<Color> {
        if let Some(p) = extern_method!("UnityEngine.Material::get_color") {
            let f: extern "C" fn(Il2CppObject) -> Color = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_color", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Color = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_float(&self, name: &str, val: f32) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method2_void!("UnityEngine.Material::SetFloat", self.0, s, val, Self::get_class().unwrap(), "SetFloat")
    }
    pub fn set_int(&self, name: &str, val: i32) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method2_void!("UnityEngine.Material::SetInt", self.0, s, val, Self::get_class().unwrap(), "SetInt")
    }
    pub fn set_texture(&self, name: &str, tex: Il2CppObject) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method2_void!("UnityEngine.Material::SetTexture", self.0, s, tex, Self::get_class().unwrap(), "SetTexture")
    }
    pub fn enable_keyword(&self, name: &str) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method1_void!("UnityEngine.Material::EnableKeyword", self.0, s, Self::get_class().unwrap(), "EnableKeyword")
    }
    pub fn disable_keyword(&self, name: &str) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method1_void!("UnityEngine.Material::DisableKeyword", self.0, s, Self::get_class().unwrap(), "DisableKeyword")
    }
}
impl Material {
    pub fn create_new_object(shader: Il2CppObject) -> Il2CppObject {
        let cls = match Self::get_class() {
            Some(c) => c,
            None => { log::error!("Material class not found"); return std::ptr::null_mut(); }
        };
        let obj = match cls.create() {
            Some(o) => o,
            None => { log::error!("Material create failed"); return std::ptr::null_mut(); }
        };
        if !shader.is_null() {
            Material(obj).set_shader(&Shader(shader));
        }
        obj
    }
}

pub struct Renderer(pub Il2CppObject);
impl Renderer {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Renderer") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_enabled(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.Renderer::set_enabled", self.0, val, Self::get_class().unwrap(), "set_enabled")
    }
    pub fn get_enabled(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.Renderer::get_enabled") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_enabled", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn get_material(&self) -> Option<Material> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Renderer::get_material") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_material", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Material(r)) }
    }
    pub fn set_material(&self, mat: &Material) {
        icall_or_method1_void!("UnityEngine.Renderer::set_material", self.0, mat.0, Self::get_class().unwrap(), "set_material")
    }
    pub fn get_material_array(&self) -> Option<Array> {
        if let Some(p) = extern_method!("UnityEngine.Renderer::GetMaterialArray") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = Self::get_class()?.method("GetMaterialArray", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn set_shadow_casting_mode(&self, mode: i32) {
        icall_or_method1_void!("UnityEngine.Renderer::set_shadowCastingMode", self.0, mode, Self::get_class().unwrap(), "set_shadowCastingMode")
    }
    pub fn set_rendering_layer_mask(&self, mask: i32) {
        icall_or_method1_void!("UnityEngine.Renderer::set_renderingLayerMask", self.0, mask, Self::get_class().unwrap(), "set_renderingLayerMask")
    }
}

pub struct SkinnedMeshRenderer(pub Il2CppObject);
impl SkinnedMeshRenderer {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "SkinnedMeshRenderer") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_material(&self) -> Option<Material> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Renderer::get_material") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Renderer::get_class()?.method("get_material", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Material(r)) }
    }
    pub fn set_material(&self, mat: &Material) {
        if let Some(m) = Renderer::get_class().and_then(|c| c.method("set_material", 1)) {
            m.call_1(self.0, mat.0);
        }
    }
    pub fn get_bones(&self) -> Option<Array> {
        if let Some(p) = extern_method!("UnityEngine.SkinnedMeshRenderer::get_bones") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = Self::get_class()?.method("get_bones", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn set_bones(&self, bones: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.SkinnedMeshRenderer::set_bones", self.0, bones, Self::get_class().unwrap(), "set_bones")
    }
    pub fn get_root_bone(&self) -> Option<Transform> {
        if let Some(p) = extern_method!("UnityEngine.SkinnedMeshRenderer::get_rootBone") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0);
            return if r.is_null() { None } else { Some(Transform(r)) };
        }
        let m = Self::get_class()?.method("get_rootBone", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0);
        if r.is_null() { None } else { Some(Transform(r)) }
    }
    pub fn get_update_when_offscreen(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.SkinnedMeshRenderer::get_updateWhenOffscreen") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_updateWhenOffscreen", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_update_when_offscreen(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.SkinnedMeshRenderer::set_updateWhenOffscreen", self.0, val, Self::get_class().unwrap(), "set_updateWhenOffscreen")
    }
}

pub struct Rigidbody(pub Il2CppObject);
impl Rigidbody {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Rigidbody") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_use_gravity(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.Rigidbody::set_useGravity", self.0, val, Self::get_class().unwrap(), "set_useGravity")
    }
    pub fn get_use_gravity(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.Rigidbody::get_useGravity") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_useGravity", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_mass(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.Rigidbody::set_mass", self.0, val, Self::get_class().unwrap(), "set_mass")
    }
    pub fn get_mass(&self) -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.Rigidbody::get_mass") {
            let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_mass", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn add_force(&self, force: Vector3) {
        icall_or_method1_void!("UnityEngine.Rigidbody::AddForce", self.0, force, Self::get_class().unwrap(), "AddForce")
    }
    pub fn set_velocity(&self, force: Vector3) {
        icall_or_method1_void!("UnityEngine.Rigidbody::set_velocity", self.0, force, Self::get_class().unwrap(), "set_velocity")
    }

    pub fn get_velocity(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.Rigidbody::get_velocity") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_velocity", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
}

pub struct Collider(pub Il2CppObject);
impl Collider {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Collider") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_enabled(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.Collider::get_enabled") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_enabled", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_enabled(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.Collider::set_enabled", self.0, val, Self::get_class().unwrap(), "set_enabled")
    }
    pub fn get_is_trigger(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.Collider::get_isTrigger") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_isTrigger", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_is_trigger(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.Collider::set_isTrigger", self.0, val, Self::get_class().unwrap(), "set_isTrigger")
    }
}

pub struct SphereCollider(pub Il2CppObject);
impl SphereCollider {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "SphereCollider") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_radius(&self) -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.SphereCollider::get_radius") {
            let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_radius", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_radius(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.SphereCollider::set_radius", self.0, val, Self::get_class().unwrap(), "set_radius")
    }
    pub fn get_center(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.SphereCollider::get_center") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_center", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_center(&self, val: Vector3) {
        icall_or_method1_void!("UnityEngine.SphereCollider::set_center", self.0, val, Self::get_class().unwrap(), "set_center")
    }
}

pub struct BoxCollider(pub Il2CppObject);
impl BoxCollider {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "BoxCollider") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_size(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.BoxCollider::get_size") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_size", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_size(&self, val: Vector3) {
        icall_or_method1_void!("UnityEngine.BoxCollider::set_size", self.0, val, Self::get_class().unwrap(), "set_size")
    }
    pub fn get_center(&self) -> Option<Vector3> {
        if let Some(p) = extern_method!("UnityEngine.BoxCollider::get_center") {
            let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_center", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Vector3 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_center(&self, val: Vector3) {
        icall_or_method1_void!("UnityEngine.BoxCollider::set_center", self.0, val, Self::get_class().unwrap(), "set_center")
    }
}

pub struct MeshCollider(pub Il2CppObject);
impl MeshCollider {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "MeshCollider") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_shared_mesh(&self) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.MeshCollider::get_sharedMesh") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_sharedMesh", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_shared_mesh(&self, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.MeshCollider::set_sharedMesh", self.0, val, Self::get_class().unwrap(), "set_sharedMesh")
    }
    pub fn get_convex(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.MeshCollider::get_convex") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_convex", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_convex(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.MeshCollider::set_convex", self.0, val, Self::get_class().unwrap(), "set_convex")
    }
}

pub struct MeshRenderer(pub Il2CppObject);
impl MeshRenderer {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "MeshRenderer") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct MeshFilter(pub Il2CppObject);
impl MeshFilter {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "MeshFilter") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_mesh(&self) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.MeshFilter::get_mesh") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_mesh", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_mesh(&self, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.MeshFilter::set_mesh", self.0, val, Self::get_class().unwrap(), "set_mesh")
    }
    pub fn get_shared_mesh(&self) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.MeshFilter::get_sharedMesh") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_sharedMesh", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_shared_mesh(&self, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.MeshFilter::set_sharedMesh", self.0, val, Self::get_class().unwrap(), "set_sharedMesh")
    }
}

pub struct TextMesh(pub Il2CppObject);
impl TextMesh {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "TextMesh") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_text(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.TextMesh::get_text") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_text", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_text(&self, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.TextMesh::set_text", self.0, val, Self::get_class().unwrap(), "set_text")
    }
    pub fn get_font(&self) -> Option<Font> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.TextMesh::get_font") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_font", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Font(r)) }
    }
    pub fn set_font(&self, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.TextMesh::set_font", self.0, val, Self::get_class().unwrap(), "set_font")
    }
    pub fn get_font_size(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.TextMesh::get_fontSize") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_fontSize", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_font_size(&self, val: i32) {
        icall_or_method1_void!("UnityEngine.TextMesh::set_fontSize", self.0, val, Self::get_class().unwrap(), "set_fontSize")
    }
}

pub struct UIBehavior(pub Il2CppObject);
impl UIBehavior {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.EventSystems", "UIBehaviour") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct CanvasScaler(pub Il2CppObject);
impl CanvasScaler {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.UI", "CanvasScaler") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_dynamic_pixels_per_unit(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.UI.CanvasScaler::set_dynamicPixelsPerUnit", self.0, val, Self::get_class().unwrap(), "set_dynamicPixelsPerUnit")
    }
    pub fn get_dynamic_pixels_per_unit(&self) -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.UI.CanvasScaler::get_dynamicPixelsPerUnit") {
            let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_dynamicPixelsPerUnit", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_scale_factor(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.UI.CanvasScaler::set_scaleFactor", self.0, val, Self::get_class().unwrap(), "set_scaleFactor")
    }
    pub fn set_reference_pixels_per_unit(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.UI.CanvasScaler::set_referencePixelsPerUnit", self.0, val, Self::get_class().unwrap(), "set_referencePixelsPerUnit")
    }
}

pub struct BaseRaycaster(pub Il2CppObject);
impl BaseRaycaster {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.EventSystems", "BaseRaycaster") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct GraphicRaycaster(pub Il2CppObject);
impl GraphicRaycaster {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.UI", "GraphicRaycaster") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct Graphic(pub Il2CppObject);
impl Graphic {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.UI", "Graphic") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_color(&self) -> Option<Color> {
        if let Some(p) = extern_method!("UnityEngine.UI.Graphic::get_color") {
            let f: extern "C" fn(Il2CppObject) -> Color = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_color", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Color = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_color(&self, val: Color) {
        icall_or_method1_void!("UnityEngine.UI.Graphic::set_color", self.0, val, Self::get_class().unwrap(), "set_color")
    }
}

pub struct MaskableGraphic(pub Il2CppObject);
impl MaskableGraphic {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.UI", "MaskableGraphic") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct Text(pub Il2CppObject);
impl Text {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.UI", "Text") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_text(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.UI.Text::get_text") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_text", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_text(&self, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.UI.Text::set_text", self.0, val, Self::get_class().unwrap(), "set_text")
    }
    pub fn get_font(&self) -> Option<Font> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.UI.Text::get_font") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_font", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Font(r)) }
    }
    pub fn set_font(&self, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.UI.Text::set_font", self.0, val, Self::get_class().unwrap(), "set_font")
    }
    pub fn get_font_size(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.UI.Text::get_fontSize") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_fontSize", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_font_size(&self, val: i32) {
        icall_or_method1_void!("UnityEngine.UI.Text::set_fontSize", self.0, val, Self::get_class().unwrap(), "set_fontSize")
    }
    pub fn get_font_style(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.UI.Text::get_fontStyle") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_fontStyle", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_font_style(&self, val: i32) {
        icall_or_method1_void!("UnityEngine.UI.Text::set_fontStyle", self.0, val, Self::get_class().unwrap(), "set_fontStyle")
    }
    pub fn get_alignment(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.UI.Text::get_alignment") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_alignment", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_alignment(&self, val: TextAnchor) {
        icall_or_method1_void!("UnityEngine.UI.Text::set_alignment", self.0, val, Self::get_class().unwrap(), "set_alignment")
    }
    pub fn set_support_rich_text(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.UI.Text::set_supportRichText", self.0, val, Self::get_class().unwrap(), "set_supportRichText")
    }
}
impl Text {
    pub fn set_text_static(instance: Il2CppObject, val: Il2CppObject) {
        Text(instance).set_text(val);
    }
    pub fn set_font_static(instance: Il2CppObject, val: Il2CppObject) {
        Text(instance).set_font(val);
    }
    pub fn set_font_style_static(instance: Il2CppObject, val: FontStyle) {
        Text(instance).set_font_style(val as i32);
    }
    pub fn set_material_static(instance: Il2CppObject, val: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.UI.Graphic::set_material", instance, val, Graphic::get_class().unwrap(), "set_material")
    }
    pub fn set_alignment_static(instance: Il2CppObject, val: TextAnchor) {
        Text(instance).set_alignment(val);
    }
}

pub struct Font(pub Il2CppObject);
impl Font {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Font") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn alive(obj: Il2CppObject) -> bool {
        if obj.is_null() { return false; }
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Object::op_Implicit") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return f(obj);
        }
        !obj.is_null()
    }
    pub fn create_dynamic_from_os_font(name: Il2CppObject, size: i32) -> Option<Font> {
        let m = Self::get_class()?.method("CreateDynamicFontFromOSFont", 2)?;
        if let Some(p) = m.ptr() {
            let f: extern "C" fn(Il2CppObject, i32) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(name, size);
            if r.is_null() { None } else { Some(Font(r)) }
        } else { None }
    }
    pub fn get_name(&self) -> Option<String> {
        NamedObject(self.0).get_name()
    }
    pub fn set_name(&self, name: &str) {
        NamedObject(self.0).set_name(name);
    }
}

pub struct LineRenderer(pub Il2CppObject);
impl LineRenderer {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "LineRenderer") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }

    pub fn get_material(&self) -> Option<Material> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Renderer::get_material") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Renderer::get_class()?.method("get_material", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Material(r)) }
    }
    pub fn set_material(&self, mat: &Material) {
        if let Some(m) = Renderer::get_class().and_then(|c| c.method("set_material", 1)) {
            m.call_1(self.0, mat.0);
        }
    }
    pub fn set_color_gradient(&self, gradient: &Gradient) {
        icall_or_method1_void!("UnityEngine.LineRenderer::set_colorGradient", self.0, gradient.0, Self::get_class().unwrap(), "set_colorGradient")
    }
    pub fn set_use_world_space(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.LineRenderer::set_useWorldSpace", self.0, val, Self::get_class().unwrap(), "set_useWorldSpace")
    }
    pub fn set_start_width(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.LineRenderer::set_startWidth", self.0, val, Self::get_class().unwrap(), "set_startWidth")
    }
    pub fn set_end_width(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.LineRenderer::set_endWidth", self.0, val, Self::get_class().unwrap(), "set_endWidth")
    }
    pub fn set_position(&self, index: i32, pos: Vector3) {
        if let Some(m) = Self::get_class().and_then(|c| c.method("SetPosition", 2)) {
            m.call_2(self.0, index, pos);
        }
    }
    pub fn get_position(&self, index: i32) -> Option<Vector3> {
        if let Some(m) = Self::get_class()?.method("GetPosition", 1) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, i32) -> Vector3 = unsafe { std::mem::transmute(p) };
                return Some(f(self.0, index));
            }
        }
        None
    }
    pub fn set_position_count(&self, val: i32) {
        icall_or_method1_void!("UnityEngine.LineRenderer::set_positionCount", self.0, val, Self::get_class().unwrap(), "set_positionCount")
    }
    pub fn set_positions(&self, positions: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.LineRenderer::SetPositions", self.0, positions, Self::get_class().unwrap(), "SetPositions")
    }
}

pub fn create_simple_gradient(color: Color) -> Option<Gradient> {
    let g = Gradient::new()?;
    let color_cls = Class::find("UnityEngine", "GradientColorKey")?;
    let mut color_arr = Array::new_specific(color_cls, 2)?;
    let keys = color_arr.as_mut_slice::<GradientColorKey>();
    keys[0] = GradientColorKey { color, time: 0.0 };
    keys[1] = GradientColorKey { color, time: 1.0 };
    let alpha_cls = Class::find("UnityEngine", "GradientAlphaKey")?;
    let mut alpha_arr = Array::new_specific(alpha_cls, 2)?;
    let akeys = alpha_arr.as_mut_slice::<GradientAlphaKey>();
    akeys[0] = GradientAlphaKey { alpha: 1.0, time: 0.0 };
    akeys[1] = GradientAlphaKey { alpha: 1.0, time: 1.0 };
    g.set_color_keys(color_arr.0);
    g.set_alpha_keys(alpha_arr.0);
    Some(g)
}

pub struct TrailRenderer(pub Il2CppObject);
impl TrailRenderer {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "TrailRenderer") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_time(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.TrailRenderer::set_time", self.0, val, Self::get_class().unwrap(), "set_time")
    }
}

pub struct Time {}
impl Time {
    pub fn get_delta_time() -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.Time::get_deltaTime") {
            let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f());
        }
        let m = Class::find("UnityEngine", "Time")?.method("get_deltaTime", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
        Some(f())
    }
    pub fn get_time() -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.Time::get_time") {
            let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f());
        }
        let m = Class::find("UnityEngine", "Time")?.method("get_time", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
        Some(f())
    }
    pub fn get_time_scale() -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.Time::get_timeScale") {
            let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f());
        }
        let m = Class::find("UnityEngine", "Time")?.method("get_timeScale", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
        Some(f())
    }
    pub fn set_time_scale(val: f32) {
        icall_or_method1_static_void!("UnityEngine.Time::set_timeScale", val, Class::find("UnityEngine", "Time").unwrap(), "set_timeScale")
    }
}

pub struct Resources {}
impl Resources {
    pub fn get_builtin_resource(type_: Il2CppObject, name: Il2CppObject) -> Il2CppObject {
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Resources::GetBuiltinResource") {
            let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return f(type_, name);
        }
        let cls = match Class::find("UnityEngine", "Resources") {
            Some(c) => c,
            None => { log::error!("Resources class not found"); return std::ptr::null_mut(); }
        };
        let m = match cls.method("GetBuiltinResource", 2) {
            Some(m) => m,
            None => { log::error!("GetBuiltinResource method not found"); return std::ptr::null_mut(); }
        };
        let p = match m.ptr() {
            Some(p) => p,
            None => { log::error!("GetBuiltinResource method pointer not found"); return std::ptr::null_mut(); }
        };
        let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        f(type_, name)
    }
    pub fn find_objects_of_type_all(type_: Il2CppObject) -> Option<Array> {
        let cls = Class::find("UnityEngine", "Resources")?;
        if let Some(p) = extern_method!("UnityEngine.Resources::FindObjectsOfTypeAll") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(type_);
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = cls.method("FindObjectsOfTypeAll", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(type_);
        if r.is_null() { None } else { Some(Array(r)) }
    }
    pub fn load(name: Il2CppObject) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Resources::Load") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(name);
            return if r.is_null() { None } else { Some(r) };
        }
        let m = Class::find("UnityEngine", "Resources")?.method("Load", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(name);
        if r.is_null() { None } else { Some(r) }
    }
}

pub struct AssetBundle(pub Il2CppObject);
impl AssetBundle {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "AssetBundle") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn load_from_file(path: Il2CppObject) -> Option<AssetBundle> {
        if let Some(p) = extern_method!("UnityEngine.AssetBundle::LoadFromFile") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(path);
            return if r.is_null() { None } else { Some(AssetBundle(r)) };
        }
        let m = Self::get_class()?.method("LoadFromFile", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(path);
        if r.is_null() { None } else { Some(AssetBundle(r)) }
    }
    pub fn load_asset(&self, name: Il2CppObject) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.AssetBundle::LoadAsset") {
            let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(self.0, name);
            return if r.is_null() { None } else { Some(r) };
        }
        let m = Self::get_class()?.method("LoadAsset", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(self.0, name);
        if r.is_null() { None } else { Some(r) }
    }
}

pub struct Physics {}
impl Physics {
    pub fn raycast(origin: Vector3, direction: Vector3, max_distance: f32) -> Option<bool> {
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Physics::Raycast_Vector3_Vector3_Single") {
            let f: extern "C" fn(Vector3, Vector3, f32) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(origin, direction, max_distance));
        }
        let cls = Class::find("UnityEngine", "Physics")?;
        let m = cls.method("Raycast", 3)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Vector3, Vector3, f32) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(origin, direction, max_distance))
    }
}

pub struct LightmapData(pub Il2CppObject);
impl LightmapData {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "LightmapData") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct LightmapSettings {}
impl LightmapSettings {
    pub fn get_lightmaps() -> Option<Array> {
        let cls = Class::find("UnityEngine", "LightmapSettings")?;
        if let Some(p) = extern_method!("UnityEngine.LightmapSettings::get_lightmaps") {
            let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f();
            return if r.is_null() { None } else { Some(Array(r)) };
        }
        let m = cls.method("get_lightmaps", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f();
        if r.is_null() { None } else { Some(Array(r)) }
    }
}

pub struct Texture2D(pub Il2CppObject);
impl Texture2D {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Texture2D") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_width(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.Texture2D::get_width") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_width", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn get_height(&self) -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.Texture2D::get_height") {
            let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_height", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_pixels32(&self, colors: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.Texture2D::SetPixels32", self.0, colors, Self::get_class().unwrap(), "SetPixels32")
    }
    pub fn apply(&self) {
        icall_or_method0_void!("UnityEngine.Texture2D::Apply", self.0, Self::get_class().unwrap(), "Apply")
    }
    pub fn apply_update_mipmaps(&self, make_no_longer_readable: bool) {
        icall_or_method1_void!("UnityEngine.Texture2D::Apply", self.0, make_no_longer_readable, Self::get_class().unwrap(), "Apply")
    }
    pub fn get_pixels32(&self) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Texture2D::GetPixels32") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("GetPixels32", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn encode_to_png(&self) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Texture2D::EncodeToPNG") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("EncodeToPNG", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
}

pub struct Gradient(pub Il2CppObject);
impl Gradient {
    pub fn new() -> Option<Gradient> {
        let cls = Class::find("UnityEngine", "Gradient")?;
        let obj = cls.create()?;
        Some(Gradient(obj))
    }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn set_color_keys(&self, keys: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.Gradient::SetColorKeys", self.0, keys, Class::find("UnityEngine", "Gradient").unwrap(), "SetColorKeys")
    }
    pub fn set_alpha_keys(&self, keys: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.Gradient::SetAlphaKeys", self.0, keys, Class::find("UnityEngine", "Gradient").unwrap(), "SetAlphaKeys")
    }
    pub fn evaluate(&self, time: f32) -> Color {
        let cls = Class::find("UnityEngine", "Gradient").unwrap();
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Gradient::Evaluate") {
            let f: extern "C" fn(Il2CppObject, f32) -> Color = unsafe { std::mem::transmute(p) };
            return f(self.0, time);
        }
        let m = cls.method("Evaluate", 1).unwrap();
        let p = m.ptr().unwrap();
        let f: extern "C" fn(Il2CppObject, f32) -> Color = unsafe { std::mem::transmute(p) };
        f(self.0, time)
    }
    pub fn get_color_keys(&self) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Gradient::get_colorKeys") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Class::find("UnityEngine", "Gradient")?.method("get_colorKeys", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
}

pub struct Skybox(pub Il2CppObject);
impl Skybox {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Skybox") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_material(&self) -> Option<Material> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Skybox::get_material") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_material", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Material(r)) }
    }
    pub fn set_material(&self, val: &Material) {
        icall_or_method1_void!("UnityEngine.Skybox::set_material", self.0, val.0, Self::get_class().unwrap(), "set_material")
    }
}

pub struct Sprite(pub Il2CppObject);
impl Sprite {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Sprite") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn create(texture: Il2CppObject, rect: Rect) -> Option<Sprite> {
        let cls = Self::get_class()?;
        let m = cls.method("Create", 2)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Rect) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(texture, rect);
        if r.is_null() { None } else { Some(Sprite(r)) }
    }
    pub fn get_rect(&self) -> Option<Rect> {
        if let Some(p) = extern_method!("UnityEngine.Sprite::get_rect") {
            let f: extern "C" fn(Il2CppObject) -> Rect = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_rect", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Rect = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
}

pub struct QualitySettings {}
impl QualitySettings {
    pub fn get_shadow_distance() -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.QualitySettings::get_shadowDistance") {
            let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f());
        }
        let m = Class::find("UnityEngine", "QualitySettings")?.method("get_shadowDistance", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> f32 = unsafe { std::mem::transmute(p) };
        Some(f())
    }
    pub fn set_shadow_distance(val: f32) {
        icall_or_method1_static_void!("UnityEngine.QualitySettings::set_shadowDistance", val, Class::find("UnityEngine", "QualitySettings").unwrap(), "set_shadowDistance")
    }
    pub fn get_anisotropic_filtering() -> Option<i32> {
        if let Some(p) = extern_method!("UnityEngine.QualitySettings::get_anisotropicFiltering") {
            let f: extern "C" fn() -> i32 = unsafe { std::mem::transmute(p) };
            return Some(f());
        }
        let m = Class::find("UnityEngine", "QualitySettings")?.method("get_anisotropicFiltering", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> i32 = unsafe { std::mem::transmute(p) };
        Some(f())
    }
    pub fn set_anisotropic_filtering(val: i32) {
        icall_or_method1_static_void!("UnityEngine.QualitySettings::set_anisotropicFiltering", val, Class::find("UnityEngine", "QualitySettings").unwrap(), "set_anisotropicFiltering")
    }
}

pub struct ParticleSystem(pub Il2CppObject);
impl ParticleSystem {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "ParticleSystem") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn play(&self) {
        icall_or_method0_void!("UnityEngine.ParticleSystem::Play", self.0, Self::get_class().unwrap(), "Play")
    }
    pub fn stop(&self) {
        icall_or_method0_void!("UnityEngine.ParticleSystem::Stop", self.0, Self::get_class().unwrap(), "Stop")
    }
    pub fn set_emission_enabled(&self, val: bool) {
        let cls = Self::get_class().unwrap();
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.ParticleSystem::set_emission_enabled") {
            let f: extern "C" fn(Il2CppObject, bool) = unsafe { std::mem::transmute(p) };
            f(self.0, val);
            return;
        }
        let m = cls.method("set_emission", 1).unwrap();
        let p = m.ptr().unwrap();
        let m2 = cls.method("get_emission", 0).unwrap();
        let p2 = m2.ptr().unwrap();
        let get_emission: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p2) };
        let emission = get_emission(self.0);
        let set_enabled: extern "C" fn(Il2CppObject, bool) = unsafe { std::mem::transmute(p) };
        set_enabled(emission, val);
    }
}

pub struct Light(pub Il2CppObject);
impl Light {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Light") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_color(&self) -> Option<Color> {
        if let Some(p) = extern_method!("UnityEngine.Light::get_color") {
            let f: extern "C" fn(Il2CppObject) -> Color = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_color", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Color = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_color(&self, val: Color) {
        icall_or_method1_void!("UnityEngine.Light::set_color", self.0, val, Self::get_class().unwrap(), "set_color")
    }
    pub fn get_intensity(&self) -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.Light::get_intensity") {
            let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_intensity", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_intensity(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.Light::set_intensity", self.0, val, Self::get_class().unwrap(), "set_intensity")
    }
    pub fn get_range(&self) -> Option<f32> {
        if let Some(p) = extern_method!("UnityEngine.Light::get_range") {
            let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_range", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> f32 = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn set_range(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.Light::set_range", self.0, val, Self::get_class().unwrap(), "set_range")
    }
}

pub struct AudioClip(pub Il2CppObject);
impl AudioClip {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "AudioClip") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct AudioSource(pub Il2CppObject);
impl AudioSource {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "AudioSource") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn play(&self) {
        icall_or_method0_void!("UnityEngine.AudioSource::Play", self.0, Self::get_class().unwrap(), "Play")
    }
    pub fn stop(&self) {
        icall_or_method0_void!("UnityEngine.AudioSource::Stop", self.0, Self::get_class().unwrap(), "Stop")
    }
    pub fn set_clip(&self, clip: Il2CppObject) {
        icall_or_method1_void!("UnityEngine.AudioSource::set_clip", self.0, clip, Self::get_class().unwrap(), "set_clip")
    }
    pub fn set_volume(&self, val: f32) {
        icall_or_method1_void!("UnityEngine.AudioSource::set_volume", self.0, val, Self::get_class().unwrap(), "set_volume")
    }
    pub fn set_loop(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.AudioSource::set_loop", self.0, val, Self::get_class().unwrap(), "set_loop")
    }
    pub fn set_mute(&self, val: bool) {
        icall_or_method1_void!("UnityEngine.AudioSource::set_mute", self.0, val, Self::get_class().unwrap(), "set_mute")
    }
}

pub struct LODGroup(pub Il2CppObject);
impl LODGroup {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "LODGroup") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct Animator(pub Il2CppObject);
impl Animator {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "Animator") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        if let Some(p) = extern_method!("UnityEngine.Animator::GetBool") {
            let f: extern "C" fn(Il2CppObject, Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0, s));
        }
        let m = Self::get_class()?.method("GetBool", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject, Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0, s))
    }
    pub fn set_bool(&self, name: &str, val: bool) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method2_void!("UnityEngine.Animator::SetBool", self.0, s, val, Self::get_class().unwrap(), "SetBool")
    }
    pub fn set_float(&self, name: &str, val: f32) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method2_void!("UnityEngine.Animator::SetFloat", self.0, s, val, Self::get_class().unwrap(), "SetFloat")
    }
    pub fn set_integer(&self, name: &str, val: i32) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method2_void!("UnityEngine.Animator::SetInteger", self.0, s, val, Self::get_class().unwrap(), "SetInteger")
    }
    pub fn set_trigger(&self, name: &str) {
        let s = string_new(name).unwrap_or(std::ptr::null_mut());
        icall_or_method1_void!("UnityEngine.Animator::SetTrigger", self.0, s, Self::get_class().unwrap(), "SetTrigger")
    }
}

pub struct MonoBehaviour(pub Il2CppObject);
impl MonoBehaviour {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine", "MonoBehaviour") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

pub struct Application {}
impl Application {
    pub fn quit() {
        icall_or_method0_static_void!("UnityEngine.Application::Quit", Class::find("UnityEngine", "Application").unwrap(), "Quit")
    }
    pub fn open_url(url: &str) {
        let s = string_new(url).unwrap_or(std::ptr::null_mut());
        icall_or_method1_static_void!("UnityEngine.Application::OpenURL", s, Class::find("UnityEngine", "Application").unwrap(), "OpenURL")
    }
    pub fn get_data_path() -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.Application::get_dataPath") {
            let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f());
        }
        let m = Class::find("UnityEngine", "Application")?.method("get_dataPath", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f())
    }
    pub fn get_persistent_data_path() -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.Application::get_persistentDataPath") {
            let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f());
        }
        let m = Class::find("UnityEngine", "Application")?.method("get_persistentDataPath", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn() -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f())
    }
}

pub struct PlayerPrefs {}
impl PlayerPrefs {
    pub fn get_int(key: &str, default: i32) -> i32 {
        let s = string_new(key).unwrap_or(std::ptr::null_mut());
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.PlayerPrefs::GetInt") {
            let f: extern "C" fn(Il2CppObject, i32) -> i32 = unsafe { std::mem::transmute(p) };
            return f(s, default);
        }
        let m = Class::find("UnityEngine", "PlayerPrefs").unwrap().method("GetInt", 2).unwrap();
        let p = m.ptr().unwrap();
        let f: extern "C" fn(Il2CppObject, i32) -> i32 = unsafe { std::mem::transmute(p) };
        f(s, default)
    }
    pub fn set_int(key: &str, val: i32) {
        let s = string_new(key).unwrap_or(std::ptr::null_mut());
        icall_or_method2_static_void!("UnityEngine.PlayerPrefs::SetInt", s, val, Class::find("UnityEngine", "PlayerPrefs").unwrap(), "SetInt")
    }
    pub fn get_float(key: &str, default: f32) -> f32 {
        let s = string_new(key).unwrap_or(std::ptr::null_mut());
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.PlayerPrefs::GetFloat") {
            let f: extern "C" fn(Il2CppObject, f32) -> f32 = unsafe { std::mem::transmute(p) };
            return f(s, default);
        }
        let m = Class::find("UnityEngine", "PlayerPrefs").unwrap().method("GetFloat", 2).unwrap();
        let p = m.ptr().unwrap();
        let f: extern "C" fn(Il2CppObject, f32) -> f32 = unsafe { std::mem::transmute(p) };
        f(s, default)
    }
    pub fn set_float(key: &str, val: f32) {
        let s = string_new(key).unwrap_or(std::ptr::null_mut());
        icall_or_method2_static_void!("UnityEngine.PlayerPrefs::SetFloat", s, val, Class::find("UnityEngine", "PlayerPrefs").unwrap(), "SetFloat")
    }
    pub fn get_string(key: &str, default: &str) -> Option<String> {
        let s_key = string_new(key).unwrap_or(std::ptr::null_mut());
        let s_def = string_new(default).unwrap_or(std::ptr::null_mut());
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.PlayerPrefs::GetString") {
            let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(s_key, s_def));
        }
        let m = Class::find("UnityEngine", "PlayerPrefs").unwrap().method("GetString", 2).unwrap();
        let p = m.ptr().unwrap();
        let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(s_key, s_def))
    }
    pub fn set_string(key: &str, val: &str) {
        let s_key = string_new(key).unwrap_or(std::ptr::null_mut());
        let s_val = string_new(val).unwrap_or(std::ptr::null_mut());
        icall_or_method2_static_void!("UnityEngine.PlayerPrefs::SetString", s_key, s_val, Class::find("UnityEngine", "PlayerPrefs").unwrap(), "SetString")
    }
    pub fn save() {
        icall_or_method0_static_void!("UnityEngine.PlayerPrefs::Save", Class::find("UnityEngine", "PlayerPrefs").unwrap(), "Save")
    }
    pub fn delete_all() {
        icall_or_method0_static_void!("UnityEngine.PlayerPrefs::DeleteAll", Class::find("UnityEngine", "PlayerPrefs").unwrap(), "DeleteAll")
    }
    pub fn delete_key(key: &str) {
        let s = string_new(key).unwrap_or(std::ptr::null_mut());
        icall_or_method1_static_void!("UnityEngine.PlayerPrefs::DeleteKey", s, Class::find("UnityEngine", "PlayerPrefs").unwrap(), "DeleteKey")
    }
    pub fn has_key(key: &str) -> bool {
        let s = string_new(key).unwrap_or(std::ptr::null_mut());
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.PlayerPrefs::HasKey") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return f(s);
        }
        let m = Class::find("UnityEngine", "PlayerPrefs").unwrap().method("HasKey", 1).unwrap();
        let p = m.ptr().unwrap();
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        f(s)
    }
}

pub struct DownloadHandler(pub Il2CppObject);
impl DownloadHandler {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.Networking", "DownloadHandler") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_text(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.Networking.DownloadHandler::get_text") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_text", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn get_data(&self) -> Option<Il2CppObject> {
        if let Some(p) = extern_method!("UnityEngine.Networking.DownloadHandler::get_data") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_data", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
}

pub struct DownloadHandlerTexture(pub Il2CppObject);
impl DownloadHandlerTexture {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.Networking", "DownloadHandlerTexture") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_texture(&self) -> Option<Texture2D> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Networking.DownloadHandlerTexture::get_texture") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_texture", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(Texture2D(r)) }
    }
}

pub struct UnityWebRequest(pub Il2CppObject);
impl UnityWebRequest {
    pub fn get_class() -> Option<Class> { Class::find("UnityEngine.Networking", "UnityWebRequest") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get(url: &str) -> Option<UnityWebRequest> {
        let s = string_new(url).unwrap_or(std::ptr::null_mut());
        let cls = Self::get_class()?;
        if let Some(p) = Il2Cpp::resolve_icall("UnityEngine.Networking.UnityWebRequest::Get") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            let r = f(s);
            return if r.is_null() { None } else { Some(UnityWebRequest(r)) };
        }
        let m = cls.method("Get", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(s);
        if r.is_null() { None } else { Some(UnityWebRequest(r)) }
    }
    pub fn send_web_request(&self) {
        icall_or_method0_void!("UnityEngine.Networking.UnityWebRequest::SendWebRequest", self.0, Self::get_class().unwrap(), "SendWebRequest")
    }
    pub fn get_download_handler(&self) -> Option<DownloadHandler> {
        let r = {
            if let Some(p) = extern_method!("UnityEngine.Networking.UnityWebRequest::get_downloadHandler") {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            } else {
                let m = Self::get_class()?.method("get_downloadHandler", 0)?;
                let p = m.ptr()?;
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                f(self.0)
            }
        };
        if r.is_null() { None } else { Some(DownloadHandler(r)) }
    }
    pub fn is_done(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.Networking.UnityWebRequest::get_isDone") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_isDone", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn is_error(&self) -> Option<bool> {
        if let Some(p) = extern_method!("UnityEngine.Networking.UnityWebRequest::get_isError") {
            let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
            return Some(f(self.0));
        }
        let m = Self::get_class()?.method("get_isError", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
        Some(f(self.0))
    }
    pub fn get_result(&self) -> Option<String> {
        if let Some(p) = extern_method!("UnityEngine.Networking.UnityWebRequest::get_result") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_result", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
}

pub struct GL {}
impl GL {
    pub fn begin(mode: i32) {
        icall_or_method1_static_void!("UnityEngine.GL::Begin", mode, Class::find("UnityEngine", "GL").unwrap(), "Begin")
    }
    pub fn end() {
        icall_or_method0_static_void!("UnityEngine.GL::End", Class::find("UnityEngine", "GL").unwrap(), "End")
    }
    pub fn push_matrix() {
        icall_or_method0_static_void!("UnityEngine.GL::PushMatrix", Class::find("UnityEngine", "GL").unwrap(), "PushMatrix")
    }
    pub fn pop_matrix() {
        icall_or_method0_static_void!("UnityEngine.GL::PopMatrix", Class::find("UnityEngine", "GL").unwrap(), "PopMatrix")
    }
    pub fn load_identity() {
        icall_or_method0_static_void!("UnityEngine.GL::LoadIdentity", Class::find("UnityEngine", "GL").unwrap(), "LoadIdentity")
    }
    pub fn load_pixel_matrix() {
        icall_or_method0_static_void!("UnityEngine.GL::LoadPixelMatrix", Class::find("UnityEngine", "GL").unwrap(), "LoadPixelMatrix")
    }
    pub fn vertex3(x: f32, y: f32, z: f32) {
        icall_or_method3_static_void!("UnityEngine.GL::Vertex3", x, y, z, Class::find("UnityEngine", "GL").unwrap(), "Vertex3")
    }
    pub fn color(color: Color) {
        icall_or_method1_static_void!("UnityEngine.GL::Color", color, Class::find("UnityEngine", "GL").unwrap(), "Color")
    }
    pub fn tex_coord2(x: f32, y: f32) {
        icall_or_method2_static_void!("UnityEngine.GL::TexCoord2", x, y, Class::find("UnityEngine", "GL").unwrap(), "TexCoord2")
    }
}

#[allow(non_camel_case_types)]
pub struct TMP_Asset(pub Il2CppObject);
impl TMP_Asset {
    pub fn get_class() -> Option<Class> { Class::find("TMPro", "TMP_Asset") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
}

#[allow(non_camel_case_types)]
pub struct TMP_FontAsset(pub Il2CppObject);
impl TMP_FontAsset {
    pub fn get_class() -> Option<Class> { Class::find("TMPro", "TMP_FontAsset") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn create_font_asset(font: Il2CppObject) -> Option<TMP_FontAsset> {
        let cls = Self::get_class()?;
        let m = cls.method("CreateFontAsset", 1)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        let r = f(font);
        if r.is_null() { None } else { Some(TMP_FontAsset(r)) }
    }
}

#[allow(non_camel_case_types)]
pub struct TMP_Text(pub Il2CppObject);
impl TMP_Text {
    pub fn get_class() -> Option<Class> { Class::find("TMPro", "TMP_Text") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_text(&self) -> Option<String> {
        if let Some(p) = extern_method!("TMPro.TMP_Text::get_text") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_text", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_text(&self, val: Il2CppObject) {
        icall_or_method1_void!("TMPro.TMP_Text::set_text", self.0, val, Self::get_class().unwrap(), "set_text")
    }
    pub fn set_font(&self, font: Il2CppObject) {
        icall_or_method1_void!("TMPro.TMP_Text::set_font", self.0, font, Self::get_class().unwrap(), "set_font")
    }
    pub fn set_font_size(&self, val: f32) {
        icall_or_method1_void!("TMPro.TMP_Text::set_fontSize", self.0, val, Self::get_class().unwrap(), "set_fontSize")
    }
    pub fn set_alignment(&self, val: TextAlignmentOptions) {
        icall_or_method1_void!("TMPro.TMP_Text::set_alignment", self.0, val as i32, Self::get_class().unwrap(), "set_alignment")
    }
    pub fn set_color(&self, val: Color) {
        icall_or_method1_void!("TMPro.TMP_Text::set_color", self.0, val, Self::get_class().unwrap(), "set_color")
    }
}

pub struct TextMeshPro(pub Il2CppObject);
impl TextMeshPro {
    pub fn get_class() -> Option<Class> { Class::find("TMPro", "TextMeshPro") }
    pub fn get_type() -> Option<Il2CppObject> { Self::get_class()?.system_type() }
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn get_text(&self) -> Option<String> {
        if let Some(p) = extern_method!("TMPro.TextMeshPro::get_text") {
            let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
            return string_from(f(self.0));
        }
        let m = Self::get_class()?.method("get_text", 0)?;
        let p = m.ptr()?;
        let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
        string_from(f(self.0))
    }
    pub fn set_text(&self, val: Il2CppObject) {
        icall_or_method1_void!("TMPro.TextMeshPro::set_text", self.0, val, Self::get_class().unwrap(), "set_text")
    }
    pub fn set_font(&self, font: Il2CppObject) {
        icall_or_method1_void!("TMPro.TextMeshPro::set_font", self.0, font, Self::get_class().unwrap(), "set_font")
    }
    pub fn set_font_size(&self, val: f32) {
        icall_or_method1_void!("TMPro.TextMeshPro::set_fontSize", self.0, val, Self::get_class().unwrap(), "set_fontSize")
    }
    pub fn set_alignment(&self, val: TextAlignmentOptions) {
        icall_or_method1_void!("TMPro.TextMeshPro::set_alignment", self.0, val as i32, Self::get_class().unwrap(), "set_alignment")
    }
    pub fn set_color(&self, val: Color) {
        icall_or_method1_void!("TMPro.TextMeshPro::set_color", self.0, val, Self::get_class().unwrap(), "set_color")
    }
}


pub struct List(pub Il2CppObject);
impl List {
    pub fn raw(&self) -> Il2CppObject { self.0 }

    pub fn count(&self) -> i32 {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("get_Count", 0)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
                return f(self.0);
            }
        }
        0
    }

    pub fn get_item(&self, index: i32) -> Il2CppObject {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("get_Item", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, i32) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                return f(self.0, index);
            }
        }
        std::ptr::null_mut()
    }

    pub fn set_item(&self, index: i32, val: Il2CppObject) {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("set_Item", 2)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, i32, Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(self.0, index, val);
            }
        }
    }

    pub fn add(&self, val: Il2CppObject) {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("Add", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(self.0, val);
            }
        }
    }

    pub fn clear(&self) {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("Clear", 0)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(self.0);
            }
        }
    }

    pub fn remove(&self, val: Il2CppObject) -> bool {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("Remove", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
                return f(self.0, val);
            }
        }
        false
    }

    pub fn remove_at(&self, index: i32) {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("RemoveAt", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, i32) = unsafe { std::mem::transmute(p) };
                f(self.0, index);
            }
        }
    }

    pub fn contains(&self, val: Il2CppObject) -> bool {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("Contains", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
                return f(self.0, val);
            }
        }
        false
    }

    pub fn index_of(&self, val: Il2CppObject) -> i32 {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("IndexOf", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
                return f(self.0, val);
            }
        }
        -1
    }

    pub fn items_array(&self) -> Option<Array> {
        let f = self.0.class_of()?.field("_items")?;
        let ptr = f.get_obj(self.0);
        if ptr.is_null() { None } else { Some(Array(ptr)) }
    }

    pub fn size_field(&self) -> i32 {
        self.0.class_of()
            .and_then(|c| c.field("_size"))
            .map(|mut f| { f.set_instance(self.0); f.getv::<i32>() })
            .unwrap_or(0)
    }
}

pub struct Dictionary(pub Il2CppObject);
impl Dictionary {
    pub fn raw(&self) -> Il2CppObject { self.0 }

    pub fn count(&self) -> i32 {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("get_Count", 0)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
                return f(self.0);
            }
        }
        0
    }

    pub fn get_item(&self, key: Il2CppObject) -> Il2CppObject {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("get_Item", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                return f(self.0, key);
            }
        }
        std::ptr::null_mut()
    }

    pub fn set_item(&self, key: Il2CppObject, val: Il2CppObject) {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("set_Item", 2)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject, Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(self.0, key, val);
            }
        }
    }

    pub fn add(&self, key: Il2CppObject, val: Il2CppObject) {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("Add", 2)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject, Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(self.0, key, val);
            }
        }
    }

    pub fn try_get_value(&self, key: Il2CppObject) -> Option<Il2CppObject> {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("TryGetValue", 2)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject, *mut Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
                let mut val: Il2CppObject = std::ptr::null_mut();
                if f(self.0, key, &mut val) { Some(val) } else { None }
            } else { None }
        } else { None }
    }

    pub fn contains_key(&self, key: Il2CppObject) -> bool {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("ContainsKey", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
                return f(self.0, key);
            }
        }
        false
    }

    pub fn remove(&self, key: Il2CppObject) -> bool {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("Remove", 1)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject, Il2CppObject) -> bool = unsafe { std::mem::transmute(p) };
                return f(self.0, key);
            }
        }
        false
    }

    pub fn clear(&self) {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("Clear", 0)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject) = unsafe { std::mem::transmute(p) };
                f(self.0);
            }
        }
    }

    pub fn keys(&self) -> Option<Il2CppObject> {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("get_Keys", 0)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                let r = f(self.0);
                return if r.is_null() { None } else { Some(r) };
            }
        }
        None
    }

    pub fn values(&self) -> Option<Il2CppObject> {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("get_Values", 0)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject) -> Il2CppObject = unsafe { std::mem::transmute(p) };
                let r = f(self.0);
                return if r.is_null() { None } else { Some(r) };
            }
        }
        None
    }
}

pub struct String_(pub Il2CppObject);
impl String_ {
    pub fn raw(&self) -> Il2CppObject { self.0 }
    pub fn new(s: &str) -> Option<Self> {
        crate::unity::string_new(s).map(String_)
    }
    pub fn to_rust(&self) -> Option<String> {
        crate::unity::string_from(self.0)
    }
    pub fn get_length(&self) -> i32 {
        if let Some(m) = self.0.class_of().and_then(|c| c.method("get_Length", 0)) {
            if let Some(p) = m.ptr() {
                let f: extern "C" fn(Il2CppObject) -> i32 = unsafe { std::mem::transmute(p) };
                return f(self.0);
            }
        }
        0
    }
    pub fn is_null_or_empty(&self) -> bool {
        if self.0.is_null() { return true; }
        self.get_length() == 0
    }
}
