#![feature(array_map)]
#![feature(min_type_alias_impl_trait)]
#![feature(const_generics_defaults)]
#![feature(associated_type_defaults)]
#![feature(const_generics)]
#![feature(const_panic)]
#![feature(in_band_lifetimes)]
#![feature(box_syntax)]
#![feature(assoc_char_funcs)]
#![feature(or_patterns)]
#![feature(destructuring_assignment)]
#![feature(specialization)]
#![feature(test)]

//from surge
#![feature(array_methods)]
#![feature(asm)]
#![feature(associated_type_bounds)]
#![feature(box_patterns)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(core_intrinsics)]
#![feature(llvm_asm)]
#![feature(platform_intrinsics)]
#![feature(rustc_private)]
#![feature(stdarch)]
#![feature(stdsimd)]
#![feature(trait_alias)]

#![allow(const_err)]

extern crate test;

extern crate atomic;
extern crate libc;

pub fn no_op() {}

#[macro_use] extern crate static_assertions;
#[macro_use] extern crate test_context;

#[macro_export] macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }

#[macro_export] macro_rules! ix { 
    () => { 
        use crate::{ imports::* , };
        use crate::type_traits::*;
        //use crate::traits::*;
        //use crate::constants::*;
        //use crate::types::*;
        //use crate::sizeof::*;
    } 
}


pub mod imports;

//x![constants];
//x![types];
//x![traits];
//x![sizeof];

//--------------------protos
x![caffe2];
x![prof_dag];
x![predictor_consts];
x![metanet];
x![hsm];
x![torch];
x![caffe2_legacy];

//--------------------------
x![system];
x![type_traits];
x![unknown];

