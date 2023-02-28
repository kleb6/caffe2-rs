// # vim: ft=none
crate::ix!();

//-------------------------------------------[.cpp/pytorch/aten/src/ATen/native/quantized/cpu/qnnpack/test/convolution-operator-tester.h]

pub fn create_convolution_op(
        conv_p:           &QnnPackConvParam,
        input_zero_point: u8) -> PyTorchQnnpOperator {
    
    todo!();
        /*
            pytorch_qnnp_operator_t convolution = nullptr;
      convolution =
          static_cast<pytorch_qnnp_operator_t>(calloc(1, sizeof(struct pytorch_qnnp_operator)));
      if (convolution == nullptr) {
        pytorch_qnnp_log_error(
            "failed to allocate %zu bytes for pytorch_qnnp_operator structure",
            sizeof(struct pytorch_qnnp_operator));
      }

      convolution->ukernel_type = conv_p.ukernel_type;
      convolution->groups = conv_p.groups;
      convolution->group_input_channels = conv_p.group_input_channels;
      convolution->kernel_height = conv_p.kernel_dims[1];
      convolution->kernel_width = conv_p.kernel_dims[0];
      convolution->stride_height = conv_p.stride_dims[1];
      convolution->stride_width = conv_p.stride_dims[0];
      convolution->dilation_height = conv_p.dilation[1];
      convolution->dilation_width = conv_p.dilation[0];
      convolution->input_padding_top = conv_p.padding[0];
      convolution->input_padding_left = conv_p.padding[1];
      convolution->input_padding_bottom = conv_p.padding[2];
      convolution->input_padding_right = conv_p.padding[3];

      const bool any_padding = (conv_p.padding[0]| conv_p.padding[1]
          |conv_p.padding[2] | conv_p.padding[3]) != 0;

      if (any_padding) {
        usize zero_size = 0, zero_offset = 0;
        if (conv_p.ukernel_type == pytorch_qnnp_ukernel_type_dwconv) {
          const u32 cr = pytorch_qnnp_params.q8dw9.cr;
          const usize group_stride = (conv_p.groups + (cr - 1)) & -cr;
          if (conv_p.groups >= 8) {
            zero_size = sizeof(u8) * group_stride;
            zero_offset = 0;
          } else {
            zero_size = sizeof(u8) * group_stride + 8;
            zero_offset = sizeof(u8) * 8;
          }
        } else if (conv_p.ukernel_type == pytorch_qnnp_ukernel_type_conv ||
            conv_p.ukernel_type == pytorch_qnnp_ukernel_type_gemm) {
          const usize group_input_channels = conv_p.group_input_channels;
          const u32 kr = pytorch_qnnp_params.q8conv.kr;
          const usize k_stride = (group_input_channels + (kr - 1)) & -kr;
          if (conv_p.group_input_channels >= 8) {
            zero_size = sizeof(u8) * k_stride;
            zero_offset = 0;
          } else {
            zero_size = sizeof(u8) * k_stride + 8;
            zero_offset = 8;
          }
        }

        void* zero_buffer = malloc(zero_size);
        if (zero_buffer == nullptr) {
          pytorch_qnnp_log_error(
              "failed to allocate bytes for zero padding:");
        }
        // Need to set to input zero point
        memset(zero_buffer, input_zero_point, zero_size);
        convolution->zero_buffer = zero_buffer;
        convolution->zero_pointer =
            (void*)((uintptr_t)zero_buffer + zero_offset);
      }

      if (conv_p.per_channel && conv_p.ukernel_type == pytorch_qnnp_ukernel_type_xzp_gemm) {
        pytorch_qnnp_log_error(
            "Per channel quantized weights are not supported for XZP kernels");
      }
      return convolution;
        */
}

pub struct ConvolutionOperatorTester {
    padding_top:           u32, // default = { 0 }
    padding_right:         u32, // default = { 0 }
    padding_bottom:        u32, // default = { 0 }
    padding_left:          u32, // default = { 0 }
    input_height:          usize, // default = { 1 }
    input_width:           usize, // default = { 1 }
    groups:                u32, // default = { 1 }
    group_input_channels:  usize, // default = { 1 }
    input_pixel_stride:    usize, // default = { 0 }
    group_output_channels: usize, // default = { 1 }
    output_pixel_stride:   usize, // default = { 0 }
    batch_size:            usize, // default = { 1 }
    kernel_height:         u32, // default = { 1 }
    kernel_width:          u32, // default = { 1 }
    dilation_height:       u32, // default = { 1 }
    dilation_width:        u32, // default = { 1 }
    subsampling_height:    u32, // default = { 1 }
    subsampling_width:     u32, // default = { 1 }
    qmin:                  u8, // default = { 0 }
    qmax:                  u8, // default = { 255 }
    iterations:            usize, // default = { 1 }
    per_channel:           bool, // default = { false }
}

