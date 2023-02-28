crate::ix!();

//-------------------------------------------[.cpp/pytorch/aten/src/ATen/native/NNPACK.cpp]


#[cfg(not(target_feature = "nnpack"))]
pub fn nnpack_available() -> bool {
    
    todo!();
        /*
            return false;
        */
}

#[cfg(target_feature = "nnpack")]
pub fn init_nnpack() -> bool {
    
    todo!();
        /*
            static once_flag once_;
      static bool nnpack_successfully_initialized_ = false;

      call_once(once_, []() {
        const nnp_status nnpack_status = nnp_initialize();
        nnpack_successfully_initialized_ = (nnp_status_success == nnpack_status);

        if (nnpack_status != nnp_status_success) {
          if (nnpack_status == nnp_status_out_of_memory) {
            LOG(WARNING) << "Could not initialize NNPACK! Reason: Out of memory.";
          } else if (nnpack_status == nnp_status_unsupported_hardware) {
            LOG(WARNING) << "Could not initialize NNPACK! Reason: Unsupported hardware.";
          } else {
            LOG(WARNING) << "Could not initialize NNPACK! Reason: Unknown error!";
          }
        }
      });

      return nnpack_successfully_initialized_;
        */
}

#[cfg(target_feature = "nnpack")]
pub fn nnpack_threadpool() -> threadpool::ThreadPool {
    
    todo!();
        /*
            #ifdef C10_MOBILE
      return pthreadpool_();
    #else
      static pthreadpool_t nnpack_threadpool_ = nullptr;
      static bool called_nnpack_threadpool_ = false;

      if (!called_nnpack_threadpool_) {
        called_nnpack_threadpool_ = true;

    #ifdef INTRA_OP_PARALLEL
        const u32 threads = get_num_threads();
    #else
        const u32 threads = thread::hardware_concurrency();
    #endif

        nnpack_threadpool_ = pthreadpool_create(threads);
        if (!nnpack_threadpool_) {
          LOG(WARNING) << "Failed to initialize pthreadpool! Running NNPACK in single-threaded mode.";
        }
      }

      return nnpack_threadpool_;
    #endif
        */
}


#[cfg(target_feature = "nnpack")]
pub fn nnpack_available() -> bool {
    
    todo!();
        /*
            return init_nnpack();
        */
}

/**
  | Make thread_local for safety in cases
  | where we have multiple threads running
  | Convs at once
  |
  */
#[cfg(target_feature = "nnpack")]
lazy_static!{
    /*
    static thread_local void* workspace = nullptr;
        static thread_local usize workspace_size = 0;
    */
}

#[cfg(target_feature = "nnpack")]
#[inline] pub fn deallocate_workspace()  {
    
    todo!();
        /*
            if (workspace) {
        free(workspace);
        workspace = nullptr;
      }
        */
}


#[cfg(target_feature = "nnpack")]
#[inline] pub fn allocate_workspace()  {
    
    todo!();
        /*
            if (workspace) {
        deallocate_workspace();
      }

      // NNPack has alignment requirements
      constexpr usize nnpack_memory_alignment_boundary = 64;

      // Won't work on Windows, but NNPACK doesn't support Windows either
      posix_memalign(&workspace, nnpack_memory_alignment_boundary, workspace_size);
        */
}


