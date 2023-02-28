/*!
  | Automatically generated header file for caffe2
  | macros.
  |
  | These macros are used to build the Caffe2
  | binary, and if you are building a dependent
  | library, they will need to be set as well for
  | your program to link correctly.
  |
  | Caffe2 version.
  |
  | The plan is to increment the minor version
  | every other week as a track point for bugs,
  | until we find a proper versioning cycle.
  */

crate::ix!();

/**
  | #define CAFFE2_VERSION_MAJOR @CAFFE2_VERSION_MAJOR@
  | #define CAFFE2_VERSION_MINOR @CAFFE2_VERSION_MINOR@
  | #define CAFFE2_VERSION_PATCH @CAFFE2_VERSION_PATCH@
  |
  */
static_assert!(
    CAFFE2_VERSION_MINOR < 100,
    "Programming error: you set a minor version that is too big.");

static_assert!(
    CAFFE2_VERSION_PATCH < 100,
    "Programming error: you set a patch version that is too big.");

#[macro_export] macro_rules! caffe2_version {
    () => {
        todo!();
        /*
        
          (CAFFE2_VERSION_MAJOR * 10000 + CAFFE2_VERSION_MINOR * 100 + 
           CAFFE2_VERSION_PATCH)
        */
    }
}

/**
  | #cmakedefine CAFFE2_BUILD_SHARED_LIBS
  | #cmakedefine CAFFE2_FORCE_FALLBACK_CUDA_MPI
  | #cmakedefine CAFFE2_HAS_MKL_DNN
  | #cmakedefine CAFFE2_HAS_MKL_SGEMM_PACK
  | #cmakedefine CAFFE2_PERF_WITH_AVX
  | #cmakedefine CAFFE2_PERF_WITH_AVX2
  | #cmakedefine CAFFE2_PERF_WITH_AVX512
  | #cmakedefine CAFFE2_THREADPOOL_MAIN_IMBALANCE
  | #cmakedefine CAFFE2_THREADPOOL_STATS
  | #cmakedefine CAFFE2_USE_EXCEPTION_PTR
  | #cmakedefine CAFFE2_USE_ACCELERATE
  | #cmakedefine CAFFE2_USE_CUDNN #cmakedefine
  | CAFFE2_USE_EIGEN_FOR_BLAS #cmakedefine
  | CAFFE2_USE_FBCODE #cmakedefine CAFFE2_USE_GOOGLE_GLOG
  | #cmakedefine CAFFE2_USE_LITE_PROTO
  | #cmakedefine CAFFE2_USE_MKL #cmakedefine
  | CAFFE2_USE_MKLDNN #cmakedefine CAFFE2_USE_NVTX
  | #cmakedefine CAFFE2_USE_TRT
  | 
  | #ifndef EIGEN_MPL2_ONLY #cmakedefine
  | EIGEN_MPL2_ONLY #endif
  |
  */

/**
  | Useful build settings that are recorded
  | in the compiled binary
  |
  */
#[macro_export] macro_rules! caffe2_build_strings {
    () => {
        todo!();
        /*
                { 
          {"TORCH_VERSION", "${TORCH_VERSION}"}, 
          {"CXX_COMPILER", "${CMAKE_CXX_COMPILER}"}, 
          {"CXX_FLAGS", "${CMAKE_CXX_FLAGS}"}, 
          {"BUILD_TYPE", "${CMAKE_BUILD_TYPE}"}, 
          {"BLAS_INFO", "${BLAS_INFO}"}, 
          {"LAPACK_INFO", "${LAPACK_INFO}"}, 
          {"USE_CUDA", "${USE_CUDA}"}, 
          {"CUDA_VERSION", "${CUDA_VERSION}"}, 
          {"USE_CUDNN", "${USE_CUDNN}"}, 
          {"CUDNN_VERSION", "${CUDNN_VERSION}"}, 
          {"USE_NCCL", "${USE_NCCL}"}, 
          {"USE_MPI", "${USE_MPI}"}, 
          {"USE_GFLAGS", "${USE_GFLAGS}"}, 
          {"USE_GLOG", "${USE_GLOG}"}, 
          {"USE_GLOO", "${USE_GLOI}"}, 
          {"USE_NNPACK", "${USE_NNPACK}"}, 
          {"USE_OPENMP", "${USE_OPENMP}"}, 
          {"FORCE_FALLBACK_CUDA_MPI", "${CAFFE2_FORCE_FALLBACK_CUDA_MPI}"}, 
          {"HAS_MKL_DNN", "${CAFFE2_HAS_MKL_DNN}"}, 
          {"HAS_MKL_SGEMM_PACK", "${CAFFE2_HAS_MKL_SGEMM_PACK}"}, 
          {"PERF_WITH_AVX", "${CAFFE2_PERF_WITH_AVX}"}, 
          {"PERF_WITH_AVX2", "${CAFFE2_PERF_WITH_AVX2}"}, 
          {"PERF_WITH_AVX512", "${CAFFE2_PERF_WITH_AVX512}"}, 
          {"USE_EXCEPTION_PTR", "${CAFFE2_USE_EXCEPTION_PTR}"}, 
          {"USE_ACCELERATE", "${CAFFE2_USE_ACCELERATE}"}, 
          {"USE_EIGEN_FOR_BLAS", "${CAFFE2_USE_EIGEN_FOR_BLAS}"}, 
          {"USE_LITE_PROTO", "${CAFFE2_USE_LITE_PROTO}"}, 
          {"USE_MKL", "${CAFFE2_USE_MKL}"}, 
          {"USE_MKLDNN", "${CAFFE2_USE_MKLDNN}"}, 
          {"USE_NVTX", "${CAFFE2_USE_NVTX}"}, 
          {"USE_TRT", "${CAFFE2_USE_TRT}"}, 
        */
    }
}