impl ConvolutionOperatorTester {

    
    #[inline] pub fn padding(&mut self, padding: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingTop_ = padding;
        this->paddingRight_ = padding;
        this->paddingBottom_ = padding;
        this->paddingLeft_ = padding;
        return *this;
        */
    }
    
    #[inline] pub fn padding(&mut self, 
        padding_height: u32,
        padding_width:  u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingTop_ = paddingHeight;
        this->paddingRight_ = paddingWidth;
        this->paddingBottom_ = paddingHeight;
        this->paddingLeft_ = paddingWidth;
        return *this;
        */
    }
    
    #[inline] pub fn padding_height(&mut self, padding_height: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingTop_ = paddingHeight;
        this->paddingBottom_ = paddingHeight;
        return *this;
        */
    }
    
    #[inline] pub fn padding_width(&mut self, padding_width: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingRight_ = paddingWidth;
        this->paddingLeft_ = paddingWidth;
        return *this;
        */
    }
    
    #[inline] pub fn padding_top(&mut self, padding_top: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingTop_ = paddingTop;
        return *this;
        */
    }
    
    #[inline] pub fn padding_top(&self) -> u32 {
        
        todo!();
        /*
            return this->paddingTop_;
        */
    }
    
    #[inline] pub fn padding_right(&mut self, padding_right: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingRight_ = paddingRight;
        return *this;
        */
    }
    
    #[inline] pub fn padding_right(&self) -> u32 {
        
        todo!();
        /*
            return this->paddingRight_;
        */
    }
    
    #[inline] pub fn padding_bottom(&mut self, padding_bottom: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingBottom_ = paddingBottom;
        return *this;
        */
    }
    
    #[inline] pub fn padding_bottom(&self) -> u32 {
        
        todo!();
        /*
            return this->paddingBottom_;
        */
    }
    
    #[inline] pub fn padding_left(&mut self, padding_left: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->paddingLeft_ = paddingLeft;
        return *this;
        */
    }
    
    #[inline] pub fn padding_left(&self) -> u32 {
        
        todo!();
        /*
            return this->paddingLeft_;
        */
    }
    
    #[inline] pub fn input_size(&mut self, 
        input_height: u32,
        input_width:  u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(inputHeight >= 1);
        assert(inputWidth >= 1);
        this->inputHeight_ = inputHeight;
        this->inputWidth_ = inputWidth;
        return *this;
        */
    }
    
    #[inline] pub fn input_height(&mut self, input_height: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(inputHeight >= 1);
        this->inputHeight_ = inputHeight;
        return *this;
        */
    }
    
    #[inline] pub fn input_height(&self) -> u32 {
        
        todo!();
        /*
            return this->inputHeight_;
        */
    }
    
    #[inline] pub fn input_width(&mut self, input_width: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(inputWidth >= 1);
        this->inputWidth_ = inputWidth;
        return *this;
        */
    }
    
    #[inline] pub fn input_width(&self) -> u32 {
        
        todo!();
        /*
            return this->inputWidth_;
        */
    }
    
    #[inline] pub fn groups(&mut self, groups: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(groups >= 1);
        this->groups_ = groups;
        return *this;
        */
    }
    
    #[inline] pub fn groups(&self) -> u32 {
        
        todo!();
        /*
            return this->groups_;
        */
    }
    
    #[inline] pub fn group_input_channels(&mut self, group_input_channels: usize) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(groupInputChannels >= 1);
        this->groupInputChannels_ = groupInputChannels;
        return *this;
        */
    }
    
    #[inline] pub fn group_input_channels(&self) -> usize {
        
        todo!();
        /*
            return this->groupInputChannels_;
        */
    }
    
    #[inline] pub fn per_channel(&mut self, per_channel: bool) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->per_channel_ = per_channel;
        return *this;
        */
    }
    
    #[inline] pub fn per_channel(&self) -> bool {
        
        todo!();
        /*
            return this->per_channel_;
        */
    }
    
    #[inline] pub fn group_output_channels(&mut self, group_output_channels: usize) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(groupOutputChannels >= 1);
        this->groupOutputChannels_ = groupOutputChannels;
        return *this;
        */
    }
    
    #[inline] pub fn group_output_channels(&self) -> usize {
        
        todo!();
        /*
            return this->groupOutputChannels_;
        */
    }
    
    #[inline] pub fn batch_size(&mut self, batch_size: usize) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->batchSize_ = batchSize;
        return *this;
        */
    }
    
    #[inline] pub fn batch_size(&self) -> usize {
        
        todo!();
        /*
            return this->batchSize_;
        */
    }
    
    #[inline] pub fn kernel_size(&mut self, kernel_size: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(kernelSize >= 1);
        this->kernelHeight_ = kernelSize;
        this->kernelWidth_ = kernelSize;
        return *this;
        */
    }
    
    #[inline] pub fn kernel_size(&mut self, 
        kernel_height: u32,
        kernel_width:  u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(kernelHeight >= 1);
        assert(kernelWidth >= 1);
        this->kernelHeight_ = kernelHeight;
        this->kernelWidth_ = kernelWidth;
        return *this;
        */
    }
    
    #[inline] pub fn kernel_height(&mut self, kernel_height: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(kernelHeight >= 1);
        this->kernelHeight_ = kernelHeight;
        return *this;
        */
    }
    
    #[inline] pub fn kernel_height(&self) -> u32 {
        
        todo!();
        /*
            return this->kernelHeight_;
        */
    }
    
    #[inline] pub fn kernel_width(&mut self, kernel_width: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(kernelWidth >= 1);
        this->kernelWidth_ = kernelWidth;
        return *this;
        */
    }
    
    #[inline] pub fn kernel_width(&self) -> u32 {
        
        todo!();
        /*
            return this->kernelWidth_;
        */
    }
    
    #[inline] pub fn dilation(&mut self, dilation: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(dilation >= 1);
        this->dilationHeight_ = dilation;
        this->dilationWidth_ = dilation;
        return *this;
        */
    }
    
    #[inline] pub fn dilation(&mut self, 
        dilation_height: u32,
        dilation_width:  u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(dilationHeight >= 1);
        assert(dilationWidth >= 1);
        this->dilationHeight_ = dilationHeight;
        this->dilationWidth_ = dilationWidth;
        return *this;
        */
    }
    
    #[inline] pub fn dilation_height(&mut self, dilation_height: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(dilationHeight >= 1);
        this->dilationHeight_ = dilationHeight;
        return *this;
        */
    }
    
    #[inline] pub fn dilation_height(&self) -> u32 {
        
        todo!();
        /*
            return this->dilationHeight_;
        */
    }
    
    #[inline] pub fn dilation_width(&mut self, dilation_width: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(dilationWidth >= 1);
        this->dilationWidth_ = dilationWidth;
        return *this;
        */
    }
    
    #[inline] pub fn dilation_width(&self) -> u32 {
        
        todo!();
        /*
            return this->dilationWidth_;
        */
    }
    
    #[inline] pub fn subsampling(&mut self, subsampling: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(subsampling >= 1);
        this->subsamplingHeight_ = subsampling;
        this->subsamplingWidth_ = subsampling;
        return *this;
        */
    }
    
    #[inline] pub fn subsampling(&mut self, 
        subsampling_height: u32,
        subsampling_width:  u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(subsamplingHeight >= 1);
        assert(subsamplingWidth >= 1);
        this->subsamplingHeight_ = subsamplingHeight;
        this->subsamplingWidth_ = subsamplingWidth;
        return *this;
        */
    }
    
    #[inline] pub fn subsampling_height(&mut self, subsampling_height: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(subsamplingHeight >= 1);
        this->subsamplingHeight_ = subsamplingHeight;
        return *this;
        */
    }
    
    #[inline] pub fn subsampling_height(&self) -> u32 {
        
        todo!();
        /*
            return this->subsamplingHeight_;
        */
    }
    
    #[inline] pub fn subsampling_width(&mut self, subsampling_width: u32) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(subsamplingWidth >= 1);
        this->subsamplingWidth_ = subsamplingWidth;
        return *this;
        */
    }
    
    #[inline] pub fn subsampling_width(&self) -> u32 {
        
        todo!();
        /*
            return this->subsamplingWidth_;
        */
    }
    
    #[inline] pub fn input_pixel_stride(&mut self, input_pixel_stride: usize) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(inputPixelStride >= 1);
        this->inputPixelStride_ = inputPixelStride;
        return *this;
        */
    }
    
    #[inline] pub fn input_pixel_stride(&self) -> usize {
        
        todo!();
        /*
            if (this->inputPixelStride_ == 0) {
          return groupInputChannels() * groups();
        } else {
          assert(this->inputPixelStride_ >= groupInputChannels() * groups());
          return this->inputPixelStride_;
        }
        */
    }
    
    #[inline] pub fn output_pixel_stride(&mut self, output_pixel_stride: usize) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            assert(outputPixelStride >= 1);
        this->outputPixelStride_ = outputPixelStride;
        return *this;
        */
    }
    
    #[inline] pub fn output_pixel_stride(&self) -> usize {
        
        todo!();
        /*
            if (this->outputPixelStride_ == 0) {
          return groupOutputChannels() * groups();
        } else {
          assert(this->outputPixelStride_ >= groupOutputChannels() * groups());
          return this->outputPixelStride_;
        }
        */
    }
    
    #[inline] pub fn dilated_kernel_height(&self) -> u32 {
        
        todo!();
        /*
            return (kernelHeight() - 1) * dilationHeight() + 1;
        */
    }
    
    #[inline] pub fn dilated_kernel_width(&self) -> u32 {
        
        todo!();
        /*
            return (kernelWidth() - 1) * dilationWidth() + 1;
        */
    }
    
    #[inline] pub fn output_height(&self) -> usize {
        
        todo!();
        /*
            const usize paddedInputHeight =
            paddingTop() + inputHeight() + paddingBottom();
        if (paddedInputHeight <= dilatedKernelHeight()) {
          return 1;
        } else {
          return (paddedInputHeight - dilatedKernelHeight()) / subsamplingHeight() +
              1;
        }
        */
    }
    
    #[inline] pub fn output_width(&self) -> usize {
        
        todo!();
        /*
            const usize paddedInputWidth =
            paddingLeft() + inputWidth() + paddingRight();
        if (paddedInputWidth <= dilatedKernelWidth()) {
          return 1;
        } else {
          return (paddedInputWidth - dilatedKernelWidth()) / subsamplingWidth() + 1;
        }
        */
    }
    
    #[inline] pub fn qmin(&mut self, qmin: u8) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->qmin_ = qmin;
        return *this;
        */
    }
    
    #[inline] pub fn qmin(&self) -> u8 {
        
        todo!();
        /*
            return this->qmin_;
        */
    }
    
    #[inline] pub fn qmax(&mut self, qmax: u8) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->qmax_ = qmax;
        return *this;
        */
    }
    
    #[inline] pub fn qmax(&self) -> u8 {
        
        todo!();
        /*
            return this->qmax_;
        */
    }
    
    #[inline] pub fn iterations(&mut self, iterations: usize) -> &mut ConvolutionOperatorTester {
        
        todo!();
        /*
            this->iterations_ = iterations;
        return *this;
        */
    }
    
    #[inline] pub fn iterations(&self) -> usize {
        
        todo!();
        /*
            return this->iterations_;
        */
    }
    
    pub fn testq8(&self, mode: Mode)  {
        let mode: Mode = mode.unwrap_or(Mode_Static);

        todo!();
        /*
            random_device randomDevice;
        auto rng = mt19937(randomDevice());
        auto s32rng =
            bind(uniform_int_distribution<i32>(-10000, 10000), rng);
        auto u8rng = bind(uniform_int_distribution<u8>(), rng);
        auto f32rng =
            bind(uniform_real_distribution<float>(1, 5), rng);

        vector<u8> input(
            batchSize() *
                ((inputHeight() * inputWidth() - 1) * inputPixelStride() +
                 groups() * groupInputChannels()) +
            8);
        vector<u8> kernel(
            groups() * groupOutputChannels() * kernelHeight() * kernelWidth() *
            groupInputChannels());
        vector<i32> bias(groups() * groupOutputChannels());
        vector<u8> output(
            batchSize() *
            ((outputHeight() * outputWidth() - 1) * outputPixelStride() +
             groups() * groupOutputChannels()));
        vector<i32> accumulators(
            batchSize() * outputHeight() * outputWidth() * groups() *
            groupOutputChannels());

        const u8* inputPtr = input.data() + 8;
        const u8 inputZeroPoint = 127;
        // Make num zero points multiple of 8.
        // This is the least common denominator for SSE/ARM kernels we have.
        usize num_zero_points_padded =
          (groups() * groupOutputChannels() + 8);
        vector<u8> kernelZeroPoints(num_zero_points_padded, 127);

        for (usize iteration = 0; iteration < iterations(); iteration++) {
          generate(input.begin(), input.end(), ref(u8rng));
          generate(kernel.begin(), kernel.end(), ref(u8rng));
          generate(bias.begin(), bias.end(), ref(s32rng));
          if (per_channel()) {
            generate(kernelZeroPoints.begin(), kernelZeroPoints.end(), ref(u8rng));
          }
          fill(output.begin(), output.end(), 0xA5);
          fill(accumulators.begin(), accumulators.end(), 0);

          for (usize i = 0; i < batchSize(); i++) {
            for (usize oy = 0; oy < outputHeight(); oy++) {
              for (usize ox = 0; ox < outputWidth(); ox++) {
                for (usize g = 0; g < groups(); g++) {
                  for (usize oc = 0; oc < groupOutputChannels(); oc++) {
                    accumulators
                        [(((i * outputHeight() + oy) * outputWidth() + ox) *
                              groups() +
                          g) *
                             groupOutputChannels() +
                         oc] = bias[g * groupOutputChannels() + oc];
                  }
                }
              }
            }
          }
          for (usize i = 0; i < batchSize(); i++) {
            for (usize oy = 0; oy < outputHeight(); oy++) {
              for (usize ox = 0; ox < outputWidth(); ox++) {
                for (usize ky = 0; ky < kernelHeight(); ky++) {
                  const usize iy = oy * subsamplingHeight() +
                      ky * dilationHeight() - paddingTop();
                  if (iy < inputHeight()) {
                    for (usize kx = 0; kx < kernelWidth(); kx++) {
                      const usize ix = ox * subsamplingWidth() +
                          kx * dilationWidth() - paddingLeft();
                      if (ix < inputWidth()) {
                        for (usize g = 0; g < groups(); g++) {
                          for (usize oc = 0; oc < groupOutputChannels(); oc++) {
                            for (usize ic = 0; ic < groupInputChannels(); ic++) {
                              accumulators
                                  [(((i * outputHeight() + oy) * outputWidth() +
                                     ox) *
                                        groups() +
                                    g) *
                                       groupOutputChannels() +
                                   oc] +=
                                  (i32(inputPtr
                                               [((i * inputHeight() + iy) *
                                                     inputWidth() +
                                                 ix) *
                                                    inputPixelStride() +
                                                g * groupInputChannels() + ic]) -
                                   i32(inputZeroPoint)) *
                                  (i32(kernel
                                               [(((g * groupOutputChannels() + oc) *
                                                      kernelHeight() +
                                                  ky) *
                                                     kernelWidth() +
                                                 kx) *
                                                    groupInputChannels() +
                                                ic]) -
                                   i32(kernelZeroPoints[g* groupOutputChannels() + oc]));
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          // Create dummy min/max for empty inputs.
          // These are only used to compute scale and zero point,
          // and real callers will just pull those values from the model.
          const i32 accumulatorsMin = accumulators.empty()
              ? 0
              : *min_element(accumulators.cbegin(), accumulators.cend());
          const i32 accumulatorsMax = accumulators.empty()
              ? 900
              : *max_element(accumulators.cbegin(), accumulators.cend());

          const double outputScale =
              double(u32(accumulatorsMax - accumulatorsMin)) / 255.0;
          const u8 outputZeroPoint = u8(max(
              min(
                  lrint(
                      127.5 -
                      0.5 * double(accumulatorsMin + accumulatorsMax) /
                          outputScale),
                  long(u8::max)),
              long(u8::min)));

          ASSERT_EQ(pytorch_qnnp_status_success, pytorch_qnnp_initialize());
          vector<float> requantization_scales(num_zero_points_padded, 1.0 * 1.0 / outputScale);
          if (per_channel()) {
            auto scale_generator = [&]() -> float {return (f32rng()/outputScale);};
            generate(
                requantization_scales.begin(),
                requantization_scales.end(),
                ref(scale_generator));
          }

          switch(mode) {
            case Mode::Static:
            {
              pytorch_qnnp_operator_t convolution = nullptr;

              ASSERT_EQ(
                  pytorch_qnnp_status_success,
                  pytorch_qnnp_create_convolution2d_nhwc_q8(
                      paddingTop(),
                      paddingRight(),
                      paddingBottom(),
                      paddingLeft(),
                      kernelHeight(),
                      kernelWidth(),
                      subsamplingHeight(),
                      subsamplingWidth(),
                      dilationHeight(),
                      dilationWidth(),
                      groups(),
                      groupInputChannels(),
                      groupOutputChannels(),
                      inputZeroPoint,
                      kernelZeroPoints.data(),
                      kernel.data(),
                      bias.data(),
                      outputZeroPoint,
                      qmin(),
                      qmax(),
                      0,
                      requantization_scales.data(),
                      per_channel(),
                      &convolution));

              ASSERT_EQ(
                  pytorch_qnnp_status_success,
                  pytorch_qnnp_setup_convolution2d_nhwc_q8(
                      convolution,
                      batchSize(),
                      inputHeight(),
                      inputWidth(),
                      inputPtr,
                      inputPixelStride(),
                      output.data(),
                      outputPixelStride(),
                      nullptr /* thread pool */));

              ASSERT_EQ(
                  pytorch_qnnp_status_success,
                  pytorch_qnnp_run_operator(convolution, nullptr /* thread pool */));

              ASSERT_EQ(
                  pytorch_qnnp_status_success,
                  pytorch_qnnp_delete_operator(convolution));
              convolution = nullptr;
            }
            break;

            case Mode::Runtime:
            {
              qnnpack::conv_param_t conv_p(
                {kernelWidth(), kernelHeight()},
                {subsamplingWidth(), subsamplingHeight()},
                {dilationWidth(), dilationHeight()},
                {paddingTop(), paddingLeft(), paddingBottom(), paddingRight()},
                /*adjustment_dims=*/{0, 0},
                groups(),
                groupInputChannels() * groups(),
                groupOutputChannels() * groups(),
                /*transpose=*/false,
                per_channel());
              auto conv_op = create_convolution_op(conv_p, inputZeroPoint);
              auto packW = unique_ptr<qnnpack::PrePackConvWeights>(
                  new qnnpack::PrePackConvWeights(
                      conv_p,
                      kernelZeroPoints.data(),
                      kernel.data(),
                      bias.data()));
              const pytorch_qnnp_status runStatus = qnnpack::qnnpackConv(
                  conv_p,
                  conv_op,
                  packW->getPackedWeights(),
                  batchSize(),
                  inputHeight(),
                  inputWidth(),
                  inputZeroPoint,
                  inputPtr,
                  kernelZeroPoints.data(),
                  requantization_scales.data(),
                  outputZeroPoint,
                  qmin(),
                  qmax(),
                  output.data(),
                  nullptr);
              ASSERT_EQ(pytorch_qnnp_status_success, runStatus);
              ASSERT_EQ(
                  pytorch_qnnp_status_success,
                  pytorch_qnnp_delete_operator(conv_op));
            }
            break;

            default:
              // Undefined!
              ASSERT_TRUE(false);
          }

          for (usize i = 0; i < batchSize(); i++) {
            for (usize y = 0; y < outputHeight(); y++) {
              for (usize x = 0; x < outputWidth(); x++) {
                for (usize g = 0; g < groups(); g++) {
                  for (usize c = 0; c < groupOutputChannels(); c++) {
                    const double scaledAccumulator =
                        accumulators
                            [(((i * outputHeight() + y) * outputWidth() + x) *
                                  groups() +
                              g) *
                                 groupOutputChannels() +
                             c] *
                        requantization_scales[g * groupOutputChannels() + c];
                    const double clampedAccumulator = max(
                        min(
                            scaledAccumulator,
                            double(qmax()) - double(outputZeroPoint)),
                        double(qmin()) - double(outputZeroPoint));
                    ASSERT_NEAR(
                        clampedAccumulator,
                        (i32(
                             output
                                 [((i * outputHeight() + y) * outputWidth() + x) *
                                      outputPixelStride() +
                                  g * groupOutputChannels() + c]) -
                         outputZeroPoint),
                        0.9)
                        << "(x, y) = (" << x << ", " << y << "), group = " << g
                        << ", channel = " << c;
                  }
                }
              }
            }
          }
        }
        */
    }
}
