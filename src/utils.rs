use crate::aom::{aom_fixed_buf_t, aom_image, aom_img_fmt, aom_img_fmt_AOM_IMG_FMT_I420};
use av_data::frame::FrameBufferConv;
use av_data::frame::{Frame, MediaKind};
use av_data::pixel::formats::YUV420;
use av_data::pixel::Formaton;
use std::{mem, ptr};

/// Utility function to convert Frame to aom_image
pub fn img_from_frame(frame: &Frame) -> aom_image {
    // create an uninitialized aom_image
    let mut img: aom_image = unsafe { mem::zeroed() };

    // if Frame is video
    if let MediaKind::Video(ref v) = frame.kind {
        // puts video to aom_image in a mutable img variable
        map_formaton(&mut img, &v.format);
        img.w = v.width as u32;
        img.h = v.height as u32;
        img.d_w = v.width as u32;
        img.d_h = v.height as u32;
    }

    for i in 0..frame.buf.count() {
        let s: &[u8] = frame.buf.as_slice(i).unwrap();
        img.planes[i] = s.as_ptr() as *mut u8;
        img.stride[i] = frame.buf.linesize(i).unwrap() as i32;
    }

    img
}

pub fn to_buffer(buf: aom_fixed_buf_t) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(buf.sz);
    unsafe {
        ptr::copy_nonoverlapping(buf.buf as *const u8, v.as_mut_ptr(), buf.sz);
        v.set_len(buf.sz);
    }
    v
}

// INCOMPLETE
fn map_formaton(img: &mut aom_image, fmt: &Formaton) {
    if fmt == YUV420 {
        img.fmt = aom_img_fmt_AOM_IMG_FMT_I420;
    } else {
        unimplemented!();
    }
    img.bit_depth = 8;
    img.bps = 12;
    img.x_chroma_shift = 1;
    img.y_chroma_shift = 1;
    map_fmt_to_img(img, fmt);
}

#[cfg(target_os = "windows")]
fn map_fmt_to_img(img: &mut aom_image, fmt: &Formaton) {
    img.cp = fmt.get_primaries() as i32;
    img.tc = fmt.get_xfer() as i32;
    img.mc = fmt.get_matrix() as i32;
}

#[cfg(not(target_os = "windows"))]
fn map_fmt_to_img(img: &mut aom_image, fmt: &Formaton) {
    img.cp = fmt.get_primaries() as u32;
    img.tc = fmt.get_xfer() as u32;
    img.mc = fmt.get_matrix() as u32;
}
