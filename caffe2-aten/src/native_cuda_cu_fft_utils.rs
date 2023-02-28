crate::ix!();

//-------------------------------------------[.cpp/pytorch/aten/src/ATen/native/cuda/CuFFTUtils.h]

/**
  | This means that max dim is 3 + 2 = 5 with
  | batch dimension and possible complex
  | dimension
  |
  */
pub const MAX_RANK: i32 = 3;

#[inline] pub fn cuda_get_error_enum(error: CuFFTResult) -> String {
    
    todo!();
        /*
            switch (error)
      {
        case CUFFT_SUCCESS:
          return "CUFFT_SUCCESS";
        case CUFFT_INVALID_PLAN:
          return "CUFFT_INVALID_PLAN";
        case CUFFT_ALLOC_FAILED:
          return "CUFFT_ALLOC_FAILED";
        case CUFFT_INVALID_TYPE:
          return "CUFFT_INVALID_TYPE";
        case CUFFT_INVALID_VALUE:
          return "CUFFT_INVALID_VALUE";
        case CUFFT_INTERNAL_ERROR:
          return "CUFFT_INTERNAL_ERROR";
        case CUFFT_EXEC_FAILED:
          return "CUFFT_EXEC_FAILED";
        case CUFFT_SETUP_FAILED:
          return "CUFFT_SETUP_FAILED";
        case CUFFT_INVALID_SIZE:
          return "CUFFT_INVALID_SIZE";
        case CUFFT_UNALIGNED_DATA:
          return "CUFFT_UNALIGNED_DATA";
        case CUFFT_INCOMPLETE_PARAMETER_LIST:
          return "CUFFT_INCOMPLETE_PARAMETER_LIST";
        case CUFFT_INVALID_DEVICE:
          return "CUFFT_INVALID_DEVICE";
        case CUFFT_PARSE_ERROR:
          return "CUFFT_PARSE_ERROR";
        case CUFFT_NO_WORKSPACE:
          return "CUFFT_NO_WORKSPACE";
        case CUFFT_NOT_IMPLEMENTED:
          return "CUFFT_NOT_IMPLEMENTED";
    #ifndef __HIP_PLATFORM_HCC__
        case CUFFT_LICENSE_ERROR:
          return "CUFFT_LICENSE_ERROR";
    #endif
        case CUFFT_NOT_SUPPORTED:
          return "CUFFT_NOT_SUPPORTED";
        default:
          ostringstream ss;
          ss << "unknown error " << error;
          return ss.str();
      }
        */
}

#[inline] pub fn CUFFT_CHECK(error: CuFFTResult)  {
    
    todo!();
        /*
            if (error != CUFFT_SUCCESS) {
        ostringstream ss;
        ss << "cuFFT error: " << _cudaGetErrorEnum(error);
        AT_ERROR(ss.str());
      }
        */
}