#[cfg(target_feature = "nnpack")]
pub fn nnpack_spatial_convolution(
        input:    &Tensor,
        weight:   &Tensor,
        bias_opt: &Option<Tensor>,
        padding:  &[i32],
        stride:   &[i32]) -> Tensor {
    
    todo!();
        /*
            // See [Note: hacky wrapper removal for optional tensor]
      MaybeOwned<Tensor> bias_maybe_owned = borrow_from_optional_tensor(bias_opt);
      const Tensor& bias = *bias_maybe_owned;

      Tensor output = empty(
          conv_output_size(input.sizes(), weight.sizes(), padding, stride),
          input.options());

      // Our input Tensor must be in the form N,C,H,W
      if (input.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolutionOutput expects 4D input Tensor N,C,H,W");
      }
      // Our weight Tensor must be in the form oC,iC,kH,kW
      if (weight.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolutionOutput expects 4D weight Tensor oC,iC,kH,kW");
      }
      // Our output Tensor must be in the form N,oC,oH,oW
      if (output.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolutionOutput expects 4D output Tensor N,oC,oH,oW");
      }

      // Some basic shape checking, not comprehensive
      if (input.size(1) != weight.size(1)) {
        stringstream err;
        err << "Mismatch between number of input channels in input Tensor ("
            << input.size(1) << ") and weight Tensor (" << weight.size(1)
            << ") in NNPack convolutionOutput";
        throw runtime_error(err.str());
      }
      if (weight.size(0) != output.size(1)) {
        stringstream err;
        err << "Mismatch between number of output channels in weight Tensor ("
            << weight.size(0) << ") and output Tensor (" << output.size(1)
            << ") in NNPack convolutionOutput";
        throw runtime_error(err.str());
      }
      if (input.size(0) != output.size(0)) {
        stringstream err;
        err << "Mismatch between batch size in input Tensor (" << input.size(0)
            << ") and output Tensor (" << output.size(0)
            << ") in NNPack convolutionOutput";
        throw runtime_error(err.str());
      }

      // All Tensors must be float Tensors
      if (input.device().type() != kCPU || input.scalar_type() != kFloat ||
          weight.device().type() != kCPU || weight.scalar_type() != kFloat ||
          output.device().type() != kCPU || output.scalar_type() != kFloat ||
          (bias.defined() && (bias.device().type() != kCPU || bias.scalar_type() != kFloat))) {
        throw runtime_error(
            "Mismatched Tensor types in NNPack convolutionOutput");
      }

      const auto algorithm = nnp_convolution_algorithm_auto;
      const usize input_channels = input.size(1);
      const usize output_channels = weight.size(0);
      const struct nnp_size input_size = {
          .width = (usize)input.size(3),
          .height = (usize)input.size(2),
      };
      const struct nnp_padding input_padding = {
          .top = (usize)padding[0],
          .right = (usize)padding[1],
          .bottom = (usize)padding[0],
          .left = (usize)padding[1],
      };
      const struct nnp_size kernel_size = {
          .width = (usize)weight.size(3),
          .height = (usize)weight.size(2),
      };
      const struct nnp_size output_size = {
          .width = (usize)output.size(3),
          .height = (usize)output.size(2),
      };
      const nnp_size output_subsample = {
          .width = stride[1],
          .height = stride[0],
      };

      const auto input_ = input.contiguous();
      const auto weight_ = weight.contiguous();
      // If we don't have a defined bias Tensor, we need to create one filled with zeroes
      const auto bias_ = bias.defined() ? bias.contiguous() : zeros({weight.size(0)}, input.options());

      const auto compute = [&](const usize batch_size) -> nnp_status {
        if ((batch_size == 1) || (output_subsample.width != 1) || (output_subsample.height != 1)) {
          const usize input_size_per_batch = input_channels * input_size.width * input_size.height;
          const usize output_size_per_batch = output_channels * output_size.width * output_size.height;

          for (usize batch = 0u; batch < batch_size; ++batch) {
            const nnp_status status = nnp_convolution_inference(
                algorithm,
                nnp_convolution_transform_strategy_compute,
                input_channels,
                output_channels,
                input_size,
                input_padding,
                kernel_size,
                output_subsample,
                input_.data_ptr<float>() + batch * input_size_per_batch,
                weight_.data_ptr<float>(),
                bias_.data_ptr<float>(),
                output.data_ptr<float>() + batch * output_size_per_batch,
                workspace,
                &workspace_size,
                nnp_activation_identity,
                nullptr,
                nnpack_threadpool(),
                nullptr );

            if (nnp_status_success != status) {
              return status;
            }
          }

          return nnp_status_success;
        }
        else {
          return nnp_convolution_output(
            algorithm,
            batch_size,
            input_channels,
            output_channels,
            input_size,
            input_padding,
            kernel_size,
            input_.data_ptr<float>(),
            weight_.data_ptr<float>(),
            bias_.data_ptr<float>(),
            output.data_ptr<float>(),
            workspace,
            &workspace_size,
            nnp_activation_identity,
            nullptr,
            nnpack_threadpool(),
            nullptr );
        }
      };

      const usize batch_size = input.size(0);

      auto size_and_allocate_ws = [&]() {
        // Run a single pass to get the size of memory workspace buffer
        const auto status = compute(batch_size);
        if (status != nnp_status_success) {
          throw runtime_error("NNPACK SpatialConvolution_updateOutput failed");
        }
        allocate_workspace();
      };

      // If no workspace created yet, allocate it
      if (workspace == nullptr) {
        size_and_allocate_ws();
      }

      // Try to run with the newly created, or existing workspace
      auto status = compute(batch_size);

      if (status == nnp_status_insufficient_buffer) {
        // Need to reallocate the workspace
        deallocate_workspace();
        size_and_allocate_ws();

        // Try one more time
        status = compute(batch_size);
      }

      if (status != nnp_status_success) {
        throw runtime_error("NNPACK SpatialConvolution_updateOutput failed");
      }

      return output;
        */
}


