crate::ix!();

#[cfg(openmp)]
#[inline] pub fn caffe_2set_open_mpthreads(i: *mut i32, c: *mut *mut *mut u8) -> bool {
    
    todo!();
    /*
        if (!getenv("OMP_NUM_THREADS")) {
        // OMP_NUM_THREADS not passed explicitly, so *disable* OMP by
        // default. The user can use the CLI flag to override.
        VLOG(1) << "OMP_NUM_THREADS not passed, defaulting to 1 thread";
        omp_set_num_threads(1);
      }

      if (FLAGS_caffe2_omp_num_threads > 0) {
        VLOG(1) << "Setting omp_num_threads to " << FLAGS_caffe2_omp_num_threads;
        omp_set_num_threads(FLAGS_caffe2_omp_num_threads);
      }
      VLOG(1) << "Caffe2 running with " << omp_get_max_threads() << " OMP threads";
      return true;
    */
}


#[cfg(openmp)]
register_caffe2_init_function!{
    Caffe2SetOpenMPThreads,
    &Caffe2SetOpenMPThreads,
    "Set OpenMP threads."
}

#[cfg(caffe2_use_mkl)]
#[inline] pub fn caffe_2set_mklthreads(
    i: *mut i32,
    c: *mut *mut *mut u8) -> bool 
{
    todo!();
    /*
        if (!getenv("MKL_NUM_THREADS")) {
        VLOG(1) << "MKL_NUM_THREADS not passed, defaulting to 1 thread";
        mkl_set_num_threads(1);
      }

      // If caffe2_omp_num_threads is set, we use that for MKL as well.
      if (FLAGS_caffe2_omp_num_threads > 0) {
        VLOG(1) << "Setting mkl_num_threads to " << FLAGS_caffe2_omp_num_threads
                << " as inherited from omp_num_threads.";
        mkl_set_num_threads(FLAGS_caffe2_omp_num_threads);
      }

      // Override omp_num_threads if mkl_num_threads is set.
      if (FLAGS_caffe2_mkl_num_threads > 0) {
        VLOG(1) << "Setting mkl_num_threads to " << FLAGS_caffe2_mkl_num_threads;
        mkl_set_num_threads(FLAGS_caffe2_mkl_num_threads);
      }
      VLOG(1) << "Caffe2 running with " << mkl_get_max_threads() << " MKL threads";
      return true;
    */
}


#[cfg(caffe2_use_mkl)]
register_caffe2_init_function!{
    Caffe2SetMKLThreads,
    &Caffe2SetMKLThreads,
    "Set MKL threads."
}
