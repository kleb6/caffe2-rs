crate::ix!();

/**
  | Optimized using AVX2 intrinsics for
  | max pool 2D in NHWC layout
  |
  */
#[inline] pub fn max_pool_avx2(
    xdata:         *const u8,
    n:             i32,
    height:        i32,
    width:         i32,
    channels:      i32,
    pooled_height: i32,
    pooled_width:  i32,
    kernel_h:      i32,
    kernel_w:      i32,
    stride_h:      i32,
    stride_w:      i32,
    pad_t:         i32,
    pad_l:         i32,
    ydata:         *mut u8)  {
    
    todo!();
    /*
        const uint8_t* Xdata_temp = Xdata + n * height * width * channels;
      uint8_t* Ydata_temp = Ydata + n * pooled_height * pooled_width * channels;
      for (int ph = 0; ph < pooled_height; ++ph) {
        int hstart = ph * stride_h - pad_t;
        int hend = hstart + kernel_h < height ? hstart + kernel_h : height;
        hstart = hstart > 0 ? hstart : 0;
        for (int pw = 0; pw < pooled_width; ++pw) {
          int wstart = pw * stride_w - pad_l;
          int wend = wstart + kernel_w < width ? wstart + kernel_w : width;
          wstart = wstart > 0 ? wstart : 0;

          uint8_t* Yh = Ydata_temp + (ph * pooled_width + pw) * channels;
          constexpr int VLEN = 32;
          // vectorized loop
          for (int c = 0; c < channels / VLEN * VLEN; c += VLEN) {
            __m256i Y_v = _mm256_setzero_si256();
            for (int h = hstart; h < hend; ++h) {
              for (int w = wstart; w < wend; ++w) {
                const int input_idx = (h * width + w) * channels + c;
                Y_v = _mm256_max_epu8(
                    _mm256_loadu_si256(
                        reinterpret_cast<const __m256i*>(Xdata_temp + input_idx)),
                    Y_v);
              }
            }
            _mm256_storeu_si256(reinterpret_cast<__m256i*>(Yh + c), Y_v);
          }

          // remainder
          for (int c = channels / VLEN * VLEN; c < channels; ++c) {
            Yh[c] = 0;
          }
          for (int h = hstart; h < hend; ++h) {
            for (int w = wstart; w < wend; ++w) {
              for (int c = channels / VLEN * VLEN; c < channels; ++c) {
                const int input_idx = (h * width + w) * channels + c;
                Yh[c] =
                    Xdata_temp[input_idx] > Yh[c] ? Xdata_temp[input_idx] : Yh[c];
              }
            }
          }
        } // pw loop
      } // ph loop
    */
}

