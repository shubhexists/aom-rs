use super::config::AV1EncoderConfig;
use crate::{
    aom::{
        aom_codec_av1_cx, aom_codec_control, aom_codec_ctx, aom_codec_ctx_t, aom_codec_cx_pkt,
        aom_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1, aom_codec_cx_pkt__bindgen_ty_1_aom_psnr_pkt,
        aom_codec_cx_pkt_kind_AOM_CODEC_CUSTOM_PKT, aom_codec_cx_pkt_kind_AOM_CODEC_CX_FRAME_PKT,
        aom_codec_cx_pkt_kind_AOM_CODEC_FPMB_STATS_PKT, aom_codec_cx_pkt_kind_AOM_CODEC_PSNR_PKT,
        aom_codec_cx_pkt_kind_AOM_CODEC_STATS_PKT, aom_codec_destroy, aom_codec_enc_init_ver,
        aom_codec_encode, aom_codec_err_t, aom_codec_get_cx_data, aom_codec_iter_t, aom_image,
        aome_enc_control_id, aome_enc_control_id_AOME_SET_CPUUSED, AOM_ENCODER_ABI_VERSION,
        AOM_FRAME_IS_KEY,
    },
    utils::{img_from_frame, to_buffer},
};
use av_data::{frame::Frame, packet::Packet};
use std::{mem::MaybeUninit, ptr};

/// aom_codec_cx_pkt__bindgen_ty_1_aom_psnr_pkt struct of C
/// hbd -> high_bit_depth_data
pub struct PSNR {
    pub samples: [u32; 4],
    pub psnr: [f64; 4],
    pub sse: [u64; 4],
    pub samples_hbd: [u32; 4],
    pub sse_hbd: [u64; 4],
    pub psnr_hbd: [f64; 4],
}

/// aom_codec_cx_pkt__bindgen_ty_1
pub enum AOMPacket {
    Frame(Packet),
    TwoPassStats(Vec<u8>),
    FirstPassMBStats(Vec<u8>),
    PSNR(PSNR),
    Raw(Vec<u8>),
}

impl AOMPacket {
    fn new(pkt: aom_codec_cx_pkt) -> Self {
        match pkt.kind {
            // Frame
            aom_codec_cx_pkt_kind_AOM_CODEC_CX_FRAME_PKT => {
                let f: aom_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 = unsafe { pkt.data.frame };
                let mut p: Packet = Packet::with_capacity(f.sz);
                unsafe {
                    ptr::copy_nonoverlapping(f.buf as *const u8, p.data.as_mut_ptr(), f.sz);
                    p.data.set_len(f.sz);
                }
                p.t.pts = Some(f.pts);
                p.is_key = (f.flags & AOM_FRAME_IS_KEY) != 0;

                AOMPacket::Frame(p)
            }
            //TWO PASS Stats
            aom_codec_cx_pkt_kind_AOM_CODEC_STATS_PKT => {
                let b: Vec<u8> = to_buffer(unsafe { pkt.data.twopass_stats });
                AOMPacket::TwoPassStats(b)
            }
            // Raw
            aom_codec_cx_pkt_kind_AOM_CODEC_CUSTOM_PKT => {
                let b: Vec<u8> = to_buffer(unsafe { pkt.data.raw });
                AOMPacket::Raw(b)
            }
            // FIRSTPASSMB Stats
            aom_codec_cx_pkt_kind_AOM_CODEC_FPMB_STATS_PKT => {
                let b = to_buffer(unsafe { pkt.data.firstpass_mb_stats });
                AOMPacket::FirstPassMBStats(b)
            }
            // PSNR
            aom_codec_cx_pkt_kind_AOM_CODEC_PSNR_PKT => {
                let p: aom_codec_cx_pkt__bindgen_ty_1_aom_psnr_pkt = unsafe { pkt.data.psnr };
                AOMPacket::PSNR(PSNR {
                    samples: p.samples,
                    psnr: p.psnr,
                    sse: p.sse,
                    samples_hbd: p.samples_hbd,
                    sse_hbd: p.sse_hbd,
                    psnr_hbd: p.psnr_hbd,
                })
            }
            _ => panic!("Invalid aom packet kind detected"),
        }
    }
}

