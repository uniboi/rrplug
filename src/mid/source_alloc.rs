use std::{
    alloc::GlobalAlloc,
    ffi::{c_char, c_void},
};

use once_cell::sync::OnceCell;
use windows::{
    core::PCSTR,
    Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
};

use crate::{create_external_interface, high::UnsafeHandle};

type CreateGlobalMemAlloc = extern "C" fn() -> *const IMemAlloc;

pub static SOURCE_ALLOC: SourceAlloc = SourceAlloc(OnceCell::new());

pub struct SourceAlloc(OnceCell<UnsafeHandle<&'static IMemAlloc>>);

impl SourceAlloc {
    pub(crate) fn init(&self) {
        let create_global_mem_alloc = unsafe {
            std::mem::transmute::<_, CreateGlobalMemAlloc>(
                GetProcAddress(
                    GetModuleHandleA(PCSTR("tier0.dll\0".as_ptr())).expect("couldn't find tier0"),
                    PCSTR("CreateGlobalMemAlloc\0".as_ptr()),
                )
                .expect("couldn't find CreateGlobalMemAlloc"),
            )
        };
        _ = self.0.set(unsafe {
            UnsafeHandle::new(
                create_global_mem_alloc()
                    .as_ref()
                    .expect("IMemAlloc is invalid"),
            )
        })
    }
}

unsafe impl GlobalAlloc for SourceAlloc {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        debug_assert!(
            self.0.get().is_some(),
            "cannot use SourceAlloc before entry::new"
        );
        unsafe { self.0.get_unchecked().copy().Alloc(layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: std::alloc::Layout) {
        debug_assert!(
            self.0.get().is_some(),
            "cannot use SourceAlloc before entry::new"
        );
        unsafe { self.0.get_unchecked().copy().Free(ptr as *mut c_void) }
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        _layout: std::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        debug_assert!(
            self.0.get().is_some(),
            "cannot use SourceAlloc before entry::new"
        );
        unsafe {
            self.0
                .get_unchecked()
                .copy()
                .Realloc(ptr as *mut c_void, new_size) as *mut u8
        }
    }
}

create_external_interface! {
    pub IMemAlloc + IMemAllocMod => {
        pub(self) fn unk0() -> ();
        pub fn Alloc(Size: usize) -> *const c_void;
        pub(self) fn unk2() -> ();
        pub fn Realloc(Mem: *mut c_void, Size: usize) -> *const c_void;
        pub(self) fn unk4() -> ();
        pub fn Free(Mem: *mut c_void) -> ();
        pub(self) fn unk6() -> ();
        pub(self) fn unk7() -> ();
        pub fn GetSize(Mem: *const c_void) -> usize;
        pub(self) fn unk9() -> ();
        pub(self) fn unk10() -> ();
        pub(self) fn unk11() -> ();
        pub(self) fn unk12() -> ();
        pub(self) fn unk13() -> ();
        pub(self) fn unk14() -> ();
        pub(self) fn unk15() -> ();
        pub(self) fn unk16() -> ();
        pub(self) fn unk17() -> ();
        pub fn DumpStats() -> ();
        pub fn DumpStatsFileBase(FileBase: *const c_char) -> ();
        pub(self) fn unk19() -> ();
        pub(self) fn unk20() -> ();
        pub(self) fn unk21() -> ();
        pub(self) fn unk22() -> ();
        pub fn heapchk() -> i32;
    }
}