//x![test];
//x![util_eigen_utils];
x![const_assert];
x![core_blob];
x![core_blob_gpu_test];
x![core_blob_stats];
x![core_blob_test];
x![core_common];
x![core_common_cudnn];
x![core_common_gpu];
x![core_common_test];
x![core_context];
x![core_context_base];
x![core_context_gpu];
x![core_context_gpu_test];
x![core_context_test];
x![core_cudnn_wrappers];
x![core_db];
x![core_distributions_stubs];
x![core_event];
x![core_event_cpu];
x![core_event_gpu];
x![core_event_gpu_test];
x![core_event_test];
x![core_export_c10_op_to_caffe2];
x![core_export_caffe2_op_to_c10];
x![core_graph];
x![core_graph_test];
x![core_init];
x![core_init_denormals];
x![core_init_intrinsics_check];
x![core_init_omp];
x![core_init_test];
x![core_int8_serialization];
x![core_macros];
x![core_memonger];
x![core_module];
x![core_module_test];
x![core_net];
x![core_net_async_base];
x![core_net_async_scheduling];
x![core_net_async_task];
x![core_net_async_task_future];
x![core_net_async_task_graph];
x![core_net_async_tracing];
x![core_net_async_tracing_test];
x![core_net_dag_utils];
x![core_net_dag_utils_test];
x![core_net_gpu_test];
x![core_net_parallel];
x![core_net_simple];
x![core_net_simple_refcount];
x![core_net_simple_refcount_test];
x![core_net_test];
x![core_observer];
x![core_observer_test];
x![core_operator];
x![core_operator_gpu_test];
x![core_operator_gradient];
x![core_operator_schema];
x![core_operator_schema_test];
x![core_operator_test];
x![core_parallel_net_test];
x![core_plan_executor];
x![core_plan_executor_test];
x![core_prof_dag_counters];
x![core_qtensor];
x![core_qtensor_serialization];
x![core_serialization_test];
x![core_static_tracepoint];
x![core_static_tracepoint_elfx86];
x![core_stats];
x![core_stats_test];
x![core_tensor];
x![core_tensor_int8];
x![core_test_utils];
x![core_timer];
x![core_timer_test];
x![core_transform];
x![core_transform_test];
x![core_types];
x![core_workspace];
x![core_workspace_test];
x![env];
x![hip_common_miopen];
x![hip_miopen_wrapper];
x![histogram_net_observer];
x![histogram_observer];
x![ideep_ideep_utils];
x![ideep_operators_adam];
x![ideep_operators_channel_shuffle];
x![ideep_operators_concat_split];
x![ideep_operators_conv];
x![ideep_operators_conv_pool_base];
x![ideep_operators_conv_transpose];
x![ideep_operators_conv_transpose_unpool_base];
x![ideep_operators_dropout];
x![ideep_operators_elementwise_sum];
x![ideep_operators_expand_squeeze_dims];
x![ideep_operators_fully_connected];
x![ideep_operators_local_response_normalization];
x![ideep_operators_momentum_sgd];
x![ideep_operators_operator_fallback_ideep];
x![ideep_operators_order_switch_ops];
x![ideep_operators_pool];
x![ideep_operators_quantization_int8_add];
x![ideep_operators_quantization_int8_conv];
x![ideep_operators_quantization_int8_dequantize];
x![ideep_operators_quantization_int8_fully_connected];
x![ideep_operators_quantization_int8_given_tensor_fill];
x![ideep_operators_quantization_int8_pool];
x![ideep_operators_quantization_int8_quantize];
x![ideep_operators_quantization_int8_relu];
x![ideep_operators_queue_ops];
x![ideep_operators_relu];
x![ideep_operators_reshape];
x![ideep_operators_shape];
x![ideep_operators_sigmoid];
x![ideep_operators_spatial_batch_norm];
x![ideep_operators_transpose];
x![ideep_operators_utility_ops];
x![ideep_utils_ideep_context];
x![ideep_utils_ideep_operator];
x![ideep_utils_ideep_register];
x![image_input];
x![make_signed];
x![nomnigraph_algorithmstest];
x![nomnigraph_binarymatchimpltest];
x![nomnigraph_converters_dot];
x![nomnigraph_generated_opclasses];
x![nomnigraph_graph_algorithms];
x![nomnigraph_graph_binarymatchimpl];
x![nomnigraph_graph_graph];
x![nomnigraph_graph_tarjansimpl];
x![nomnigraph_graph_toposort];
x![nomnigraph_graphtest];
x![nomnigraph_matchtest];
x![nomnigraph_neuralnet];
x![nomnigraph_neuralnettest];
x![nomnigraph_representations_compiler];
x![nomnigraph_representations_controlflow];
x![nomnigraph_representations_neuralnet];
x![nomnigraph_subgraphmatchertest];
x![nomnigraph_support_common];
x![nomnigraph_tarjansimpltest];
x![nomnigraph_test_util];
x![nomnigraph_toposorttest];
x![nomnigraph_transformations_match];
x![nomnigraph_transformations_subgraphmatcher];
x![observer_operator_attaching_net_observer];
x![observer_profile_observer];
x![observer_runcnt_observer];
x![observer_time_observer];
x![observer_time_observer_test];
x![op_abs];
x![op_accumulate];
x![op_accuracy];
x![op_acos];
x![op_activation_ops_cudnn];
x![op_affine_channel];
x![op_alias_with_name];
x![op_apmeter];
x![op_arg_ops];
x![op_asin];
x![op_assert];
x![op_async_net_barrier];
x![op_atan];
x![op_atomic_ops];
x![op_batch_box_cox];
x![op_batch_bucketize];
x![op_batch_gather_ops];
x![op_batch_matmul];
x![op_batch_matmul_op_gpu_test];
x![op_batch_matmul_op_test];
x![op_batch_moments];
x![op_batch_permutation];
x![op_batch_permutation_op_gpu_test];
x![op_batch_sparse_to_dense];
x![op_bbox_transform];
x![op_bisect_percentile];
x![op_boolean_mask_ops];
x![op_boolean_unmask_ops];
x![op_boolean_unmask_ops_test];
x![op_box_with_nms_limit];
x![op_bucketize];
x![op_byte_weight_dequant];
x![op_cast];
x![op_cbrt];
x![op_cc_bmm_bg];
x![op_ceil];
x![op_channel_backprop_stats];
x![op_channel_shuffle];
x![op_channel_stats];
x![op_clip];
x![op_collect_and_distribute_fpn_rpn_proposals];
x![op_communicator];
x![op_concat_split];
x![op_conditional];
x![op_conv];
x![op_conv_gradient];
x![op_conv_op_cache_cudnn];
x![op_conv_op_cache_cudnn_test];
x![op_conv_op_cudnn];
x![op_conv_op_eigen];
x![op_conv_op_gpu];
x![op_conv_op_impl];
x![op_conv_op_shared];
x![op_conv_pool_op_base];
x![op_conv_transpose];
x![op_conv_transpose_gradient];
x![op_conv_transpose_op_cudnn];
x![op_conv_transpose_op_impl];
x![op_conv_transpose_op_mobile];
x![op_conv_transpose_op_mobile_impl];
x![op_conv_transpose_op_mobile_test];
x![op_conv_transpose_unpool_op_base];
x![op_copy];
x![op_copy_rows_to_tensor];
x![op_cos];
x![op_cosh];
x![op_cosine_embedding_criterion];
x![op_counter_ops];
x![op_crash];
x![op_create_scope];
x![op_crf_viterbi];
x![op_cross_entropy];
x![op_ctc_beam_search_decoder];
x![op_ctc_greedy_decoder];
x![op_cube];
x![op_data_couple];
x![op_dataset_ops];
x![op_deform_conv];
x![op_dense_vector_to_id_list];
x![op_distance];
x![op_do];
x![op_dropout];
x![op_elementwise_add];
x![op_elementwise_div];
x![op_elementwise_div_gradient];
x![op_elementwise_linear];
x![op_elementwise_logical_ops];
x![op_elementwise_mul];
x![op_elementwise_op_gpu_test];
x![op_elementwise_op_test];
x![op_elementwise_ops];
x![op_elementwise_ops_schema];
x![op_elementwise_ops_utils];
x![op_elementwise_sub];
x![op_elementwise_sum];
x![op_elu];
x![op_elu_op_cudnn];
x![op_enforce_finite];
x![op_ensure_clipped];
x![op_ensure_cpu_output];
x![op_erf];
x![op_exp];
x![op_expand];
x![op_expand_squeeze_dims];
x![op_fc_inference];
x![op_feature_maps_ops];
x![op_feed_blob];
x![op_filler];
x![op_find];
x![op_find_duplicate_elements];
x![op_flatten];
x![op_flexible_top_k];
x![op_floor];
x![op_free];
x![op_fully_connected];
x![op_fully_connected_op_gpu];
x![op_fused_rowwise_8bit_conversion_ops];
x![op_fused_rowwise_nbit_conversion_ops];
x![op_fused_rowwise_nbitfake_conversion_ops];
x![op_fused_rowwise_random_quantization_ops];
x![op_gather];
x![op_gather_fused_8bit_rowwise];
x![op_gather_ranges_to_dense];
x![op_gelu];
x![op_generate_proposals];
x![op_generate_proposals_op_gpu_test];
x![op_generate_proposals_op_test];
x![op_generate_proposals_op_util_boxes];
x![op_generate_proposals_op_util_boxes_test];
x![op_generate_proposals_op_util_nms];
x![op_generate_proposals_op_util_nms_gpu];
x![op_generate_proposals_op_util_nms_gpu_test];
x![op_generate_proposals_op_util_nms_test];
x![op_given_tensor_byte_string_to_uint8_fill];
x![op_given_tensor_fill];
x![op_glu];
x![op_group_norm];
x![op_gru_unit];
x![op_h_softmax];
x![op_half_float_ops];
x![op_half_float_ops_test];
x![op_hard_sigmoid];
x![op_heatmap_max_keypoint];
x![op_hip_activation_ops_miopen];
x![op_histogram];
x![op_if];
x![op_im2col];
x![op_index_hash_ops];
x![op_index_ops];
x![op_inference_lstm];
x![op_instance_norm];
x![op_integral_image];
x![op_is_empty];
x![op_jsd];
x![op_key_split_ops];
x![op_last_n_window_collector];
x![op_layer_norm];
x![op_leaky_relu];
x![op_length_split];
x![op_lengths_pad];
x![op_lengths_reducer_fused_8bit_rowwise_ops];
x![op_lengths_reducer_fused_nbit_rowwise_ops];
x![op_lengths_reducer_ops];
x![op_lengths_reducer_rowwise_8bit_ops];
x![op_lengths_tile];
x![op_lengths_top_k];
x![op_listwise_l2r];
x![op_load_save];
x![op_load_save_op_util];
x![op_local_response_normalization];
x![op_local_response_normalization_op_cudnn];
x![op_locally_connected];
x![op_locally_connected_op_impl];
x![op_locally_connected_op_util];
x![op_log];
x![op_logit];
x![op_loss];
x![op_lp_pool];
x![op_lpnorm];
x![op_lstm_unit];
x![op_lstm_utils];
x![op_map_ops];
x![op_margin_ranking_criterion];
x![op_matmul];
x![op_max_pool_with_index_gpu];
x![op_mean];
x![op_merge_id_lists];
x![op_minmax_ops];
x![op_mish];
x![op_mod];
x![op_moments];
x![op_multi_class_accuracy];
x![op_negate_gradient];
x![op_negative];
x![op_ngram_ops];
x![op_no_default_engine];
x![op_norm_planar_yuv];
x![op_normalize];
x![op_normalize_l1];
x![op_numpy_tile];
x![op_one_hot_ops];
x![op_onnx_while];
x![op_operator_fallback_gpu];
x![op_operator_fallback_gpu_test];
x![op_order_switch_ops];
x![op_order_switch_ops_cudnn];
x![op_pack_rnn_sequence];
x![op_pack_segments];
x![op_pad];
x![op_partition_ops];
x![op_percentile];
x![op_perplexity];
x![op_piecewise_linear_transform];
x![op_pool];
x![op_pool_gradient];
x![op_pool_op_cudnn];
x![op_pool_op_util];
x![op_pow];
x![op_prefetch];
x![op_prelu];
x![op_prepend_dim];
x![op_quant_decode];
x![op_quantile];
x![op_quantized_int8_add];
x![op_quantized_int8_average_pool];
x![op_quantized_int8_channel_shuffle];
x![op_quantized_int8_concat];
x![op_quantized_int8_conv];
x![op_quantized_int8_conv_transpose];
x![op_quantized_int8_dequantize];
x![op_quantized_int8_fc];
x![op_quantized_int8_flatten];
x![op_quantized_int8_given_tensor_fill];
x![op_quantized_int8_leaky_relu];
x![op_quantized_int8_max_pool];
x![op_quantized_int8_quantize];
x![op_quantized_int8_relu];
x![op_quantized_int8_reshape];
x![op_quantized_int8_resize_nearest];
x![op_quantized_int8_roi_align];
x![op_quantized_int8_roi_align_op_test];
x![op_quantized_int8_sigmoid];
x![op_quantized_int8_slice];
x![op_quantized_int8_softmax];
x![op_quantized_int8_test];
x![op_quantized_int8_test_utils];
x![op_quantized_int8_transpose];
x![op_quantized_int8_utils];
x![op_rank_loss];
x![op_reciprocal];
x![op_reduce_front_back_max_ops];
x![op_reduce_front_back_mean_ops];
x![op_reduce_front_back_sum_mean_ops];
x![op_reduce_front_back_sum_ops];
x![op_reduce_ops];
x![op_reducer_functors];
x![op_reduction_ops];
x![op_relu];
x![op_relu_n];
x![op_remove_data_blocks];
x![op_replace_nan];
x![op_reservoir_sampling];
x![op_reshape];
x![op_reshape_op_gpu_test];
x![op_resize];
x![op_resize_3d];
x![op_reverse_packed_segs];
x![op_rmac_regions];
x![op_rms_norm];
x![op_rnn_hip_recurrent_op_miopen];
x![op_rnn_recurrent_network];
x![op_rnn_recurrent_network_blob_fetcher];
x![op_rnn_recurrent_network_executor];
x![op_rnn_recurrent_network_executor_gpu];
x![op_rnn_recurrent_network_executor_incl];
x![op_rnn_recurrent_op_cudnn];
x![op_roi_align];
x![op_roi_align_gradient];
x![op_roi_align_op_gpu_test];
x![op_roi_align_rotated];
x![op_roi_align_rotated_gradient];
x![op_roi_pool];
x![op_rowmul];
x![op_rsqrt];
x![op_scale];
x![op_scale_blobs];
x![op_segment_reduction];
x![op_self_binning_histogram];
x![op_selu];
x![op_sequence_ops];
x![op_shape];
x![op_sigmoid];
x![op_sigmoid_gradient];
x![op_sin];
x![op_sinh];
x![op_sinusoid_position_encoding];
x![op_slice];
x![op_softmax];
x![op_softmax_op_cudnn];
x![op_softmax_utils];
x![op_softmax_with_loss];
x![op_softplus];
x![op_softsign];
x![op_space_batch];
x![op_sparse_dropout_with_replacement];
x![op_sparse_lp_regularizer];
x![op_sparse_normalize];
x![op_sparse_to_dense];
x![op_sparse_to_dense_mask];
x![op_spatial_batch_norm];
x![op_spatial_softmax_with_loss];
x![op_sqr];
x![op_sqrt];
x![op_square_root_divide];
x![op_stats_ops];
x![op_stats_put_ops];
x![op_stop_gradient];
x![op_string_ops];
x![op_string_ops_test];
x![op_stump_func];
x![op_stylizer_ops];
x![op_summarize];
x![op_swish];
x![op_tan];
x![op_tanh];
x![op_tanh_gradient];
x![op_tensor_protos_db_input];
x![op_text_file_reader];
x![op_text_file_reader_utils];
x![op_text_file_reader_utils_test];
x![op_thresholded_relu];
x![op_tile];
x![op_top_k];
x![op_transpose];
x![op_transpose_op_cudnn];
x![op_tt_linear];
x![op_unique_ops];
x![op_unsafe_coalesce];
x![op_upsample];
x![op_utility_ops];
x![op_utility_ops_gpu_test];
x![op_utility_ops_test];
x![op_utils_cudnn];
x![op_variable_length_sequence_padding];
x![op_weighted_multi_sampling];
x![op_weighted_sample];
x![op_while];
x![op_workspace_ops];
x![op_zero_gradient];
x![operator_info];
x![opt_annotations];
x![opt_backend_cutting];
x![opt_backend_cutting_test];
x![opt_backend_transformer_base];
x![opt_bound_shape_inference_test];
x![opt_bound_shape_inferencer];
x![opt_converter];
x![opt_converter_nomigraph_test];
x![opt_custom_cc_amrc];
x![opt_custom_concat_elim];
x![opt_custom_concat_elim_test];
x![opt_custom_converter];
x![opt_custom_converter_test];
x![opt_custom_freeze_quantization_params];
x![opt_custom_in_batch_broadcast];
x![opt_custom_in_batch_broadcast_test];
x![opt_custom_pointwise_elim];
x![opt_dead_code_elim];
x![opt_dead_code_elim_test];
x![opt_device];
x![opt_device_test];
x![opt_distributed];
x![opt_distributed_converter];
x![opt_distributed_test];
x![opt_fakefp16_transform];
x![opt_fusion];
x![opt_glow_net_transform];
x![opt_mobile];
x![opt_mobile_test];
x![opt_nql_ast];
x![opt_nql_graphmatcher];
x![opt_nql_tests_graphmatchertest];
x![opt_onnx_convert];
x![opt_onnxifi];
x![opt_onnxifi_transformer];
x![opt_optimize_ideep];
x![opt_optimizer];
x![opt_passes];
x![opt_shape_info];
x![opt_split_slss_test];
x![opt_tvm_transformer];
x![output_column_max_histogram_net_observer];
x![output_column_max_histogram_observer];
x![output_min_max_net_observer];
x![output_min_max_observer];
x![perfkernels_adagrad];
x![perfkernels_adagrad_avx2];
x![perfkernels_common];
x![perfkernels_cvtsh_ss_bugfix];
x![perfkernels_embedding_lookup];
x![perfkernels_embedding_lookup_avx2];
x![perfkernels_embedding_lookup_fused_8bit_rowwise_avx2];
x![perfkernels_embedding_lookup_fused_8bit_rowwise_idx_avx2];
x![perfkernels_embedding_lookup_idx];
x![perfkernels_embedding_lookup_idx_avx2];
x![perfkernels_fused_8bit_rowwise_embedding_lookup];
x![perfkernels_fused_8bit_rowwise_embedding_lookup_idx];
x![perfkernels_fused_nbit_rowwise_conversion];
x![perfkernels_lstm_unit_cpu];
x![perfkernels_lstm_unit_cpu_avx2];
x![perfkernels_lstm_unit_cpu_common];
x![perfkernels_math];
x![perfkernels_math_cpu_avx2];
x![perfkernels_math_cpu_base];
x![perfkernels_typed_axpy];
x![perfkernels_typed_axpy_avx2];
x![perfkernels_typed_axpy_avx];
x![predictor_config];
x![predictor_emulator_benchmark];
x![predictor_emulator_data_filler];
x![predictor_emulator_data_filler_test];
x![predictor_emulator_emulator];
x![predictor_emulator_net_supplier];
x![predictor_emulator_output_formatter];
x![predictor_emulator_profiler];
x![predictor_emulator_std_output_formatter];
x![predictor_emulator_time_profiler];
x![predictor_emulator_utils];
x![predictor_inferencegraph];
x![predictor_predictor];
x![predictor_test];
x![predictor_threadlocalptr];
x![predictor_transforms];
x![predictor_utils];
x![quant_utils];
x![queue_blobs_queue];
x![queue_blobs_queue_db];
x![queue_queue_ops];
x![queue_rebatching_queue];
x![queue_rebatching_queue_ops];
x![register_quantization_params_net_observer];
x![register_quantization_params_with_histogram_net_observer];
x![registry];
x![serialization];
x![serialize_crc];
x![serialize_crc_alt];
x![serialize_file_adapter];
x![serialize_inline_container];
x![serialize_inline_container_test];
x![serialize_istream_adapter];
x![serialize_read_adapter_interface];
x![serialize_versions];
x![server_quantize_activation_distribution_observer];
x![server_quantize_batch_matmul_dnnlowp];
x![server_quantize_batch_permutation_dnnlowp];
x![server_quantize_caffe2_dnnlowp_utils];
x![server_quantize_channel_shuffle_dnnlowp];
x![server_quantize_compute_equalization_scale];
x![server_quantize_concat_dnnlowp];
x![server_quantize_conv_dnnlowp];
x![server_quantize_conv_dnnlowp_acc16];
x![server_quantize_conv_pool_dnnlowp_op_base];
x![server_quantize_conv_relu];
x![server_quantize_dequantize_dnnlowp];
x![server_quantize_dnnlowp];
x![server_quantize_dnnlowp_partition];
x![server_quantize_dynamic_histogram];
x![server_quantize_dynamic_histogram_test];
x![server_quantize_elementwise_add_dnnlowp];
x![server_quantize_elementwise_dnnlowp];
x![server_quantize_elementwise_linear_dnnlowp];
x![server_quantize_elementwise_mul_dnnlowp];
x![server_quantize_elementwise_sum_benchmark];
x![server_quantize_elementwise_sum_dnnlowp];
x![server_quantize_elementwise_sum_dnnlowp_op_avx2];
x![server_quantize_elementwise_sum_relu];
x![server_quantize_fb_fc_packed];
x![server_quantize_fbgemm_fp16_pack];
x![server_quantize_fbgemm_pack];
x![server_quantize_fbgemm_pack_blob];
x![server_quantize_fbgemm_pack_matrix_cache];
x![server_quantize_fc_fake_lowp_test];
x![server_quantize_fully_connected_dnnlowp];
x![server_quantize_fully_connected_dnnlowp_acc16];
x![server_quantize_fully_connected_fake_lowp];
x![server_quantize_fully_connected_fake_lowp_op_avx2];
x![server_quantize_group_norm_dnnlowp];
x![server_quantize_group_norm_dnnlowp_op_avx2];
x![server_quantize_im2col_dnnlowp];
x![server_quantize_int8_gen_quant_params];
x![server_quantize_int8_gen_quant_params_min_max];
x![server_quantize_int8_quant_scheme_blob_fill];
x![server_quantize_kl_minimization];
x![server_quantize_kl_minimization_example];
x![server_quantize_l1_minimization_example];
x![server_quantize_l2_minimization];
x![server_quantize_l2_minimization_approx_example];
x![server_quantize_l2_minimization_example];
x![server_quantize_l2_minimization_test];
x![server_quantize_lstm_unit_dnnlowp];
x![server_quantize_mmio];
x![server_quantize_norm_minimization];
x![server_quantize_norm_minimization_avx2];
x![server_quantize_op_wrapper];
x![server_quantize_p99_example];
x![server_quantize_pool_dnnlowp];
x![server_quantize_pool_dnnlowp_op_avx2];
x![server_quantize_quantization_error_minimization];
x![server_quantize_quantize_dnnlowp];
x![server_quantize_relu_dnnlowp];
x![server_quantize_relu_dnnlowp_op_avx2];
x![server_quantize_requantization_test];
x![server_quantize_resize_nearest_3d_dnnlowp];
x![server_quantize_resize_nearest_dnnlowp];
x![server_quantize_sigmoid];
x![server_quantize_sigmoid_dnnlowp];
x![server_quantize_sigmoid_test];
x![server_quantize_spatial_batch_norm_dnnlowp];
x![server_quantize_spatial_batch_norm_dnnlowp_op_avx2];
x![server_quantize_spatial_batch_norm_relu];
x![server_quantize_tanh];
x![server_quantize_tanh_dnnlowp];
x![server_quantize_tanh_test];
x![server_quantize_transpose];
x![server_quantize_utility_dnnlowp_ops];
x![sgd_adadelta];
x![sgd_adagrad];
x![sgd_adagrad_fused];
x![sgd_adam];
x![sgd_clip_tensor];
x![sgd_fp16_momentum_sgd];
x![sgd_fp32_momentum_sgd];
x![sgd_ftrl];
x![sgd_gftrl];
x![sgd_iter];
x![sgd_lars];
x![sgd_learning_rate];
x![sgd_learning_rate_adaption];
x![sgd_learning_rate_functors];
x![sgd_math_lp];
x![sgd_momentum_sgd];
x![sgd_rmsprop];
x![sgd_rowwise_adagrad_fused];
x![sgd_rowwise_counter];
x![sgd_storm];
x![sgd_weight_scale];
x![sgd_wngrad];
x![sgd_yellowfin];
x![store_interleaved];
x![sum_into];
x![tensor_deserializer];
x![tensor_serializer];
x![txform_common_subexpression_elimination];
x![txform_common_subexpression_elimination_test];
x![txform_conv_to_nnpack_transform];
x![txform_conv_to_nnpack_transform_test];
x![txform_pattern_net_transform];
x![txform_pattern_net_transform_test];
x![txform_single_op_transform];
x![util_bench_utils];
x![util_cast];
x![util_cast_test];
x![util_conversions];
x![util_cpu_neon];
x![util_cpuid];
x![util_cpuid_test];
x![util_fatal_signal_asan_no_sig_test];
x![util_filler];
x![util_fixed_divisor];
x![util_fixed_divisor_test];
x![util_hip_math_blas_gpu_test];
x![util_math];
x![util_math_broadcast];
x![util_math_cpu];
x![util_math_elementwise];
x![util_math_gpu_test];
x![util_math_half_utils];
x![util_math_reduce];
x![util_math_test];
x![util_math_transpose];
x![util_math_utils];
x![util_murmur_hash3];
x![util_proto_utils];
x![util_proto_utils_test];
x![util_proto_wrap];
x![util_signal_handler];
x![util_smart_tensor_printer];
x![util_smart_tensor_printer_test];
x![util_string_utils];
x![util_threadpool_pthreadpool];
x![util_threadpool_pthreadpool_impl];
x![util_threadpool_thread_pool_guard];
x![util_threadpool_threadpool];
x![util_threadpool_workerspool];
x![util_zmq_helper];
x![video_decoder];
x![video_input];
x![video_io];
x![video_optical_flow];
x![align];
x![graph];
x![named];

pub trait GradientMakerBase {
    #[inline] fn get_gradient_defs(&mut self) -> Vec<OperatorDef>;
    #[inline] fn copy_arguments(&self) -> bool { false }
}