pub struct AV1Encoder {
    pub(crate) ctx: aom_codec_ctx_t,
    pub(crate) iter: aom_codec_iter_t,
}

impl AV1Encoder {
    /// This calls the aom_codec_enc_init_ver function under the hood
    pub fn new(cfg: &mut AV1EncoderConfig) -> Result<AV1Encoder, aom_codec_err_t> {
        let mut ctx: MaybeUninit<aom_codec_ctx> = MaybeUninit::uninit();
        // If result is 0, it passed, otherwise failed
        // TODO - Add custom error class
        let result: u32 = unsafe {
            aom_codec_enc_init_ver(
                ctx.as_mut_ptr(),
                aom_codec_av1_cx(),
                &cfg.enc_cfg,
                0,
                AOM_ENCODER_ABI_VERSION as i32,
            )
        };

        match result {
            0 => {
                let ctx: aom_codec_ctx = unsafe { ctx.assume_init() };
                let mut enc: AV1Encoder = AV1Encoder {
                    ctx,
                    iter: ptr::null(),
                };

                // check about this
                // CPU usage level (which balances encoding speed and quality),
                enc.aom_codec_control(aome_enc_control_id_AOME_SET_CPUUSED, 2)
                    .expect("Cannot set CPUUSED");

                Ok(enc)
            }
            _ => Err(result),
        }
    }

    /// Calls aom_codec_control. Changes the codec configuration for an existing Av1Encoder Instance
    pub fn aom_codec_control(
        &mut self,
        id: aome_enc_control_id,
        val: i32,
    ) -> Result<(), aom_codec_err_t> {
        let result: u32 = unsafe { aom_codec_control(&mut self.ctx, id as i32, val) };

        match result {
            aom_codec_err_t_AOM_CODEC_OK => Ok(()),
            _ => Err(result),
        }
    }

    // calls aom_codec_encode internally with Frame objects.
    pub fn aom_codec_encode(&mut self, frame: &Frame) -> Result<(), aom_codec_err_t> {
        let img: aom_image = img_from_frame(frame);

        let ret = unsafe { aom_codec_encode(&mut self.ctx, &img, frame.t.pts.unwrap(), 1, 0) };
        self.iter = ptr::null();

        match ret {
            aom_codec_err_t_AOM_CODEC_OK => Ok(()),
            _ => Err(ret),
        }
    }

    // calls aom_codec_encode internally. It clears out all the frames from the pointer
    pub fn flush(&mut self) -> Result<(), aom_codec_err_t> {
        let ret: u32 = unsafe { aom_codec_encode(&mut self.ctx, ptr::null_mut(), 0, 1, 0) };

        self.iter = ptr::null();

        match ret {
            aom_codec_err_t_AOM_CODEC_OK => Ok(()),
            _ => Err(ret),
        }
    }

    // calls aom_codec_get_cx_data internally. Returns packet information
    pub fn get_packet(&mut self) -> Option<AOMPacket> {
        let pkt: *const crate::aom::aom_codec_cx_pkt =
            unsafe { aom_codec_get_cx_data(&mut self.ctx, &mut self.iter) };

        if pkt.is_null() {
            None
        } else {
            Some(AOMPacket::new(unsafe { *pkt }))
        }
    }
}

// When our AV1 Encoder goes out of scope, we need to call the aom_codec_destroy internally.
impl Drop for AV1Encoder {
    fn drop(&mut self) {
        unsafe { aom_codec_destroy(&mut self.ctx) };
    }
}

unsafe impl Send for AV1Encoder {}
