crate::ix!();

declare_int!{caffe2_streams_per_gpu}
declare_int!{caffe2_net_async_max_gpus}
declare_int!{caffe2_net_async_max_numa_nodes}
declare_int!{caffe2_net_async_thread_pool_size}
declare_bool!{caffe2_net_async_check_stream_status}
declare_bool!{caffe2_net_async_use_single_pool}
declare_bool!{caffe2_net_async_use_per_net_pools}
declare_bool!{caffe2_net_async_run_root_tasks_inline}
declare_bool!{caffe2_net_async_profile_operators}
declare_string!{caffe2_net_async_tracing_filepath}
declare_string!{caffe2_net_async_names_to_trace}
declare_int!{caffe2_net_async_tracing_nth}