#[cfg(target_feature = "nnpack")]
pub fn nnpack_spatial_convolution_backward_input(
        input:       &Tensor,
        grad_output: &Tensor,
        weight:      &Tensor,
        padding:     &[i32]) -> Tensor {
    
    todo!();
        /*
            Tensor gradInput = empty(input.sizes(), input.options());

      // Our input and gradInput Tensors must be in the form N,C,H,W
      if (input.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolution updateGradInput expects 4D input Tensor N,C,H,W");
      }
      if (gradInput.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolution updateGradInput expects 4D gradInput Tensor N,C,H,W");
      }
      // Our weight Tensor must be in the form oC,iC,kH,kW
      if (weight.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolution updateGradInput expects 4D weight Tensor oC,iC,kH,kW");
      }
      // Our gradOutput Tensor must be in the form N,oC,oH,oW
      if (gradOutput.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolution updateGradInput expects 4D gradOutput Tensor N,oC,oH,oW");
      }

      // Some basic shape checking, not comprehensive
      if (!input.sizes().equals(gradInput.sizes())) {
        stringstream err;
        err << "Mismatch between input size (" << input.sizes()
            << ") and gradInput size (" << gradInput.sizes()
            << ") in NNPack convolution updateGradInput";
        throw runtime_error(err.str());
      }
      if (input.size(1) != weight.size(1)) {
        stringstream err;
        err << "Mismatch between number of input channels in input Tensor ("
            << input.size(1) << ") and weight Tensor (" << weight.size(1)
            << ") in NNPack convolution updateGradInput";
        throw runtime_error(err.str());
      }
      if (weight.size(0) != gradOutput.size(1)) {
        stringstream err;
        err << "Mismatch between number of output channels in weight Tensor ("
            << weight.size(0) << ") and gradOutput Tensor (" << gradOutput.size(1)
            << ") in NNPack convolution updateGradInput";
        throw runtime_error(err.str());
      }
      if (input.size(0) != gradOutput.size(0)) {
        stringstream err;
        err << "Mismatch between batch size in input Tensor (" << input.size(0)
            << ") and gradOutput Tensor (" << gradOutput.size(0)
            << ") in NNPack convolution updateGradInput";
        throw runtime_error(err.str());
      }

      // Setup parameters for the NNPACK convolution input gradient call

      // Use the default algorithm
      auto algorithm = nnp_convolution_algorithm_auto;

      const usize batch_size = input.size(0);
      const usize input_channels = input.size(1);
      const usize output_channels = weight.size(0);
      const struct nnp_size input_size = {.width = (usize)input.size(3),
                                          .height = (usize)input.size(2)};
      const struct nnp_padding input_padding = {.top = (usize)padding[0],
                                                .right = (usize)padding[1],
                                                .bottom = (usize)padding[0],
                                                .left = (usize)padding[1]};
      const struct nnp_size kernel_size = {.width = (usize)weight.size(3),
                                           .height = (usize)weight.size(2)};

      auto run = [&]() -> nnp_status {
        return nnp_convolution_input_gradient(
            algorithm,
            batch_size,
            input_channels,
            output_channels,
            input_size,
            input_padding,
            kernel_size,
            gradOutput.data_ptr<float>(),
            weight.data_ptr<float>(),
            gradInput.data_ptr<float>(),
            workspace, // workspace_buffer
            &workspace_size, // workspace_size
            nnp_activation_identity,
            nullptr, // activation_parameters
            nnpack_threadpool(),
            nullptr // profile
        );
      };

      auto size_and_allocate_ws = [&]() {
        // Run a single pass to get the size of memory workspace buffer
        auto status = run();
        if (status != nnp_status_success) {
          throw runtime_error(
              "NNPACK SpatialConvolution_updateGradInput failed");
        }
        allocate_workspace();
      };

      // If no workspace created yet, allocate it
      if (workspace == nullptr) {
        size_and_allocate_ws();
      }

      // Try to run with the newly created, or existing workspace
      auto status = run();

      if (status == nnp_status_insufficient_buffer) {
        // Need to reallocate the workspace
        deallocate_workspace();
        size_and_allocate_ws();

        // Try one more time
        status = run();
      }

      if (status != nnp_status_success) {
        throw runtime_error(
            "NNPACK SpatialConvolution_updateGradInput failed");
      }

      return gradInput;
        */
}


