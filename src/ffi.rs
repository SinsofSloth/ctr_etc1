#[link(name = "etc1_encoder", kind="static")]
extern "C" {
    pub(crate) fn encodeETC1(input_data: *const u8, output_data: &mut *mut u8, width: u16, height: u16, has_alpha: bool, quality: u8);
    pub(crate) fn free_ptr(res: *mut u8);
}