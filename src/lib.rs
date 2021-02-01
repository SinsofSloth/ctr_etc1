mod ffi;

use std::{io::Result, ptr, slice};

pub fn compress(pixel_data: &[u8], width: usize, height: usize, has_alpha: bool, quality: ETC1Quality) -> Result<Vec<u8>> {
    let mut output_data: *mut u8 = core::ptr::null_mut();
    unsafe {
        ffi::encodeETC1(pixel_data.as_ptr(), &mut output_data, width as u16, height as u16, has_alpha, quality as u8);
    }
    let output_data_ptr: ptr::NonNull<u8> = ptr::NonNull::new(output_data)
        .expect("Error: got NULL ptr");
    let output_data_len = ((width * height * 4)/8) as usize;

    // Take ownership of ptr
    let result: &mut [u8];
    unsafe {
        result = slice::from_raw_parts_mut(output_data_ptr.as_ptr(), output_data_len);
        // Deallocate memory
        ffi::free_ptr(output_data_ptr.as_ptr());
     };
     Ok(result.to_vec())
}
pub enum ETC1Quality
{ 
   LowQuality,
   MediumQuality,
   HighQuality,
}