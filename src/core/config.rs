use super::AomCodecEncCfgTrait;
use crate::aom::{
    aom_bit_depth, aom_codec_av1_cx, aom_codec_ctx, aom_codec_enc_cfg,
    aom_codec_enc_config_default, aom_codec_er_flags_t, aom_codec_err_t,
    aom_codec_err_t_AOM_CODEC_OK, aom_enc_pass, aom_fixed_buf_t, aom_kf_mode, aom_rational,
    aom_rc_mode, aom_superres_mode, cfg_options_t,
};
use core::mem::MaybeUninit;
use std::{
    error::Error,
    ops::{Deref, DerefMut},
};

/// A struct representing an AV1 Encoder.
///
/// This struct encapsulates the encoder configuration options needed to encode video
/// using the AV1 codec. It holds the configuration settings defined in the
/// `aom_codec_enc_cfg` structure, allowing users to set various encoding parameters.
///
/// # Fields
/// - `enc_cfg`: An instance of `aom_codec_enc_cfg` that contains all the necessary
///   configuration options for the encoder.
///
/// # Usage
/// To create an instance of `AV1Encoder`, you can initialize the `enc_cfg` field
/// with the desired encoding parameters. After initializing, you can modify the
/// settings using method chaining.
///
/// # Example
/// ```rust
/// let mut encoder = AV1Encoder {
///     enc_cfg: aom_codec_enc_cfg {
///         // Initialize with default values or your custom settings.
///         ..Default::default()
///     },
/// };
///
/// encoder.rc_target_bitrate(3000) // Set target bitrate to 3000 kbps
///     .rc_end_usage(aom_rc_mode::AOM_VBR); // Set rate control mode to VBR
/// ```
pub struct AV1EncoderConfig {
    pub enc_cfg: aom_codec_enc_cfg,
}

impl AV1EncoderConfig {
    pub fn init(config: u32) -> Result<Self, Box<dyn Error>> {
        // Initialize the variable only when it has a value later on.
        let mut cfg: MaybeUninit<aom_codec_enc_cfg> = MaybeUninit::uninit();
        // This is either 0 or 1 depending on whether the function passes or fails
        let is_success: u32 =
            unsafe { aom_codec_enc_config_default(aom_codec_av1_cx(), cfg.as_mut_ptr(), config) };
        match is_success {
            // SUCCESS -> 0
            aom_codec_err_t_AOM_CODEC_OK => {
                let cfg: aom_codec_enc_cfg = unsafe { cfg.assume_init() };

                Ok(AV1EncoderConfig { enc_cfg: cfg })
            }
            // Convert aom_codec_err_t to Box<dyn Error>> and return
            _ => Err(format!("Failed to initialize encoder: error code {is_success}").into()),
        }
    }
}