#[inline] pub fn average_pool_avx2(
    xdata:          *const u8,
    n:              i32,
    height:         i32,
    width:          i32,
    channels:       i32,
    pooled_height:  i32,
    pooled_width:   i32,
    kernel_h:       i32,
    kernel_w:       i32,
    stride_h:       i32,
    stride_w:       i32,
    pad_t:          i32,
    pad_l:          i32,
    ydata:          *mut u8,
    in_scale:       f32,
    out_scale:      f32,
    in_zero_point:  i32,
    out_zero_point: i32,
    minimum:        i32,
    maximum:        i32)  {
    
    todo!();
    /*
        const uint8_t* Xdata_temp = Xdata + n * height * width * channels;
      uint8_t* Ydata_temp = Ydata + n * pooled_height * pooled_width * channels;

      const __m256i shuffle_mask_v = _mm256_set_epi8(
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0x0c,
          0x08,
          0x04,
          0x00,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0x0c,
          0x08,
          0x04,
          0x00);
      const __m256i permute_mask_v =
          _mm256_set_epi32(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00);

      const __m256i min_v = _mm256_set1_epi32(minimum);
      const __m256i max_v = _mm256_set1_epi32(maximum);
      __m256 out_zero_point_v = _mm256_set1_ps(out_zero_point);

      for (int ph = 0; ph < pooled_height; ++ph) {
        int hstart = ph * stride_h - pad_t;
        int hend = hstart + kernel_h < height ? hstart + kernel_h : height;
        hstart = hstart > 0 ? hstart : 0;
        for (int pw = 0; pw < pooled_width; ++pw) {
          int wstart = pw * stride_w - pad_l;
          int wend = wstart + kernel_w < width ? wstart + kernel_w : width;
          wstart = wstart > 0 ? wstart : 0;

          int size = (hend - hstart) * (wend - wstart);
          float multiplier = in_scale / out_scale / size;
          __m256 multiplier_v = _mm256_set1_ps(multiplier);

          uint8_t* Yh = Ydata_temp + (ph * pooled_width + pw) * channels;
          constexpr int VLEN = 8;
          int32_t Yh0 = -in_zero_point * size;

          // vectorized loop
          for (int c = 0; c < channels / VLEN * VLEN; c += VLEN) {
            __m256i Yh0_v = _mm256_set1_epi32(Yh0);

            for (int h = hstart; h < hend; ++h) {
              for (int w = wstart; w < wend; ++w) {
                const int input_idx = (h * width + w) * channels + c;
                const __m256i temp_v = _mm256_cvtepu8_epi32(_mm_loadl_epi64(
                    reinterpret_cast<const __m128i*>(Xdata_temp + input_idx)));
                Yh0_v = _mm256_add_epi32(Yh0_v, temp_v);
              }
            }

            __m256 Yh0_fp = _mm256_cvtepi32_ps(Yh0_v);
            __m256 Y_float_v =
                _mm256_fmadd_ps(Yh0_fp, multiplier_v, out_zero_point_v);
            __m256i Y_rounded_v = _mm256_cvtps_epi32(Y_float_v);
            __m256i Y_clipped_v =
                _mm256_max_epi32(min_v, _mm256_min_epi32(max_v, Y_rounded_v));

            Y_clipped_v = _mm256_shuffle_epi8(Y_clipped_v, shuffle_mask_v);
            Y_clipped_v = _mm256_permutevar8x32_epi32(Y_clipped_v, permute_mask_v);
            *reinterpret_cast<int64_t*>(Yh + c) =
                _mm256_extract_epi64(Y_clipped_v, 0);
          }

          // remainder
          for (int c = channels / VLEN * VLEN; c < channels; ++c) {
            Yh[c] = 0;
          }

          for (int c = channels / VLEN * VLEN; c < channels; ++c) {
            const int pool_idx = (ph * pooled_width + pw) * channels + c;
            int32_t Yh_t = -in_zero_point * size;
            for (int h = hstart; h < hend; ++h) {
              for (int w = wstart; w < wend; ++w) {
                const int input_idx = (h * width + w) * channels + c;
                Yh_t += Xdata_temp[input_idx];
              }
            }

            Ydata_temp[pool_idx] = std::min<int32_t>(
                std::max<int32_t>(
                    nearbyint(Yh_t * multiplier + out_zero_point), minimum),
                maximum);
          }
        } // pw loop
      } // ph loop
    */
}

