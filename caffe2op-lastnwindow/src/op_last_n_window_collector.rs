crate::ix!();

/**
 | Collect the last N rows from input data. The
 | purpose is to keep track of data accross batches,
 | so for example suppose the LastNWindowCollector is
 | called successively with the following input data
 |
 |   [1, 2, 3, 4]
 |   [5, 6, 7]
 |   [8, 9, 10, 11]
 |
 | And the number of items is set to 6, then the
 | output after the 3rd call will contain the
 | following elements:
 |
 |   [6, 7, 8, 9, 10, 11]
 |
 | No guarantee is made on the ordering of elements
 | in input. 
 |
 | So a valid value for output could have been
 |
 |   [11, 10, 9, 8, 7, 6]
 |
 | Also, this method works for any order tensor,
 | treating the first dimension as input rows and
 | keeping the last N rows seen as input. So for
 | instance:
 |
 |   [[1, 2], [2, 3], [3, 4], [4, 5]]
 |   [[5, 6], [6, 7], [7, 8]]
 |   [[8, 9], [9, 10], [10, 11], [11, 12]]
 |
 | A possible output would be
 |
 |   [[6, 7], [7, 8], [8, 9], [9, 10], [10, 11], [11, 12]]
 |
 | This is not thread safe unless a mutex is given.
 */
#[USE_OPERATOR_CONTEXT_FUNCTIONS]
pub struct LastNWindowCollectorOp<Context> {
    storage:        OperatorStorage,
    context:        Context,
    num_to_collect: i32,
}

num_inputs!{LastNWindowCollector, (3,4,5)}

num_outputs!{LastNWindowCollector, (2,3)}

inputs!{LastNWindowCollector, 
    0 => ("last-N buffer", "The buffer for last-N record. Should be initialized to empty tensor"),
    1 => ("next cursor",   "The cursor pointing to the next position that should be replaced. Should be initialized to 0."),
    2 => ("DATA",          "tensor to collect from"),
    3 => ("MUTEX",         "(optional) mutex to use to make this thread-safe"),
    4 => ("NUM_VISITED",   "")
}

outputs!{LastNWindowCollector, 
    0 => ("last-N buffer", "Data stored in sessions"),
    1 => ("next cursor",   "Updated input cursor"),
    2 => ("NUM_VISITED",   "number of records seen so far")
}

args!{LastNWindowCollector, 
    0 => ("num_to_collect", "The number of random samples to append for each positive samples")
}

tensor_inference_function!{LastNWindowCollector, /* [](const OperatorDef& def,
                                const vector<TensorShape>& in) {
      auto output_size = def.output_size();
      vector<TensorShape> out(output_size);
      const ArgumentHelper helper(def);
      const auto num_to_collect =
          helper.GetSingleArgument<int>("num_to_collect", -1);

      const auto data_dims = GetDimsVector(in[2]);
      vector<int64_t> last_n_shape(data_dims.size());
      last_n_shape[0] = num_to_collect;
      std::copy(data_dims.begin() + 1, data_dims.end(), last_n_shape.begin() + 1);
      out[0] = CreateTensorShape(last_n_shape, in[2].data_type());

      out[1] = in[1];

      if (output_size > 2) {
        vector<int64_t> num_visited_shape(1);
        num_visited_shape[0] = 1;
        out[2] = CreateTensorShape(num_visited_shape, TensorProto::INT64);
      }

      return out;
    })*/
}

enforce_inplace!{LastNWindowCollector, vec![(0, 0), (1, 1), (4, 2)]}

input_tags!{
    LastNWindowCollectorOp {
        LastNIn,
        NextIn,
        Data,
        Mutex,
        NumVisitedIn
    }
}

output_tags!{
    LastNWindowCollectorOp {
        LastN,
        Next,
        NumVisited
    }
}

impl<Context> LastNWindowCollectorOp<Context> {
    
    pub fn new<Args>(args: Args) -> Self {
        todo!();
        /*
            : Operator<Context>(std::forward<Args>(args)...),
            numToCollect_( OperatorStorage::GetSingleArgument<int>("num_to_collect", -1)) 

        CAFFE_ENFORCE_GT(numToCollect_, 0);
        */
    }
    
