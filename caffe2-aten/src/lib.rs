#![feature(test)]
extern crate test;

#[macro_use]
extern crate derive_error;

#[macro_use] mod imports; use imports::*;

x!{accumulate_type}
x!{adaption}
x!{alias_info}
x!{autocast_mode}
//x!{backend_select_fallback_kernel}
//x!{batched_fallback}
//x!{batched_tensor_impl}
//x!{batching_registrations}
//x!{benchmarks_quantize_per_channel}
//x!{benchmarks_stateful_conv1d}
//x!{benchmarks_tensor_add}
//x!{blob}
//x!{boxing_impl_}
//x!{boxing_impl__kernel_function_legacy_test}
//x!{boxing_impl__kernel_function_test}
//x!{boxing_impl__kernel_lambda_legacy_test}
//x!{boxing_impl__kernel_lambda_test}
//x!{boxing_impl__kernel_stackbased_test}
//x!{boxing_impl__make_boxed_from_unboxed_functor}
//x!{boxing_impl__make_boxed_from_unboxed_functor_test}
//x!{boxing_impl__test_helpers}
//x!{boxing_impl__wrap_function_into_functor}
//x!{boxing_impl__wrap_function_into_runtime_functor}
//x!{boxing_kernel_function}
//x!{boxing_kernel_function_impl}
//x!{boxing_kernel_function_test}
//x!{builtin_function}
//x!{conjugate_fallback}
//x!{context}
//x!{cpu_apply_utils}
//x!{cpu_fixed_allocator}
//x!{cpu_flush_denormal}
//x!{cpu_generator_impl}
//x!{cpu_vec_vec256}
//x!{cpu_vec_vec256_functional}
//x!{cpu_vec_vec256_missing_vld1_neon}
//x!{cpu_vec_vec256_missing_vst1_neon}
//x!{cpu_vec_vec256_vec256_base}
//x!{cpu_vec_vec256_vec256_bfloat16}
//x!{cpu_vec_vec256_vec256_complex_double}
//x!{cpu_vec_vec256_vec256_complex_float}
//x!{cpu_vec_vec256_vec256_double}
//x!{cpu_vec_vec256_vec256_float}
//x!{cpu_vec_vec256_vec256_float_neon}
//x!{cpu_vec_vec256_vec256_int}
//x!{cpu_vec_vec256_vec256_qint}
//x!{cpu_vec_vec256_vsx_vec256_common_vsx}
//x!{cpu_vec_vec256_vsx_vec256_complex_double_vsx}
//x!{cpu_vec_vec256_vsx_vec256_complex_float_vsx}
//x!{cpu_vec_vec256_vsx_vec256_double_vsx}
//x!{cpu_vec_vec256_vsx_vec256_float_vsx}
//x!{cpu_vec_vec256_vsx_vec256_int16_vsx}
//x!{cpu_vec_vec256_vsx_vec256_int32_vsx}
//x!{cpu_vec_vec256_vsx_vec256_int64_vsx}
//x!{cpu_vec_vec256_vsx_vec256_qint32_vsx}
//x!{cpu_vec_vec256_vsx_vec256_qint8_vsx}
//x!{cpu_vec_vec256_vsx_vec256_quint8_vsx}
//x!{cpu_vec_vec256_vsx_vsx_helpers}
//x!{cpu_vml}
//x!{cuda_cu_sparse_handle_pool}
//x!{cuda_cublas_handle_pool}
//x!{cuda_cuda_blas}
//x!{cuda_cuda_context}
//x!{cuda_cuda_device}
//x!{cuda_cuda_event}
//x!{cuda_cuda_generator_impl}
//x!{cuda_cuda_graph}
//x!{cuda_cuda_solver}
//x!{cuda_cuda_utils}
//x!{cuda_cusolver_dn_handle_pool}
//x!{cuda_detail_cuda_hooks}
//x!{cuda_detail_device_thread_handles}
//x!{cuda_detail_kernel_utils}
//x!{cuda_detail_lazynvrtc}
//x!{cuda_exceptions}
//x!{cuda_generator_impl}
//x!{cuda_legacy_thf_unctionscuda}
//x!{cuda_nvrtc_stub_a_tennvrtc}
//x!{cuda_pinned_memory_allocator}
//x!{cudnn_autocast_rnn}
//x!{cudnn_descriptors}
//x!{cudnn_handle}
//x!{cudnn_types}
//x!{cudnn_utils}
//x!{custom_class}
//x!{deprecated_type_properties}
//x!{deprecated_type_properties_registry}
//x!{detail_cpu_guard_impl}
//x!{detail_cuda_hooks_interface}
//x!{detail_function_traits}
//x!{detail_hip_hooks_interface}
//x!{detail_meta_guard_impl}
//x!{device_guard}
//x!{dict}
//x!{dict_inl}
//x!{dimname}
//x!{dispatch}
//x!{dispatch_backend_fallback_test}
//x!{dispatch_cpp_signature}
//x!{dispatch_cpp_signature_test}
//x!{dispatch_dispatch_key_extractor}
//x!{dispatch_dispatcher}
//x!{dispatch_observed_operators}
//x!{dispatch_operator_entry}
//x!{dispatch_operator_options}
//x!{dispatch_registration_handleraii}
//x!{distributions_helper}
//x!{div_rtn}
//x!{dl_convertor}
//x!{dlpack}
//x!{dynamic_library}
//x!{expand_utils}
//x!{formatting}
//x!{function}
//x!{function_schema}
//x!{function_schema_inl}
//x!{functional}
//x!{generator}
//x!{grad_mode}
//x!{hip_impl__hip_allocator_masquerading_ascuda}
//x!{hip_impl__hip_caching_allocator_masquerading_ascuda}
//x!{hip_impl__hip_guard_impl_masquerading_ascuda}
//x!{hip_impl__hip_stream_masquerading_ascuda}
//x!{infer_size}
//x!{initial_tensor_options}
//x!{interned_strings}
//x!{interned_strings_class}
//x!{ivalue}
//x!{ivalue_inl}
//x!{jit_type}
//x!{jit_type_base}
//x!{legacy_thf_unctions_cpu}
//x!{legacy_thf_unctionscuda}
//x!{legacy_type_dispatch}
//x!{library}
//x!{list}
//x!{list_inl}
//x!{list_test}
//x!{matrix_ref}
//x!{memory_overlap}
//x!{metal_context}
//x!{miopen_descriptors}
//x!{miopen_exceptions}
//x!{miopen_handle}
//x!{miopen_types}
//x!{miopen_utils}
//x!{mkl_descriptors}
//x!{mkl_exceptions}
//x!{mkl_limits}
//x!{mt19937rng_engine}
//x!{named_registrations}
//x!{named_tensor}
//x!{named_tensor_utils}
//x!{native_activation}
//x!{native_adaptive_average_pooling}
//x!{native_adaptive_average_pooling3d}
//x!{native_adaptive_max_pooling2d}
//x!{native_adaptive_max_pooling3d}
//x!{native_adaptive_pooling}
//x!{native_affine_grid_generator}
//x!{native_ao_sparse_library}
//x!{native_ao_sparse_quantized_cpu_fbgemm_utils}
//x!{native_ao_sparse_quantized_cpu_packed_params}
//x!{native_ao_sparse_quantized_cpu_qlinear}
//x!{native_ao_sparse_quantized_cpu_qlinear_dynamic}
//x!{native_ao_sparse_quantized_cpu_qlinear_prepack}
//x!{native_ao_sparse_quantized_cpu_qlinear_unpack}
//x!{native_ao_sparse_quantized_cpu_qnnpack_utils}
//x!{native_autograd_composite}
//x!{native_average_pool2d}
//x!{native_average_pool3d}
//x!{native_batch_linear_algebra}
//x!{native_batch_linear_algebra_kernel}
//x!{native_batch_norm}
//x!{native_batching}
//x!{native_binary_ops}
//x!{native_blas}
//x!{native_blas_kernel}
//x!{native_bucketization}
//x!{native_bucketization_utils}
//x!{native_chanel_shuffle}
//x!{native_col_2im}
//x!{native_complex_helper}
//x!{native_composite_random_accessor}
//x!{native_composite_random_accessor_common}
//x!{native_constant_pad_nd}
//x!{native_conv_utils}
//x!{native_convolution}
//x!{native_convolution_mm2d}
//x!{native_convolution_mm3d}
//x!{native_convolution_tbc}
//x!{native_copy_}
//x!{native_cpu_activation}
//x!{native_cpu_adaptive_avg_pool_kernel}
//x!{native_cpu_atomic_add_float}
//x!{native_cpu_avg_pool_kernel}
//x!{native_cpu_avx_mathfun}
//x!{native_cpu_batch_norm_kernel}
//x!{native_cpu_binary_ops_kernel}
//x!{native_cpu_blas}
//x!{native_cpu_blas_kernel}
//x!{native_cpu_cat_kernel}
//x!{native_cpu_complex_kernel}
//x!{native_cpu_copy_kernel}
//x!{native_cpu_cross_kernel}
//x!{native_cpu_depthwise_conv_kernel}
//x!{native_cpu_distance_ops_kernel}
//x!{native_cpu_distribution_templates}
//x!{native_cpu_fill_kernel}
//x!{native_cpu_function_of_amatrix_utils_kernel}
//x!{native_cpu_grid_sampler_kernel}
//x!{native_cpu_group_norm_kernel}
//x!{native_cpu_index_kernel}
//x!{native_cpu_is_contiguous}
//x!{native_cpu_layer_norm_kernel}
//x!{native_cpu_lerp_kernel}
//x!{native_cpu_linear_algebra_kernel}
//x!{native_cpu_loops}
//x!{native_cpu_max_pool_kernel}
//x!{native_cpu_max_pooling}
//x!{native_cpu_moments_utils}
//x!{native_cpu_multinomial_kernel}
//x!{native_cpu_pointwise_ops_kernel}
//x!{native_cpu_pow_kernel}
//x!{native_cpu_range_factories_kernel}
//x!{native_cpu_reduce}
//x!{native_cpu_reduce_all_ops_kernel}
//x!{native_cpu_reduce_ops_kernel}
//x!{native_cpu_renorm_kernel}
//x!{native_cpu_scatter_gather_kernel}
//x!{native_cpu_soft_max_kernel}
//x!{native_cpu_softmax_kernel}
//x!{native_cpu_sorting_kernel}
//x!{native_cpu_stack_kernel}
//x!{native_cpu_sum_kernel}
//x!{native_cpu_tensor_compare_kernel}
//x!{native_cpu_unary_ops_kernel}
//x!{native_cpu_unfold2d}
//x!{native_cpu_unfold_backward_kernel}
//x!{native_cpu_up_sample_kernel}
//x!{native_cpu_up_sample_more_kernel}
//x!{native_cpu_utils}
//x!{native_cpu_zmath}
//x!{native_cross}
//x!{native_cuda_batch_linear_algebra_lib}
//x!{native_cuda_blas}
//x!{native_cuda_composite_random_accessor}
//x!{native_cuda_cu_fft_plan_cache}
//x!{native_cuda_cu_fft_utils}
//x!{native_cuda_distribution_templates}
//x!{native_cuda_launch_utils}
//x!{native_cuda_misc_utils}
//x!{native_cuda_spectral_ops}
//x!{native_cuda_tensor_shapecuda}
//x!{native_cudnn_affine_grid_generator}
//x!{native_cudnn_batch_norm}
//x!{native_cudnn_conv_placeholders}
//x!{native_cudnn_conv_shared}
//x!{native_cudnn_conv_v7}
//x!{native_cudnn_conv_v8}
//x!{native_cudnn_grid_sampler}
//x!{native_cudnn_loss_ctc}
//x!{native_cudnn_macros}
//x!{native_cudnn_rnn}
//x!{native_cudnn_rnn_utils}
//x!{native_dilated_convolution_utils}
//x!{native_dilated_max_pool2d}
//x!{native_dilated_max_pool3d}
//x!{native_dispatch_stub}
//x!{native_distance}
//x!{native_distribution_templates}
//x!{native_distributions}
//x!{native_dropout}
//x!{native_embedding}
//x!{native_embedding_bag}
//x!{native_fill}
//x!{native_foreach_ops_kernels}
//x!{native_foreach_utils}
//x!{native_fractional_max_pool2d}
//x!{native_fractional_max_pool3d}
//x!{native_function_of_amatrix_utils}
//x!{native_gated_linear_unit}
//x!{native_grid_sampler}
//x!{native_group_norm}
//x!{native_im2col}
//x!{native_im2col_shape_check}
//x!{native_im_2col}
//x!{native_indexing_utils}
//x!{native_integration}
//x!{native_itertools}
//x!{native_layer_norm}
//x!{native_legacy_nnd_efinitions}
//x!{native_lerp}
//x!{native_linear}
//x!{native_linear_algebra}
//x!{native_linear_algebra_utils}
//x!{native_loss}
//x!{native_loss_ctc}
//x!{native_loss_multi}
//x!{native_loss_multi_label_margin}
//x!{native_loss_multi_margin}
//x!{native_loss_nll}
//x!{native_loss_nll2d}
//x!{native_math}
//x!{native_max_pooling}
//x!{native_max_unpooling}
//x!{native_memory}
//x!{native_meta_tensor}
//x!{native_metal_metal_conv_params}
//x!{native_metal_metal_device}
//x!{native_metal_metal_guard_impl}
//x!{native_metal_metal_neuron_type}
//x!{native_metal_metal_prepack_op_context}
//x!{native_metal_metal_prepack_op_register}
//x!{native_metal_metal_shaders}
//x!{native_metal_metal_tensor_impl}
//x!{native_metal_metal_tensor_impl_storage}
//x!{native_metal_metal_tensor_utils}
//x!{native_metal_metal_utils}
//x!{native_metal_mpscnn_mps_image_utils}
//x!{native_metal_mpscnn_mps_image_wrapper}
//x!{native_metal_mpscnn_mpscnn_utils}
//x!{native_metal_mpscnn_tests_mpscnn_tests}
//x!{native_metal_ops_metal_convolution}
//x!{native_metal_ops_metal_copy}
//x!{native_miopen_batch_norm_miopen}
//x!{native_miopen_conv_miopen}
//x!{native_miopen_rnn_miopen}
//x!{native_mkl_linear_algebra}
//x!{native_mkl_sparse_csr_linear_algebra}
//x!{native_mkl_spectral_ops}
//x!{native_mkldnn_binary_ops}
//x!{native_mkldnn_conv}
//x!{native_mkldnn_copy_}
//x!{native_mkldnn_deep_registration}
//x!{native_mkldnn_linear}
//x!{native_mkldnn_mkldnn_common}
//x!{native_mkldnn_mkldnn_conversions}
//x!{native_mkldnn_mkldnn_tensor_math}
//x!{native_mkldnn_normalization}
//x!{native_mkldnn_pooling}
//x!{native_mkldnn_relu}
//x!{native_mkldnn_soft_max}
//x!{native_mkldnn_tensor_factories}
//x!{native_mkldnn_tensor_shape}
//x!{native_mkldnn_unary_ops}
//x!{native_mkldnn_utils}
//x!{native_naive_convolution_transpose2d}
//x!{native_naive_convolution_transpose3d}
//x!{native_naive_dilated_convolution}
//x!{native_named_tensor}
//x!{native_nnpack}
//x!{native_normalization}
//x!{native_onehot}
//x!{native_packed_sequence}
//x!{native_pixel_shuffle}
//x!{native_pointwise_ops}
//x!{native_pool}
//x!{native_pooling}
//x!{native_pow}
//x!{native_quantized_affine_quantizer}
//x!{native_quantized_affine_quantizer_base}
//x!{native_quantized_copy_}
//x!{native_quantized_cpu_conv_packed_params}
//x!{native_quantized_cpu_conv_serialization}
//x!{native_quantized_cpu_embedding_packed_params}
//x!{native_quantized_cpu_fbgemm_utils}
//x!{native_quantized_cpu_init_qnnpack}
//x!{native_quantized_cpu_int_repr_quant}
//x!{native_quantized_cpu_kernels_quantized_op_kernels}
//x!{native_quantized_cpu_make_per_tensor_quantized_tensor}
//x!{native_quantized_cpu_packed_params}
//x!{native_quantized_cpu_q_adaavgpool}
//x!{native_quantized_cpu_q_avgpool}
//x!{native_quantized_cpu_q_avgpool3d}
//x!{native_quantized_cpu_qadd}
//x!{native_quantized_cpu_qbatch_norm}
//x!{native_quantized_cpu_qchannel_shuffle}
//x!{native_quantized_cpu_qclamp}
//x!{native_quantized_cpu_qconcat}
//x!{native_quantized_cpu_qconv}
//x!{native_quantized_cpu_qconv_prepack}
//x!{native_quantized_cpu_qconv_unpack}
//x!{native_quantized_cpu_qelu}
//x!{native_quantized_cpu_qembeddingbag}
//x!{native_quantized_cpu_qembeddingbag_prepack}
//x!{native_quantized_cpu_qembeddingbag_unpack}
//x!{native_quantized_cpu_qhardsigmoid}
//x!{native_quantized_cpu_qhardswish}
//x!{native_quantized_cpu_qlinear}
//x!{native_quantized_cpu_qlinear_dynamic}
//x!{native_quantized_cpu_qlinear_prepack}
//x!{native_quantized_cpu_qlinear_unpack}
//x!{native_quantized_cpu_qmul}
//x!{native_quantized_cpu_qnnpack_add}
//x!{native_quantized_cpu_qnnpack_aligned_allocator}
//x!{native_quantized_cpu_qnnpack_assembly}
//x!{native_quantized_cpu_qnnpack_average_pooling}
//x!{native_quantized_cpu_qnnpack_bench_add}
//x!{native_quantized_cpu_qnnpack_bench_average_pooling}
//x!{native_quantized_cpu_qnnpack_bench_channel_shuffle}
//x!{native_quantized_cpu_qnnpack_bench_convolution}
//x!{native_quantized_cpu_qnnpack_bench_global_average_pooling}
//x!{native_quantized_cpu_qnnpack_bench_hardsigmoid}
//x!{native_quantized_cpu_qnnpack_bench_hardswish}
//x!{native_quantized_cpu_qnnpack_bench_hgemm}
//x!{native_quantized_cpu_qnnpack_bench_max_pooling}
//x!{native_quantized_cpu_qnnpack_bench_q8gemm}
//x!{native_quantized_cpu_qnnpack_bench_q8gemm_sparse}
//x!{native_quantized_cpu_qnnpack_bench_requantization}
//x!{native_quantized_cpu_qnnpack_bench_sgemm}
//x!{native_quantized_cpu_qnnpack_bench_sigmoid}
//x!{native_quantized_cpu_qnnpack_bench_softargmax}
//x!{native_quantized_cpu_qnnpack_bench_tanh}
//x!{native_quantized_cpu_qnnpack_channel_shuffle}
//x!{native_quantized_cpu_qnnpack_clamp}
//x!{native_quantized_cpu_qnnpack_common}
//x!{native_quantized_cpu_qnnpack_conv_prepack}
//x!{native_quantized_cpu_qnnpack_conv_run}
//x!{native_quantized_cpu_qnnpack_conv_utils}
//x!{native_quantized_cpu_qnnpack_convolution}
//x!{native_quantized_cpu_qnnpack_deconv_run}
//x!{native_quantized_cpu_qnnpack_deconvolution}
//x!{native_quantized_cpu_qnnpack_deps_clog}
//x!{native_quantized_cpu_qnnpack_deps_clog_test}
//x!{native_quantized_cpu_qnnpack_fc_dynamic_run}
//x!{native_quantized_cpu_qnnpack_fc_prepack}
//x!{native_quantized_cpu_qnnpack_fc_run}
//x!{native_quantized_cpu_qnnpack_fully_connected}
//x!{native_quantized_cpu_qnnpack_fully_connected_sparse}
//x!{native_quantized_cpu_qnnpack_global_average_pooling}
//x!{native_quantized_cpu_qnnpack_hardsigmoid}
//x!{native_quantized_cpu_qnnpack_hardswish}
//x!{native_quantized_cpu_qnnpack_hgemm}
//x!{native_quantized_cpu_qnnpack_hgemm_8x8_aarch32_neonfp_16arith}
//x!{native_quantized_cpu_qnnpack_hgemm_8x8_neonfp_16arith}
//x!{native_quantized_cpu_qnnpack_indirection}
//x!{native_quantized_cpu_qnnpack_init}
//x!{native_quantized_cpu_qnnpack_isa_checks}
//x!{native_quantized_cpu_qnnpack_leaky_relu}
//x!{native_quantized_cpu_qnnpack_log}
//x!{native_quantized_cpu_qnnpack_math}
//x!{native_quantized_cpu_qnnpack_max_pooling}
//x!{native_quantized_cpu_qnnpack_operator}
//x!{native_quantized_cpu_qnnpack_operator_delete}
//x!{native_quantized_cpu_qnnpack_operator_run}
//x!{native_quantized_cpu_qnnpack_pack}
//x!{native_quantized_cpu_qnnpack_pack_block_sparse}
//x!{native_quantized_cpu_qnnpack_params}
//x!{native_quantized_cpu_qnnpack_q8avgpool}
//x!{native_quantized_cpu_qnnpack_q8avgpool_mp8x9p8q_neon}
//x!{native_quantized_cpu_qnnpack_q8avgpool_mp8x9p8q_sse2}
//x!{native_quantized_cpu_qnnpack_q8avgpool_up8x9_neon}
//x!{native_quantized_cpu_qnnpack_q8avgpool_up8x9_sse2}
//x!{native_quantized_cpu_qnnpack_q8avgpool_up8xm_neon}
//x!{native_quantized_cpu_qnnpack_q8avgpool_up8xm_sse2}
//x!{native_quantized_cpu_qnnpack_q8conv}
//x!{native_quantized_cpu_qnnpack_q8conv_4x4c2_sse2}
//x!{native_quantized_cpu_qnnpack_q8conv_4x8_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8conv_4x8_neon}
//x!{native_quantized_cpu_qnnpack_q8conv_8x8_aarch64_neon}
//x!{native_quantized_cpu_qnnpack_q8conv_8x8_neon}
//x!{native_quantized_cpu_qnnpack_q8dwconv}
//x!{native_quantized_cpu_qnnpack_q8dwconv_mp8x25_neon}
//x!{native_quantized_cpu_qnnpack_q8dwconv_mp8x25_neon_per_channel}
//x!{native_quantized_cpu_qnnpack_q8dwconv_mp8x25_sse2}
//x!{native_quantized_cpu_qnnpack_q8dwconv_mp8x25_sse2_per_channel}
//x!{native_quantized_cpu_qnnpack_q8dwconv_up8x9_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8dwconv_up8x9_aarch32_neon_per_channel}
//x!{native_quantized_cpu_qnnpack_q8dwconv_up8x9_neon}
//x!{native_quantized_cpu_qnnpack_q8dwconv_up8x9_neon_per_channel}
//x!{native_quantized_cpu_qnnpack_q8dwconv_up8x9_sse2}
//x!{native_quantized_cpu_qnnpack_q8dwconv_up8x9_sse2_per_channel}
//x!{native_quantized_cpu_qnnpack_q8gavgpool}
//x!{native_quantized_cpu_qnnpack_q8gavgpool_mp8x7p7q_neon}
//x!{native_quantized_cpu_qnnpack_q8gavgpool_mp8x7p7q_sse2}
//x!{native_quantized_cpu_qnnpack_q8gavgpool_up8x7_neon}
//x!{native_quantized_cpu_qnnpack_q8gavgpool_up8x7_sse2}
//x!{native_quantized_cpu_qnnpack_q8gavgpool_up8xm_neon}
//x!{native_quantized_cpu_qnnpack_q8gavgpool_up8xm_sse2}
//x!{native_quantized_cpu_qnnpack_q8gemm}
//x!{native_quantized_cpu_qnnpack_q8gemm_2x4c8_sse2}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x4c2_dq_sse2}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x4c2_sse2}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x8_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x8_dq_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x8_dq_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x8_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x8c2_xzp_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x8c2_xzp_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_4x_sumrows_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_6x4_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_8x8_aarch64_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_8x8_dq_aarch64_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_8x8_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_4x4_packa_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_4x8c1x4_dq_packeda_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_4x8c8x1_dq_packeda_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_8x4_packa_aarch32_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_8x4_packa_aarch64_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_8x4_packa_sse2}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_8x4c1x4_dq_packeda_sse2}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_8x4c1x4_packed_sse2}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_8x8c1x4_dq_packeda_aarch64_neon}
//x!{native_quantized_cpu_qnnpack_q8gemm_sparse_8x8c8x1_dq_packeda_aarch64_neon}
//x!{native_quantized_cpu_qnnpack_q8vadd}
//x!{native_quantized_cpu_qnnpack_q8vadd_neon}
//x!{native_quantized_cpu_qnnpack_q8vadd_sse2}
//x!{native_quantized_cpu_qnnpack_qnnpack}
//x!{native_quantized_cpu_qnnpack_qnnpack_func}
//x!{native_quantized_cpu_qnnpack_requantization}
//x!{native_quantized_cpu_qnnpack_requantization_fp32_neon}
//x!{native_quantized_cpu_qnnpack_requantization_fp32_psimd}
//x!{native_quantized_cpu_qnnpack_requantization_fp32_scalar}
//x!{native_quantized_cpu_qnnpack_requantization_fp32_sse2}
//x!{native_quantized_cpu_qnnpack_requantization_gemmlowp_neon}
//x!{native_quantized_cpu_qnnpack_requantization_gemmlowp_scalar}
//x!{native_quantized_cpu_qnnpack_requantization_gemmlowp_sse}
//x!{native_quantized_cpu_qnnpack_requantization_gemmlowp_sse2}
//x!{native_quantized_cpu_qnnpack_requantization_gemmlowp_sse4}
//x!{native_quantized_cpu_qnnpack_requantization_gemmlowp_ssse3}
//x!{native_quantized_cpu_qnnpack_requantization_precise_neon}
//x!{native_quantized_cpu_qnnpack_requantization_precise_psimd}
//x!{native_quantized_cpu_qnnpack_requantization_precise_scalar}
//x!{native_quantized_cpu_qnnpack_requantization_precise_sse2}
//x!{native_quantized_cpu_qnnpack_requantization_precise_sse4}
//x!{native_quantized_cpu_qnnpack_requantization_precise_ssse3}
//x!{native_quantized_cpu_qnnpack_requantization_q31_neon}
//x!{native_quantized_cpu_qnnpack_requantization_q31_scalar}
//x!{native_quantized_cpu_qnnpack_requantization_q31_sse2}
//x!{native_quantized_cpu_qnnpack_requantization_q31_sse4}
//x!{native_quantized_cpu_qnnpack_requantization_q31_ssse3}
//x!{native_quantized_cpu_qnnpack_requantization_runtime_assembly}
//x!{native_quantized_cpu_qnnpack_requantization_runtime_neon}
//x!{native_quantized_cpu_qnnpack_requantization_runtime_sse2}
//x!{native_quantized_cpu_qnnpack_requantization_stubs}
//x!{native_quantized_cpu_qnnpack_scalar_utils}
//x!{native_quantized_cpu_qnnpack_sconv}
//x!{native_quantized_cpu_qnnpack_sconv_6x8_psimd}
//x!{native_quantized_cpu_qnnpack_sdwconv}
//x!{native_quantized_cpu_qnnpack_sdwconv_up4x9_psimd}
//x!{native_quantized_cpu_qnnpack_sgemm}
//x!{native_quantized_cpu_qnnpack_sgemm_5x8_neon}
//x!{native_quantized_cpu_qnnpack_sgemm_6x8_neon}
//x!{native_quantized_cpu_qnnpack_sgemm_6x8_psimd}
//x!{native_quantized_cpu_qnnpack_sigmoid}
//x!{native_quantized_cpu_qnnpack_softargmax}
//x!{native_quantized_cpu_qnnpack_tanh}
//x!{native_quantized_cpu_qnnpack_test_add}
//x!{native_quantized_cpu_qnnpack_test_add_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_average_pooling}
//x!{native_quantized_cpu_qnnpack_test_average_pooling_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_avgpool_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_channel_shuffle}
//x!{native_quantized_cpu_qnnpack_test_channel_shuffle_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_clamp}
//x!{native_quantized_cpu_qnnpack_test_clamp_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_clamp_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_convolution}
//x!{native_quantized_cpu_qnnpack_test_convolution_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_deconvolution}
//x!{native_quantized_cpu_qnnpack_test_deconvolution_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_dwconv_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_fully_connected}
//x!{native_quantized_cpu_qnnpack_test_fully_connected_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_fully_connected_sparse}
//x!{native_quantized_cpu_qnnpack_test_fully_connected_sparse_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_gavgpool_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_gemm_block_sparse_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_gemm_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_global_average_pooling}
//x!{native_quantized_cpu_qnnpack_test_global_average_pooling_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_hardsigmoid}
//x!{native_quantized_cpu_qnnpack_test_hardsigmoid_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_hardswish}
//x!{native_quantized_cpu_qnnpack_test_hardswish_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_hgemm}
//x!{native_quantized_cpu_qnnpack_test_leaky_relu}
//x!{native_quantized_cpu_qnnpack_test_leaky_relu_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_lut_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_lut_norm_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_max_pooling}
//x!{native_quantized_cpu_qnnpack_test_max_pooling_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_maxpool_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_q8avgpool}
//x!{native_quantized_cpu_qnnpack_test_q8conv}
//x!{native_quantized_cpu_qnnpack_test_q8dwconv}
//x!{native_quantized_cpu_qnnpack_test_q8gavgpool}
//x!{native_quantized_cpu_qnnpack_test_q8gemm}
//x!{native_quantized_cpu_qnnpack_test_q8gemm_sparse}
//x!{native_quantized_cpu_qnnpack_test_q8vadd}
//x!{native_quantized_cpu_qnnpack_test_requantization}
//x!{native_quantized_cpu_qnnpack_test_requantization_tester}
//x!{native_quantized_cpu_qnnpack_test_rmax_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_sconv}
//x!{native_quantized_cpu_qnnpack_test_sgemm}
//x!{native_quantized_cpu_qnnpack_test_sigmoid}
//x!{native_quantized_cpu_qnnpack_test_sigmoid_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_softargmax}
//x!{native_quantized_cpu_qnnpack_test_softargmax_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_tanh}
//x!{native_quantized_cpu_qnnpack_test_tanh_operator_tester}
//x!{native_quantized_cpu_qnnpack_test_test_utils}
//x!{native_quantized_cpu_qnnpack_test_u8clamp}
//x!{native_quantized_cpu_qnnpack_test_u8lut_32norm}
//x!{native_quantized_cpu_qnnpack_test_u8maxpool}
//x!{native_quantized_cpu_qnnpack_test_u8rmax}
//x!{native_quantized_cpu_qnnpack_test_vadd_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_test_x8lut}
//x!{native_quantized_cpu_qnnpack_test_x8zip}
//x!{native_quantized_cpu_qnnpack_test_zip_microkernel_tester}
//x!{native_quantized_cpu_qnnpack_u8clamp}
//x!{native_quantized_cpu_qnnpack_u8clamp_neon}
//x!{native_quantized_cpu_qnnpack_u8clamp_sse2}
//x!{native_quantized_cpu_qnnpack_u8lut_32norm}
//x!{native_quantized_cpu_qnnpack_u8lut_32norm_scalar}
//x!{native_quantized_cpu_qnnpack_u8maxpool}
//x!{native_quantized_cpu_qnnpack_u8maxpool_16x9p8q_neon}
//x!{native_quantized_cpu_qnnpack_u8maxpool_16x9p8q_sse2}
//x!{native_quantized_cpu_qnnpack_u8maxpool_sub16_neon}
//x!{native_quantized_cpu_qnnpack_u8maxpool_sub16_sse2}
//x!{native_quantized_cpu_qnnpack_u8rmax}
//x!{native_quantized_cpu_qnnpack_u8rmax_neon}
//x!{native_quantized_cpu_qnnpack_u8rmax_sse2}
//x!{native_quantized_cpu_qnnpack_utils}
//x!{native_quantized_cpu_qnnpack_x8lut}
//x!{native_quantized_cpu_qnnpack_x8lut_scalar}
//x!{native_quantized_cpu_qnnpack_x8zip}
//x!{native_quantized_cpu_qnnpack_x8zip_x2_neon}
//x!{native_quantized_cpu_qnnpack_x8zip_x2_sse2}
//x!{native_quantized_cpu_qnnpack_x8zip_x3_neon}
//x!{native_quantized_cpu_qnnpack_x8zip_x3_sse2}
//x!{native_quantized_cpu_qnnpack_x8zip_x4_neon}
//x!{native_quantized_cpu_qnnpack_x8zip_x4_sse2}
//x!{native_quantized_cpu_qnnpack_x8zip_xm_neon}
//x!{native_quantized_cpu_qnnpack_x8zip_xm_sse2}
//x!{native_quantized_cpu_qnormalization}
//x!{native_quantized_cpu_qpool}
//x!{native_quantized_cpu_qreduction}
//x!{native_quantized_cpu_qrelu}
//x!{native_quantized_cpu_qsigmoid}
//x!{native_quantized_cpu_qsort}
//x!{native_quantized_cpu_qtanh}
//x!{native_quantized_cpu_qthreshold}
//x!{native_quantized_cpu_quant_utils}
//x!{native_quantized_cpu_quantized_ops}
//x!{native_quantized_cpu_qupsample_bilinear2d}
//x!{native_quantized_cpu_qupsample_nearest2d}
//x!{native_quantized_cpu_qupsample_nearest3d}
//x!{native_quantized_cpu_tensor_operators}
//x!{native_quantized_fake_quant_affine}
//x!{native_quantized_fake_quant_per_channel_affine}
//x!{native_quantized_fake_quant_per_tensor_affine}
//x!{native_quantized_library}
//x!{native_quantized_linear}
//x!{native_quantized_q_tensor}
//x!{native_quantized_tensor_compare}
//x!{native_quantized_tensor_factories}
//x!{native_range_factories}
//x!{native_reduce_all_ops}
//x!{native_reduce_ops}
//x!{native_reduce_ops_utils}
//x!{native_reflection_pad}
//x!{native_repeat}
//x!{native_replication_padding}
//x!{native_resize}
//x!{native_resize_common}
//x!{native_rnn}
//x!{native_rowwise_prune}
//x!{native_scalar}
//x!{native_scatter_gather_checks}
//x!{native_segment_reduce}
//x!{native_shared_reduce_ops}
//x!{native_sobol_engine_ops}
//x!{native_sobol_engine_ops_utils}
//x!{native_soft_max}
//x!{native_sorting}
//x!{native_sorting_utils}
//x!{native_sparse_cuda_sparse_cuda_tensor}
//x!{native_sparse_param_utils}
//x!{native_sparse_soft_max}
//x!{native_sparse_sparse_csr_tensor}
//x!{native_sparse_sparse_csr_tensor_math}
//x!{native_sparse_sparse_mat_mul}
//x!{native_sparse_sparse_tensor}
//x!{native_sparse_sparse_tensor_math}
//x!{native_spectral_ops}
//x!{native_spectral_ops_utils}
//x!{native_strided_random_accessor}
//x!{native_summary_ops}
//x!{native_tensor_advanced_indexing}
//x!{native_tensor_compare}
//x!{native_tensor_conversions}
//x!{native_tensor_dim_apply}
//x!{native_tensor_factories}
//x!{native_tensor_iterator_dynamic_casting}
//x!{native_tensor_iterator_reduce}
//x!{native_tensor_properties}
//x!{native_tensor_shape}
//x!{native_tensor_transformations}
//x!{native_test_ops}
//x!{native_triangular_ops}
//x!{native_triangular_ops_utils}
//x!{native_type_properties}
//x!{native_unary_ops}
//x!{native_unfold2d}
//x!{native_unfold3d}
//x!{native_unfold_backward}
//x!{native_unique}
//x!{native_up_sample}
//x!{native_up_sample_bicubic2d}
//x!{native_up_sample_bilinear2d}
//x!{native_up_sample_linear1d}
//x!{native_up_sample_nearest1d}
//x!{native_up_sample_nearest2d}
//x!{native_up_sample_nearest3d}
//x!{native_up_sample_trilinear3d}
//x!{native_utils_factory}
//x!{native_utils_param_utils}
//x!{native_utils_params_hash}
//x!{native_variable_method_stubs}
//x!{native_vol2col}
//x!{native_vulkan_api_adapter}
//x!{native_vulkan_api_allocator}
//x!{native_vulkan_api_cache}
//x!{native_vulkan_api_command}
//x!{native_vulkan_api_common}
//x!{native_vulkan_api_context}
//x!{native_vulkan_api_descriptor}
//x!{native_vulkan_api_pipeline}
//x!{native_vulkan_api_resource}
//x!{native_vulkan_api_runtime}
//x!{native_vulkan_api_shader}
//x!{native_vulkan_api_utils}
//x!{native_vulkan_api_vk_mem_alloc}
//x!{native_vulkan_ops_arithmetic}
//x!{native_vulkan_ops_clamp}
//x!{native_vulkan_ops_common}
//x!{native_vulkan_ops_convolution}
//x!{native_vulkan_ops_copy_}
//x!{native_vulkan_ops_factory}
//x!{native_vulkan_ops_mean}
//x!{native_vulkan_ops_mm}
//x!{native_vulkan_ops_padding}
//x!{native_vulkan_ops_persistent}
//x!{native_vulkan_ops_pool}
//x!{native_vulkan_ops_register}
//x!{native_vulkan_ops_shape}
//x!{native_vulkan_ops_tensor}
//x!{native_vulkan_ops_upsample}
//x!{native_vulkan_ops_utils}
//x!{native_vulkan_vulkan}
//x!{native_vulkan_vulkan_aten}
//x!{native_vulkan_vulkan_common}
//x!{native_vulkan_vulkan_convolution}
//x!{native_vulkan_vulkan_guard_impl}
//x!{native_vulkan_vulkan_op_context}
//x!{native_vulkan_vulkan_opaque_tensor_impl}
//x!{native_vulkan_vulkan_ops}
//x!{native_vulkan_vulkan_register_op_context_class}
//x!{native_weight_norm}
//x!{native_xnnpack_activation}
//x!{native_xnnpack_average_pooling}
//x!{native_xnnpack_channel_shuffle}
//x!{native_xnnpack_common}
//x!{native_xnnpack_convolution}
//x!{native_xnnpack_init}
//x!{native_xnnpack_linear}
//x!{native_xnnpack_max_pooling}
//x!{native_xnnpack_op_context}
//x!{native_xnnpack_pooling}
//x!{native_xnnpack_register_op_context_class}
//x!{nnapi_neural_networks}
//x!{nnapi_nnapi_bind}
//x!{nnapi_nnapi_model_loader}
//x!{nnapi_nnapi_wrapper}
//x!{numeric_utils}
//x!{op_list}
//x!{op_registration}
//x!{op_registration_adaption}
//x!{op_registration_infer_schema}
//x!{op_registration_op_allowlist}
//x!{op_registration_op_allowlist_test}
//x!{op_registration_op_registration_test}
//x!{opaque_tensor_impl}
//x!{operator_name}
//x!{parallel}
//x!{parallel_common}
//x!{parallel_native}
//x!{parallel_native_tbb}
//x!{parallel_openmp}
//x!{parallel_thread_pool_native}
//x!{philox_rng_engine}
//x!{pt_thread_pool}
//x!{qualified_name}
//x!{quantized_q_tensor_impl}
//x!{quantized_quantizer}
//x!{quantizer_base}
//x!{range}
//x!{record_function}
//x!{reduction}
//x!{register_symbols}
//x!{rref_interface}
//x!{scalar_ops}
//x!{sequence_number}
//x!{sparse_csr_tensor_impl}
//x!{sparse_csr_tensor_utils}
//x!{sparse_tensor_impl}
//x!{sparse_tensor_utils}
//x!{stack}
//x!{templates_functions}
//x!{templates_op_list}
//x!{templates_operators}
//x!{templates_register_backend_select}
//x!{templates_register_dispatch_key}
//x!{templates_register_schema}
//x!{templates_tensor_body}
//x!{templates_tensor_methods}
//x!{templates_xla_type_default}
//x!{tensor}
//x!{tensor_geometry}
//x!{tensor_impl_test}
//x!{tensor_indexing}
//x!{tensor_iterator}
//x!{tensor_iterator_internal}
//x!{tensor_meta}
//x!{tensor_names}
//x!{tensor_operators}
//x!{tensor_utils}
//x!{test_apply_utils_test}
//x!{test_atest}
//x!{test_basic}
//x!{test_broadcast_test}
//x!{test_cpu_caching_allocator_test}
//x!{test_cpu_generator_test}
//x!{test_cpu_profiling_allocator_test}
//x!{test_cpu_rng_test}
//x!{test_cuda_apply_test}
//x!{test_cuda_cudnn_test}
//x!{test_cuda_dlconvertor_test}
//x!{test_cuda_stream_test}
//x!{test_cuda_tensor_interop_test}
//x!{test_dict_test}
//x!{test_dimname_test}
//x!{test_dlconvertor_test}
//x!{test_exclusively_owned_test}
//x!{test_extension_backend_test}
//x!{test_half_test}
//x!{test_ivalue_test}
//x!{test_math_kernel_test}
//x!{test_maybe_owned_test}
//x!{test_memory_format_test}
//x!{test_memory_overlapping_test}
//x!{test_mobile_memory_cleanup}
//x!{test_named_tensor_test}
//x!{test_native_test}
//x!{test_operators_test}
//x!{test_pow_test}
//x!{test_quantized_test}
//x!{test_reduce_ops_test}
//x!{test_rng_test}
//x!{test_scalar_tensor_test}
//x!{test_scalar_test}
//x!{test_tensor_interop_test}
//x!{test_tensor_iterator_test}
//x!{test_test_assert}
//x!{test_test_install_main}
//x!{test_test_parallel}
//x!{test_test_thread_pool_guard}
//x!{test_thread_init_test}
//x!{test_type_test}
//x!{test_undefined_tensor_test}
//x!{test_variant_test}
//x!{test_vec_test_all_types}
//x!{test_vitals}
//x!{test_vmap_test}
//x!{test_vulkan_api_test}
//x!{test_vulkan_test}
//x!{test_weakref_test}
//x!{test_wrapdim_test}
//x!{test_xla_tensor_test}
//x!{test_xnnpack_test}
//x!{th_generic_th_blas}
//x!{th_generic_th_storage}
//x!{th_generic_th_storage_copy}
//x!{th_generic_th_tensor}
//x!{th_generic_th_tensor_apply}
//x!{th_generic_th_tensor_fast_get_set}
//x!{th_generic_th_tensor_math}
//x!{th_generic_th_tensor_more_math}
//x!{th_generic_th_vector}
//x!{th_th_allocator}
//x!{th_th_blas}
//x!{th_th_general}
//x!{th_th_storage_functions}
//x!{th_th_tensor}
//x!{th_th_tensor_apply}
//x!{th_th_tensor_dim_apply}
//x!{th_th_vector}
//x!{thc_generic_thc_storage}
//x!{thc_generic_thc_storage_copy}
//x!{thc_generic_thc_tensor}
//x!{thc_generic_thc_tensor_copy}
//x!{thc_generic_thc_tensor_math}
//x!{thc_generic_thc_tensor_math_magma}
//x!{thc_generic_thc_tensor_math_pairwise}
//x!{thc_generic_thc_tensor_math_pointwise}
//x!{thc_generic_thc_tensor_scatter_gather}
//x!{thc_thc_allocator}
//x!{thc_thc_caching_host_allocator}
//x!{thc_thc_general}
//x!{thc_thc_generate_all_types}
//x!{thc_thc_sleep}
//x!{thc_thc_storage}
//x!{thc_thc_tensor}
//x!{thc_thc_tensor_math_magma}
//x!{thcunn}
//x!{thcunn_common}
//x!{thcunn_generic}
//x!{thread_local_state}
//x!{tracer_mode}
//x!{transformation_helper}
//x!{ty}
//x!{type_default}
//x!{unknown}
//x!{unsafe_fromth}
//x!{utils}
//x!{variable_fallback_kernel}
//x!{variable_hooks_interface}
//x!{variadic}
//x!{version}
//x!{vitals}
//x!{vmap_mode}
//x!{vmap_mode_registrations}
//x!{vmap_transforms}
//x!{vulkan_context}
//x!{wrap_dim_utils}
//x!{wrap_dim_utils_multi}
