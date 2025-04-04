#![allow(clippy::missing_safety_doc)]
#![no_std]

extern crate alloc;

use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    fmt::{self, Write as _},
};

use linkme::distributed_slice;
// Rexport these macros
pub use primeapi_macros::{cpp_method, cw_link_name, patch_fn, prolog_fn};

pub mod rstl;
pub mod dol_sdk {
    pub mod dvd;
    pub mod os;
}
pub mod alignment_utils;
pub mod mp1;

#[doc(hidden)]
pub mod reexport {
    pub use paste;
}

#[macro_export]
macro_rules! cpp_field {
    ($id:ident: $ty:ty; ptr @ $e:expr) => {
        #[inline(always)]
        pub fn $id(this: *const Self) -> *const $ty
        {
            (this as usize + ($e)) as *mut _
        }

        $crate::reexport::paste::item! {
            #[inline(always)]
            pub fn [<$id _mut>](this: *mut Self) -> *mut $ty
            {
                (this as usize + ($e)) as *mut _
            }
        }
    };
    ($id:ident: $ty:ty; ro_val @ $e:expr) => {
        #[inline(always)]
        pub unsafe fn $id(this: *const Self) -> $ty
        {
            core::ptr::read((this as usize + ($e)) as *const $ty)
        }
    };
    ($id:ident: $ty:ty; val @ $e:expr) => {
        $crate::cpp_field!($id: $ty; ro_val @ $e);
        $crate::reexport::paste::item! {
            #[inline(always)]
            pub unsafe fn [<set_ $id>](this: *const Self, val: $ty)
            {
                core::ptr::write((this as usize + ($e)) as *mut $ty, val)
            }
        }
    };
}

extern "C" {
    fn fwrite(bytes: *const u8, len: usize, count: usize) -> usize;

    // Sometime around 2022-11-12, the use of printf in this file started requiring external symbol "puts" which
    // can't be found because puts isn't in the shipping dol or something. Anyways, if you use printf anywhere in
    // this lib, it will break litterally everything you ever cared about
    // pub fn printf(fmt: *const u8, ...);

    pub fn sprintf(s: *mut u8, fmt: *const u8, ...);
    // #[link_name = "__nw__FUlPCcPCc"]
    #[cw_link_name(operator new(unsigned long, const char *, const char *))]
    fn operator_new(len: usize, loc: *const u8, type_: *const u8) -> *mut c_void;

    // #[link_name = "Free__7CMemoryFPCv"]
    #[cw_link_name(CMemory::Free(const void *))]
    fn free(ptr: *const c_void);
}

#[allow(clippy::manual_c_str_literals)]
pub unsafe fn malloc(len: usize) -> *mut c_void {
    operator_new(len, b"??\0".as_ptr(), b"??\0".as_ptr())
}

struct Mp1Allocator;

unsafe impl GlobalAlloc for Mp1Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *const c_void)
    }
}

#[global_allocator]
static A: Mp1Allocator = Mp1Allocator;

pub struct Mp1Stdout;

impl fmt::Write for Mp1Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            // TODO: Check result?
            fwrite(s.as_bytes().as_ptr(), s.len(), 1);
        }
        Ok(())
    }
}

