crate::ix!();

#[inline] pub fn run_average_pool_gradient1d<T, const kOrder: StorageOrder>(
    n:                 i32,
    c:                 i32,
    x_size:            i32,
    y_size:            i32,
    kernel:            i32,
    stride:            i32,
    pad:               i32,
    count_include_pad: bool,
    dy:                *const T,
    dx:                *mut T)  {

    todo!();
    /*
        const int batch_size = kOrder == StorageOrder::NCHW ? N * C : N;
      const int X_stride = kOrder == StorageOrder::NCHW ? X_size : X_size * C;
      const int Y_stride = kOrder == StorageOrder::NCHW ? Y_size : Y_size * C;
      std::memset(dX, 0, sizeof(T) * N * C * X_size);
      const T* dY_ptr = dY;
      T* dX_ptr = dX;
      for (int i = 0; i < batch_size; ++i) {
        ConstEigenArrayMap<T> dY_arr = kOrder == StorageOrder::NCHW
            ? ConstEigenArrayMap<T>(dY_ptr, Y_size, 1)
            : ConstEigenArrayMap<T>(dY_ptr, C, Y_size);
        EigenArrayMap<T> dX_arr = kOrder == StorageOrder::NCHW
            ? EigenArrayMap<T>(dX_ptr, X_size, 1)
            : EigenArrayMap<T>(dX_ptr, C, X_size);
        for (int y = 0; y < Y_size; ++y) {
          const int l = std::max(y * stride - pad, 0);
          const int r = std::min(y * stride - pad + kernel, X_size);
          const T scale = T(1) / static_cast<T>(count_include_pad ? kernel : r - l);
          ComputeAveragePoolGradient1D<T, kOrder>(l, r, y, scale, dY_arr, &dX_arr);
        }
        dY_ptr += Y_stride;
        dX_ptr += X_stride;
      }
    */
}

#[inline] pub fn run_average_pool_gradient2D<T, const kOrder: StorageOrder>(
    n:                 i32,
    c:                 i32,
    x_H:               i32,
    x_W:               i32,
    y_H:               i32,
    y_W:               i32,
    kernel_h:          i32,
    kernel_w:          i32,
    stride_h:          i32,
    stride_w:          i32,
    pad_t:             i32,
    pad_l:             i32,
    count_include_pad: bool,
    dY:                *const T,
    dX:                *mut T) 
{
    todo!();
    /*
        const int batch_size = kOrder == StorageOrder::NCHW ? N * C : N;
      const int X_HxW = X_H * X_W;
      const int Y_HxW = Y_H * Y_W;
      const int X_stride = kOrder == StorageOrder::NCHW ? X_HxW : X_HxW * C;
      const int Y_stride = kOrder == StorageOrder::NCHW ? Y_HxW : Y_HxW * C;
      std::memset(dX, 0, sizeof(T) * N * C * X_HxW);
      const T* dY_ptr = dY;
      T* dX_ptr = dX;
      for (int i = 0; i < batch_size; ++i) {
        ConstEigenArrayMap<T> dY_arr = kOrder == StorageOrder::NCHW
            ? ConstEigenArrayMap<T>(dY_ptr, Y_W, Y_H)
            : ConstEigenArrayMap<T>(dY_ptr, C, Y_HxW);
        EigenArrayMap<T> dX_arr = kOrder == StorageOrder::NCHW
            ? EigenArrayMap<T>(dX_ptr, X_W, X_H)
            : EigenArrayMap<T>(dX_ptr, C, X_HxW);
        for (int h = 0; h < Y_H; ++h) {
          const int t = std::max(h * stride_h - pad_t, 0);
          const int b = std::min(h * stride_h - pad_t + kernel_h, X_H);
          for (int w = 0; w < Y_W; ++w) {
            const int l = std::max(w * stride_w - pad_l, 0);
            const int r = std::min(w * stride_w - pad_l + kernel_w, X_W);
            const int y = h * Y_W + w;
            const T scale = T(1) /
                static_cast<T>(count_include_pad ? kernel_h * kernel_w
                                                 : (b - t) * (r - l));
            ComputeAveragePoolGradient2D<T, kOrder>(
                X_W, t, b, l, r, y, scale, dY_arr, &dX_arr);
          }
        }
        dY_ptr += Y_stride;
        dX_ptr += X_stride;
      }
    */
}

#[inline] pub fn run_average_pool_gradient3D<T, const kOrder: StorageOrder>(
    n:                  i32,
    c:                  i32,
    x_D:                i32,
    x_H:                i32,
    x_W:                i32,
    y_D:                i32,
    y_H:                i32,
    y_W:                i32,
    kernel_d:           i32,
    kernel_h:           i32,
    kernel_w:           i32,
    stride_d:           i32,
    stride_h:           i32,
    stride_w:           i32,
    pad_p:              i32,
    pad_t:              i32,
    pad_l:              i32,
    count_include_pad:  bool,
    dY:                 *const T,
    dX:                 *mut T) 
{
    todo!();
    /*
        const int batch_size = kOrder == StorageOrder::NCHW ? N * C : N;
      const int X_HxW = X_D * X_H * X_W;
      const int Y_HxW = Y_D * Y_H * Y_W;
      const int X_stride = kOrder == StorageOrder::NCHW ? X_HxW : X_HxW * C;
      const int Y_stride = kOrder == StorageOrder::NCHW ? Y_HxW : Y_HxW * C;
      std::memset(dX, 0, sizeof(T) * N * C * X_HxW);
      const T* dY_ptr = dY;
      T* dX_ptr = dX;
      for (int i = 0; i < batch_size; ++i) {
        ConstEigenArrayMap<T> dY_arr = kOrder == StorageOrder::NCHW
            ? ConstEigenArrayMap<T>(dY_ptr, Y_W, Y_D * Y_H)
            : ConstEigenArrayMap<T>(dY_ptr, C, Y_HxW);
        EigenArrayMap<T> dX_arr = kOrder == StorageOrder::NCHW
            ? EigenArrayMap<T>(dX_ptr, X_W, X_D * X_H)
            : EigenArrayMap<T>(dX_ptr, C, X_HxW);
        for (int d = 0; d < Y_D; ++d) {
          const int p = std::max(d * stride_d - pad_p, 0);
          const int a = std::min(d * stride_d - pad_p + kernel_d, X_D);
          for (int h = 0; h < Y_H; ++h) {
            const int t = std::max(h * stride_h - pad_t, 0);
            const int b = std::min(h * stride_h - pad_t + kernel_h, X_H);
            for (int w = 0; w < Y_W; ++w) {
              const int l = std::max(w * stride_w - pad_l, 0);
              const int r = std::min(w * stride_w - pad_l + kernel_w, X_W);
              const int y = d * Y_H * Y_W + h * Y_W + w;
              const T scale = T(1) /
                  static_cast<T>(count_include_pad ? kernel_d * kernel_h * kernel_w
                                                   : (a - p) * (b - t) * (r - l));
              ComputeAveragePoolGradient3D<T, kOrder>(
                  X_H, X_W, p, a, t, b, l, r, y, scale, dY_arr, &dX_arr);
            }
          }
        }
        dY_ptr += Y_stride;
        dX_ptr += X_stride;
      }
    */
}

