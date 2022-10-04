use std::ffi::{c_char, c_double, c_int, c_void};

#[repr(C)]
pub struct WrenVM([u8; 0]);

#[repr(C)]
pub struct WrenHandle([u8; 0]);

pub type WrenReallocateFn =
    extern "C" fn(memory: *mut c_void, new_size: usize, user_data: *const c_void) -> *mut c_void;

pub type WrenForeignMethodFn = extern "C" fn(vm: *mut WrenVM);

pub type WrenFinalizerFn = extern "C" fn(data: *mut c_void);

pub type WrenResolveModuleFn =
    extern "C" fn(vm: *mut WrenVM, importer: *const c_char, name: *const c_char) -> *const c_char;

pub type WrenLoadModuleCompleteFn =
    extern "C" fn(vm: *mut WrenVM, name: *const c_char, result: WrenLoadModuleResult);

#[repr(C)]
pub struct WrenLoadModuleResult {
    pub source: *mut c_char,
    pub on_complete: WrenLoadModuleCompleteFn,
    pub user_data: *mut c_void,
}

pub type WrenLoadModuleFn =
    extern "C" fn(vm: *mut WrenVM, name: *const c_char) -> WrenLoadModuleResult;

pub type WrenBindForeignMethodFn = extern "C" fn(
    vm: *mut WrenVM,
    module: *const c_char,
    class_name: *const c_char,
    is_static: bool,
    signature: *const c_char,
) -> WrenForeignMethodFn;

pub type WrenWriteFn = extern "C" fn(vm: *mut WrenVM, text: *const c_char);

#[repr(C)]
pub enum WrenErrorType {
    Compile,
    Runtime,
    StackTrace,
}

pub type WrenErrorFn = extern "C" fn(
    vm: *mut WrenVM,
    ty: WrenErrorType,
    module: *const c_char,
    line: c_int,
    message: *const c_char,
);

#[repr(C)]
pub struct WrenForeignClassMethods {
    pub allocate: WrenForeignMethodFn,
    pub finalize: WrenFinalizerFn,
}

pub type WrenBindForeignClassFn = extern "C" fn(
    vm: *mut WrenVM,
    module: *const c_char,
    class_name: *const c_char,
) -> WrenForeignClassMethods;

#[repr(C)]
pub struct WrenConfiguration {
    pub reallocate_fn: WrenReallocateFn,
    pub resolve_module_fn: WrenResolveModuleFn,
    pub load_module_fn: WrenLoadModuleFn,
    pub bind_foreign_method_fn: WrenBindForeignMethodFn,
    pub bind_foreign_class_fn: WrenBindForeignClassFn,
    pub write_fn: WrenWriteFn,
    pub error_fn: WrenErrorFn,
    pub initial_heap_size: usize,
    pub min_heap_size: usize,
    pub heap_growth_percent: c_int,
    pub user_data: *mut c_void,
}

#[repr(C)]
pub enum WrenInterpretResult {
    Success,
    CompileError,
    RuntimeError,
}

#[repr(C)]
pub enum WrenType {
    Bool,
    Num,
    Foreign,
    List,
    Map,
    Null,
    String,
    Unknown,
}

