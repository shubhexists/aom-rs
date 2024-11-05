use crate::aom::{
    aom_bit_depth_t, aom_codec_er_flags_t, aom_enc_pass, aom_fixed_buf_t, aom_kf_mode,
    aom_rational, aom_rc_mode, aom_superres_mode, cfg_options_t,
};

pub mod config;
pub mod encoder;
mod errors;

/// A trait for configuring the AV1 Encoder with builder-style methods.
///
/// This trait defines a set of builder functions for setting values in all fields
/// related to the AV1 encoder's configuration. The functions are categorized as follows:
///
/// - **General Settings**: Functions prefixed with `g_` allow you to set general encoder parameters.
/// - **Rate Control Settings**: Functions prefixed with `rc_` facilitate the configuration of rate control parameters.
/// - **KeyFrame Settings**: Functions prefixed with `kf_` handle settings specific to keyframes.
///
/// Each function takes a corresponding parameter type to set the desired value for the configuration field,
/// allowing for a fluent interface for encoder configuration. The trait is designed to be implemented by
/// structs representing an AV1 encoder, enabling customization and flexible encoding configurations.
///
/// Example usage:
/// ```rust
/// struct MyEncoder;
///
/// impl AomCodecEncCfgTrait for MyEncoder {
///     // Implement the methods here...
/// }
///
/// let encoder = MyEncoder {};
/// encoder.g_usage(1)
///        .rc_target_bitrate(1000)
///        .kf_mode(0);
/// ```
pub trait AomCodecEncCfgTrait {
    fn g_usage(&mut self, value: u32) -> &mut Self;
    fn g_threads(&mut self, value: u32) -> &mut Self;
    fn g_profile(&mut self, value: u32) -> &mut Self;
    fn g_w(&mut self, value: u32) -> &mut Self;
    fn g_h(&mut self, value: u32) -> &mut Self;
    fn g_limit(&mut self, value: u32) -> &mut Self;
    fn g_forced_max_frame_width(&mut self, value: u32) -> &mut Self;
    fn g_forced_max_frame_height(&mut self, value: u32) -> &mut Self;
    fn g_bit_depth(&mut self, value: aom_bit_depth_t) -> &mut Self;
    fn g_input_bit_depth(&mut self, value: u32) -> &mut Self;
    fn g_timebase(&mut self, value: aom_rational) -> &mut Self;
    fn g_error_resilient(&mut self, value: aom_codec_er_flags_t) -> &mut Self;
    fn g_pass(&mut self, value: aom_enc_pass) -> &mut Self;
    fn g_lag_in_frames(&mut self, value: u32) -> &mut Self;
    fn rc_dropframe_thresh(&mut self, value: u32) -> &mut Self;
    fn rc_resize_mode(&mut self, value: u32) -> &mut Self;
    fn rc_resize_denominator(&mut self, value: u32) -> &mut Self;
    fn rc_resize_kf_denominator(&mut self, value: u32) -> &mut Self;
    fn rc_superres_mode(&mut self, value: aom_superres_mode) -> &mut Self;
    fn rc_superres_denominator(&mut self, value: u32) -> &mut Self;
    fn rc_superres_kf_denominator(&mut self, value: u32) -> &mut Self;
    fn rc_superres_qthresh(&mut self, value: u32) -> &mut Self;
    fn rc_superres_kf_qthresh(&mut self, value: u32) -> &mut Self;
    fn rc_end_usage(&mut self, value: aom_rc_mode) -> &mut Self;
    fn rc_twopass_stats_in(&mut self, value: aom_fixed_buf_t) -> &mut Self;
    fn rc_firstpass_mb_stats_in(&mut self, value: aom_fixed_buf_t) -> &mut Self;
    fn rc_target_bitrate(&mut self, value: u32) -> &mut Self;
    fn rc_min_quantizer(&mut self, value: u32) -> &mut Self;
    fn rc_max_quantizer(&mut self, value: u32) -> &mut Self;
    fn rc_undershoot_pct(&mut self, value: u32) -> &mut Self;
    fn rc_overshoot_pct(&mut self, value: u32) -> &mut Self;
    fn rc_buf_sz(&mut self, value: u32) -> &mut Self;
    fn rc_buf_initial_sz(&mut self, value: u32) -> &mut Self;
    fn rc_buf_optimal_sz(&mut self, value: u32) -> &mut Self;
    fn rc_2pass_vbr_bias_pct(&mut self, value: u32) -> &mut Self;
    fn rc_2pass_vbr_minsection_pct(&mut self, value: u32) -> &mut Self;
    fn rc_2pass_vbr_maxsection_pct(&mut self, value: u32) -> &mut Self;
    fn fwd_kf_enabled(&mut self, value: i32) -> &mut Self;
    fn kf_mode(&mut self, value: aom_kf_mode) -> &mut Self;
    fn kf_min_dist(&mut self, value: u32) -> &mut Self;
    fn kf_max_dist(&mut self, value: u32) -> &mut Self;
    fn sframe_dist(&mut self, value: u32) -> &mut Self;
    fn sframe_mode(&mut self, value: u32) -> &mut Self;
    fn large_scale_tile(&mut self, value: u32) -> &mut Self;
    fn monochrome(&mut self, value: u32) -> &mut Self;
    fn full_still_picture_hdr(&mut self, value: u32) -> &mut Self;
    fn save_as_annexb(&mut self, value: u32) -> &mut Self;
    fn tile_width_count(&mut self, value: i32) -> &mut Self;
    fn tile_height_count(&mut self, value: i32) -> &mut Self;
    fn tile_widths(&mut self, value: [i32; 64]) -> &mut Self;
    fn tile_heights(&mut self, value: [i32; 64]) -> &mut Self;
    fn use_fixed_qp_offsets(&mut self, value: u32) -> &mut Self;
    fn fixed_qp_offsets(&mut self, value: [i32; 5]) -> &mut Self;
    fn encoder_cfg(&mut self, value: cfg_options_t) -> &mut Self;
}
