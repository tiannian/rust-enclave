#[repr(C)]
pub struct SgxAttributes {
    pub flags: u64,
    pub xfrm: u64,
}

#[repr(C)]
pub struct SgxMiscAttribute {
    pub secs_attr: SgxAttributes,
    pub misc_select: u32,
}
