crate::ix!();

use crate::{
    OperatorStorage,
};

/**
  | Shrink the data tensor by removing data
  | blocks with given zero-based indices
  | in the outermost dimension of the tensor.
  | 
  | Indices are not assumed in any order
  | or unique but with the range [0, blocks_size).
  | Indices could be empty.
  |
  */
pub struct RemoveDataBlocksOp<Context> {
    //USE_OPERATOR_CONTEXT_FUNCTIONS
    //USE_SIMPLE_CTOR_DTOR(RemoveDataBlocksOp);
    //USE_DISPATCH_HELPER;
    storage: OperatorStorage,
    context: Context,
}

input_tags!{
    RemoveDataBlocksOp {
        Data,
        Indices
    }
}

register_cpu_operator!{RemoveDataBlocks, RemoveDataBlocksOp<CPUContext>}

num_inputs!{RemoveDataBlocks, 2}

num_outputs!{RemoveDataBlocks, 1}

inputs!{RemoveDataBlocks, 
    0 => ("data", "a N-D data tensor, N >= 1"),
    1 => ("indices", "zero-based indices of blocks to be removed")
}

outputs!{RemoveDataBlocks, 
    0 => ("shrunk data", "data after removing data blocks indexed by 'indices'")
}

should_not_do_gradient!{RemoveDataBlocks}

impl<Context> RemoveDataBlocksOp<Context> {
    
    #[inline] pub fn run_on_device(&mut self) -> bool {
        
        todo!();
        /*
            if (Input(INDICES).sizes()[0] == 0) {
          Output(0)->CopyFrom(Input(0));
          return true;
        } else {
          return DispatchHelper<TensorTypes<int, long>>::call(this, Input(INDICES));
        }
        */
    }
    
    #[inline] pub fn do_run_with_type<T>(&mut self) -> bool {
    
        todo!();
        /*
            const auto& data = Input(DATA);
        const auto& indices = Input(INDICES);
        CAFFE_ENFORCE(data.dim() > 0, "DATA should be at leat 1-D.");
        CAFFE_ENFORCE(indices.dim() == 1, "INDICES should be 1-D.");

        const auto outer_size = data.sizes()[0];
        const auto block_size = data.size_from_dim(1);
        const auto block_size_bytes = block_size * data.dtype().itemsize();
        auto indices_size = indices.sizes()[0];
        const char* data_ptr = (char*)data.raw_data();
        const auto* ind_ptr = indices.template data<T>();

        std::vector<T> ind_vec;
        for (int64_t i = 0; i < indices_size; i++) {
          ind_vec.push_back(ind_ptr[i]);
        }
        std::sort(ind_vec.begin(), ind_vec.end());
        CAFFE_ENFORCE(ind_vec[0] >= 0, "The min index should be larger than zero.");
        CAFFE_ENFORCE(
            ind_vec[indices_size - 1] < outer_size,
            "The max index should be smaller than the data outer size.");
        // removes duplicate indices
        ind_vec.erase(std::unique(ind_vec.begin(), ind_vec.end()), ind_vec.end());
        indices_size = ind_vec.size();

        auto* output = Output(0);
        auto shape = data.sizes().vec();
        shape[0] -= indices_size;
        output->Resize(shape);
        char* out_ptr = (char*)output->raw_mutable_data(data.dtype());

        ind_vec.insert(ind_vec.begin(), -1);
        int64_t ind_vec_size = ind_vec.size();
        for (auto i = 0; i < ind_vec_size; i++) {
          int64_t interval_start = ind_vec[i] + 1;
          int64_t interval_end =
              (i == ind_vec_size - 1) ? outer_size : ind_vec[i + 1];
          auto num_items = interval_end - interval_start;
          context_.CopyItemsSameDevice(
              data.dtype(),
              num_items * block_size,
              data_ptr + block_size_bytes * interval_start,
              out_ptr);
          out_ptr += block_size_bytes * num_items;
        }

        return true;
        */
    }
}
