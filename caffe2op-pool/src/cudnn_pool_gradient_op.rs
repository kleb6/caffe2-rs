crate::ix!();

pub struct CudnnPoolGradientOp<Functor> {
    base:            ConvPoolOpBase<CUDAContext>,
    cudnn_wrapper:   CudnnWrapper,
    x_desc:          CudnnTensorDescriptor,
    y_desc:          CudnnTensorDescriptor,
    pooling_desc:    CudnnPoolingDescriptor,
    functor:         Functor,
    equal_padding:   bool,
    cached_X_dims:   Vec<i64>,
}

impl<Functor> Drop for CudnnPoolGradientOp<Functor> {
    fn drop(&mut self) {
        todo!();
        /* 
        CUDNN_ENFORCE(cudnnDestroyTensorDescriptor(X_desc_));
        CUDNN_ENFORCE(cudnnDestroyTensorDescriptor(Y_desc_));
        CUDNN_ENFORCE(cudnnDestroyPoolingDescriptor(pooling_desc_));
       */
    }
}

impl<Functor> CudnnPoolGradientOp<Functor> {
    
    pub fn new<Args>(args: Args) -> Self {
        todo!();
        /*
            : ConvPoolOpBase<CUDAContext>(std::forward<Args>(args)...),
            cudnn_wrapper_(&context_),
            functor_(*this),
            equal_padding_(std::equal(
                pads_.cbegin(),
                pads_.cbegin() + kernel_.size(),
                pads_.cbegin() + kernel_.size())) 

        CUDNN_ENFORCE(cudnnCreateTensorDescriptor(&X_desc_));
        CUDNN_ENFORCE(cudnnCreateTensorDescriptor(&Y_desc_));
        CUDNN_ENFORCE(cudnnCreatePoolingDescriptor(&pooling_desc_));
        if (!global_pooling_ && equal_padding_) {
          if (kernel_.size() == 2) {
            CUDNN_ENFORCE(cudnnSetPooling2dDescriptor(
                pooling_desc_,
                functor_.GetPoolingMode(),
                CUDNN_NOT_PROPAGATE_NAN,
                kernel_h(),
                kernel_w(),
                pad_t(),
                pad_l(),
                stride_h(),
                stride_w()));
          } else if (kernel_.size() == 3) {
            CUDNN_ENFORCE(cudnnSetPoolingNdDescriptor(
                pooling_desc_,
                functor_.GetPoolingMode(),
                CUDNN_NOT_PROPAGATE_NAN,
                kernel_.size(),
                kernel_.data(),
                pads_.data(),
                stride_.data()));
          }
        }
        */
    }
    
    #[inline] pub fn run_on_device(&mut self) -> bool {
        
        todo!();
        /*
            return DispatchHelper<TensorTypes<float>>::call(this, Input(0));
        */
    }
    
    #[inline] pub fn do_run_with_type<T>(&mut self, ) -> bool {
        todo!();
        /*
            const auto& X = Input(0);
        const auto& Y = Input(1);
        const auto& dY = Input(2);
        auto* dX = Output(0, X.sizes(), at::dtype<T>());
        const int ndim = X.dim();
        const int N = X.dim32(0);
        const int C = order_ == StorageOrder::NCHW ? X.dim32(1) : X.dim32(ndim - 1);
        const std::vector<int> X_HW_dims = GetDims(X);
        const std::vector<int> Y_HW_dims = GetDims(Y);
        ConvPoolOpBase<CUDAContext>::ComputePads(X_HW_dims);
        const T* dY_data = dY.template data<T>();
        const T* X_data = X.template data<T>();
        const T* Y_data = Y.template data<T>();
        T* dX_data = dX->template mutable_data<T>();

        if (N == 0) {
          return true;
        }

        if (global_pooling_) {
          const int HxW = X.numel() / (N * C);
          if (order_ == StorageOrder::NCHW) {
            return functor_.template GlobalPoolingBackward<T, StorageOrder::NCHW>(
                N, C, HxW, dY_data, X_data, Y_data, dX_data, &context_);
          } else {
            return functor_.template GlobalPoolingBackward<T, StorageOrder::NHWC>(
                N, C, HxW, dY_data, X_data, Y_data, dX_data, &context_);
          }
        }

        if (order_ == StorageOrder::NHWC) {
          // Cudnn Pooling on NHWC order is very slow, fallback to CUDA
          // implementation.
          return functor_.template Backward<T, StorageOrder::NHWC>(
              N,
              C,
              X_HW_dims,
              Y_HW_dims,
              kernel_,
              dilation_,
              stride_,
              pads_,
              dY_data,
              X_data,
              Y_data,
              dX_data,
              &context_);
        } else if (!equal_padding_ || ndim == 3) {
          return functor_.template Backward<T, StorageOrder::NCHW>(
              N,
              C,
              X_HW_dims,
              Y_HW_dims,
              kernel_,
              dilation_,
              stride_,
              pads_,
              dY_data,
              X_data,
              Y_data,
              dX_data,
              &context_);
        }

        const std::vector<std::int64_t> X_dims = X.sizes().vec();
        const std::vector<std::int64_t> Y_dims = Y.sizes().vec();
        if (cached_X_dims_ != X_dims) {
          constexpr cudnnDataType_t data_type = cudnnTypeWrapper<T>::type;
          SetTensorDescriptor(data_type, order_, X_dims, &X_desc_);
          SetTensorDescriptor(data_type, order_, Y_dims, &Y_desc_);
          cached_X_dims_ = X_dims;
        }
        CUDNN_ENFORCE(cudnnPoolingBackward(
            cudnn_wrapper_.inline_cudnn_handle(),
            pooling_desc_,
            cudnnTypeWrapper<T>::kOne(),
            Y_desc_,
            Y_data,
            Y_desc_,
            dY_data,
            X_desc_,
            X_data,
            cudnnTypeWrapper<T>::kZero(),
            X_desc_,
            dX_data));

        return true;
        */
    }
}
