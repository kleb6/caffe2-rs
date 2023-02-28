crate::ix!();

#[inline] pub fn embedding_lookup_idx_int32_t_float_float_avx2_fma<const IS_WEIGHT_POSITIONAL: bool>(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f32,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {

    todo!();
    /*
        const int prefdist_T0 = 16;
      const int fused_block_size = block_size + 0;
      int64_t dataInd = 0;
      if (block_size == 128) {
        // unrolling 16 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          __m256 vop64 = _mm256_setzero_ps();
          __m256 vop72 = _mm256_setzero_ps();
          __m256 vop80 = _mm256_setzero_ps();
          __m256 vop88 = _mm256_setzero_ps();
          __m256 vop96 = _mm256_setzero_ps();
          __m256 vop104 = _mm256_setzero_ps();
          __m256 vop112 = _mm256_setzero_ps();
          __m256 vop120 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (16)), vop16);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[16]), _MM_HINT_T0);
            vop24 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (24)), vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (32)), vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (40)), vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (48)), vop48);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[48]), _MM_HINT_T0);
            vop56 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (56)), vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
            vop64 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (64)), vop64);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[64]), _MM_HINT_T0);
            vop72 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (72)), vop72);
            // skip unnecessary prefetch of (&ip_next_T0[72])
            vop80 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (80)), vop80);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[80]), _MM_HINT_T0);
            vop88 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (88)), vop88);
            // skip unnecessary prefetch of (&ip_next_T0[88])
            vop96 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (96)), vop96);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[96]), _MM_HINT_T0);
            vop104 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (104)), vop104);
            // skip unnecessary prefetch of (&ip_next_T0[104])
            vop112 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (112)), vop112);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[112]), _MM_HINT_T0);
            vop120 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (120)), vop120);
            // skip unnecessary prefetch of (&ip_next_T0[120])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
            _mm256_storeu_ps(&op[64], vop64);
            _mm256_storeu_ps(&op[72], vop72);
            _mm256_storeu_ps(&op[80], vop80);
            _mm256_storeu_ps(&op[88], vop88);
            _mm256_storeu_ps(&op[96], vop96);
            _mm256_storeu_ps(&op[104], vop104);
            _mm256_storeu_ps(&op[112], vop112);
            _mm256_storeu_ps(&op[120], vop120);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
            _mm256_storeu_ps(&op[64], _mm256_mul_ps(vop64, vlen_inv));
            _mm256_storeu_ps(&op[72], _mm256_mul_ps(vop72, vlen_inv));
            _mm256_storeu_ps(&op[80], _mm256_mul_ps(vop80, vlen_inv));
            _mm256_storeu_ps(&op[88], _mm256_mul_ps(vop88, vlen_inv));
            _mm256_storeu_ps(&op[96], _mm256_mul_ps(vop96, vlen_inv));
            _mm256_storeu_ps(&op[104], _mm256_mul_ps(vop104, vlen_inv));
            _mm256_storeu_ps(&op[112], _mm256_mul_ps(vop112, vlen_inv));
            _mm256_storeu_ps(&op[120], _mm256_mul_ps(vop120, vlen_inv));
          }
        }
      } else if (block_size == 64) {
        // unrolling 8 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (16)), vop16);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[16]), _MM_HINT_T0);
            vop24 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (24)), vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (32)), vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (40)), vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (48)), vop48);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[48]), _MM_HINT_T0);
            vop56 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (56)), vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
          }
        }
      } else if (block_size == 32) {
        // unrolling 4 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (16)), vop16);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[16]), _MM_HINT_T0);
            vop24 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (24)), vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
          }
        }
      } else if (block_size == 16) {
        // unrolling 2 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
          }
        }
      } else {
        // generic code
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          int64_t j = 0;
          for (; j + 8 <= block_size; j += 8) {
            _mm256_storeu_ps(op + j, _mm256_setzero_ps());
          }
          for (; j < block_size; j++) {
            op[j] = 0.0f;
          }
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j],
                  _mm256_fmadd_ps(
                      vwgt, _mm256_loadu_ps(&ip[j]), _mm256_loadu_ps(&op[j])));
              _mm_prefetch(
                  reinterpret_cast<const char*>(&ip_next_T0[j]), _MM_HINT_T0);
            }
            for (; j < block_size; j++) {
              op[j] = std::fma(wgt, ip[j], op[j]);
            }
          }
          if (normalize_by_lengths && length) {
            float len_inv = 1.0f / length;
            __m256 vlen_inv = _mm256_set1_ps(len_inv);
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j], _mm256_mul_ps(_mm256_loadu_ps(&op[j]), vlen_inv));
            }
            for (; j < block_size; j++) {
              op[j] = len_inv * op[j];
            }
          }
        }
      }
      return dataInd == index_size;
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_float_float_false_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f32,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int32_t_float_float__avx2_fma<false>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_float_float_true_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f32,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int32_t_float_float__avx2_fma<true>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_float_float_avx2_fma<const IS_WEIGHT_POSITIONAL: bool>(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f32,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {

    todo!();
    /*
        const int64_t prefdist_T0 = 16;
      const int64_t fused_block_size = block_size + 0;
      int64_t dataInd = 0;
      if (block_size == 128) {
        // unrolling 16 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          __m256 vop64 = _mm256_setzero_ps();
          __m256 vop72 = _mm256_setzero_ps();
          __m256 vop80 = _mm256_setzero_ps();
          __m256 vop88 = _mm256_setzero_ps();
          __m256 vop96 = _mm256_setzero_ps();
          __m256 vop104 = _mm256_setzero_ps();
          __m256 vop112 = _mm256_setzero_ps();
          __m256 vop120 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (16)), vop16);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[16]), _MM_HINT_T0);
            vop24 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (24)), vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (32)), vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (40)), vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (48)), vop48);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[48]), _MM_HINT_T0);
            vop56 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (56)), vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
            vop64 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (64)), vop64);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[64]), _MM_HINT_T0);
            vop72 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (72)), vop72);
            // skip unnecessary prefetch of (&ip_next_T0[72])
            vop80 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (80)), vop80);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[80]), _MM_HINT_T0);
            vop88 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (88)), vop88);
            // skip unnecessary prefetch of (&ip_next_T0[88])
            vop96 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (96)), vop96);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[96]), _MM_HINT_T0);
            vop104 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (104)), vop104);
            // skip unnecessary prefetch of (&ip_next_T0[104])
            vop112 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (112)), vop112);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[112]), _MM_HINT_T0);
            vop120 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (120)), vop120);
            // skip unnecessary prefetch of (&ip_next_T0[120])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
            _mm256_storeu_ps(&op[64], vop64);
            _mm256_storeu_ps(&op[72], vop72);
            _mm256_storeu_ps(&op[80], vop80);
            _mm256_storeu_ps(&op[88], vop88);
            _mm256_storeu_ps(&op[96], vop96);
            _mm256_storeu_ps(&op[104], vop104);
            _mm256_storeu_ps(&op[112], vop112);
            _mm256_storeu_ps(&op[120], vop120);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
            _mm256_storeu_ps(&op[64], _mm256_mul_ps(vop64, vlen_inv));
            _mm256_storeu_ps(&op[72], _mm256_mul_ps(vop72, vlen_inv));
            _mm256_storeu_ps(&op[80], _mm256_mul_ps(vop80, vlen_inv));
            _mm256_storeu_ps(&op[88], _mm256_mul_ps(vop88, vlen_inv));
            _mm256_storeu_ps(&op[96], _mm256_mul_ps(vop96, vlen_inv));
            _mm256_storeu_ps(&op[104], _mm256_mul_ps(vop104, vlen_inv));
            _mm256_storeu_ps(&op[112], _mm256_mul_ps(vop112, vlen_inv));
            _mm256_storeu_ps(&op[120], _mm256_mul_ps(vop120, vlen_inv));
          }
        }
      } else if (block_size == 64) {
        // unrolling 8 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (16)), vop16);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[16]), _MM_HINT_T0);
            vop24 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (24)), vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (32)), vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (40)), vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (48)), vop48);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[48]), _MM_HINT_T0);
            vop56 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (56)), vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
          }
        }
      } else if (block_size == 32) {
        // unrolling 4 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (16)), vop16);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[16]), _MM_HINT_T0);
            vop24 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (24)), vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
          }
        }
      } else if (block_size == 16) {
        // unrolling 2 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (0)), vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(vwgt, _mm256_loadu_ps(ip + (8)), vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
          }
        }
      } else {
        // generic code
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          int64_t j = 0;
          for (; j + 8 <= block_size; j += 8) {
            _mm256_storeu_ps(op + j, _mm256_setzero_ps());
          }
          for (; j < block_size; j++) {
            op[j] = 0.0f;
          }
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const float* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const float* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j],
                  _mm256_fmadd_ps(
                      vwgt, _mm256_loadu_ps(&ip[j]), _mm256_loadu_ps(&op[j])));
              _mm_prefetch(
                  reinterpret_cast<const char*>(&ip_next_T0[j]), _MM_HINT_T0);
            }
            for (; j < block_size; j++) {
              op[j] = std::fma(wgt, ip[j], op[j]);
            }
          }
          if (normalize_by_lengths && length) {
            float len_inv = 1.0f / length;
            __m256 vlen_inv = _mm256_set1_ps(len_inv);
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j], _mm256_mul_ps(_mm256_loadu_ps(&op[j]), vlen_inv));
            }
            for (; j < block_size; j++) {
              op[j] = len_inv * op[j];
            }
          }
        }
      }
      return dataInd == index_size;
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_float_float_false_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f32,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int64_t_float_float__avx2_fma<false>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_float_float_true_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f32,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int64_t_float_float__avx2_fma<true>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_half_float_avx2_fma<const IS_WEIGHT_POSITIONAL: bool>(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f16,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {

    todo!();
    /*
        const int prefdist_T0 = 16;
      const int fused_block_size = block_size + 0;
      int64_t dataInd = 0;
      if (block_size == 128) {
        // unrolling 16 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          __m256 vop64 = _mm256_setzero_ps();
          __m256 vop72 = _mm256_setzero_ps();
          __m256 vop80 = _mm256_setzero_ps();
          __m256 vop88 = _mm256_setzero_ps();
          __m256 vop96 = _mm256_setzero_ps();
          __m256 vop104 = _mm256_setzero_ps();
          __m256 vop112 = _mm256_setzero_ps();
          __m256 vop120 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (16)))),
                vop16);
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (24)))),
                vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (32)))),
                vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (40)))),
                vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (48)))),
                vop48);
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (56)))),
                vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
            vop64 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (64)))),
                vop64);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[64]), _MM_HINT_T0);
            vop72 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (72)))),
                vop72);
            // skip unnecessary prefetch of (&ip_next_T0[72])
            vop80 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (80)))),
                vop80);
            // skip unnecessary prefetch of (&ip_next_T0[80])
            vop88 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (88)))),
                vop88);
            // skip unnecessary prefetch of (&ip_next_T0[88])
            vop96 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (96)))),
                vop96);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[96]), _MM_HINT_T0);
            vop104 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (104)))),
                vop104);
            // skip unnecessary prefetch of (&ip_next_T0[104])
            vop112 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (112)))),
                vop112);
            // skip unnecessary prefetch of (&ip_next_T0[112])
            vop120 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (120)))),
                vop120);
            // skip unnecessary prefetch of (&ip_next_T0[120])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
            _mm256_storeu_ps(&op[64], vop64);
            _mm256_storeu_ps(&op[72], vop72);
            _mm256_storeu_ps(&op[80], vop80);
            _mm256_storeu_ps(&op[88], vop88);
            _mm256_storeu_ps(&op[96], vop96);
            _mm256_storeu_ps(&op[104], vop104);
            _mm256_storeu_ps(&op[112], vop112);
            _mm256_storeu_ps(&op[120], vop120);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
            _mm256_storeu_ps(&op[64], _mm256_mul_ps(vop64, vlen_inv));
            _mm256_storeu_ps(&op[72], _mm256_mul_ps(vop72, vlen_inv));
            _mm256_storeu_ps(&op[80], _mm256_mul_ps(vop80, vlen_inv));
            _mm256_storeu_ps(&op[88], _mm256_mul_ps(vop88, vlen_inv));
            _mm256_storeu_ps(&op[96], _mm256_mul_ps(vop96, vlen_inv));
            _mm256_storeu_ps(&op[104], _mm256_mul_ps(vop104, vlen_inv));
            _mm256_storeu_ps(&op[112], _mm256_mul_ps(vop112, vlen_inv));
            _mm256_storeu_ps(&op[120], _mm256_mul_ps(vop120, vlen_inv));
          }
        }
      } else if (block_size == 64) {
        // unrolling 8 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (16)))),
                vop16);
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (24)))),
                vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (32)))),
                vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (40)))),
                vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (48)))),
                vop48);
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (56)))),
                vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
          }
        }
      } else if (block_size == 32) {
        // unrolling 4 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (16)))),
                vop16);
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (24)))),
                vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
          }
        }
      } else if (block_size == 16) {
        // unrolling 2 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
          }
        }
      } else {
        // generic code
        alignas(64) Half vtmp1[8] = {0};
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          int64_t j = 0;
          for (; j + 8 <= block_size; j += 8) {
            _mm256_storeu_ps(op + j, _mm256_setzero_ps());
          }
          for (; j < block_size; j++) {
            op[j] = 0.0f;
          }
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j],
                  _mm256_fmadd_ps(
                      vwgt,
                      _mm256_cvtph_ps(_mm_loadu_si128(
                          reinterpret_cast<const __m128i*>(&ip[j]))),
                      _mm256_loadu_ps(&op[j])));
              _mm_prefetch(
                  reinterpret_cast<const char*>(&ip_next_T0[j]), _MM_HINT_T0);
            }
            for (; j < block_size; j++) {
              vtmp1[0] = ip[j];
              __m256 vtmp2 =
                  _mm256_cvtph_ps(*(reinterpret_cast<const __m128i*>(vtmp1)));
              op[j] = std::fma(wgt, ((float*)(&vtmp2))[0], op[j]);
            }
          }
          if (normalize_by_lengths && length) {
            float len_inv = 1.0f / length;
            __m256 vlen_inv = _mm256_set1_ps(len_inv);
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j], _mm256_mul_ps(_mm256_loadu_ps(&op[j]), vlen_inv));
            }
            for (; j < block_size; j++) {
              op[j] = len_inv * op[j];
            }
          }
        }
      }
      return dataInd == index_size;
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_half_float_false_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f16,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int32_t_half_float__avx2_fma<false>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_half_float_true_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f16,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int32_t_half_float__avx2_fma<true>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_half_float_avx2_fma<const IS_WEIGHT_POSITIONAL: bool>(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f16,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {

    todo!();
    /*
        const int64_t prefdist_T0 = 16;
      const int64_t fused_block_size = block_size + 0;
      int64_t dataInd = 0;
      if (block_size == 128) {
        // unrolling 16 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          __m256 vop64 = _mm256_setzero_ps();
          __m256 vop72 = _mm256_setzero_ps();
          __m256 vop80 = _mm256_setzero_ps();
          __m256 vop88 = _mm256_setzero_ps();
          __m256 vop96 = _mm256_setzero_ps();
          __m256 vop104 = _mm256_setzero_ps();
          __m256 vop112 = _mm256_setzero_ps();
          __m256 vop120 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (16)))),
                vop16);
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (24)))),
                vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (32)))),
                vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (40)))),
                vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (48)))),
                vop48);
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (56)))),
                vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
            vop64 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (64)))),
                vop64);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[64]), _MM_HINT_T0);
            vop72 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (72)))),
                vop72);
            // skip unnecessary prefetch of (&ip_next_T0[72])
            vop80 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (80)))),
                vop80);
            // skip unnecessary prefetch of (&ip_next_T0[80])
            vop88 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (88)))),
                vop88);
            // skip unnecessary prefetch of (&ip_next_T0[88])
            vop96 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (96)))),
                vop96);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[96]), _MM_HINT_T0);
            vop104 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (104)))),
                vop104);
            // skip unnecessary prefetch of (&ip_next_T0[104])
            vop112 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (112)))),
                vop112);
            // skip unnecessary prefetch of (&ip_next_T0[112])
            vop120 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (120)))),
                vop120);
            // skip unnecessary prefetch of (&ip_next_T0[120])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
            _mm256_storeu_ps(&op[64], vop64);
            _mm256_storeu_ps(&op[72], vop72);
            _mm256_storeu_ps(&op[80], vop80);
            _mm256_storeu_ps(&op[88], vop88);
            _mm256_storeu_ps(&op[96], vop96);
            _mm256_storeu_ps(&op[104], vop104);
            _mm256_storeu_ps(&op[112], vop112);
            _mm256_storeu_ps(&op[120], vop120);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
            _mm256_storeu_ps(&op[64], _mm256_mul_ps(vop64, vlen_inv));
            _mm256_storeu_ps(&op[72], _mm256_mul_ps(vop72, vlen_inv));
            _mm256_storeu_ps(&op[80], _mm256_mul_ps(vop80, vlen_inv));
            _mm256_storeu_ps(&op[88], _mm256_mul_ps(vop88, vlen_inv));
            _mm256_storeu_ps(&op[96], _mm256_mul_ps(vop96, vlen_inv));
            _mm256_storeu_ps(&op[104], _mm256_mul_ps(vop104, vlen_inv));
            _mm256_storeu_ps(&op[112], _mm256_mul_ps(vop112, vlen_inv));
            _mm256_storeu_ps(&op[120], _mm256_mul_ps(vop120, vlen_inv));
          }
        }
      } else if (block_size == 64) {
        // unrolling 8 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (16)))),
                vop16);
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (24)))),
                vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (32)))),
                vop32);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[32]), _MM_HINT_T0);
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (40)))),
                vop40);
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (48)))),
                vop48);
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (56)))),
                vop56);
            // skip unnecessary prefetch of (&ip_next_T0[56])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
          }
        }
      } else if (block_size == 32) {
        // unrolling 4 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (16)))),
                vop16);
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (24)))),
                vop24);
            // skip unnecessary prefetch of (&ip_next_T0[24])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
          }
        }
      } else if (block_size == 16) {
        // unrolling 2 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (0)))),
                vop0);
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtph_ps(
                    _mm_loadu_si128(reinterpret_cast<const __m128i*>(ip + (8)))),
                vop8);
            // skip unnecessary prefetch of (&ip_next_T0[8])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
          }
        }
      } else {
        // generic code
        alignas(64) Half vtmp1[8] = {0};
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          int64_t j = 0;
          for (; j + 8 <= block_size; j += 8) {
            _mm256_storeu_ps(op + j, _mm256_setzero_ps());
          }
          for (; j < block_size; j++) {
            op[j] = 0.0f;
          }
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            __m256 vwgt = _mm256_set1_ps(wgt);
            const Half* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const Half* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j],
                  _mm256_fmadd_ps(
                      vwgt,
                      _mm256_cvtph_ps(_mm_loadu_si128(
                          reinterpret_cast<const __m128i*>(&ip[j]))),
                      _mm256_loadu_ps(&op[j])));
              _mm_prefetch(
                  reinterpret_cast<const char*>(&ip_next_T0[j]), _MM_HINT_T0);
            }
            for (; j < block_size; j++) {
              vtmp1[0] = ip[j];
              __m256 vtmp2 =
                  _mm256_cvtph_ps(*(reinterpret_cast<const __m128i*>(vtmp1)));
              op[j] = std::fma(wgt, ((float*)(&vtmp2))[0], op[j]);
            }
          }
          if (normalize_by_lengths && length) {
            float len_inv = 1.0f / length;
            __m256 vlen_inv = _mm256_set1_ps(len_inv);
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j], _mm256_mul_ps(_mm256_loadu_ps(&op[j]), vlen_inv));
            }
            for (; j < block_size; j++) {
              op[j] = len_inv * op[j];
            }
          }
        }
      }
      return dataInd == index_size;
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_half_float_false_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f16,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int64_t_half_float__avx2_fma<false>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_half_float_true_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const f16,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int64_t_half_float__avx2_fma<true>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_uint8_t_float_avx2_fma<const IS_WEIGHT_POSITIONAL: bool>(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const u8,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {

    todo!();
    /*
        const int prefdist_T0 = 16;
      const int fused_block_size = block_size + 0;
      int64_t dataInd = 0;
      if (block_size == 128) {
        // unrolling 16 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          __m256 vop64 = _mm256_setzero_ps();
          __m256 vop72 = _mm256_setzero_ps();
          __m256 vop80 = _mm256_setzero_ps();
          __m256 vop88 = _mm256_setzero_ps();
          __m256 vop96 = _mm256_setzero_ps();
          __m256 vop104 = _mm256_setzero_ps();
          __m256 vop112 = _mm256_setzero_ps();
          __m256 vop120 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (16))))),
                _mm256_add_ps(vop16, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (24))))),
                _mm256_add_ps(vop24, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (32))))),
                _mm256_add_ps(vop32, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[32])
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (40))))),
                _mm256_add_ps(vop40, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (48))))),
                _mm256_add_ps(vop48, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (56))))),
                _mm256_add_ps(vop56, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[56])
            vop64 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (64))))),
                _mm256_add_ps(vop64, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[64]), _MM_HINT_T0);
            vop72 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (72))))),
                _mm256_add_ps(vop72, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[72])
            vop80 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (80))))),
                _mm256_add_ps(vop80, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[80])
            vop88 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (88))))),
                _mm256_add_ps(vop88, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[88])
            vop96 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (96))))),
                _mm256_add_ps(vop96, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[96])
            vop104 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (104))))),
                _mm256_add_ps(vop104, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[104])
            vop112 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (112))))),
                _mm256_add_ps(vop112, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[112])
            vop120 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (120))))),
                _mm256_add_ps(vop120, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[120])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
            _mm256_storeu_ps(&op[64], vop64);
            _mm256_storeu_ps(&op[72], vop72);
            _mm256_storeu_ps(&op[80], vop80);
            _mm256_storeu_ps(&op[88], vop88);
            _mm256_storeu_ps(&op[96], vop96);
            _mm256_storeu_ps(&op[104], vop104);
            _mm256_storeu_ps(&op[112], vop112);
            _mm256_storeu_ps(&op[120], vop120);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
            _mm256_storeu_ps(&op[64], _mm256_mul_ps(vop64, vlen_inv));
            _mm256_storeu_ps(&op[72], _mm256_mul_ps(vop72, vlen_inv));
            _mm256_storeu_ps(&op[80], _mm256_mul_ps(vop80, vlen_inv));
            _mm256_storeu_ps(&op[88], _mm256_mul_ps(vop88, vlen_inv));
            _mm256_storeu_ps(&op[96], _mm256_mul_ps(vop96, vlen_inv));
            _mm256_storeu_ps(&op[104], _mm256_mul_ps(vop104, vlen_inv));
            _mm256_storeu_ps(&op[112], _mm256_mul_ps(vop112, vlen_inv));
            _mm256_storeu_ps(&op[120], _mm256_mul_ps(vop120, vlen_inv));
          }
        }
      } else if (block_size == 64) {
        // unrolling 8 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (16))))),
                _mm256_add_ps(vop16, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (24))))),
                _mm256_add_ps(vop24, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (32))))),
                _mm256_add_ps(vop32, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[32])
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (40))))),
                _mm256_add_ps(vop40, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (48))))),
                _mm256_add_ps(vop48, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (56))))),
                _mm256_add_ps(vop56, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[56])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
          }
        }
      } else if (block_size == 32) {
        // unrolling 4 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (16))))),
                _mm256_add_ps(vop16, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (24))))),
                _mm256_add_ps(vop24, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[24])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
          }
        }
      } else if (block_size == 16) {
        // unrolling 2 times
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
          }
        }
      } else {
        // generic code
        for (int rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          int64_t j = 0;
          for (; j + 8 <= block_size; j += 8) {
            _mm256_storeu_ps(op + j, _mm256_setzero_ps());
          }
          for (; j < block_size; j++) {
            op[j] = 0.0f;
          }
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j],
                  _mm256_fmadd_ps(
                      vwgt,
                      _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(_mm_loadl_epi64(
                          reinterpret_cast<const __m128i*>(&ip[j])))),
                      _mm256_add_ps(_mm256_loadu_ps(&op[j]), vbio)));
              _mm_prefetch(
                  reinterpret_cast<const char*>(&ip_next_T0[j]), _MM_HINT_T0);
            }
            for (; j < block_size; j++) {
              op[j] = std::fma(wgt, (float)ip[j], bio + op[j]);
            }
          }
          if (normalize_by_lengths && length) {
            float len_inv = 1.0f / length;
            __m256 vlen_inv = _mm256_set1_ps(len_inv);
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j], _mm256_mul_ps(_mm256_loadu_ps(&op[j]), vlen_inv));
            }
            for (; j < block_size; j++) {
              op[j] = len_inv * op[j];
            }
          }
        }
      }
      return dataInd == index_size;
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_uint8_t_float_false_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const u8,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int32_t_uint8_t_float__avx2_fma<false>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int32_t_uint8_t_float_true_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const u8,
    indices:              *const i32,
    offsets:              *const i32,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int32_t_uint8_t_float__avx2_fma<true>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_uint8_t_float_avx2_fma<const IS_WEIGHT_POSITIONAL: bool>(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const u8,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {

    todo!();
    /*
        const int64_t prefdist_T0 = 16;
      const int64_t fused_block_size = block_size + 0;
      int64_t dataInd = 0;
      if (block_size == 128) {
        // unrolling 16 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          __m256 vop64 = _mm256_setzero_ps();
          __m256 vop72 = _mm256_setzero_ps();
          __m256 vop80 = _mm256_setzero_ps();
          __m256 vop88 = _mm256_setzero_ps();
          __m256 vop96 = _mm256_setzero_ps();
          __m256 vop104 = _mm256_setzero_ps();
          __m256 vop112 = _mm256_setzero_ps();
          __m256 vop120 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (16))))),
                _mm256_add_ps(vop16, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (24))))),
                _mm256_add_ps(vop24, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (32))))),
                _mm256_add_ps(vop32, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[32])
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (40))))),
                _mm256_add_ps(vop40, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (48))))),
                _mm256_add_ps(vop48, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (56))))),
                _mm256_add_ps(vop56, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[56])
            vop64 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (64))))),
                _mm256_add_ps(vop64, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[64]), _MM_HINT_T0);
            vop72 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (72))))),
                _mm256_add_ps(vop72, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[72])
            vop80 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (80))))),
                _mm256_add_ps(vop80, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[80])
            vop88 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (88))))),
                _mm256_add_ps(vop88, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[88])
            vop96 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (96))))),
                _mm256_add_ps(vop96, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[96])
            vop104 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (104))))),
                _mm256_add_ps(vop104, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[104])
            vop112 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (112))))),
                _mm256_add_ps(vop112, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[112])
            vop120 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (120))))),
                _mm256_add_ps(vop120, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[120])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
            _mm256_storeu_ps(&op[64], vop64);
            _mm256_storeu_ps(&op[72], vop72);
            _mm256_storeu_ps(&op[80], vop80);
            _mm256_storeu_ps(&op[88], vop88);
            _mm256_storeu_ps(&op[96], vop96);
            _mm256_storeu_ps(&op[104], vop104);
            _mm256_storeu_ps(&op[112], vop112);
            _mm256_storeu_ps(&op[120], vop120);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
            _mm256_storeu_ps(&op[64], _mm256_mul_ps(vop64, vlen_inv));
            _mm256_storeu_ps(&op[72], _mm256_mul_ps(vop72, vlen_inv));
            _mm256_storeu_ps(&op[80], _mm256_mul_ps(vop80, vlen_inv));
            _mm256_storeu_ps(&op[88], _mm256_mul_ps(vop88, vlen_inv));
            _mm256_storeu_ps(&op[96], _mm256_mul_ps(vop96, vlen_inv));
            _mm256_storeu_ps(&op[104], _mm256_mul_ps(vop104, vlen_inv));
            _mm256_storeu_ps(&op[112], _mm256_mul_ps(vop112, vlen_inv));
            _mm256_storeu_ps(&op[120], _mm256_mul_ps(vop120, vlen_inv));
          }
        }
      } else if (block_size == 64) {
        // unrolling 8 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          __m256 vop32 = _mm256_setzero_ps();
          __m256 vop40 = _mm256_setzero_ps();
          __m256 vop48 = _mm256_setzero_ps();
          __m256 vop56 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (16))))),
                _mm256_add_ps(vop16, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (24))))),
                _mm256_add_ps(vop24, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[24])
            vop32 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (32))))),
                _mm256_add_ps(vop32, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[32])
            vop40 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (40))))),
                _mm256_add_ps(vop40, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[40])
            vop48 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (48))))),
                _mm256_add_ps(vop48, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[48])
            vop56 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (56))))),
                _mm256_add_ps(vop56, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[56])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
            _mm256_storeu_ps(&op[32], vop32);
            _mm256_storeu_ps(&op[40], vop40);
            _mm256_storeu_ps(&op[48], vop48);
            _mm256_storeu_ps(&op[56], vop56);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
            _mm256_storeu_ps(&op[32], _mm256_mul_ps(vop32, vlen_inv));
            _mm256_storeu_ps(&op[40], _mm256_mul_ps(vop40, vlen_inv));
            _mm256_storeu_ps(&op[48], _mm256_mul_ps(vop48, vlen_inv));
            _mm256_storeu_ps(&op[56], _mm256_mul_ps(vop56, vlen_inv));
          }
        }
      } else if (block_size == 32) {
        // unrolling 4 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          __m256 vop16 = _mm256_setzero_ps();
          __m256 vop24 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
            vop16 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (16))))),
                _mm256_add_ps(vop16, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[16])
            vop24 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (24))))),
                _mm256_add_ps(vop24, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[24])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
            _mm256_storeu_ps(&op[16], vop16);
            _mm256_storeu_ps(&op[24], vop24);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
            _mm256_storeu_ps(&op[16], _mm256_mul_ps(vop16, vlen_inv));
            _mm256_storeu_ps(&op[24], _mm256_mul_ps(vop24, vlen_inv));
          }
        }
      } else if (block_size == 16) {
        // unrolling 2 times
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          __m256 vop0 = _mm256_setzero_ps();
          __m256 vop8 = _mm256_setzero_ps();
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            vop0 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (0))))),
                _mm256_add_ps(vop0, vbio));
            _mm_prefetch(
                reinterpret_cast<const char*>(&ip_next_T0[0]), _MM_HINT_T0);
            vop8 = _mm256_fmadd_ps(
                vwgt,
                _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(
                    _mm_loadl_epi64(reinterpret_cast<const __m128i*>(ip + (8))))),
                _mm256_add_ps(vop8, vbio));
            // skip unnecessary prefetch of (&ip_next_T0[8])
          }
          if (!normalize_by_lengths || length == 0) {
            _mm256_storeu_ps(&op[0], vop0);
            _mm256_storeu_ps(&op[8], vop8);
          } else {
            __m256 vlen_inv = _mm256_set1_ps(1.0f / length);
            _mm256_storeu_ps(&op[0], _mm256_mul_ps(vop0, vlen_inv));
            _mm256_storeu_ps(&op[8], _mm256_mul_ps(vop8, vlen_inv));
          }
        }
      } else {
        // generic code
        for (int64_t rangeIndex = 0; rangeIndex < output_size; ++rangeIndex) {
          float* op = &out[rangeIndex * block_size];
          int64_t j = 0;
          for (; j + 8 <= block_size; j += 8) {
            _mm256_storeu_ps(op + j, _mm256_setzero_ps());
          }
          for (; j < block_size; j++) {
            op[j] = 0.0f;
          }
          if (dataInd != offsets[rangeIndex] - offsets[0]) {
            return false;
          }
          int64_t end_offset = offsets[rangeIndex + 1];
          int64_t length = end_offset - offsets[rangeIndex];
          for (int64_t start = dataInd; dataInd < end_offset - offsets[0];
               ++dataInd) {
            const int64_t idx = indices[dataInd];
            if (idx < 0 || idx >= data_size) {
              return false;
            }
            float wgt = 1.f;
            float bio;
            if (weights) {
              wgt = weights[IS_WEIGHT_POSITIONAL ? (dataInd - start) : dataInd];
            }
            bio = wgt * scale_bias[2 * idx + 1];
            wgt = wgt * scale_bias[2 * idx];
            __m256 vbio = _mm256_set1_ps(bio);
            __m256 vwgt = _mm256_set1_ps(wgt);
            const uint8_t* ip = &input[idx * fused_block_size];
            const int64_t next_T0 = (dataInd < index_size - prefdist_T0)
                ? (dataInd + prefdist_T0)
                : dataInd;
            const int64_t idx_pref_T0 = indices[next_T0];
            if (idx_pref_T0 < 0 || idx_pref_T0 >= data_size) {
              return false;
            }
            const uint8_t* ip_next_T0 = &input[idx_pref_T0 * fused_block_size];
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j],
                  _mm256_fmadd_ps(
                      vwgt,
                      _mm256_cvtepi32_ps(_mm256_cvtepu8_epi32(_mm_loadl_epi64(
                          reinterpret_cast<const __m128i*>(&ip[j])))),
                      _mm256_add_ps(_mm256_loadu_ps(&op[j]), vbio)));
              _mm_prefetch(
                  reinterpret_cast<const char*>(&ip_next_T0[j]), _MM_HINT_T0);
            }
            for (; j < block_size; j++) {
              op[j] = std::fma(wgt, (float)ip[j], bio + op[j]);
            }
          }
          if (normalize_by_lengths && length) {
            float len_inv = 1.0f / length;
            __m256 vlen_inv = _mm256_set1_ps(len_inv);
            j = 0;
            for (; j + 8 <= block_size; j += 8) {
              _mm256_storeu_ps(
                  &op[j], _mm256_mul_ps(_mm256_loadu_ps(&op[j]), vlen_inv));
            }
            for (; j < block_size; j++) {
              op[j] = len_inv * op[j];
            }
          }
        }
      }
      return dataInd == index_size;
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_uint8_t_float_false_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const u8,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int64_t_uint8_t_float__avx2_fma<false>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}

#[inline] pub fn embedding_lookup_idx_int64_t_uint8_t_float_true_avx2_fma(
    block_size:           i64,
    output_size:          i64,
    index_size:           i64,
    data_size:            i64,
    input:                *const u8,
    indices:              *const i64,
    offsets:              *const i64,
    weights:              *const f32,
    scale_bias:           *const f32,
    normalize_by_lengths: bool,
    out:                  *mut f32) -> bool {
    
    todo!();
    /*
        return EmbeddingLookupIdx_int64_t_uint8_t_float__avx2_fma<true>(
          block_size,
          output_size,
          index_size,
          data_size,
          input,
          indices,
          offsets,
          weights,
          scale_bias,
          normalize_by_lengths,
          out);
    */
}
