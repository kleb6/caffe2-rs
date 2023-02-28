crate::ix!();

use crate::{
    OperatorStorage,
    CPUContext,
    Tensor
};

/**
 | This operator works as bucketize in tensorflow and
 | digitize in numpy. It bucketizes the input 'X'
 | based on argument 'boundaries'.
 |
 | For each value x in input 'data', the operator
 | returns index i given boundaries[i-1] < x <=
 | boundaries[i].
 |
 | If values in 'data' are beyond the bounds of
 | boundaries, 0 or len(boundaries) is returned as
 | appropriate.
 |
 | The boundaries need to be monotonically
 | increasing.
 |
 | For example
 |
 | If data = [2, 4, 1] and boundaries = [0.1, 2.5],
 | then
 |
 | output = [1, 2, 1]
 |
 | If data = [[2, 3], [4, 1], [2, 5]] 
 | and boundaries = [0.1, 2.5], then
 |
 | output = [[1, 2], [2, 1], [1, 2]]
 */
pub struct BucketizeOp<Context> {

    //USE_OPERATOR_CONTEXT_FUNCTIONS;
    storage: OperatorStorage,
    context: Context,

    boundaries: Vec<f32>,

    boundaries_device: Tensor, //{Context::GetDeviceType()};
}

impl<Context> BucketizeOp<Context> {
    
    pub fn new<Args>(args: Args) -> Self {
        todo!();
        /*
            : Operator<Context>(std::forward<Args>(args)...),
            boundaries_(this->template GetRepeatedArgument<float>("boundaries")) 

        CAFFE_ENFORCE(
            std::is_sorted(boundaries_.begin(), boundaries_.end()),
            "The boundaries need to be monotonically increasing");

        boundaries_device_.Resize(boundaries_.size());
        context_.template CopyFromCPU<float>(
            boundaries_.size(),
            boundaries_.data(),
            boundaries_device_.mutable_data<float>());
        context_.FinishDeviceComputation();
        */
    }
}

impl BucketizeOp<CPUContext> {

    #[inline] pub fn run_on_device(&mut self) -> bool {
        
        todo!();
        /*
            auto& input = Input(X);
      CAFFE_ENFORCE_GE(input.dim(), 1);

      auto N = input.numel();
      auto* output = Output(INDICES, input.sizes(), at::dtype<int32_t>());
      const auto* input_data = input.template data<float>();
      auto* output_data = output->template mutable_data<int32_t>();

      math::Set<int32_t, CPUContext>(output->numel(), 0.0, output_data, &context_);

      for (int64_t pos = 0; pos < N; pos++) {
        // here we assume the boundary values for each feature are sorted
        auto bucket_idx =
            std::lower_bound(
                boundaries_.begin(), boundaries_.end(), input_data[pos]) -
            boundaries_.begin();
        output_data[pos] = bucket_idx;
      }

      return true;
        */
    }
}

input_tags!{
    BucketizeOp {
        X
    }
}

output_tags!{
    BucketizeOp {
        Indices
    }
}

register_cpu_operator!{Bucketize, BucketizeOp<CPUContext>}

num_inputs!{Bucketize, 1}

num_outputs!{Bucketize, 1}

inputs!{Bucketize, 
    0 => ("data", "input tensor")
}

outputs!{Bucketize, 
    0 => ("output", "indices of bins given by boundaries to which each value in data belongs")
}

args!{Bucketize, 
    0 => ("boundaries", "bucketization boundaries")
}

tensor_inference_function!{Bucketize,
        /*[](const OperatorDef& /* def */,
                                const vector<TensorShape>& in) {
      vector<TensorShape> out(in);
      out[0].set_data_type(TensorProto::INT32);
      return out;
    }*/}

no_gradient!{BucketizeOp}

type BucketizeInt = BucketizeOp<CPUContext>;