impl AomCodecEncCfgTrait for AV1EncoderConfig {
    /// Sets the usage type for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` representing the usage type. This affects how the encoder optimizes for different scenarios.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_usage(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_usage = value;
        self
    }

    /// Sets the number of threads to be used by the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating the number of threads.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_threads(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_threads = value;
        self
    }

    /// Sets the profile for the encoder configuration.
    ///
    /// # Parameters
    /// - `value`: A `u32` representing the profile type.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_profile(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_profile = value;
        self
    }

    /// Sets the width of the frame to be encoded.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the width.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_w(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_w = value;
        self
    }

    /// Sets the height of the frame to be encoded.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the height.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_h(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_h = value;
        self
    }

    /// Sets the limit on the number of frames to be processed.
    ///
    /// # Parameters
    /// - `value`: A `u32` that sets the frame limit.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_limit(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_limit = value;
        self
    }

    /// Sets the maximum frame width to be enforced during encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating the maximum frame width.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_forced_max_frame_width(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_forced_max_frame_width = value;
        self
    }

    /// Sets the maximum frame height to be enforced during encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating the maximum frame height.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_forced_max_frame_height(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_forced_max_frame_height = value;
        self
    }

    /// Sets the bit depth of the encoder configuration.
    ///
    /// # Parameters
    /// - `value`: An `aom_bit_depth` specifying the bit depth (e.g., 8, 10, or 12 bits).
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_bit_depth(&mut self, value: aom_bit_depth) -> &mut Self {
        self.enc_cfg.g_bit_depth = value;
        self
    }

    /// Sets the input bit depth for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` representing the input bit depth.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_input_bit_depth(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_input_bit_depth = value;
        self
    }

    /// Sets the time base for the encoder configuration.
    ///
    /// # Parameters
    /// - `value`: An `aom_rational` that represents the time base.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_timebase(&mut self, value: aom_rational) -> &mut Self {
        self.enc_cfg.g_timebase = value;
        self
    }

    /// Sets the error resilience flags for the encoder.
    ///
    /// # Parameters
    /// - `value`: An `aom_codec_er_flags_t` that specifies the error resilience options.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_error_resilient(&mut self, value: aom_codec_er_flags_t) -> &mut Self {
        self.enc_cfg.g_error_resilient = value;
        self
    }

    /// Sets the encoding pass type.
    ///
    /// # Parameters
    /// - `value`: An `aom_enc_pass` indicating the encoding pass type (e.g., single pass, two-pass).
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_pass(&mut self, value: aom_enc_pass) -> &mut Self {
        self.enc_cfg.g_pass = value;
        self
    }

    /// Sets the number of frames to be processed for lag.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating the number of lag frames.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn g_lag_in_frames(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.g_lag_in_frames = value;
        self
    }

    /// Sets the threshold for dropping frames during encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` that sets the drop frame threshold.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_dropframe_thresh(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_dropframe_thresh = value;
        self
    }

    /// Sets the resize mode for the rate control.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the resize mode.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_resize_mode(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_resize_mode = value;
        self
    }

    /// Sets the resize denominator for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the resize denominator.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_resize_denominator(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_resize_denominator = value;
        self
    }

    /// Sets the keyframe resize denominator for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the keyframe resize denominator.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_resize_kf_denominator(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_resize_kf_denominator = value;
        self
    }

    /// Sets the super-resolution mode for the rate control.
    ///
    /// # Parameters
    /// - `value`: An `aom_superres_mode` specifying the super-resolution mode.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_superres_mode(&mut self, value: aom_superres_mode) -> &mut Self {
        self.enc_cfg.rc_superres_mode = value;
        self
    }

    /// Sets the super-resolution denominator for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the super-resolution denominator.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_superres_denominator(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_superres_denominator = value;
        self
    }

    /// Sets the keyframe super-resolution denominator for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the keyframe super-resolution denominator.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_superres_kf_denominator(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_superres_kf_denominator = value;
        self
    }

    /// Sets the quantizer threshold for super-resolution.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the quantizer threshold for super-resolution.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_superres_qthresh(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_superres_qthresh = value;
        self
    }

    /// Sets the quantizer threshold for keyframe super-resolution.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the keyframe quantizer threshold for super-resolution.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_superres_kf_qthresh(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_superres_kf_qthresh = value;
        self
    }

    /// Sets the end usage mode for rate control.
    ///
    /// # Parameters
    /// - `value`: An `aom_rc_mode` specifying the end usage mode.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_end_usage(&mut self, value: aom_rc_mode) -> &mut Self {
        self.enc_cfg.rc_end_usage = value;
        self
    }

    /// Sets the input statistics for the two-pass encoding.
    ///
    /// # Parameters
    /// - `value`: An `aom_fixed_buf_t` that contains the statistics buffer.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_twopass_stats_in(&mut self, value: aom_fixed_buf_t) -> &mut Self {
        self.enc_cfg.rc_twopass_stats_in = value;
        self
    }

    /// Sets the first-pass macroblock statistics for the encoder.
    ///
    /// # Parameters
    /// - `value`: An `aom_fixed_buf_t` that contains the first-pass macroblock statistics buffer.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_firstpass_mb_stats_in(&mut self, value: aom_fixed_buf_t) -> &mut Self {
        self.enc_cfg.rc_firstpass_mb_stats_in = value;
        self
    }

    /// Sets the target bitrate for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the target bitrate in kbps.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_target_bitrate(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_target_bitrate = value;
        self
    }

    /// Sets the minimum quantizer value for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the minimum quantizer.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_min_quantizer(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_min_quantizer = value;
        self
    }

    /// Sets the maximum quantizer value for the encoder.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the maximum quantizer.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_max_quantizer(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_max_quantizer = value;
        self
    }

    /// Sets the percentage of undershoot for rate control.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating the undershoot percentage.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_undershoot_pct(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_undershoot_pct = value;
        self
    }

    /// Sets the percentage of overshoot for rate control.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating the overshoot percentage.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_overshoot_pct(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_overshoot_pct = value;
        self
    }

    /// Sets the buffer size for rate control.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the buffer size.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_buf_sz(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_buf_sz = value;
        self
    }

    /// Sets the initial buffer size for rate control.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the initial buffer size.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_buf_initial_sz(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_buf_initial_sz = value;
        self
    }

    /// Sets the optimal buffer size for rate control.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the optimal buffer size.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_buf_optimal_sz(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_buf_optimal_sz = value;
        self
    }

    /// Sets the VBR bias percentage for two-pass encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the VBR bias percentage.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_2pass_vbr_bias_pct(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_2pass_vbr_bias_pct = value;
        self
    }

    /// Sets the minimum section percentage for VBR in two-pass encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the minimum section percentage.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_2pass_vbr_minsection_pct(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_2pass_vbr_minsection_pct = value;
        self
    }

    /// Sets the maximum section percentage for VBR in two-pass encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the maximum section percentage.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn rc_2pass_vbr_maxsection_pct(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.rc_2pass_vbr_maxsection_pct = value;
        self
    }

    /// Enables or disables forward keyframe generation.
    ///
    /// # Parameters
    /// - `value`: An `i32` indicating whether to enable (1) or disable (0) forward keyframes.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn fwd_kf_enabled(&mut self, value: i32) -> &mut Self {
        self.enc_cfg.fwd_kf_enabled = value;
        self
    }

    /// Sets the keyframe mode for the encoder.
    ///
    /// # Parameters
    /// - `value`: An `aom_kf_mode` specifying the keyframe mode.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn kf_mode(&mut self, value: aom_kf_mode) -> &mut Self {
        self.enc_cfg.kf_mode = value;
        self
    }

    /// Sets the minimum distance between keyframes.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the minimum distance between keyframes.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn kf_min_dist(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.kf_min_dist = value;
        self
    }

    /// Sets the maximum distance between keyframes.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the maximum distance between keyframes.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn kf_max_dist(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.kf_max_dist = value;
        self
    }

    /// Sets the distance for the spatially-scaled frame.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the distance for the spatially-scaled frame.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn sframe_dist(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.sframe_dist = value;
        self
    }

    /// Sets the mode for spatially-scaled frames.
    ///
    /// # Parameters
    /// - `value`: A `u32` specifying the mode for spatially-scaled frames.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn sframe_mode(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.sframe_mode = value;
        self
    }

    /// Sets the large scale tile flag for encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating whether to enable large scale tiles (1) or disable (0).
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn large_scale_tile(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.large_scale_tile = value;
        self
    }

    /// Sets the monochrome flag for encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating whether to enable monochrome encoding (1) or disable (0).
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn monochrome(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.monochrome = value;
        self
    }

    /// Sets the full still picture HDR flag for encoding.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating whether to enable full still picture HDR encoding (1) or disable (0).
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn full_still_picture_hdr(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.full_still_picture_hdr = value;
        self
    }

    /// Sets the flag for saving as Annex B format.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating whether to save in Annex B format (1) or not (0).
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn save_as_annexb(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.save_as_annexb = value;
        self
    }

    /// Sets the width count for tiles.
    ///
    /// # Parameters
    /// - `value`: An `i32` specifying the count of tile widths.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn tile_width_count(&mut self, value: i32) -> &mut Self {
        self.enc_cfg.tile_width_count = value;
        self
    }

    /// Sets the height count for tiles.
    ///
    /// # Parameters
    /// - `value`: An `i32` specifying the count of tile heights.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn tile_height_count(&mut self, value: i32) -> &mut Self {
        self.enc_cfg.tile_height_count = value;
        self
    }

    /// Sets the tile widths for encoding.
    ///
    /// # Parameters
    /// - `value`: An array of `i32` specifying the widths of tiles.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn tile_widths(&mut self, value: [i32; 64]) -> &mut Self {
        self.enc_cfg.tile_widths = value;
        self
    }

    /// Sets the tile heights for encoding.
    ///
    /// # Parameters
    /// - `value`: An array of `i32` specifying the heights of tiles.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn tile_heights(&mut self, value: [i32; 64]) -> &mut Self {
        self.enc_cfg.tile_heights = value;
        self
    }

    /// Enables or disables fixed quantization offsets.
    ///
    /// # Parameters
    /// - `value`: A `u32` indicating whether to enable (1) or disable (0) fixed quantization offsets.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn use_fixed_qp_offsets(&mut self, value: u32) -> &mut Self {
        self.enc_cfg.use_fixed_qp_offsets = value;
        self
    }

    /// Sets the fixed quantization offsets.
    ///
    /// # Parameters
    /// - `value`: An array of `i32` specifying the fixed quantization offsets.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn fixed_qp_offsets(&mut self, value: [i32; 5]) -> &mut Self {
        self.enc_cfg.fixed_qp_offsets = value;
        self
    }

    /// Sets the encoder configuration options.
    ///
    /// # Parameters
    /// - `value`: A `cfg_options_t` specifying the configuration options for the encoder.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing method chaining.
    fn encoder_cfg(&mut self, value: cfg_options_t) -> &mut Self {
        self.enc_cfg.encoder_cfg = value;
        self
    }
}

impl Deref for AV1EncoderConfig {
    type Target = aom_codec_enc_cfg;

    fn deref(&self) -> &Self::Target {
        &self.enc_cfg
    }
}

impl DerefMut for AV1EncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.enc_cfg
    }
}