    #[inline] pub fn run_on_device(&mut self) -> bool {
        
        todo!();
        /*
        if (InputSize() > MUTEX) {
          auto& mutex = OperatorStorage::Input<std::unique_ptr<std::mutex>>(MUTEX);
          std::lock_guard<std::mutex> guard(*mutex);
          return collect();
        } else {
          return collect();
        }
        */
    }
    
    #[inline] pub fn collect(&mut self) -> bool {
        
        todo!();
        /*
        auto* output = Output(LAST_N);
        const auto& input = Input(DATA);

        CAFFE_ENFORCE_GE(input.dim(), 1);
        bool output_initialized = output->numel() > 0 &&
            (static_cast<std::shared_ptr<std::vector<TensorCPU>>*>(
                 output->raw_mutable_data(input.dtype()))[0] != nullptr);
        if (output_initialized) {
          CAFFE_ENFORCE_EQ(output->dim(), input.dim());
          for (size_t i = 1; i < input.dim(); ++i) {
            CAFFE_ENFORCE_EQ(output->size(i), input.size(i));
          }
        }

        auto num_entries = input.sizes()[0];

        if (OutputSize() > NUM_VISITED) {
          auto* num_visited_tensor = Output(NUM_VISITED);
          CAFFE_ENFORCE_EQ(1, num_visited_tensor->numel());
          auto* num_visited = num_visited_tensor->template mutable_data<int64_t>();
          if (!output_initialized) {
            *num_visited = 0;
          }
          CAFFE_ENFORCE_GE(*num_visited, 0);
          *num_visited += num_entries;
        }

        if (!output_initialized) {
          auto dims = input.sizes().vec();
          dims[0] = 0;
          output->Resize(dims);
          // pass meta to output
          output->raw_mutable_data(input.dtype());
          output->ReserveSpace(numToCollect_);
        }

        if (num_entries == 0) {
          if (!output_initialized) {
            // Get both shape and meta
            output->CopyFrom(input, true /*async*/);
          }
          return true;
        }

        auto num_to_copy = std::min<int32_t>(num_entries, numToCollect_);
        auto output_batch_size = output_initialized ? output->size(0) : 0;
        auto output_num =
            std::min<size_t>(numToCollect_, output_batch_size + num_to_copy);

        // output_num is >= output_batch_size
        if (output_num > output_batch_size) {
          output->ExtendTo(output_num, 50);
        }

        auto* output_data =
            static_cast<char*>(output->raw_mutable_data(input.dtype()));

        auto* next = Output(NEXT);
        CAFFE_ENFORCE_EQ(0, next->dim());
        auto* next_data = next->template mutable_data<int32_t>();
        if (!output_initialized) {
          *next_data = 0;
        }
        CAFFE_ENFORCE_LT(*next_data, output->size(0));

        auto block_size = input.size_from_dim(1);
        auto block_bytesize = block_size * input.itemsize();
        const auto* input_data = static_cast<const char*>(input.raw_data());

        if (num_entries > numToCollect_) {
          // just copy the last N rows
          context_.CopyItemsSameDevice(
              input.dtype(),
              num_to_copy * block_size,
              input_data + (num_entries - numToCollect_) * block_bytesize,
              output_data);
          *next_data = 0;
          return true;
        }
        auto start = *next_data;
        auto first_chunk_size =
            std::min<size_t>(num_to_copy + start, numToCollect_) - start;
        context_.CopyItemsSameDevice(
            input.dtype(),
            first_chunk_size * block_size,
            input_data,
            output_data + start * block_bytesize);

        context_.CopyItemsSameDevice(
            input.dtype(),
            (num_to_copy - first_chunk_size) * block_size,
            input_data + first_chunk_size * block_bytesize,
            output_data);

        *next_data = (start + num_to_copy) % numToCollect_;

        return true;
        */
    }
}

register_cpu_operator!{
    LastNWindowCollector, 
    LastNWindowCollectorOp<CPUContext>
}

should_not_do_gradient!{LastNWindowCollector}
