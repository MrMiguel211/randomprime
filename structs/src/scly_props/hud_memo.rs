use auto_struct_macros::auto_struct;
use reader_writer::CStr;

use crate::{res_id::*, ResId, SclyPropertyData};

#[auto_struct(Readable, Writable)]
#[derive(Debug, Clone)]
pub struct HudMemo<'r> {
    #[auto_struct(expect = 6)]
    prop_count: u32,

    pub name: CStr<'r>,

    pub first_message_timer: f32,
    pub unknown: u8,
    pub memo_type: u32,
    pub strg: ResId<STRG>,
    pub active: u8,
}

impl SclyPropertyData for HudMemo<'_> {
    const OBJECT_TYPE: u8 = 0x17;
}
