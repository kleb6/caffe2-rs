crate::ix!();

use crate::{
    OperatorStorage
};

/**
  | Computes a histogram for values in the
  | given list of tensors.
  | 
  | For logging activation histograms
  | for post-hoc analyses, consider using
  | the
  | 
  | HistogramObserver observer.
  | 
  | For iteratively computing a histogram
  | for all input tensors encountered through
  | history, consider using the AccumulateHistogram
  | operator.
  |
  */
pub struct SelfBinningHistogramOp<Context> {

    //USE_OPERATOR_CONTEXT_FUNCTIONS
    storage:         OperatorStorage,
    context:         Context,

    num_bins:        i32,
    num_edges:       i32,
    bin_spacing:     String,
    logspace_start:  f32,

    /**
      | automatically apply abs() on the input
      | values
      |
      */
    abs:             bool,
}

output_tags!{
    SelfBinningHistogramOp {
        HistogramValues,
        HistogramCounts
    }
}

register_cpu_operator!{SelfBinningHistogram, SelfBinningHistogramOp<CPUContext>}

num_inputs!{SelfBinningHistogram, (1,INT_MAX)}

num_outputs!{SelfBinningHistogram, 2}

inputs!{SelfBinningHistogram, 
    0 => ("X1, X2, ...",      "*(type: Tensor`<float>`)* List of input tensors.")
}

outputs!{SelfBinningHistogram, 
    0 => ("histogram_values", "1D tensor of edges of the bins, of dimension [num_bins+1]. The range appears as: [first, ..., last), wherein the i-th element expresses the start of a bin and i+1-th value represents the exclusive end of that bin."),
    1 => ("histogram_counts", "1D tensor of counts of each bin, of dimension [num_bins+1]. It is guaranteed to end with a 0 since the last edge is exclusive.")
}

args!{SelfBinningHistogram, 
    0 => ("num_bins",         "Number of bins to use for the histogram. Must be >= 1."),
    1 => ("bin_spacing",      "A string indicating 'linear' or 'logarithmic' spacing for the bins."),
    2 => ("logspace_start",   "A float that's used as the starting point for logarithmic spacing. Since logarithmic spacing cannot contain <=0 values this value will be used to represent all such values."),
    3 => ("abs",              "Apply abs() on every input value.")
}

should_not_do_gradient!{SelfBinningHistogram}

impl<Context> SelfBinningHistogramOp<Context> {

    pub fn new<Args>(args: Args) -> Self {
    
        todo!();
        /*
            : Operator<Context>(std::forward<Args>(args)...),
            num_bins_(this->template GetSingleArgument<int>("num_bins", 0)),
            num_edges_(num_bins_ + 1),
            bin_spacing_(this->template GetSingleArgument<std::string>( "bin_spacing", "linear")),
            logspace_start_(this->template GetSingleArgument<float>("logspace_start", 1e-24)),
            abs_(this->template GetSingleArgument<bool>("abs", false))

        CAFFE_ENFORCE_GE(
            num_bins_, 1, "Number of bins must be greater than or equal to 1.");
        CAFFE_ENFORCE(
            bin_spacing_ == "linear" || bin_spacing_ == "logarithmic",
            "Bin spacing can be one of 'linear' or 'logarithmic'."
        );
        CAFFE_ENFORCE_GT(
            logspace_start_, 0,
            "Logarithmic spacing base is a multiplier and is expected to be >1.");
        */
    }
    
    #[inline] pub fn run_on_device(&mut self) -> bool {
        
