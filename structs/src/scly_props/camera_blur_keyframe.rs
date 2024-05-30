use auto_struct_macros::auto_struct;
use reader_writer::{generic_array::GenericArray, typenum::*, CStr};

use crate::SclyPropertyData;

#[auto_struct(Readable, Writable)]
#[derive(Debug, Clone)]
pub struct CameraBlurKeyframe<'r> {
    #[auto_struct(expect = 7)]
    pub prop_count: u32,

    pub name: CStr<'r>,
    pub active: u8,
    pub blur_type: u32,
    pub amount: f32,
    pub filter_index: u32,
    pub fade_in_time: f32,
    pub fade_out_time: f32
}

impl<'r> SclyPropertyData for CameraBlurKeyframe<'r> {
    const OBJECT_TYPE: u8 = 0x19;
}
