crate::ix!();

/**
  | Note [What is CudnnWrapper good for?]
  | ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Suppose
  | you are writing a kernel that calls into Cudnn,
  | and you need a cudnnHandle_t to pass to the
  | kernel call.
  |
  | How should you go about getting one of those
  | handles?  You'd prefer not to make a new
  | cudnnHandle_t every call; this can be somewhat
  | expensive (1-2%, according to some measurements
  | in TensorFlow.)
  |
  | But cudnnHandle_t is not thread-safe, so we
  | can't just have a single global cudnnHandle_t
  | that everyone uses.
  |
  | Thus, the most common method in Caffe2 for
  | getting a Cudnn handle is to get a per-thread,
  | per-stream Cudnn handle from CUDAContext (which
  | knows what the current thread and stream are).
  |
  | The idiomatic way to do this in Caffe2 today is
  | to make a CudnnWrapper and then call
  | inline_cudnn_handle(), although you didn't
  | really need the CudnnWrapper at all (you could
  | have gotten it directly from CUDAContext.)
  |
  | So, what's all this business about
  | CudnnWrapper?  In theory, it was designed with
  | a more specialized use-case in mind, where you
  | need to make multiple calls to Cudnn in
  | parallel; e.g., when manually computing group
  | convolution.
  |
  | By using with_cudnn_state(), you can get
  | separate cudnnHandle_t and CUDA stream per
  | parallel thread of execution, and run all of
  | the cuDNN calls in parallel.
  |
  | CudnnWrapper handles the business of
  | synchronizing with the stream prior to this
  | call.
  |
  | (By the way, this is why no such CUBLASWrapper
  | exists; there isn't ever any reason you need to
  | call cublas in parallel, since most cublas
  | operations have batched variants.)
  |
  | Now, that's the theory... in practice, this is
  | only ever used when multiple operators are run
  | in parallel, and not to actually parallelize
  | multiple Cudnn calls (for example, group
  | convolution is now supported natively in
  | Cudnn.)
  |
  | So... while the kit provided here might be
  | useful for someone else in the future, it's not
  | really used now.
  |
  | So we might consider deleting it, or unifying
  | this mechanism with PyTorch's own Cudnn handle
  | pool.  (which is it's own thing.)
  */

/**
  | CudnnWorkspace is a wrapper around
  | a raw cuda pointer that holds the cudnn
  | scratch space.
  | 
  | This struct is meant to be only used in
  | 
  | CudnnWrapper to provide a program-wide
  | scratch space for Cudnn.
  | 
  | The reason behind it is that cudnn function
  | calls are usually very efficient, hence
  | one probably does not want to run multiple
  | cudnn calls at the same time.
  | 
  | As a result, one should not need more
  | than one cudnn workspace per device.
  |
  */
pub struct CudnnWorkspace {

    /// {nullptr, nullptr, &NoDelete, at::Device(CUDA)};
    data: DataPtr,

    /// {0};
    nbytes: usize,
}

impl CudnnWorkspace {
    
    #[inline] pub fn get(&mut self, nbytes: usize)  {
        
        todo!();
        /*
            if (nbytes_ < nbytes) {
                reset();
                data_ = CUDAContext::New(nbytes);
                nbytes_ = nbytes;
            }
            CAFFE_ENFORCE_GE(nbytes_, nbytes);
            return data_.get();
        */
    }
    
    #[inline] pub fn reset(&mut self)  {
        
        todo!();
        /*
            data_.clear();
            nbytes_ = 0;
        */
    }
}

/**
  | CudnnState is the owner of the CudnnWorkspace,
  | and serializes all executions of operations
  | that use the state onto it's own stream
  | (so multiple Net workers can reuse the same workspace
  | from different threads and CUDA streams).
  |
  */
pub struct CudnnState {
    cudnn_handle:   CudnnHandle,  // default = nullptr
    before:         CudaEvent,    // default = nullptr
    after:          CudaEvent,    // default = nullptr
    stream:         CudaStream,   // default = nullptr
    workspace:      CudnnWorkspace,
    gpu_id:         usize,          // default = 0
}

impl CudnnState {
    
    pub fn new(gpu_id: usize) -> Self {
    
        todo!();
        /*
            : gpu_id_(gpu_id) 

                CUDAGuard g(gpu_id_);
                CUDNN_ENFORCE(cudnnCreate(&cudnn_handle_));
                CUDA_ENFORCE(cudaEventCreate(&before_));
                CUDA_ENFORCE(cudaEventCreate(&after_));
                CUDA_ENFORCE(cudaStreamCreate(&stream_));
                CUDNN_ENFORCE(cudnnSetStream(cudnn_handle_, stream_));
        */
    }
}