        todo!();
        /*
            return DispatchHelper<TensorTypes<float, double>>::call(this, Input(0));
        */
    }
    
    #[inline] pub fn do_run_with_type<T>(&mut self) -> bool {
    
        todo!();
        /*
            CheckInputs();

        // Scale the range so that the last count is always 0.
        const T RANGE_SCALING = 1.0001;

        const auto* histogram_values = Output(HISTOGRAM_VALUES);
        histogram_values->Resize(num_edges_);
        auto* histogram_values_data = histogram_values->template mutable_data<T>();
        const auto* histogram_counts = Output(HISTOGRAM_COUNTS);
        histogram_counts->Resize(num_edges_);
        auto* histogram_counts_data =
            histogram_counts->template mutable_data<int64_t>();

        // Calculate the max and min.
        bool first_seen = false;
        // 0 initialization is arbitrary here to suppress linter warnings.
        // The actual initialization check happens through the first_seen variable.
        T max = 0;
        T min = 0;
        int64_t total_count = 0;
        for (int input_idx = 0; input_idx < InputSize(); input_idx++) {
          const auto& x = Input(input_idx);
          const int64_t N = x.numel();
          total_count += N;
          const auto* x_data = x.template data<T>();
          for (int64_t data_idx = 0; data_idx < N; data_idx++) {
            const T val = this->abs_ ? abs(x_data[data_idx]) :  x_data[data_idx];
            if (!first_seen) {
              max = val;
              min = val;
              first_seen = true;
            } else {
              max = std::max(val, max);
              min = std::min(val, min);
            }
          }
        }

        if (!first_seen) {
          math::Set<T, Context>(num_edges_, 0, histogram_values_data, &context_);
          math::Set<int64_t, Context>(
              num_edges_, 0, histogram_counts_data, &context_);
          return true;
        }

        CAFFE_ENFORCE(min <= max, "Incorrect min-max computation");
        T scaled_max = 0;  // this is set in both branches
        if (bin_spacing_ == "linear") {
          // Let's scale the range so that the last count is 0.
          scaled_max = min + (max - min) * RANGE_SCALING;
          T scaled_range = (scaled_max - min);
          // Avoid underflow by calculating advancement through multiplication.
          for (int i = 0; i < num_edges_; i++) {
            T advancement_ratio = T(i) / num_bins_;
            histogram_values_data[i] = min + advancement_ratio * scaled_range;
          }
        } else if (bin_spacing_ == "logarithmic") {
          // First, we need to sanitize the range.
          if (min < logspace_start_) {
            min = logspace_start_;
          }
          if (max < logspace_start_) {
            max = logspace_start_;
          }
          T linear_range = max - min;
          linear_range = linear_range * RANGE_SCALING;
          scaled_max = min + linear_range;
          // Calculate base interval using geometric sum.
          // Simply: multiplier = exp((log(max) - log(min))/N)
          // Avoid underflow by delaying division and exp.
          T log_multiplier_numerator =log(scaled_max) - log(min);
          // Avoid underflow by:
          // - Calculating each advancement separately for each i.
          for (int i = 0; i < num_edges_; i++) {
            T advancement_ratio = T(i)/num_bins_;
            histogram_values_data[i] = min * exp(log_multiplier_numerator * advancement_ratio);
          }
        }

        math::Set<int64_t, Context>(
          num_edges_, 0, histogram_counts_data, &context_);
        if (histogram_values_data[num_edges_-1] <= max) {
          // In cases of min&max being equal (or any unexpected numerical underflow) we
          // may not have a final edge larger than the max.
          histogram_values_data[num_edges_-1] = scaled_max;
          histogram_counts_data[0] = total_count;
        }
        else {
          for (int input_idx = 0; input_idx < InputSize(); input_idx++) {
            const auto& x = Input(input_idx);
            const int64_t N = x.numel();
            const auto* x_data = x.template data<T>();
            for (int64_t data_idx = 0; data_idx < N; data_idx++) {
              const T val = this->abs_ ? abs(x_data[data_idx]) :  x_data[data_idx];
              const auto bisection_it = std::upper_bound(
                  histogram_values_data,
                  histogram_values_data + num_edges_,
                  val);
              const int bisection_idx = bisection_it - histogram_values_data;
              if (bisection_idx > 0 && bisection_idx < num_edges_) {
                histogram_counts_data[bisection_idx - 1]++;
              }
              if (bisection_idx == 0) {
                histogram_counts_data[0]++;
              }
            }
          }
        }

        return true;
        */
    }
    
    #[inline] pub fn check_inputs(&mut self)  {
        
        todo!();
        /*
            const auto& input_zero = Input(0);
        for (int i = 1; i < InputSize(); i++) {
          CAFFE_ENFORCE_EQ(
              Input(i).dtype(),
              input_zero.dtype(),
              "All inputs must have the same type; expected ",
              input_zero.dtype().name(),
              " but got ",
              Input(i).dtype().name(),
              " for input ",
              i);
        }
        */
    }
}