#[inline] pub fn average_pool_3d_avx2(
    xdata:          *const u8,
    n:              i32,
    height:         i32,
    width:          i32,
    depth:          i32,
    channels:       i32,
    pooled_height:  i32,
    pooled_width:   i32,
    pooled_depth:   i32,
    kernel_h:       i32,
    kernel_w:       i32,
    kernel_d:       i32,
    stride_h:       i32,
    stride_w:       i32,
    stride_d:       i32,
    pad_t:          i32,
    pad_l:          i32,
    pad_d:          i32,
    ydata:          *mut u8,
    in_scale:       f32,
    out_scale:      f32,
    in_zero_point:  i32,
    out_zero_point: i32,
    minimum:        i32,
    maximum:        i32)  {
    
    todo!();
    /*
        const uint8_t* Xdata_temp = Xdata + n * height * width * depth * channels;
      uint8_t* Ydata_temp =
          Ydata + n * pooled_height * pooled_width * pooled_depth * channels;

      const __m256i shuffle_mask_v = _mm256_set_epi8(
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0x0c,
          0x08,
          0x04,
          0x00,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0xff,
          0x0c,
          0x08,
          0x04,
          0x00);
      const __m256i permute_mask_v =
          _mm256_set_epi32(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00);

      const __m256i min_v = _mm256_set1_epi32(minimum);
      const __m256i max_v = _mm256_set1_epi32(maximum);
      __m256 out_zero_point_v = _mm256_set1_ps(out_zero_point);

      for (int ph = 0; ph < pooled_height; ++ph) {
        int hstart = ph * stride_h - pad_t;
        int hend = hstart + kernel_h < height ? hstart + kernel_h : height;
        hstart = hstart > 0 ? hstart : 0;
        for (int pw = 0; pw < pooled_width; ++pw) {
          int wstart = pw * stride_w - pad_l;
          int wend = wstart + kernel_w < width ? wstart + kernel_w : width;
          wstart = wstart > 0 ? wstart : 0;
          for (int pd = 0; pd < pooled_depth; ++pd) {
            int dstart = pd * stride_d - pad_d;
            int dend = dstart + kernel_d < depth ? dstart + kernel_d : depth;
            dstart = max(dstart, 0);

            int size = (hend - hstart) * (wend - wstart) * (dend - dstart);
            float multiplier = in_scale / out_scale / size;
            __m256 multiplier_v = _mm256_set1_ps(multiplier);

            uint8_t* Yh = Ydata_temp +
                ((ph * pooled_width + pw) * pooled_depth + pd) * channels;
            constexpr int VLEN = 8;
            int32_t Yh0 = -in_zero_point * size;

            // vectorized loop
            for (int c = 0; c < channels / VLEN * VLEN; c += VLEN) {
              __m256i Yh0_v = _mm256_set1_epi32(Yh0);

              for (int h = hstart; h < hend; ++h) {
                for (int w = wstart; w < wend; ++w) {
                  for (int d = dstart; d < dend; ++d) {
                    const int input_idx =
                        ((h * width + w) * depth + d) * channels + c;
                    const __m256i temp_v = _mm256_cvtepu8_epi32(_mm_loadl_epi64(
                        reinterpret_cast<const __m128i*>(Xdata_temp + input_idx)));
                    Yh0_v = _mm256_add_epi32(Yh0_v, temp_v);
                  }
                }
              }

              __m256 Yh0_fp = _mm256_cvtepi32_ps(Yh0_v);
              __m256 Y_float_v =
                  _mm256_fmadd_ps(Yh0_fp, multiplier_v, out_zero_point_v);
              __m256i Y_rounded_v = _mm256_cvtps_epi32(Y_float_v);
              __m256i Y_clipped_v =
                  _mm256_max_epi32(min_v, _mm256_min_epi32(max_v, Y_rounded_v));

              Y_clipped_v = _mm256_shuffle_epi8(Y_clipped_v, shuffle_mask_v);
              Y_clipped_v =
                  _mm256_permutevar8x32_epi32(Y_clipped_v, permute_mask_v);
              *reinterpret_cast<int64_t*>(Yh + c) =
                  _mm256_extract_epi64(Y_clipped_v, 0);
            }

            // remainder
            for (int c = channels / VLEN * VLEN; c < channels; ++c) {
              Yh[c] = 0;
            }

            for (int c = channels / VLEN * VLEN; c < channels; ++c) {
              const int pool_idx =
                  ((ph * pooled_width + pw) * pooled_depth + pd) * channels + c;

              int32_t Yh_t = -in_zero_point * size;
              for (int h = hstart; h < hend; ++h) {
                for (int w = wstart; w < wend; ++w) {
                  for (int d = dstart; d < dend; ++d) {
                    const int input_idx =
                        ((h * width + w) * depth + d) * channels + c;

                    Yh_t += Xdata_temp[input_idx];
                  }
                }
              }

              Ydata_temp[pool_idx] = std::min<int32_t>(
                  std::max<int32_t>(
                      nearbyint(Yh_t * multiplier + out_zero_point), minimum),
                  maximum);
            }

          } // pd loop
        } // pw loop
      } // ph loop
    */
}
