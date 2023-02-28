crate::ix!();

//-------------------------------------------[.cpp/pytorch/aten/src/ATen/native/quantized/cpu/qnnpack/bench/tanh.cc]

pub fn tanh_q8(state: &mut BenchmarkState)  {
    
    todo!();
        /*
            const usize batchSize = static_cast<usize>(state.range(0));
      const usize channels = static_cast<usize>(state.range(1));

      std::random_device randomDevice;
      auto rng = std::mt19937(randomDevice());
      auto u8rng = std::bind(std::uniform_int_distribution<u8>(), rng);

      std::vector<u8> input(batchSize * channels);
      std::vector<u8> output(batchSize * channels);
      std::generate(input.begin(), input.end(), std::ref(u8rng));
      std::fill(output.begin(), output.end(), 0xA5);

      pytorch_qnnp_status status = pytorch_qnnp_initialize();
      if (status != pytorch_qnnp_status_success) {
        state.SkipWithError("failed to initialize QNNPACK");
      }

      pytorch_qnnp_operator_t tanhOperator = nullptr;
      status = pytorch_qnnp_create_tanh_nc_q8(
          channels,
          127 /* input zero point */,
          1.0f /* input scale */,
          0 /* output zero point */,
          1.0f / 256.0f /* output scale */,
          0 /* output min */,
          255 /* output max */,
          0 /* flags */,
          &tanhOperator);
      if (status != pytorch_qnnp_status_success || tanhOperator == nullptr) {
        state.SkipWithError("failed to create TanH operator");
      }

      status = pytorch_qnnp_setup_tanh_nc_q8(
          tanhOperator,
          batchSize,
          input.data(),
          channels /* input:stride */,
          output.data(),
          channels /* output:stride */);
      if (status != pytorch_qnnp_status_success) {
        state.SkipWithError("failed to setup TanH operator");
      }

      for (auto _ : state) {
        status =
            pytorch_qnnp_run_operator(tanhOperator, nullptr /* thread pool */);
        if (status != pytorch_qnnp_status_success) {
          state.SkipWithError("failed to run TanH operator");
        }
      }

      const usize itemsPerIteration = batchSize * channels;
      state.SetItemsProcessed(
          i64(state.iterations()) * i64(itemsPerIteration));

      const usize bytesPerIteration = 2 * itemsPerIteration * sizeof(u8);
      state.SetBytesProcessed(
          i64(state.iterations()) * i64(bytesPerIteration));

      status = pytorch_qnnp_delete_operator(tanhOperator);
      if (status != pytorch_qnnp_status_success) {
        state.SkipWithError("failed to delete TanH operator");
      }
        */
}

pub fn characteristic_arguments(b: *mut Benchmark)  {
    
    todo!();
        /*
            b->ArgNames({"N", "C"});

      i32 c = 16;
      for (i32 n = 224; n >= 7; n /= 2) {
        b->Args({n * n, c});
        c *= 2;
      }
        */
}

lazy_static!{
    /*
    BENCHMARK(tanh_q8)->Apply(CharacteristicArguments);

    #ifndef PYTORCH_QNNPACK_BENCHMARK_NO_MAIN
    BENCHMARK_MAIN();
    #endif
    */
}