fn halt() -> ! {
    // extern "C" {
    //     fn PPCHalt() -> !;
    // }
    // unsafe {
    //     PPCHalt()
    // }
    #![allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if cfg!(debug_assertions) {
        writeln!(Mp1Stdout, "{}", info).ok();
    }

    halt()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PatchKind {
    Call,
    Return,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameVersion {
    Any,
    Ntsc0_00,
    Ntsc0_01,
    Ntsc0_02,
    NtscK,
    NtscJ,
    Pal,
}

impl GameVersion {
    pub fn current() -> Self {
        extern "C" {
            static __build_info: u8;
        }
        static mut CACHED: Option<GameVersion> = None;
        unsafe {
            if let Some(v) = CACHED {
                return v;
            }
        }
        let build_info_slice = unsafe { core::slice::from_raw_parts(&__build_info, 36) };
        // Skip the common prefix "!#$MetroidBuildInfo!#$Build "
        let v = match &build_info_slice[28..] {
            b"v1.088 1" => GameVersion::Ntsc0_00,
            b"v1.093 1" => GameVersion::Ntsc0_01,
            b"v1.097 1" => GameVersion::NtscK,
            b"v1.110 2" => GameVersion::Pal,
            b"v1.111 3" => GameVersion::Ntsc0_02,
            b"v1.111 2" => GameVersion::NtscJ,
            _ => unreachable!(),
        };
        unsafe {
            CACHED = Some(v);
        }
        v
    }

    #[inline(always)]
    pub fn matches(self, other: Self) -> bool {
        self == other || self == GameVersion::Any || other == GameVersion::Any
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Patch {
    fn_ptr_to_patch: *const u8,
    patch_offset: usize,
    target_fn_ptr: *const u8,
    kind: PatchKind,
    version: GameVersion,
}

impl Patch {
    pub const fn call_patch(
        fn_ptr_to_patch: *const u8,
        patch_offset: usize,
        target_fn_ptr: *const u8,
        version: GameVersion,
    ) -> Patch {
        Patch {
            fn_ptr_to_patch,
            patch_offset,
            target_fn_ptr,
            kind: PatchKind::Call,
            version,
        }
    }

    pub const fn return_patch(
        fn_ptr_to_patch: *const u8,
        patch_offset: usize,
        target_fn_ptr: *const u8,
        version: GameVersion,
    ) -> Patch {
        Patch {
            fn_ptr_to_patch,
            patch_offset,
            target_fn_ptr,
            kind: PatchKind::Return,
            version,
        }
    }
}

unsafe impl Sync for Patch {}

#[distributed_slice]
pub static PATCHES: [Patch] = [..];

#[distributed_slice]
pub static PROLOG_FUNCS: [unsafe extern "C" fn()] = [..];

#[cfg(feature = "rel_prolog")]
#[no_mangle]
unsafe extern "C" fn __rel_prolog() {
    // printf(b"prolog called\n\0".as_ptr());
    let version = GameVersion::current();
    for patch in PATCHES.iter() {
        if !version.matches(patch.version) {
            continue;
        }
        let instr_ptr = patch.fn_ptr_to_patch.add(patch.patch_offset) as *mut u32;
        let instr = core::ptr::read(instr_ptr);

        let bounds_check_and_mask = |len: u8, addr: i64| {
            // XXX Only len + 1 because this is a sign-extended value
            debug_assert!(
                !(addr > (1 << (len + 1)) - 1 || addr < -1 << (len + 1) || addr as u64 & 0x3 != 0)
            );

            (addr as u64 & ((1 << (len + 2)) - 1)) as u32
        };

        let instr = match patch.kind {
            PatchKind::Call => {
                let rel_addr = patch.target_fn_ptr as i64 - instr_ptr as i64;
                let imm = bounds_check_and_mask(24, rel_addr);
                (instr & 0xfc000003) | imm
            }
            PatchKind::Return => {
                // Assert the instr is actually a return
                debug_assert_eq!(instr, 0x4e800020);

                let rel_addr = patch.target_fn_ptr as i64 - instr_ptr as i64;
                let imm = bounds_check_and_mask(24, rel_addr);
                0x48000000 | imm // Uncondtional jump
            }
        };

        core::ptr::write(instr_ptr, instr);
    }

    for prolog_func in PROLOG_FUNCS.iter() {
        // printf(b"calling prolog func ptr\n\0".as_ptr());
        prolog_func();
    }
}

// TODO: Maybe re-enable this later? The core::fmt machinery seems to need it sometimes
// #[no_mangle]
// unsafe extern "C" fn bcmp(mut b1: *const u8, mut b2: *const u8, mut len: u32) -> u32
// {
//     if len == 0 {
//         return 0
//     }

//     while len > 0 {
//         if ptr::read(b1) != ptr::read(b2) {
//             break
//         }

//         b1 = b1.offset(1);
//         b2 = b2.offset(1);
//         len -= 1;
//     }

//     len
// }
