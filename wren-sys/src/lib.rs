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
    source: *const c_char,
    on_complete: WrenLoadModuleCompleteFn,
    user_data: *mut c_void,
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
    allocate: WrenForeignMethodFn,
    finalize: WrenFinalizerFn,
}

pub type WrenBindForeignClassFn = extern "C" fn(
    vm: *mut WrenVM,
    module: *const c_char,
    class_name: *const c_char,
) -> WrenForeignClassMethods;

#[repr(C)]
pub struct WrenConfiguration {
    reallocate_fn: WrenReallocateFn,
    resolve_module_fn: WrenResolveModuleFn,
    load_module_fn: WrenLoadModuleFn,
    bind_foreign_method_fn: WrenBindForeignMethodFn,
    bind_foreign_class_fn: WrenBindForeignClassFn,
    write_fn: WrenWriteFn,
    error_fn: WrenErrorFn,
    initial_heap_size: usize,
    min_heap_size: usize,
    heap_growth_percent: c_int,
    user_data: *mut c_void,
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
    //WREN_API void wrenInitConfiguration(WrenConfiguration* configuration);
    pub fn wrenInitConfiguration(configuration: *mut WrenConfiguration);

    //WREN_API WrenVM* wrenNewVM(WrenConfiguration* configuration);
    pub fn wrenNewVM(configuration: *const WrenConfiguration) -> *mut WrenVM;

    //WREN_API void wrenFreeVM(WrenVM* vm);
    pub fn wrenFreeVM(vm: *mut WrenVM);

    //WREN_API void wrenCollectGarbage(WrenVM* vm);
    pub fn wrenCollectGarbage(vm: *mut WrenVM);

    //WREN_API WrenInterpretResult wrenInterpret(WrenVM* vm, const char* module,
    //                                   const char* source);
    pub fn wrenInterpret(
        vm: *mut WrenVM,
        module: *const c_char,
        source: *const c_char,
    ) -> WrenInterpretResult;

    //WREN_API WrenHandle* wrenMakeCallHandle(WrenVM* vm, const char* signature);
    pub fn wrenMakeCallHandle(vm: *mut WrenVM, signature: *const c_char) -> *const WrenHandle;

    //WREN_API WrenInterpretResult wrenCall(WrenVM* vm, WrenHandle* method);
    pub fn wrenCall(vm: *mut WrenVM, method: *const WrenHandle) -> WrenInterpretResult;

    //WREN_API void wrenReleaseHandle(WrenVM* vm, WrenHandle* handle);
    pub fn wrenReleaseHandle(vm: *mut WrenVM, handle: *const WrenHandle);

    //WREN_API int wrenGetSlotCount(WrenVM* vm);
    pub fn wrenGetSlotCount(vm: *mut WrenVM) -> c_int;

    //WREN_API void wrenEnsureSlots(WrenVM* vm, int numSlots);
    pub fn wrenEnsureSlots(vm: *mut WrenVM, num_slots: c_int);

    //WREN_API WrenType wrenGetSlotType(WrenVM* vm, int slot);
    pub fn wrenGetSlotType(vm: *mut WrenVM, slot: c_int) -> WrenType;

    //WREN_API bool wrenGetSlotBool(WrenVM* vm, int slot);
    pub fn wrenGetSlotBool(vm: *mut WrenVM, slot: c_int) -> bool;

    //WREN_API const char* wrenGetSlotBytes(WrenVM* vm, int slot, int* length);
    pub fn wrenGetSlotBytes(vm: *mut WrenVM, slot: c_int, length: *mut c_int) -> *const c_char;

    //WREN_API double wrenGetSlotDouble(WrenVM* vm, int slot);
    pub fn wrenGetSlotDouble(vm: *mut WrenVM, slot: c_int) -> c_double;

    //WREN_API void* wrenGetSlotForeign(WrenVM* vm, int slot);
    pub fn wrenGetSlotForeign(vm: *mut WrenVM, slot: c_int) -> *mut c_void;

    //WREN_API const char* wrenGetSlotString(WrenVM* vm, int slot);
    pub fn wrenGetSlotString(vm: *mut WrenVM, slot: c_int) -> *const c_char;

    //WREN_API WrenHandle* wrenGetSlotHandle(WrenVM* vm, int slot);
    pub fn wrenGetSlotHandle(vm: *mut WrenVM, slot: c_int) -> *const WrenHandle;

    //WREN_API void wrenSetSlotBool(WrenVM* vm, int slot, bool value);
    pub fn wrenSetSlotBool(vm: *mut WrenVM, slot: c_int, value: bool);

    //WREN_API void wrenSetSlotBytes(WrenVM* vm, int slot, const char* bytes, size_t length);
    pub fn wrenSetSlotBytes(vm: *mut WrenVM, slot: c_int, bytes: *const c_char, length: usize);