#[cfg(target_feature = "nnpack")]
pub fn nnpack_spatial_convolution_backward_weight(
        input:       &Tensor,
        weight_size: &[i32],
        grad_output: &Tensor,
        padding:     &[i32]) -> Tensor {
    
    todo!();
        /*
            Tensor gradWeight = empty(weight_size, input.options());

      // Our input and gradInput Tensors must be in the form N,C,H,W
      if (input.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolutionOutput expects 4D input Tensor N,C,H,W");
      }
      // Our gradWeight Tensor must be in the form oC,iC,kH,kW
      if (gradWeight.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolutionOutput expects 4D gradWeight Tensor oC,iC,kH,kW");
      }
      // Our weight Tensor must be in the form N,oC,oH,oW
      if (gradOutput.ndimension() != 4) {
        throw runtime_error(
            "NNPack convolutionOutput expects 4D gradOutput Tensor N,oC,oH,oW");
      }

      // Some basic shape checking, not comprehensive
      if (input.size(1) != gradWeight.size(1)) {
        stringstream err;
        err << "Mismatch between number of input channels in input Tensor ("
            << input.size(1) << ") and gradWeight Tensor (" << gradWeight.size(1)
            << ") in NNPack convolution accGradWeight";
        throw runtime_error(err.str());
      }
      if (gradWeight.size(0) != gradOutput.size(1)) {
        stringstream err;
        err << "Mismatch between number of output channels in gradWeight Tensor ("
            << gradWeight.size(0) << ") and gradOutput Tensor ("
            << gradOutput.size(1) << ") in NNPack convolution accGradWeight";
        throw runtime_error(err.str());
      }
      if (input.size(0) != gradOutput.size(0)) {
        stringstream err;
        err << "Mismatch between batch size in input Tensor (" << input.size(0)
            << ") and gradOutput Tensor (" << gradOutput.size(0)
            << ") in NNPack convolution accGradWeight";
        throw runtime_error(err.str());
      }

      // Setup parameters for the NNPACK convolution kernel gradient call

      // Use the default algorithm
      auto algorithm = nnp_convolution_algorithm_auto;

      const usize batch_size = input.size(0);
      const usize input_channels = input.size(1);
      const usize output_channels = gradWeight.size(0);
      const struct nnp_size input_size = {.width = (usize)input.size(3),
                                          .height = (usize)input.size(2)};
      const struct nnp_padding input_padding = {.top = (usize)padding[0],
                                                .right = (usize)padding[1],
                                                .bottom = (usize)padding[0],
                                                .left = (usize)padding[1]};
      const struct nnp_size kernel_size = {.width = (usize)weight_size[3],
                                           .height = (usize)weight_size[2]};

      auto input_ = input.contiguous();
      auto run = [&]() -> nnp_status {
        return nnp_convolution_kernel_gradient(
            algorithm,
            batch_size,
            input_channels,
            output_channels,
            input_size,
            input_padding,
            kernel_size,
            input_.data_ptr<float>(),
            gradOutput.data_ptr<float>(),
            gradWeight.data_ptr<float>(),
            workspace, // workspace_buffer
            &workspace_size, // workspace_size
            nnp_activation_identity,
            nullptr, // activation_parameters
            nnpack_threadpool(),
            nullptr // profile
        );
      };

      auto size_and_allocate_ws = [&]() {
        // Run a single pass to get the size of memory workspace buffer
        auto status = run();
        if (status != nnp_status_success) {
          throw runtime_error(
              "NNPACK SpatialConvolution_accGradWeight failed");
        }
        allocate_workspace();
      };

      // If no workspace created yet, allocate it
      if (workspace == nullptr) {
        size_and_allocate_ws();
      }

      // Try to run with the newly created, or existing workspace
      auto status = run();

      if (status == nnp_status_insufficient_buffer) {
        // Need to reallocate the workspace
        deallocate_workspace();
        size_and_allocate_ws();

        // Try one more time
        status = run();
      }

      if (status != nnp_status_success) {
        throw runtime_error("NNPACK SpatialConvolution_accGradWeight failed");
      }

      return gradWeight;
        */
}


#[cfg(target_feature = "nnpack")]
pub fn nnpack_spatial_convolution_backward(
        input:       &Tensor,
        grad_output: &Tensor,
        weight:      &Tensor,
        padding:     &[i32],
        output_mask: [bool; 3]) -> (Tensor,Tensor,Tensor) {
    
    todo!();
        /*
            Tensor grad_input, grad_weight, grad_bias;
      if (output_mask[0]) {
        grad_input = _nnpack_spatial_convolution_backward_input(
            input, grad_output, weight, padding);
      }
      if (output_mask[1]) {
        grad_weight = _nnpack_spatial_convolution_backward_weight(
            input, weight.sizes(), grad_output, padding);
      }
      if (output_mask[2]) {
        grad_bias = grad_output.contiguous()
                        .view({grad_output.size(0), grad_output.size(1), -1})
                        .sum(0)
                        .sum(1);
      }

      return tuple<Tensor, Tensor, Tensor>{grad_input, grad_weight, grad_bias};
        */
}