impl Drop for CudnnState {
    fn drop(&mut self) {
        todo!();
        /* 
                CUDAGuard g(gpu_id_);
                CUDNN_CHECK(cudnnDestroy(cudnn_handle_));
                CUDA_CHECK(cudaStreamDestroy(stream_));
                CUDA_CHECK(cudaEventDestroy(after_));
                CUDA_CHECK(cudaEventDestroy(before_));
             */
    }
}

impl CudnnState {
    
    #[inline] pub fn cudnn_handle(&mut self) -> &mut CudnnHandle {
        
        todo!();
        /*
            return cudnn_handle_;
        */
    }
    
    #[inline] pub fn workspace(&mut self) -> &mut CudnnWorkspace {
        
        todo!();
        /*
            return workspace_;
        */
    }
    
    #[inline] pub fn execute<F>(&mut self, stream: CudaStream, f: F)  {
    
        todo!();
        /*
            CUDA_ENFORCE(cudaEventRecord(before_, stream));
                    CUDA_ENFORCE(cudaStreamWaitEvent(stream_, before_, 0));
                    f(this);
                    CUDA_ENFORCE(cudaEventRecord(after_, stream_));
                    CUDA_ENFORCE(cudaStreamWaitEvent(stream, after_, 0));
        */
    }
}

/**
  | CudnnWrapper is a class that wraps the
  | cudnn handles and cudnn workspaces.
  | 
  | The wrapper ensures that for each thread
  | and each gpu, there is one identical
  | cudnn handle, which is also associated
  | with the thread-local per-device cuda
  | stream. The wrapper also hosts the device-specific
  | cudnn workspace (scratch space for
  | some cudnn functions).
  |
  */
pub struct CudnnWrapper {

    /**
      | Pointer to an external cuda context
      | that the cudnn wrapper will use.
      |
      */
    context: *mut CUDAContext,
}

pub const CAFFE2_COMPILE_TIME_MAX_CUDNN_STATES: usize = 4;
pub const COMPILE_TIME_MAX_GPUS: usize = 16;

pub type PerGPUCudnnStates = [[SyncedCudnnState; CAFFE2_COMPILE_TIME_MAX_CUDNN_STATES]; COMPILE_TIME_MAX_GPUS];

impl CudnnWrapper {

    /**
      | Creates a cudnn wrapper associated
      | with a CUDAContext object. Note that
      | the CUDAContext object should outlive
      | the CudnnWrapper.
      |
      */
    pub fn new(context: *mut CUDAContext) -> Self {
    
        todo!();
        /*
            : context_(context)
        */
    }
    
    /**
      | Returns the inline cudnn handle that
      | executes on the current thread's cuda_stream.
      |
      */
    #[inline] pub fn inline_cudnn_handle(&mut self) -> CudnnHandle {
        
        todo!();
        /*
            return context_->cudnn_handle();
        */
    }

    /**
      | Executes the closure F on the CudnnState
      | associated with state_idx
      |
      */
    #[inline] pub fn with_cudnn_state<F>(&mut self, state_idx: usize, f: F)  {
    
        todo!();
        /*
            CAFFE_ENFORCE(
                        state_idx < CAFFE2_COMPILE_TIME_MAX_CUDNN_STATES, "Invalid state_idx");
                    auto& sync_state = cudnn_states()[context_->device_id()][state_idx];

                    CUDAGuard dg(context_->device_id());

                    // We need to serialize execution on the CudnnState as we can't
                    // allow multiple threads to race through the cudaEventRecord
                    // calls (so a worker thread might wait on another worker thread's
                    // execution)
                    std::lock_guard<std::mutex> g(sync_state.mutex);
                    if (!sync_state.state.get()) {
                        sync_state.state.reset(new CudnnState(context_->device_id()));
                    }
                    CHECK_NOTNULL(sync_state.state.get())->execute(context_->cuda_stream(), f);
        */
    }
    
    #[inline] pub fn cudnn_states<'a>() -> &'a mut PerGpuCudnnStates {
        
        todo!();
        /*
            // New it (never delete) to avoid calling the destructors on process
      // exit and racing against the CUDA shutdown sequence.
      static auto* p = new CudnnWrapper::PerGPUCudnnStates();
      CHECK_NOTNULL(p);
      return *p;
        */
    }
}

pub struct SyncedCudnnState {
    mutex: parking_lot::RawMutex,
    state: Box<CudnnState>,
}