    //WREN_API void wrenSetSlotDouble(WrenVM* vm, int slot, double value);
    pub fn wrenSetSlotDouble(vm: *mut WrenVM, slot: c_int, value: c_double);

    //WREN_API void* wrenSetSlotNewForeign(WrenVM* vm, int slot, int classSlot, size_t size);
    pub fn wrenSetSlotNewForeign(
        vm: *mut WrenVM,
        slot: c_int,
        class_slot: c_int,
        size: usize,
    ) -> *mut c_void;

    //WREN_API void wrenSetSlotNewList(WrenVM* vm, int slot);
    pub fn wrenSetSlotNewList(vm: *mut WrenVM, slot: c_int);

    //WREN_API void wrenSetSlotNewMap(WrenVM* vm, int slot);
    pub fn wrenSetSlotNewMap(vm: *mut WrenVM, slot: c_int);

    //WREN_API void wrenSetSlotNull(WrenVM* vm, int slot);
    pub fn wrenSetSlotNull(vm: *mut WrenVM, slot: c_int);

    //WREN_API void wrenSetSlotString(WrenVM* vm, int slot, const char* text);
    pub fn wrenSetSlotString(vm: *mut WrenVM, slot: c_int, text: *const c_char);

    //WREN_API void wrenSetSlotHandle(WrenVM* vm, int slot, WrenHandle* handle);
    pub fn wrenSetSlotHandle(vm: *mut WrenVM, slot: c_int, handle: *const WrenHandle);

    //WREN_API int wrenGetListCount(WrenVM* vm, int slot);
    pub fn wrenGetListCount(vm: *mut WrenVM, slot: c_int) -> c_int;

    //WREN_API void wrenGetListElement(WrenVM* vm, int listSlot, int index, int elementSlot);
    pub fn wrenGetListElement(vm: *mut WrenVM, list_slot: c_int, index: c_int, element_slot: c_int);

    //WREN_API void wrenSetListElement(WrenVM* vm, int listSlot, int index, int elementSlot);
    pub fn wrenSetListElement(vm: *mut WrenVM, list_slot: c_int, index: c_int, element_slot: c_int);

    //WREN_API void wrenInsertInList(WrenVM* vm, int listSlot, int index, int elementSlot);
    pub fn wrenInsertInList(vm: *mut WrenVM, list_slot: c_int, index: c_int, element_slot: c_int);

    //WREN_API int wrenGetMapCount(WrenVM* vm, int slot);
    pub fn wrenGetMapCount(vm: *mut WrenVM, slot: c_int) -> c_int;

    //WREN_API bool wrenGetMapContainsKey(WrenVM* vm, int mapSlot, int keySlot);
    pub fn wrenGetMapContainsKey(vm: *mut WrenVM, map_slot: c_int, key_slot: c_int) -> bool;

    //WREN_API void wrenGetMapValue(WrenVM* vm, int mapSlot, int keySlot, int valueSlot);
    pub fn wrenGetMapValue(vm: *mut WrenVM, map_slot: c_int, key_slot: c_int, value_slot: c_int);

    //WREN_API void wrenSetMapValue(WrenVM* vm, int mapSlot, int keySlot, int valueSlot);
    pub fn wrenSetMapValue(vm: *mut WrenVM, map_slot: c_int, key_slot: c_int, value_slot: c_int);

    //WREN_API void wrenRemoveMapValue(WrenVM* vm, int mapSlot, int keySlot,
    //                         int removedValueSlot);
    pub fn wrenRemoveMapValue(
        vm: *mut WrenVM,
        map_slot: c_int,
        key_slot: c_int,
        removed_value_slot: c_int,
    );

    //WREN_API void wrenGetVariable(WrenVM* vm, const char* module, const char* name,
    //                      int slot);
    pub fn wrenGetVariable(
        vm: *mut WrenVM,
        module: *const c_char,
        name: *const c_char,
        slot: c_int,
    );

    //WREN_API bool wrenHasVariable(WrenVM* vm, const char* module, const char* name);
    pub fn wrenHasVariable(vm: *mut WrenVM, module: *const c_char, name: *const c_char) -> bool;

    //WREN_API bool wrenHasModule(WrenVM* vm, const char* module);
    pub fn wrenHasModule(vm: *mut WrenVM, module: *const c_char) -> bool;

    //WREN_API void wrenAbortFiber(WrenVM* vm, int slot);
    pub fn wrenAbortFiber(vm: *mut WrenVM, slot: c_int);

    //WREN_API void* wrenGetUserData(WrenVM* vm);
    pub fn wrenGetUserData(vm: *mut WrenVM) -> *mut c_void;

    //WREN_API void wrenSetUserData(WrenVM* vm, void* userData);
    pub fn wrenSetUserData(vm: *mut WrenVM, user_data: *mut c_void);
}