extern "C" {
    pub fn wrenInitConfiguration(configuration: *mut WrenConfiguration);
    pub fn wrenNewVM(configuration: *const WrenConfiguration) -> *mut WrenVM;
    pub fn wrenFreeVM(vm: *mut WrenVM);
    pub fn wrenCollectGarbage(vm: *mut WrenVM);
    pub fn wrenInterpret(
        vm: *mut WrenVM,
        module: *const c_char,
        source: *const c_char,
    ) -> WrenInterpretResult;
    pub fn wrenMakeCallHandle(vm: *mut WrenVM, signature: *const c_char) -> *const WrenHandle;
    pub fn wrenCall(vm: *mut WrenVM, method: *const WrenHandle) -> WrenInterpretResult;
    pub fn wrenReleaseHandle(vm: *mut WrenVM, handle: *const WrenHandle);
    pub fn wrenGetSlotCount(vm: *mut WrenVM) -> c_int;
    pub fn wrenEnsureSlots(vm: *mut WrenVM, num_slots: c_int);
    pub fn wrenGetSlotType(vm: *mut WrenVM, slot: c_int) -> WrenType;
    pub fn wrenGetSlotBool(vm: *mut WrenVM, slot: c_int) -> bool;
    pub fn wrenGetSlotBytes(vm: *mut WrenVM, slot: c_int, length: *mut c_int) -> *const c_char;
    pub fn wrenGetSlotDouble(vm: *mut WrenVM, slot: c_int) -> c_double;
    pub fn wrenGetSlotForeign(vm: *mut WrenVM, slot: c_int) -> *mut c_void;
    pub fn wrenGetSlotString(vm: *mut WrenVM, slot: c_int) -> *const c_char;
    pub fn wrenGetSlotHandle(vm: *mut WrenVM, slot: c_int) -> *const WrenHandle;
    pub fn wrenSetSlotBool(vm: *mut WrenVM, slot: c_int, value: bool);
    pub fn wrenSetSlotBytes(vm: *mut WrenVM, slot: c_int, bytes: *const c_char, length: usize);
    pub fn wrenSetSlotDouble(vm: *mut WrenVM, slot: c_int, value: c_double);
    pub fn wrenSetSlotNewForeign(
        vm: *mut WrenVM,
        slot: c_int,
        class_slot: c_int,
        size: usize,
    ) -> *mut c_void;
    pub fn wrenSetSlotNewList(vm: *mut WrenVM, slot: c_int);
    pub fn wrenSetSlotNewMap(vm: *mut WrenVM, slot: c_int);
    pub fn wrenSetSlotNull(vm: *mut WrenVM, slot: c_int);
    pub fn wrenSetSlotString(vm: *mut WrenVM, slot: c_int, text: *const c_char);
    pub fn wrenSetSlotHandle(vm: *mut WrenVM, slot: c_int, handle: *const WrenHandle);
    pub fn wrenGetListCount(vm: *mut WrenVM, slot: c_int) -> c_int;
    pub fn wrenGetListElement(vm: *mut WrenVM, list_slot: c_int, index: c_int, element_slot: c_int);
    pub fn wrenSetListElement(vm: *mut WrenVM, list_slot: c_int, index: c_int, element_slot: c_int);
    pub fn wrenInsertInList(vm: *mut WrenVM, list_slot: c_int, index: c_int, element_slot: c_int);
    pub fn wrenGetMapCount(vm: *mut WrenVM, slot: c_int) -> c_int;
    pub fn wrenGetMapContainsKey(vm: *mut WrenVM, map_slot: c_int, key_slot: c_int) -> bool;
    pub fn wrenGetMapValue(vm: *mut WrenVM, map_slot: c_int, key_slot: c_int, value_slot: c_int);
    pub fn wrenSetMapValue(vm: *mut WrenVM, map_slot: c_int, key_slot: c_int, value_slot: c_int);
    pub fn wrenRemoveMapValue(
        vm: *mut WrenVM,
        map_slot: c_int,
        key_slot: c_int,
        removed_value_slot: c_int,
    );
    pub fn wrenGetVariable(
        vm: *mut WrenVM,
        module: *const c_char,
        name: *const c_char,
        slot: c_int,
    );
    pub fn wrenHasVariable(vm: *mut WrenVM, module: *const c_char, name: *const c_char) -> bool;
    pub fn wrenHasModule(vm: *mut WrenVM, module: *const c_char) -> bool;
    pub fn wrenAbortFiber(vm: *mut WrenVM, slot: c_int);
    pub fn wrenGetUserData(vm: *mut WrenVM) -> *mut c_void;
    pub fn wrenSetUserData(vm: *mut WrenVM, user_data: *mut c_void);
}
