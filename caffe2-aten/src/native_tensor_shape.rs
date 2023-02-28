crate::ix!();

//-------------------------------------------[.cpp/pytorch/aten/src/ATen/native/TensorShape.cpp]

define_dispatch!{cat_serial_stub}
define_dispatch!{stack_serial_stub}


pub fn reshape_from_tensor(
        self_:        &Tensor,
        shape_tensor: &Tensor) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(shape_tensor.dim() == 1);
      vector<i64> shape;
      auto accessor = shape_tensor.accessor<i64, 1>();
      for (const auto i : irange(shape_tensor.numel())) {
        shape.push_back(accessor[i]);
      }
      return self.reshape(IntArrayRef(shape));
        */
}


pub fn shape_as_tensor(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            auto options = TensorOptions(kLong);
      return tensor(self.sizes(), options);
        */
}


pub fn set(
        result: &mut Tensor,
        source: Storage) -> &mut Tensor {
    
    todo!();
        /*
            i64 new_size =
          static_cast<i64>(source.nbytes() / result.dtype().itemsize());
      return result.set_(source, 0, new_size, {});
        */
}

/**
  | unify with cuda implementation? This
  | is not done to avoid a dispatch in resize_impl_cpu_
  |
  */
pub fn set_storage_cpu(
        result:         &mut Tensor,
        storage:        Storage,
        storage_offset: i64,
        size:           &[i32],
        stride:         &[i32]) -> &mut Tensor {
    
    todo!();
        /*
            checkSetStorage(result, storage, storage_offset, size, stride);

      result.unsafeGetTensorImpl()->set_storage_offset(storage_offset);
      optional<IntArrayRef> stride_opt = stride.data() != nullptr ?
                                              optional<IntArrayRef>(stride) : nullopt;
      native::resize_impl_cpu_(result.unsafeGetTensorImpl(), size, stride_opt);
      return result;
        */
}


pub fn set_tensor(
        result: &mut Tensor,
        source: &Tensor) -> &mut Tensor {
    
    todo!();
        /*
            if (result.unsafeGetTensorImpl() != source.unsafeGetTensorImpl()) {
        return result.set_(source.storage(), source.storage_offset(), source.sizes(), source.strides());
      }
      return result;
        */
}

/**
  | this needs to be split along CPU/CUDA
  | lines because we don't have a consistent
  | way of getting the allocator to use for
  | a device (GetAllocator is not the same
  | as getCUDADeviceAllocator().
  |
  */
pub fn set_cpu(result: &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TypeMeta dtype = result.dtype();
      Storage storage(
          Storage::use_byte_Size(),
          0,
          GetAllocator(kCPU),
          true);
      result.set_(storage, 0, {0}, {});
      TORCH_INTERNAL_ASSERT(dtype == result.dtype());
      return result;
        */
}


pub fn broadcast_to(
        self_: &Tensor,
        size:  &[i32]) -> Tensor {
    
    todo!();
        /*
            return self.expand(size);
        */
}


pub fn broadcast_tensors(tensors: TensorList) -> Vec<Tensor> {
    
    todo!();
        /*
            return expand_outplace(tensors);
        */
}

/**
  | Check to see if the shape of tensors is
  | compatible for being concatenated
  | along a given dimension.
  |
  */
#[inline] pub fn check_cat_shape_except_dim(
        first:     &Tensor,
        second:    &Tensor,
        dimension: i64,
        index:     i64)  {
    
    todo!();
        /*
            i64 first_dims = first.dim();
      i64 second_dims = second.dim();
      TORCH_CHECK(first_dims == second_dims, "torch.cat(): Tensors must have same number of dimensions: got ",
                  first_dims, " and ", second_dims);
      for (i64 dim = 0; dim < first_dims; dim++) {
        if (dim == dimension) {
          continue;
        }
        i64 first_dim_size = first.sizes()[dim];
        i64 second_dim_size = second.sizes()[dim];
        TORCH_CHECK(first_dim_size == second_dim_size, "torch.cat(): Sizes of tensors must match except in dimension ",
                    dimension, ". Got ", first_dim_size, " and ", second_dim_size, " in dimension ", dim,
                    " (The offending index is ", index, ")");
      }
        */
}


pub fn should_skip(t: &Tensor) -> bool {
    
    todo!();
        /*
            return t.numel() == 0 && t.dim() == 1;
        */
}


pub fn cat_out_cpu(
        tensors: TensorList,
        dim:     i64,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            // previously, size [0] tensors were the only possible empty tensors; thus, it wasn't possible
      // to cat empty tensors unless all the other tensors were 1-dimensional, so we allowed these tensors
      // to be "skipped".  We maintain this behavior for backwards compatibility, but only for this specific
      // size (i.e. other empty sizes are not skipped).

      bool allContiguous = true;

      // Inputs cannot alias the output tensor
      for (const auto i : irange(tensors.size())) {
        auto lap = get_overlap_status(result, tensors[i]);
        TORCH_CHECK(lap != MemOverlapStatus::PARTIAL &&
            lap != MemOverlapStatus::FULL, 0,
            "unsupported operation: the input tensors cannot refer to any of the "
            "output memory locations. Found overlap in input tensor ", i);
      }
      assert_no_internal_overlap(result);

      const Tensor* pnotSkippedTensor = [](TensorList tensors) -> const Tensor* {
        for (auto const &tensor : tensors) {
          if (should_skip(tensor)) {
            continue;
          }
          // we've found a non-empty tensor
          return &tensor;
        }
        return nullptr;
      }(tensors);

      if (!pnotSkippedTensor) {
        // FIXME: warn if this is the case -- see comment about skipped
        // tensors at top of function.
        return result;
      }
      const Tensor& notSkippedTensor = *pnotSkippedTensor;

      TORCH_CHECK(tensors.size() > 0, "torch.cat(): expected a non-empty list of Tensors");
      TORCH_CHECK(dim <= notSkippedTensor.dim(), "torch.cat(): dimension ", dim, "out of range");

      // when the input tensors are of the same size and strides,
      // reuse the same iterator for all input tensors
      bool reuse_iterator = true;
      bool no_type_promotion = true;
      // Check the type of the result
      no_type_promotion = result.dtype() == notSkippedTensor.dtype();

      // compute size of the result in the cat dimension
      i64 cat_dim_size = 0;
      auto first_tensor_mem_format = tensors[0].suggest_memory_format();
      for (const auto i : irange(tensors.size())) {
        auto const &tensor = tensors[i];
        if (should_skip(tensor)) {
          // don't use fast path for empty tensor
          allContiguous = false;
          continue;
        }
        check_cat_shape_except_dim(notSkippedTensor, tensor, dim, i);
        cat_dim_size += tensor.sizes()[dim];

        if (!tensor.is_contiguous(first_tensor_mem_format)) {
          allContiguous = false;
        }

        if (tensor.sizes() != notSkippedTensor.sizes() ||
            tensor.strides() != notSkippedTensor.strides()) {
          reuse_iterator = false;
        }
        if (tensor.dtype() != notSkippedTensor.dtype()) {
          no_type_promotion = false;
        }
      }
      // compute the size of the result
      auto result_size = notSkippedTensor.sizes().vec();
      result_size[dim] = cat_dim_size;

      // skip resizing if size of result is same as expected
      if (result.sizes() != result_size) {
        result.resize_(result_size, first_tensor_mem_format);
      }

      if (result.numel() == 0) {
        return result;
      }

      // fast path for single thread when both inputs and result are contiguous and not empty
      allContiguous = allContiguous && result.is_contiguous(first_tensor_mem_format);
      bool use_serial_kernel = result.numel() < internal::GRAIN_SIZE || get_num_threads() == 1;
      ScalarType dtype = notSkippedTensor.scalar_type();
      bool serial_dtype = (dtype == ScalarType::Double || dtype == ScalarType::Float || dtype == ScalarType::BFloat16);
      if (use_serial_kernel && allContiguous && no_type_promotion && serial_dtype) {
        cat_serial_stub(kCPU, result, tensors, dim);
        return result;
      }

      i64 offset = 0;
      if (reuse_iterator &&
          result.is_contiguous(first_tensor_mem_format) &&
          no_type_promotion) {
        const auto& source_slice = notSkippedTensor;
        auto slice_dim_size = source_slice.sizes()[dim];
        auto result_slice = result.narrow(dim, 0, slice_dim_size);
        auto result_slice_data = result_slice.data_ptr();
        auto result_stride_bytes = result.stride(dim) * elementSize(result.scalar_type());

        auto iter = TensorIteratorConfig()
          .set_check_mem_overlap(false)  // Already checked above
          .resize_outputs(false)
          .add_output(result_slice)
          .add_input(source_slice)
          .enforce_safe_casting_to_output(true)
          .build();

        for (auto const &tensor : tensors) {
          if (should_skip(tensor)) {
            continue;
          }
          auto source_data = static_cast<char*>(tensor.data_ptr());
          auto result_data = static_cast<char*>(result_slice_data) + offset * result_stride_bytes;
          iter.unsafe_replace_operand(0, result_data);
          iter.unsafe_replace_operand(1, source_data);
          copy_stub(iter.device_type(), iter, false);
          offset += slice_dim_size;
        }
      } else {
        for (auto const &tensor: tensors) {
          if (should_skip(tensor)) {
            continue;
          }
          auto slice_dim_size = tensor.sizes()[dim];
          auto result_slice = result.narrow(dim, offset, slice_dim_size);

          auto iter = TensorIteratorConfig()
            .set_check_mem_overlap(false)  // Already checked above
            .resize_outputs(false)
            .add_output(result_slice)
            .add_input(tensor)
            .promote_inputs_to_common_dtype(true)
            .cast_common_dtype_to_outputs(true)
            .enforce_safe_casting_to_output(true)
            .build();
          copy_stub(iter.device_type(), iter, false);
          offset += slice_dim_size;
        }
      }

      return result;
        */
}


pub fn cat_cpu(
        tensors: TensorList,
        dim:     i64) -> Tensor {
    
    todo!();
        /*
            ScalarType high_type = result_type(tensors);
      Tensor result = empty({0}, tensors[0].options().dtype(high_type));
      return native::_cat_out_cpu(tensors, dim, result);
        */
}


pub fn check_cat_no_zero_dim(tensors: TensorList)  {
    
    todo!();
        /*
            for(const auto i : irange(tensors.size())) {
        auto& t = tensors[i];
        TORCH_CHECK(t.dim() > 0,
                 "zero-dimensional tensor (at position ", i, ") cannot be concatenated");
      }
        */
}


pub fn cat_out_a(
        tensors: TensorList,
        dim:     i64,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            check_cat_no_zero_dim(tensors);
      dim = legacy_cat_wrap_dim(dim, tensors);
      auto maybe_outnames = namedinference::compute_cat_outnames(tensors);
      {
        NoNamesGuard guard;
        _cat_out(result, tensors, dim);
      }
      namedinference::propagate_names_if_nonempty(result, maybe_outnames);
      return result;
        */
}

pub fn cat_out_b(
        tensors: TensorList,
        dim:     Dimname,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TORCH_CHECK(!tensors.empty(), "expected a non-empty list of Tensors");
      return cat_out(result, tensors, dimname_to_position(tensors[0], dim));
        */
}


pub fn cat_a(
        tensors: TensorList,
        dim:     Dimname) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(!tensors.empty(), "expected a non-empty list of Tensors");
      return cat(tensors, dimname_to_position(tensors[0], dim));
        */
}

pub fn sizes_match_except(
        s1:         &[i32],
        s2:         &[i32],

        /* should already be wrapped */
        dim_except: i64) -> bool {
    
    todo!();
        /*
            if (s1.size() != s2.size()) {
        return false;
      }
      for (const auto i : irange(s1.size())) {
        if (i != dim_except && s1[i] != s2[i]) {
          return false;
        }
      }
      return true;
        */
}

/**
  | Check to see if the shape of tensors is
  | compatible for being concatenated
  | along a given dimension.
  |
  */
pub fn check_cat_sparse_dims(
        t:          &Tensor,

        /* used only for debug messages */
        pos:        i64,
        sizes:      &[i32],
        wrapped:    i64,
        sparse_dim: i64,
        dense_dim:  i64)  {
    
    todo!();
        /*
            TORCH_CHECK(t.is_sparse(),
                "Concatenating sparse tensors, but a dense tensor was found at position ", pos, ".");
        TORCH_CHECK(sizes_match_except(sizes, t.sizes(), wrapped),
                "All tensors must have the same shape: ", sizes, " (except in the concatenating dimension),"
                " but found shape: ", t.sizes(), " at position ", pos, ".");
        TORCH_CHECK(t.sparse_dim() == sparse_dim && t.dense_dim() == dense_dim,
                "All tensors must have the same sparse_dim and dense_dim: ", sparse_dim, ", ", dense_dim,
                ", but tensor at position ", pos, " has ", t.sparse_dim(), ", ", t.dense_dim(), ".");
        */
}


pub fn cat_sparse(
        tensors: TensorList,
        dim:     i64) -> Tensor {
    
    todo!();
        /*
            vector<Tensor> indices;
      vector<Tensor> values;
      i64 wrapped = maybe_wrap_dim(dim, tensors[0].dim());
      i64 sparse_dim = tensors[0].sparse_dim();
      i64 dense_dim = tensors[0].dense_dim();
      IntArrayRef sizes = tensors[0].sizes();
      if (wrapped < sparse_dim) {
        for (const auto i : irange(tensors.size())) {
          auto const &t = tensors[i];
          check_cat_sparse_dims(t, i, sizes, wrapped, sparse_dim, dense_dim);
          indices.push_back(t._indices());
          values.push_back(t._values());
        }
        Tensor idxs = cat(indices, 1);
        Tensor vals = cat(values, 0);

        // We now need to move the indices of each
        // input tensor up along `dim` by an appropriate amount.
        // E.g., if t1 has indices [[2,3,4],[5,6,7]],
        // and sizes [10, 7]
        // then torch.cat((t1,t1,t1),1) should have indices
        // [[2,3,4,2,3,4,2,3,4],[5,6,7,12,13,14,19,20,21]],
        // so we need to increase idxs[1][3:6] by 7
        // and idxs[1][6:9] by 14.
        i64 col = 0;
        i64 cumulative_offset = 0;
        for (const auto i : irange(tensors.size())) {
          auto const &t = tensors[i];
          i64 this_piece_size = t._nnz();
          // cumulative_offset is zero for the first piece, so
          // don't waste time doing this operation unless i > 0.
          if (i > 0) {
            idxs[wrapped].narrow(0, col, this_piece_size) += cumulative_offset;
          }
          cumulative_offset += t.size(wrapped);
          col += this_piece_size;
        }
        auto sizes_copy = sizes.vec();
        sizes_copy[wrapped] = cumulative_offset;
        return native::sparse_coo_tensor(
            idxs,
            vals,
            sizes_copy,
            optTypeMetaToScalarType(tensors[0].options().dtype_opt()),
            tensors[0].options().layout_opt(),
            tensors[0].options().device_opt(),
            tensors[0].options().pinned_memory_opt());
      }
      else {
        // Catting along a dense dimension requires us to create new values.
        // For illustration, consider the sparse 3d tensors t1 and t2,
        // given by t1 = [[[1,2],[3,4]], ... (zeros) ..., [[5,6],[7,8]]]
        // and t2 = [... (zeros) ..., [[9, 10], [11,12]], ... (zeros) ...],
        // Their concatenation along dimension 2 is:
        // [[[1,2,0,0],[3,4,0,0]], ... (zeros) ..., [[0,0,9,10],[0,0,11,12]], ... (zeros) ..., [[5,6,0,0],[7,8,0,0]]]
        //
        // Their values tensors are, respectively,
        // [[[1,2],[3,4]],[[5,6],[7,8]]] and [[[9,10],[11,12]]].
        //
        // and so the values tensor of their concatenation along dim 2 will be:
        // [[[1,2,0,0],[3,4,0,0]],[[5,6,0,0],[7,8,0,0]],[[0,0,9,10],[0,0,11,12]]]
        //
        // which we can get by taking the values tensor of each tensor, catting it with zeros of the appropriate size on the left and right,
        // and then catting all those results together.

        // The dimension in each tensor's values object that corresponds to the overall dimension along which we're catting.
        i64 values_dim = wrapped - sparse_dim + 1;
        // The final size along the catted dimension.
        const i64 total_size = accumulate(tensors.begin(), tensors.end(), static_cast<i64>(0), [values_dim](i64 l, Tensor const &r) {
          return l + r._values().size(values_dim);
        });
        auto zeros_sizes = tensors[0]._values().sizes().vec();
        i64 cumulative_size = 0;
        vector<Tensor> vals_pieces;
        vector<Tensor> idxs_pieces;
        for (const auto i : irange(tensors.size())) {
          auto const &t = tensors[i];
          check_cat_sparse_dims(t, i, sizes, wrapped, sparse_dim, dense_dim);
          // dimension 0 of values corresponds to the number of values,
          // rather than to any logical dimension of the sparse tensor.
          zeros_sizes[0] = t._values().size(0);
          zeros_sizes[values_dim] = cumulative_size;
          cumulative_size += t._values().size(values_dim);
          auto z1 = native::zeros(
              zeros_sizes,
              optTypeMetaToScalarType(t._values().options().dtype_opt()),
              t._values().options().layout_opt(),
              t._values().options().device_opt(),
              t._values().options().pinned_memory_opt());
          zeros_sizes[values_dim] = total_size - cumulative_size;
          auto z2 = native::zeros(
              zeros_sizes,
              optTypeMetaToScalarType(t._values().options().dtype_opt()),
              t._values().options().layout_opt(),
              t._values().options().device_opt(),
              t._values().options().pinned_memory_opt());
          vals_pieces.push_back(native::cat({z1, t._values(), z2}, values_dim));
          idxs_pieces.push_back(t._indices());
        }
        auto sizes_copy = sizes.vec();
        sizes_copy[wrapped] = total_size;
        // This can create an uncoalesced tensor
        return native::sparse_coo_tensor(
            native::cat(idxs_pieces, 1),
            native::cat(vals_pieces),
            sizes_copy,
            optTypeMetaToScalarType(tensors[0].options().dtype_opt()),
            tensors[0].options().layout_opt(),
            tensors[0].options().device_opt(),
            tensors[0].options().pinned_memory_opt());
      }
        */
}


pub fn cat_b(
        tensors: TensorList,
        dim:     i64) -> Tensor {
    
    todo!();
        /*
            if (tensors.size() > 0 &&
            tensors[0].is_sparse()) {
        return cat_sparse(tensors, dim);
      }

      check_cat_no_zero_dim(tensors);
      dim = legacy_cat_wrap_dim(dim, tensors);
      auto maybe_outnames = namedinference::compute_cat_outnames(tensors);
      Tensor result;
      {
        NoNamesGuard guard;
        result = _cat(tensors, dim);
      }
      namedinference::propagate_names_if_nonempty(result, maybe_outnames);
      return result;
        */
}


pub fn block_diag(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            Tensor result;
      if (tensors.size() == 0) {
        result = empty({1, 0});
        return result;
      }

      const Device& device = tensors[0].device();
      for (const auto tensor_idx : irange(tensors.size())) {
        const Tensor& tensor = tensors[tensor_idx];

        TORCH_CHECK(
          tensor.device() == device,
          "torch.block_diag: input tensors must all be on the same device.",
          " Input 0 is on device ", device,
          " and input ", tensor_idx, " is on device ", tensor.device()
        );
      }

      ScalarType output_scalar_type = native::result_type(tensors);
      i64 result_dim0 = 0;
      i64 result_dim1 = 0;
      vector<Tensor> tensors_2D(tensors.size());

      // Sum the dimensions of the tensors, check tensor sizes,
      // and expand all 0-D and 1-D tensors so that everything
      // is 2-D
      for (const auto tensor_idx : irange(tensors.size())) {
        const Tensor& tensor = tensors[tensor_idx];
        i64 ndims = tensor.dim();
        TORCH_CHECK(
          ndims <= 2,
          "torch.block_diag: Input tensors must have 2 or fewer dimensions. Input ",
          tensor_idx, " has ", ndims, " dimensions"
        );

        i64 dim0 = 1;
        i64 dim1 = 1;

        if (ndims == 2) {
          dim0 = tensor.size(0);
          dim1 = tensor.size(1);
          tensors_2D[tensor_idx] = tensor;
        } else if (ndims == 1) {
          // Switching dim 0 to dim 1 is intentional
          dim1 = tensor.size(0);
          tensors_2D[tensor_idx] = tensor.expand({dim0, dim1});
        } else {
          tensors_2D[tensor_idx] = tensor.expand({dim0, dim1});
        }
        result_dim0 += dim0;
        result_dim1 += dim1;
      }

      result = zeros(
        {result_dim0, result_dim1},
        tensors[0].options().dtype(output_scalar_type)
      );

      i64 cur_dim0 = 0;
      i64 cur_dim1 = 0;

      // Copy each tensor into the appropriate location in the result matrix
      for (const auto& tensor : tensors_2D) {
        i64 dim0 = tensor.size(0);
        i64 dim1 = tensor.size(1);
        result.slice(0, cur_dim0, cur_dim0+dim0).slice(1, cur_dim1, cur_dim1+dim1).copy_(tensor);

        cur_dim0 += dim0;
        cur_dim1 += dim1;
      }

      return result;
        */
}


pub fn chunk(
        self_:  &Tensor,
        chunks: i64,
        dim:    i64) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() > 0,
               "chunk expects at least a 1-dimensional tensor");
      TORCH_CHECK(chunks > 0,
               "chunk expects `chunks` to be greater than 0, got: ", chunks);

      const auto dim_size = self.size(dim);
      i64 split_size = (dim_size + chunks - 1) / chunks;

      // We need to call split_with_sizes in the case where split_size and dimension size are 0, because
      // a call to split would discard the number of chunks (because we can have an arbitrary number of
      // 0-sized chunks adding up to 0).  So, call split_with_sizes with the correct number of chunks,
      // eventually we will do this for all cases.
      if (split_size == 0 && dim_size == 0) {
        vector<i64> split_sizes(chunks, split_size);
        split_sizes[chunks - 1] = split_size - (split_size * chunks - dim_size);
        return self.split_with_sizes(split_sizes, dim);
      } else {
        return self.split(split_size, dim);
      }
        */
}

pub fn tensor_split_a(
    self_:    &Tensor,
    sections: i64,
    dim:      i64) -> Vec<Tensor> {

    todo!();
        /*
      TORCH_CHECK(self.dim() > 0, "tensor_split expected at least a 1-dimensional tensor, but got a tensor with ", self.dim()," dims");
      i64 dim_ = maybe_wrap_dim(dim, self.dim());
      TORCH_CHECK(sections > 0, "number of sections must be larger than 0, got ", sections);
      const auto dim_size = self.size(dim_);
      vector<Tensor> splits(sections);
      i64 min_split_size = dim_size / sections;
      i64 num_splits_one_extra = dim_size % sections;
      i64 start_idx = 0;
      for (const auto split_idx : irange(sections)) {
        i64 split_size = (split_idx < num_splits_one_extra) ? (min_split_size + 1) : min_split_size;
        splits[split_idx] = slice(self, dim_, start_idx, start_idx + split_size);
        start_idx += split_size;
      }
      return splits;
    */
}

pub fn tensor_split_b(
    self_:   &Tensor,
    indices: &[i32],
    dim:     i64) -> Vec<Tensor> {
    
    todo!();
        /*
      TORCH_CHECK(self.dim() > 0, "tensor_split expected at least a 1-dimensional tensor, but got a tensor with ", self.dim()," dims");
      i64 dim_ = maybe_wrap_dim(dim, self.dim());
      i64 num_indices = indices.size();
      vector<Tensor> splits(num_indices + 1);
      i64 start_idx = 0;
      for (const auto split_idx : irange(num_indices)) {
        i64 end_idx = indices[split_idx];
        splits[split_idx] = slice(self, dim_, start_idx, end_idx);
        start_idx = end_idx;
      }
      splits[num_indices] = slice(self, dim_, start_idx, self.size(dim_));
      return splits;
        */
}

pub fn tensor_split_c(
    self_:                      &Tensor,
    tensor_indices_or_sections: &Tensor,
    dim:                        i64) -> Vec<Tensor> {

    todo!();
        /*
      TORCH_CHECK(self.dim() > 0, "tensor_split expected at least a 1-dimensional tensor, but got a tensor with ", self.dim()," dims");
      auto split_device = tensor_indices_or_sections.device();
      TORCH_CHECK(split_device == kCPU,
        "tensor_split expected tensor_indices_or_sections to be on cpu, but it's on ", split_device);
      auto split_dtype = tensor_indices_or_sections.scalar_type();
      TORCH_CHECK(split_dtype == kLong,
        "tensor_split expected tensor_indices_or_sections to have dtype of long, but got ", split_dtype);
      auto split_dim = tensor_indices_or_sections.dim();
      TORCH_CHECK(split_dim == 1 || split_dim == 0,
        "tensor_split expected tensor_indices_or_sections to be a zero-dimensional or one-dimensional tensor, but got a tensor with ", split_dim, " dims");

      if (split_dim == 0) {
        i64 sections = tensor_indices_or_sections.item<i64>();
        return self.tensor_split(sections, dim);
      } else {
        auto indices_data = tensor_indices_or_sections.data_ptr<i64>();
        vector<i64> indices(indices_data, indices_data + tensor_indices_or_sections.numel());
        return self.tensor_split(indices, dim);
      }
        */
}


pub fn unsafe_chunk(
        self_:  &Tensor,
        chunks: i64,
        dim:    i64) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() > 0,
               "chunk expects at least a 1-dimensional tensor");
      TORCH_CHECK(chunks > 0,
               "chunk expects `chunks` to be greater than 0, got: ", chunks);

      const auto dim_size = self.size(dim);
      i64 split_size = (dim_size + chunks - 1) / chunks;

      // See the comment above in chunk(...)
      if (split_size == 0 && dim_size == 0) {
        vector<i64> split_sizes(chunks, split_size);
        split_sizes[chunks - 1] = split_size - (split_size * chunks - dim_size);
        return self.unsafe_split_with_sizes(split_sizes, dim);
      } else {
        return self.unsafe_split(split_size, dim);
      }
        */
}

pub fn diagflat(
    self_:  &Tensor,
    offset: i64) -> Tensor {
    
    todo!();
        /*
            return self.contiguous().view(-1).diag(offset);
        */
}

pub fn diagonal_a(
    self_:  &Tensor,
    offset: i64,
    dim1:   i64,
    dim2:   i64) -> Tensor {
    
    todo!();
        /*
            i64 nDims = self.dim();
      i64 dim1 = maybe_wrap_dim(dim1_, nDims);
      i64 dim2 = maybe_wrap_dim(dim2_, nDims);
      TORCH_CHECK(dim1 != dim2, "diagonal dimensions cannot be identical ", dim1_, ", ", dim2_);
      auto outnames = namedinference::compute_diagonal_outnames(self, dim1, dim2);
      NoNamesGuard no_names_guard;

      i64 diag_size;
      i64 storage_offset = self.storage_offset();
      // compute storage offset and size for the diagonal
      // for positive values of offset (above the main diagonal)
      // "leftmost columns" (along dim2) are dropped
      // for negative values of offset (below the main diagonal)
      // "topmost rows" (along dim1) are dropped.
      // Note that we invert +/- in the second to absorb the negative
      // sign in the offset.
      if (offset >= 0) {
        diag_size = max<i64>(min(self.size(dim1), self.size(dim2)-offset), 0);
      } else {
        diag_size = max<i64>(min(self.size(dim1)+offset, self.size(dim2)), 0);
      }

      // NumPy allows you to specify offsets "off the end"; let's just be careful not to
      // set a ridiculous storage_offset in that case (technically it shouldn't matter
      // because there are no elements in the tensor, but let's be kosher).
      if (diag_size == 0) {
        // skip
      } else if (offset >= 0) {
        storage_offset += offset * self.stride(dim2);
      } else {
        storage_offset -= offset * self.stride(dim1);
      }

      // construct new size and stride: we drop dim1 and dim2 (maximum first for not changing the index of the minimum)
      // the new ("joint") dimension is appended to the end of the shape / stride to match numpy semantics
      DimVector sizes(self.sizes().begin(), self.sizes().end());
      DimVector strides(self.strides().begin(), self.strides().end());
      sizes.erase(sizes.begin() + max(dim1, dim2));
      strides.erase(strides.begin() + max(dim1, dim2));
      sizes.erase(sizes.begin() + min(dim1, dim2));
      strides.erase(strides.begin() + min(dim1, dim2));
      sizes.push_back(diag_size);
      strides.push_back(self.stride(dim1)+self.stride(dim2));

      // return view with new parameters
      auto result = self.as_strided(sizes, strides, storage_offset);

      no_names_guard.reset();
      namedinference::propagate_names_if_nonempty(result, outnames);
      return result;
        */
}

pub fn diagonal_b(
        self_:  &Tensor,
        outdim: Dimname,
        dim1:   Dimname,
        dim2:   Dimname,
        offset: i64) -> Tensor {
    
    todo!();
        /*
            auto result = diagonal(
          self,
          offset,
          dimname_to_position(self, dim1),
          dimname_to_position(self, dim2));
      // This is slower than it needs to be because there is no way to modify
      // the names of a tensor in-place right now. In the future we should consider
      // offering that functionality.
      vector<Dimname> new_names = result.names().vec();
      new_names[new_names.size() - 1] = outdim;
      return result.refine_names(new_names);
        */
}


pub fn diag_embed(
        self_:  &Tensor,
        offset: i64,
        dim1:   i64,
        dim2:   i64) -> Tensor {
    
    todo!();
        /*
            i64 nDims = self.dim() + 1;
      i64 dim1 = maybe_wrap_dim(dim1_, nDims);
      i64 dim2 = maybe_wrap_dim(dim2_, nDims);
      TORCH_CHECK(dim1 != dim2, "diagonal dimensions cannot be identical ", dim1_, ", ", dim2_);
      i64 new_dim_len = abs(offset) + self.size(-1);
      auto sizes = self.sizes().vec();
      sizes.pop_back();
      sizes.insert(sizes.begin() + min(dim1, dim2), new_dim_len);
      sizes.insert(sizes.begin() + max(dim1, dim2), new_dim_len);
      auto result = zeros(sizes, self.options());
      auto diag = result.diagonal(offset, dim1, dim2);
      diag.copy_(self);
      return result;
        */
}


pub fn expand(
        self_:  &Tensor,
        size:   &[i32],
        unused: bool) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(size.size() >= (usize)self.dim(),
               "expand(", self.toString(), "{", self.sizes(), "}, size=", size,
               "): the number of sizes provided (", size.size(), ") ",
               "must be greater or equal to the number of dimensions in the tensor (",
               self.dim(), ")");

      auto expandedSizesAndStrides = inferExpandGeometry_dimvector(self.sizes(), self.strides(), size);

      auto result = self.as_strided(
          expandedSizesAndStrides.sizes, expandedSizesAndStrides.strides);
      namedinference::propagate_names_for_expand(result, self);
      return result;
        */
}


pub fn expand_as(
        self_: &Tensor,
        other: &Tensor) -> Tensor {
    
    todo!();
        /*
            return self.expand(other.sizes());
        */
}


pub fn sum_to_size(
        self_: &Tensor,
        size:  &[i32]) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(is_expandable_to(size, self.sizes()),
               "size {", size, "} is not expandable to size {", self.sizes(), "}.");

      return sum_to(self, size);
        */
}

/**
  | We currently do not support per-channel
  | quant for unfold, diagonal, expand,
  | permute.
  | 
  | TODO: Make this an aten function and
  | replace as_strided_qtensorimpl once
  | that is done.
  |
  */
pub fn make_qtensor(
        self_:     &Tensor,
        size:      &[i32],
        stride:    &[i32],
        quantizer: QuantizerPtr) -> Tensor {
    
    todo!();
        /*
            auto result = make_tensor<QTensorImpl>(
          TensorImpl::VIEW, Storage(self.storage()), self.key_set(), self.dtype(), quantizer);
      setStrided(result, size, stride, self.storage_offset());
      return result;
        */
}



pub fn as_strided_tensorimpl(
        self_:          &Tensor,
        size:           &[i32],
        stride:         &[i32],
        storage_offset: Option<i64>) -> Tensor {
    
    todo!();
        /*
            auto storage_offset = storage_offset_.value_or(self.storage_offset());
      auto result = make_tensor<TensorImpl>(
          TensorImpl::VIEW, Storage(self.storage()), self.key_set(), self.dtype());
      setStrided(result, size, stride, storage_offset);
      return result;
        */
}


pub fn as_strided_qtensorimpl(
        self_:          &Tensor,
        size:           &[i32],
        stride:         &[i32],
        storage_offset: Option<i64>) -> Tensor {
    
    todo!();
        /*
            auto storage_offset = storage_offset_.value_or(self.storage_offset());
      auto quantizer = get_qtensorimpl(self)->quantizer();
      TORCH_CHECK(
          quantizer->qscheme() == QScheme::PER_TENSOR_AFFINE,
          "Setting strides is possible only on uniformly quantized tensor");
      auto result = make_tensor<QTensorImpl>(
          TensorImpl::VIEW, Storage(self.storage()), self.key_set(), self.dtype(), quantizer);
      setStrided(result, size, stride, storage_offset);
      return result;
        */
}


pub fn as_strided(
        self_:          &Tensor,
        size:           &[i32],
        stride:         &[i32],
        storage_offset: Option<i64>) -> &Tensor {
    
    todo!();
        /*
            auto storage_offset = storage_offset_.value_or(self.storage_offset());
      setStrided(self, size, stride, storage_offset);
      return self;
        */
}


pub fn narrow_copy_sparse(
        self_:  &Tensor,
        dim:    i64,
        start:  i64,
        length: i64) -> Tensor {
    
    todo!();
        /*
            i64 allDim = self.dim();
      i64 end = start+length;
      TORCH_CHECK(allDim > 0, "narrow() cannot be applied to a 0-dim tensor.");
      TORCH_CHECK(dim >= 0 && dim < allDim,
        "Dimension ", dim, " out of range. Expecting 0 <= dim < ", allDim, ".");
      TORCH_CHECK(start >= 0 && length >= 0 && end <= self.size(dim),
        "Invalid range to narrow. range(start, start+length) must be a subset of range(0, ", self.size(dim), ").")
      Tensor indices = self._indices();
      i64 sparse_dim = self.sparse_dim();

      vector<i64> new_sizes = self.sizes().vec();
      new_sizes[dim] = length;

      Tensor new_values;
      Tensor new_indices;
      if (dim < sparse_dim) {
        Tensor mask = (indices[dim] >= start).__and__((indices[dim] < end));
        new_indices = indices.masked_select(mask).view({sparse_dim, -1});
        new_indices[dim].sub_(start);
        Tensor nzIndices = mask.nonzero().view(-1);
        new_values = self._values().index_select(0, nzIndices);
      } else {
        /* This means we are narrowing on a dense dim, which is in effect just a
            regular narrow on _values() */
        new_indices = indices;
        i64 dense_dim = dim - sparse_dim + 1;
        new_values = self._values().narrow_copy(dense_dim, start, length);
      }

      auto newTensor = sparse_coo_tensor(new_indices, new_values, new_sizes);
      return newTensor._coalesced_(self.is_coalesced());
        */
}


pub fn narrow_copy_dense_cpu_out(
        self_:  &Tensor,
        dim:    i64,
        start:  i64,
        length: i64,
        output: &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() > 0, "narrow() cannot be applied to a 0-dim tensor.");
      TORCH_CHECK(self.dtype() == output.dtype());

      auto self_contig = self.expect_contiguous();
      const auto self_sizes = self_contig->sizes();

      // wrap dim if negative and do bound check
      if (dim < 0) {
        dim = maybe_wrap_dim(dim, self_sizes.size());
      } else {
        TORCH_CHECK(dim < self_sizes.size());
      }

      // wrap start and do bound check
      const auto cur_size = self_sizes[dim];
      if (start != cur_size && start < 0) { // start being the end is valid, but
                                            // not a valid dim specification.
        start = maybe_wrap_dim(start, cur_size);
      }
      TORCH_CHECK(
          length >= 0 && start <= cur_size - length,
          "start (",
          start,
          ") + length (",
          length,
          ") exceeds dimension size (",
          cur_size,
          ").");

      // resize output
      auto output_sizes = self_sizes.vec();
      output_sizes[dim] = length;
      native::resize_(output, output_sizes);

      const i64 unit = size_from_dim_(dim + 1, self_sizes);
      const i64 num_blocks = Sizeo_dim_(dim, self_sizes);

      const auto itemsize = self_contig->dtype().itemsize();
      usize src_nbytes = itemsize * self_contig->numel();
      usize dst_nbytes = itemsize * output.numel();

      usize src_block_size = unit * self_sizes[dim];
      usize dst_block_size = unit * length;

      if (num_blocks == 0 || dst_block_size == 0) {
        return output;
      }

      char* src_bytes = static_cast<char*>(self_contig->data_ptr());
      char* dst_bytes = static_cast<char*>(output.data_ptr());

      usize src_block_size_bytes = itemsize * src_block_size;
      usize dst_block_size_bytes = itemsize * dst_block_size;
      usize src_offset = unit * start;

      char* src_offset_bytes = src_bytes + itemsize * src_offset;
      char* dst_offset_bytes = dst_bytes;

      for (const auto i : irange(num_blocks)) {
        char* local_src_offset_bytes = src_offset_bytes + i * src_block_size_bytes;
        char* local_dst_offset_bytes = dst_offset_bytes + i * dst_block_size_bytes;
        TORCH_INTERNAL_ASSERT_DEBUG_ONLY(
            static_cast<void*>(local_src_offset_bytes + dst_block_size_bytes) <=
            static_cast<void*>(src_bytes + src_nbytes));
        TORCH_INTERNAL_ASSERT_DEBUG_ONLY(
            static_cast<void*>(local_dst_offset_bytes + dst_block_size_bytes) <=
            static_cast<void*>(dst_bytes + dst_nbytes));

        memcpy(
            local_dst_offset_bytes, local_src_offset_bytes, dst_block_size_bytes);
      }
      return output;
        */
}


pub fn narrow_copy_dense(
        self_:  &Tensor,
        dim:    i64,
        start:  i64,
        length: i64) -> Tensor {
    
    todo!();
        /*
            return self.narrow(dim, start, length).clone(MemoryFormat::Contiguous);
        */
}


pub fn narrow_copy_dense_cpu(
        self_:  &Tensor,
        dim:    i64,
        start:  i64,
        length: i64) -> Tensor {
    
    todo!();
        /*
            auto output = empty_like(self);
      return narrow_copy_dense_cpu_out(self, dim, start, length, output);
        */
}


pub fn narrow_a(
        self_:  &Tensor,
        dim:    i64,
        start:  i64,
        length: i64) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() > 0, "narrow() cannot be applied to a 0-dim tensor.");
      auto cur_size = self.size(dim);
      if (start != cur_size) {  // start being the end is valid, but not a valid dim specification.
        start = maybe_wrap_dim(start, cur_size);
      }
      TORCH_CHECK(length >= 0 && start <= cur_size - length,
               "start (", start, ") + length (", length, ") exceeds dimension size (", cur_size, ").");
      return slice(self, dim, start, start + length, 1);
        */
}

pub fn narrow_b(
        self_:  &Tensor,
        dim:    i64,
        start:  &Tensor,
        length: i64) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(start.dim() == 0 && isIntegralType(start.scalar_type(), /*includeBool=*/false),
                  "start must be an 0-dim integral Tensor.");
      i64 st = start.item<i64>();
      return narrow(self, dim, st, length);
        */
}


pub fn permute(
        self_: &Tensor,
        dims:  &[i32]) -> Tensor {
    
    todo!();
        /*
            auto nDims = self.dim();
      TORCH_CHECK(dims.size() == (usize)nDims,
               "number of dims don't match in permute");
      auto oldSizes = self.sizes();
      auto oldStrides = self.strides();
      DimVector newSizes(nDims);
      DimVector newStrides(nDims);
      vector<bool> seen(nDims);
      for (const auto i : irange(nDims)) {
        auto dim = maybe_wrap_dim(dims[i], nDims);
        TORCH_CHECK(!seen[dim],
                 "repeated dim in permute");
        seen[dim] = true;
        newSizes[i] = oldSizes[dim];
        newStrides[i] = oldStrides[dim];
      }
      return self.as_strided(newSizes, newStrides);
        */
}


pub fn repeat(
        self_:   &Tensor,
        repeats: &[i32]) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(repeats.size() >= (usize)self.dim(),
               "Number of dimensions of repeat dims can not be smaller than number of dimensions of tensor");

      // Add new leading dimensions to the tensor if the
      // number of target dimensions is larger than the
      // number of source dimensions.
      i64 num_new_dimensions = repeats.size() - self.dim();
      DimVector padded_size(num_new_dimensions, 1);
      padded_size.insert(padded_size.end(), self.sizes().begin(), self.sizes().end());
      DimVector target_size(repeats.size());
      bool zero_tensor = false;
      for(const auto idx : irange(repeats.size())) {
        if (repeats[idx] == 0) {
          zero_tensor = true;
        }
        target_size[idx] = padded_size[idx] * repeats[idx];
      }

      Tensor xtensor = self.expand(padded_size);

      Tensor result;
      if (self.is_quantized()) {
        result = empty_quantized(target_size, self);
      } else {
        result = empty(target_size, self.options());
      }

      // return an empty tensor if one of the repeat dimensions is zero
      if (zero_tensor) {
        return result;
      }

      Tensor urtensor = alias(result);
      for (const auto i : irange(xtensor.dim())) {
        // can't unfold with step 0, so make sure step is at least 1
        // (it doesn't matter what it is in that case, because the size is 0).
        auto size_i = xtensor.sizes()[i];
        urtensor = urtensor.unfold(i, size_i, max<i64>(size_i, 1));
      }

      urtensor.copy_(xtensor.expand_as(urtensor));

      return result;
        */
}


pub fn tile(
        self_: &Tensor,
        reps:  &[i32]) -> Tensor {
    
    todo!();
        /*
            // If self.size() > len(reps), reps is promoted to self.size() by pre-pending
      // 1’s to it to keep the same behaviour as `numpy.tile`.
      // Thus for a tensor of shape (2, 3, 4, 5), a dims of (2, 2) is treated
      // as (1, 1, 2, 2).
      const i64 size_diff = self.dim() - static_cast<i64>(reps.size());
      if (size_diff > 0){
        vector<i64> new_reps(size_diff, 1);
        for (const auto i : irange(reps.size())) {
          new_reps.emplace_back(reps[i]);
        }
        return self.repeat(IntArrayRef(new_reps));
      }
      // `torch.tile` is equivalent to the already implemented `torch.Tensor.repeat`
      return self.repeat(reps);
        */
}

/**
  | templated for &[i64] and
  | SmallVector<i64> use cases
  |
  */
pub fn alias_with_sizes_and_strides<Vec>(
        self_:   &Tensor,
        sizes:   &Vec,
        strides: &Vec) -> Tensor {

    todo!();
        /*
            Tensor self_;
      if (self.is_quantized()) {
        self_ = make_tensor<QTensorImpl>(
          TensorImpl::VIEW, Storage(self.storage()), self.key_set(), self.dtype(), get_qtensorimpl(self)->quantizer());
        setStrided(self_, sizes, strides, self.storage_offset());
      } else {
        self_ = make_tensor<TensorImpl>(
          TensorImpl::VIEW, Storage(self.storage()), self.key_set(), self.dtype());
        setStrided(self_, sizes, strides, self.storage_offset());
      }
      namedinference::propagate_names(self_, self);
      return self_;
        */
}


pub fn reshape(
        self_:          &Tensor,
        proposed_shape: &[i32]) -> Tensor {
    
    todo!();
        /*
            if (self.is_sparse()) {
        AT_ERROR("reshape is not implemented for sparse tensors");
      }
      DimVector shape = infer_size_dv(proposed_shape, self.numel());

      if (self.is_mkldnn()) {
        return _mkldnn_reshape(self, shape);
      }

      auto stride =
          computeStride(self.sizes(), self.strides(), shape);
        // `computeStride` returns the proper strides to use if this
        // `reshape` can be just a view.
        //
        // NB: Even though we have viewable geometry and the target strides here,
        //     we do not just call `as_strided` on `self` because the backward
        //     for `as_strided` is not as efficient as that of `view` (since the
        //     former is meant to handle general cases).
      if (stride.has_value()) {
        return self.view(shape);
      }
      return _unsafe_view(self.clone(MemoryFormat::Contiguous), shape);
        */
}


pub fn reshape_as(
        self_: &Tensor,
        other: &Tensor) -> Tensor {
    
    todo!();
        /*
            return self.reshape(other.sizes());
        */
}


pub fn select_sparse(
        self_: &Tensor,
        dim:   i64,
        index: i64) -> Tensor {
    
    todo!();
        /*
            i64 sparse_dim = self.sparse_dim();
      i64 dense_dim = self.dense_dim();
      TORCH_INTERNAL_ASSERT(dim >= 0 && dim < sparse_dim + dense_dim);

      auto indices = self._indices();
      auto values = self._values();
      auto new_sizes = self.sizes().vec();
      new_sizes.erase(new_sizes.begin() + dim);

      if (dim < sparse_dim) {
        auto nzIndices = (indices[dim] == index).nonzero().view(-1);
        auto new_values = values.index_select(0, nzIndices);
        if (sparse_dim == 1) {
          // return dense part:
          if (new_values.size(0) == 1) {
            return new_values[0];
          } else {
            return new_values.sum(0);
          }
        } else {
          auto dimIndices = (arange(
                                 0,
                                 sparse_dim,
                                 nullopt /* dtype */,
                                 nullopt /* layout */,
                                 self.device(),
                                 nullopt /* pin_memory */) != dim)
                                .nonzero()
                                .view(-1);
          auto new_indices = indices.index_select(1, nzIndices).index_select(0, dimIndices);
          return _sparse_coo_tensor_with_dims_and_tensors(
                sparse_dim - 1, dense_dim, new_sizes, new_indices, new_values, self.options());
        }
      } else {
        auto new_values = values.select(dim - sparse_dim + 1, index);
        return _sparse_coo_tensor_with_dims_and_tensors(
             sparse_dim, dense_dim - 1, new_sizes, indices, new_values, self.options());
      }
        */
}

pub fn select_a(
        self_: &Tensor,
        dim:   i64,
        index: i64) -> Tensor {
    
    todo!();
        /*
            i64 ndim = self.dim();
      if (ndim == 0) {
        TORCH_CHECK_INDEX(false, "select() cannot be applied to a 0-dim tensor.");
      }
      dim = maybe_wrap_dim(dim, ndim);
      auto size = self.size(dim);
      if (index < -size || index >= size) {
        if (self.has_names() && self.names()[dim] != Dimname::wildcard()) {
          TORCH_CHECK_INDEX(false, "select(): index ", index, " out of range for tensor of size ",
                         self.sizes(), " at dimension ", self.names()[dim]);
        }
        TORCH_CHECK_INDEX(false, "select(): index ", index, " out of range for tensor of size ",
                       self.sizes(), " at dimension ", dim);
      }
      if (index < 0) {
        index += size;
      }
      if (self.is_sparse()) {
        return select_sparse(self, dim, index);
      }
      DimVector sizes(self.sizes().begin(), self.sizes().end());
      DimVector strides(self.strides().begin(), self.strides().end());
      auto storage_offset = self.storage_offset() + index * strides[dim];
      sizes.erase(sizes.begin() + dim);
      strides.erase(strides.begin() + dim);
      auto result = self.as_strided(sizes, strides, storage_offset);
      namedinference::propagate_names_except(result, self, {dim});
      return result;
        */
}

pub fn select_b(
        self_: &Tensor,
        dim:   Dimname,
        index: i64) -> Tensor {
    
    todo!();
        /*
            return select(self, dimname_to_position(self, dim), index);
        */
}

pub fn select_backward(
        grad:        &Tensor,
        input_sizes: &[i32],
        dim:         i64,
        index:       i64) -> Tensor {
    
    todo!();
        /*
            auto grad_input = zeros(input_sizes, grad.options());
      grad_input.select(dim, index).copy_(grad);
      return grad_input;
        */
}

pub fn index_select_sparse(
    self_: &Tensor,
    dim:   i64,
    index: &Tensor) -> Tensor {
    
    todo!();
        /*
            /*
        Algorithm:
        index - a 1-D tensor of indicies with shape (n,)
        self - sparse tensor, its shape is sizes = sparse_shape + dense_shape
          indices - 2-D tensor of indices, shape is (sparse_dims, nnz)
          values - (1+len(dense_shape))-D tensor of values, shape is (nnz,) + dense_shape
        index_select(dim, index) returns a sparse tensor with the following data
          new_sizes = sizes[:dim] + (n,) + sizes[dim+1:]
          new_indices - shape is (sparse_dims, new_nnz)
          new_values - shape is (new_nnz,) + dense_shape

          if dim < len(sparse_shape):
              for i, idx in enumerate(index):
                  for j, jdx in enumerate(indices[dim]):
                      if idx == jdx:
                          icol = indices[:dim][j] + (i,) + indices[dim+1:][j]
                          new_indices.add_column(icol)
                          new_values.add_row(values[j])
          else:
              new_indices = indices
              new_values[k] = values[k].index_select(dim - len(sparse_shape), index) for k in range(nnz)
        */
      auto ndim = self.dim();
      if (ndim == 0) {
        TORCH_CHECK_INDEX(false, "index_select() cannot be applied to a 0-dim tensor.");
      }
      if (!(index.dim() == 1 && index.dtype() == kLong)) {
        TORCH_CHECK_INDEX(false, "index_select() argument index must be 1-D long-tensor.");
      }
      dim = maybe_wrap_dim(dim, ndim);
      auto size = self.size(dim);
      auto sparse_dim = self.sparse_dim();
      auto dense_dim = self.dense_dim();
      auto indices = self._indices();
      auto values = self._values();
      auto nnz = values.size(0);
      auto new_sizes = self.sizes().vec();
      new_sizes[dim] = index.size(0);

      if (dim < sparse_dim) {

        auto dim_indices = indices[dim];
        vector<i64> zindices;
        vector<i64> iindices;
        i64 new_nnz = 0;
        for (const auto i : irange(new_sizes[dim])) {
          auto idx = index[i].item<i64>();
          if (idx < -size || idx >= size) {
            TORCH_CHECK_INDEX(false, "index_select(): index contains ", idx, " that is out of range for tensor of size ",
                       self.sizes(), " at dimension ", dim);
          }
          if (idx < 0) {
            idx += size;
          }
          for (const auto j : irange(nnz)) {
            auto jdx = dim_indices[j].item<i64>();
            if (idx == jdx) {
              new_nnz++;
              iindices.push_back(i);
              zindices.push_back(j);
            }
          }
        }
        auto zIndices = from_blob(zindices.data(), {new_nnz}, kLong).to(indices.device());
        auto new_indices = indices.index_select(1, zIndices);
        new_indices[dim] = from_blob(iindices.data(), {new_nnz}, kLong).to(indices.device());
        auto new_values = values.index_select(0, zIndices);
        return _sparse_coo_tensor_with_dims_and_tensors(
            sparse_dim, dense_dim, new_sizes, new_indices, new_values, self.options());

      } else {

        auto vsize = values.sizes().vec();
        vsize[dim + 1 - sparse_dim] = index.size(0);
        auto new_values = empty(vsize, values.options());
        for (const auto k : irange(nnz)) {
          new_values[k] = values[k].index_select(dim - sparse_dim, index);
        }
        return _sparse_coo_tensor_with_dims_and_tensors(
            sparse_dim, dense_dim, new_sizes, indices, new_values, self.options());

      }
        */
}

pub fn slice(
        self_: &Tensor,
        dim:   i64,
        start: Option<i64>,
        end:   Option<i64>,
        step:  i64) -> Tensor {
    
    todo!();
        /*
            i64 ndim = self.dim();
      if (ndim == 0) {
        TORCH_CHECK_INDEX(false, "slice() cannot be applied to a 0-dim tensor.");
      }
      dim = maybe_wrap_dim(dim, ndim);
      DimVector sizes(self.sizes().begin(), self.sizes().end());
      DimVector strides(self.strides().begin(), self.strides().end());

      // handle optional parameters
      i64 start_val = start.has_value() ? start.value() : 0;
      i64 end_val = end.has_value() ? end.value() : INT64_MAX;

      // TODO: support negative strides
      TORCH_CHECK(step > 0, "slice step must be positive");

      // INT64_MAX stands for default value.
      if (start_val == INT64_MAX) {
        start_val = 0;
      }
      if (start_val < 0) {
        start_val += sizes[dim];
      }
      if (end_val < 0) {
        end_val += sizes[dim];
      }
      if (start_val < 0) {
        start_val = 0;
      } else if (start_val >= sizes[dim]) {
        start_val = sizes[dim];
      }
      if (end_val < start_val) {
        end_val = start_val;
      } else if (end_val >= sizes[dim]) {
        end_val = sizes[dim];
      }
      auto storage_offset = self.storage_offset() + start_val * strides[dim];
      auto len = end_val - start_val;
      sizes[dim] = (len + step - 1) / step; // round-up
      strides[dim] *= step;
      auto result = self.as_strided(sizes, strides, storage_offset);
      namedinference::propagate_names(result, self);
      return result;
        */
}


pub fn slice_backward(
        grad:        &Tensor,
        input_sizes: &[i32],
        dim:         i64,
        start:       i64,
        end:         i64,
        step:        i64) -> Tensor {
    
    todo!();
        /*
            auto grad_input = zeros(input_sizes, grad.options());
      grad_input.slice(dim, start, end, step).copy_(grad);
      return grad_input;
        */
}


pub fn split(
        self_:      &Tensor,
        split_size: i64,
        dim:        i64) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() != 0, "split expects at least a 1-dimensional tensor");
      TORCH_CHECK(split_size >= 0,  "split expects split_size be non-negative, but got split_size=", split_size);
      i64 dim_size = self.size(dim);
      TORCH_CHECK(split_size > 0 || dim_size == 0,
               "split_size can only be 0 if dimension size is 0, "
               "but got dimension size of ", dim_size);
      // if split_size is 0 and dimension size is 0, there is 1 split.
      i64 num_splits = 1;
      if (split_size != 0) {
        // ensuring num_splits is at least 1 makes consistent the case where split_size > dim_size
        // (returns a single split).  We might want to error here, but keep it for BC.
        num_splits = max<i64>((dim_size + split_size - 1) / split_size, 1);
      }
      vector<Tensor> splits(num_splits);
      i64 last_split_size = split_size - (split_size * num_splits - dim_size);

      for (const auto i : irange(num_splits)) {
        auto length = i < num_splits - 1 ? split_size : last_split_size;
        splits[i] = self.narrow(dim, i * split_size, length);
      }
      return splits;
        */
}



pub fn unsafe_split(
        self_:      &Tensor,
        split_size: i64,
        dim:        i64) -> Vec<Tensor> {
    
    todo!();
        /*
            auto result = native::split(self, split_size, dim);
      for (auto& t : result) {
        // TODO(Ailing): do we need to set version_counter here?
        if (!t.is_inference()) {
          t.unsafeGetTensorImpl()->set_version_counter(VariableVersion(/*version=*/0));
        }
      }
      return result;
        */
}


pub fn hsplit_a(
        self_:      &Tensor,
        split_size: i64) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() >= 1, "torch.hsplit requires a tensor with at least 1 dimension, but got a tensor with ", self.dim(), " dimensions!")
      i64 dim = (self.dim() == 1) ? 0 : 1;
      TORCH_CHECK(self.sizes()[dim] % split_size == 0, "torch.hsplit attempted to split along dimension ", dim,", but the size of the dimension ", self.sizes()[dim], " is not divisible by the split_size ", split_size, "!");
      return tensor_split(self, split_size, dim);
        */
}


pub fn vsplit_a(
        self_:      &Tensor,
        split_size: i64) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() >= 2, "torch.vsplit requires a tensor with at least 2 dimension, but got a tensor with ", self.dim(), " dimensions!")
      TORCH_CHECK(self.sizes()[0] % split_size == 0, "torch.vsplit attempted to split along dimension ", 0,", but the size of the dimension ", self.sizes()[0], " is not divisible by the split_size ", split_size, "!");
      return tensor_split(self, split_size, 0);
        */
}


pub fn dsplit_a(
        self_:      &Tensor,
        split_size: i64) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() >= 3, "torch.dsplit requires a tensor with at least 3 dimension, but got a tensor with ", self.dim(), " dimensions!")
      TORCH_CHECK(self.sizes()[2] % split_size == 0, "torch.dsplit attempted to split along dimension ", 2,", but the size of the dimension ", self.sizes()[2], " is not divisible by the split_size ", split_size, "!");
      return tensor_split(self, split_size, 2);
        */
}

pub fn split_with_sizes(
        self_:       &Tensor,
        split_sizes: &[i32],
        dim:         i64) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() != 0, "split expects at least a 1-dimensional tensor");
      i64 dim_size = self.size(dim);
      i64 num_splits = split_sizes.size();
      vector<Tensor> splits(num_splits);
      i64 start_idx = 0;

      for (const auto i : irange(num_splits)) {
        auto length = split_sizes[i];
        TORCH_CHECK(length >= 0,
                 "split_with_sizes expects split_sizes have only non-negative ",
                 "entries, but got split_sizes=", split_sizes);
        splits[i] = self.narrow(dim, start_idx, length);
        start_idx += length;
      }
      TORCH_CHECK(start_idx == dim_size,
               "split_with_sizes expects split_sizes to sum exactly to ", dim_size,
               " (input tensor's size at dimension ", dim, "), ", "but got split_sizes=", split_sizes);
      return splits;
        */
}


pub fn unsafe_split_with_sizes(
        self_:       &Tensor,
        split_sizes: &[i32],
        dim:         i64) -> Vec<Tensor> {
    
    todo!();
        /*
            auto result = native::split_with_sizes(self, split_sizes, dim);
      for (auto& t : result) {
        // TODO(Ailing): do we need to set version_counter here?
        if (!t.is_inference()) {
          t.unsafeGetTensorImpl()->set_version_counter(VariableVersion(/*version=*/0));
        }
      }
      return result;
        */
}

pub fn hsplit_b(
        self_:       &Tensor,
        split_sizes: &[i32]) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() >= 1, "torch.hsplit requires a tensor with at least 1 dimension, but got a tensor with ", self.dim(), " dimensions!")
      return tensor_split(self, split_sizes, (self.dim() == 1) ? 0 : 1);
        */
}

pub fn vsplit_b(
        self_:       &Tensor,
        split_sizes: &[i32]) -> Vec<Tensor> {
    
    todo!();
        /*
            TORCH_CHECK(self.dim() >= 2, "torch.vsplit requires a tensor with at least 2 dimension, but got a tensor with ", self.dim(), " dimensions!")
      return tensor_split(self, split_sizes, 0);
        */
}

pub fn dsplit_b(
        self_:       &Tensor,
        split_sizes: &[i32]) -> Vec<Tensor> {
    
    todo!();
        /*
      TORCH_CHECK(self.dim() >= 3, "torch.dsplit requires a tensor with at least 3 dimension, but got a tensor with ", self.dim(), " dimensions!")
      return tensor_split(self, split_sizes, 2);
        */
}

/**
  | Precondition: tensors is non-empty
  |
  */
#[inline] pub fn get_stack_inputs(
        tensors: TensorList,
        dim:     i64) -> Vec<Tensor> {
    
    todo!();
        /*
            vector<Tensor> inputs(tensors.size());
      IntArrayRef entry_shape = tensors[0].sizes();
      inputs[0] = tensors[0].unsqueeze(dim);
      for (usize i = 1; i < tensors.size(); ++i) {
        TORCH_CHECK(tensors[i].sizes() == entry_shape,
          "stack expects each tensor to be equal size, but got ", entry_shape,
          " at entry 0 and ", tensors[i].sizes(), " at entry ", i);
        inputs[i] = tensors[i].unsqueeze(dim);
      }
      return inputs;
        */
}

/**
  | Checks to see whether native stack can
  | be invoked under these conditions:
  | 
  | - result and input tensors are contiguous
  | 
  | - only one thread is used
  | 
  | - no type promotion has to occur
  | 
  | - tensors dtype is Double or Float
  |
  */
#[inline] pub fn can_use_native_serial_stack(
        result:  &mut Tensor,
        tensors: TensorList,
        dim:     i64) -> bool {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0, "expected a non-empty list of Tensors");

      const Tensor& firstTensor = tensors[0];
      // stack dimension should be in range [0,firstTensor.dim())
      // dim == firstTensor.dim() is a valid input, but it is handled by default code path
      // that uses unsqueeze
      if (dim >= firstTensor.dim()) return false;
      // Native stack doesn't apply any tensor is skipped.
      if (should_skip(firstTensor)) return false;
      // there should be no type promotion
      if (result.dtype() != firstTensor.dtype()) return false;

      // Inputs cannot alias the output tensor
      for (usize i = 0; i < tensors.size(); i++) {
        auto lap = get_overlap_status(result, tensors[i]);
        TORCH_CHECK(lap != MemOverlapStatus::PARTIAL &&
            lap != MemOverlapStatus::FULL, 0,
            "unsupported operation: the input tensors cannot refer to any of the "
            "output memory locations. Found overlap in input tensor ", i);
      }

      auto first_tensor_mem_format = firstTensor.suggest_memory_format();
      ScalarType dtype = firstTensor.scalar_type();

      if (!result.is_contiguous(first_tensor_mem_format)) {
        return false;
      }

      // fast path only works for Double and Float
      if (dtype != ScalarType::Double && dtype != ScalarType::Float) {
        return false;
      }

      // check remainder of inputs
      auto const &first_tensor_shape = firstTensor.sizes();
      for (usize i = 1; i < tensors.size(); i++) {
        auto const &tensor = tensors[i];
        TORCH_CHECK(tensors[i].sizes() == firstTensor.sizes(),
          "stack expects each tensor to be equal size, but got ", first_tensor_shape,
          " at entry 0 and ", tensor.sizes(), " at entry ", i);

        // every tensor must be contiguous
        // tensor sizes and strides must be the same
        // there should be no type promotion
        if (!tensor.is_contiguous(first_tensor_mem_format) ||
          tensor.strides() != firstTensor.strides() ||
          tensor.dtype() != dtype) {
          return false;
        }
      }

      // fast native stack should only be used when it is not worth using multiple threads
      // or there is only one thread. Note that we aren't checking result.numel() here because
      // it may not have been resized and we want to defer that cost till later.
      i64 numel_in_stack = firstTensor.numel() * tensors.size();
      return numel_in_stack < internal::GRAIN_SIZE && get_num_threads() == 1;
        */
}

#[inline] pub fn maybe_native_stack(
        result:  &mut Tensor,
        tensors: TensorList,
        dim:     i64) -> bool {
    
    todo!();
        /*
            if (can_use_native_serial_stack(result, tensors, dim)) {
        // compute the size of the result
        auto result_sizes = tensors[0].sizes().vec();
        result_sizes.insert(result_sizes.begin() + dim, tensors.size());

        // skip resizing if size of result is same as expected
        if (result.sizes() != result_sizes) {
          result.resize_(result_sizes);
        }
        stack_serial_stub(kCPU, result, tensors, dim);
        return true;
      }
      return false;
        */
}



pub fn stack_a(
        tensors: TensorList,
        dim:     i64) -> Tensor {
    
    todo!();
        /*
            dim = maybe_wrap_dim(dim, tensors[0].dim() + 1);
      ScalarType high_type = result_type(tensors);
      Tensor result = empty({0}, tensors[0].options().dtype(high_type));
      return native::_stack_out(get_stack_inputs(tensors, dim), dim, result);
        */
}

pub fn stack_cpu(
        tensors: TensorList,
        dim:     i64) -> Tensor {
    
    todo!();
        /*
            dim = maybe_wrap_dim(dim, tensors[0].dim() + 1);
      ScalarType high_type = result_type(tensors);
      Tensor result = empty({0}, tensors[0].options().dtype(high_type));
      return native::_stack_out_cpu(tensors, dim, result);
        */
}

/**
  | TODO(msubkhankulov): refactor to
  | use _stack
  |
  */
pub fn stack_b(
        tensors: TensorList,
        dim:     i64) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "stack expects a non-empty TensorList");
      dim = maybe_wrap_dim(dim, tensors[0].dim() + 1);
      return cat(get_stack_inputs(tensors, dim), dim);
        */
}

/**
  | CPU specific implementation
  |
  */
pub fn stack_out_cpu(
        tensors: TensorList,
        dim:     i64,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            if (maybe_native_stack(result, tensors, dim)) {
        return result;
      } else {
        return cat_out(result, get_stack_inputs(tensors, dim), dim);
      }
        */
}

/**
  | default backend
  |
  */

pub fn stack_out_a(
        tensors: TensorList,
        dim:     i64,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            return cat_out(result, tensors, dim);
        */
}

/**
  | TODO(msubkhankulov): refactor to
  | use _stack_out
  |
  */
pub fn stack_out_b(
        tensors: TensorList,
        dim:     i64,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "stack expects a non-empty TensorList");
      dim = maybe_wrap_dim(dim, tensors[0].dim() + 1);
      return cat_out(result, get_stack_inputs(tensors, dim), dim);
        */
}

pub fn hstack(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "hstack expects a non-empty TensorList");
      auto rep = atleast_1d(tensors);
      if (rep[0].dim() == 1) {
        return cat(rep, 0);
      }
      return cat(rep, 1);
        */
}


pub fn hstack_out(
        tensors: TensorList,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "hstack expects a non-empty TensorList");
      auto rep = atleast_1d(tensors);
      if (rep[0].dim() == 1) {
        return cat_out(result, rep, 0);
      }
      return cat_out(result, rep, 1);
        */
}


pub fn vstack(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "vstack expects a non-empty TensorList");
      auto rep = atleast_2d(tensors);
      return cat(rep, 0);
        */
}


pub fn vstack_out(
        tensors: TensorList,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "vstack expects a non-empty TensorList");
      auto rep = atleast_2d(tensors);
      return cat_out(result, rep, 0);
        */
}


pub fn dstack(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "dstack expects a non-empty TensorList");
      auto rep = atleast_3d(tensors);
      return cat(rep, 2);
        */
}


pub fn dstack_out(
        tensors: TensorList,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
               "dstack expects a non-empty TensorList");
      auto rep = atleast_3d(tensors);
      return cat_out(result, rep, 2);
        */
}


#[inline] pub fn sparse_transpose(
        self_: &mut Tensor,
        dim0:  i64,
        dim1:  i64) -> &mut Tensor {
    
    todo!();
        /*
            i64 nsparse_dim = self.sparse_dim();
      TORCH_CHECK(dim0 < nsparse_dim && dim1 < nsparse_dim,
               "sparse transpose: transposed dimensions must be sparse ",
               "Got sparse_dim: ", nsparse_dim, ", d0: ", dim0, ", d1: ", dim1);

      if (self._indices().numel() == 0 && self._values().numel() == 0) {
        auto sizes = self.sizes().vec();
        swap(sizes[dim0], sizes[dim1]);

        sparse::get_sparse_impl(self)->raw_resize_(self.sparse_dim(), self.dense_dim(), sizes);
      } else {
        auto indices = self._indices();
        auto row0 = indices.select(0, dim0);
        auto row1 = indices.select(0, dim1);

        // swap row0 and row1
        auto tmp = zeros_like(row0, LEGACY_CONTIGUOUS_MEMORY_FORMAT);
        tmp.copy_(row0);
        row0.copy_(row1);
        row1.copy_(tmp);

        self._coalesced_(false);

        auto sizes = self.sizes().vec();
        swap(sizes[dim0], sizes[dim1]);

        sparse::get_sparse_impl(self)->raw_resize_(self._indices().size(0), self._values().dim() - 1, sizes);
      }
      return self;
        */
}

/**
  | torch.row_stack, alias for torch.vstack
  |
  */
pub fn row_stack_out(
        tensors: TensorList,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            return vstack_out(result, tensors);
        */
}


pub fn row_stack(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            return vstack(tensors);
        */
}


pub fn reshape_input_for_column_stack(tensors: TensorList) -> Vec<Tensor> {
    
    todo!();
        /*
            vector<Tensor> result(tensors.size());
      auto transform_lambda = [](const Tensor& input) -> Tensor {
        // reshape 0D or 1D tensor t into (t.numel(), 1)
        if (input.dim() <= 1) {
          return input.reshape({input.numel(), 1});
        }
        return input;
      };
      transform(tensors.cbegin(),
                     tensors.cend(),
                     result.begin(),
                     transform_lambda);
      return result;
        */
}


pub fn column_stack_out(
        tensors: TensorList,
        result:  &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
                  "column_stack expects a non-empty TensorList");

      auto reshaped_tensors = reshape_input_for_column_stack(tensors);
      return hstack_out(result, reshaped_tensors);
        */
}


pub fn column_stack(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(tensors.size() > 0,
                  "column_stack expects a non-empty TensorList");

      auto reshaped_tensors = reshape_input_for_column_stack(tensors);
      return hstack(reshaped_tensors);
        */
}


pub fn propagate_transposed_names(
        result: &mut Tensor,
        other:  &Tensor,
        dim0:   i64,
        dim1:   i64) -> &mut Tensor {
    
    todo!();
        /*
            if (other.has_names()) {
        auto names = other.names().vec();
        swap(names[dim0], names[dim1]);
        namedinference::propagate_names_if_nonempty(result, names);
      }
      return result;
        */
}


pub fn transpose_a(
        self_: &Tensor,
        dim0:  Dimname,
        dim1:  Dimname) -> Tensor {
    
    todo!();
        /*
            return transpose(
          self, dimname_to_position(self, dim0), dimname_to_position(self, dim1));
        */
}


pub fn transpose_b(
        self_: &mut Tensor,
        dim0:  i64,
        dim1:  i64) -> &mut Tensor {
    
    todo!();
        /*
            auto ndims = self.dim();
      dim0 = maybe_wrap_dim(dim0, ndims);
      dim1 = maybe_wrap_dim(dim1, ndims);
      if (dim0 == dim1) {
        return self;
      }

      if (self.is_sparse()) {
        return sparse_transpose_(self, dim0, dim1);
      }

      if (self.is_mkldnn()) {
        return _mkldnn_transpose_(self, dim0, dim1);
      }

      DimVector sizes(self.sizes().begin(), self.sizes().end());
      DimVector strides(self.strides().begin(), self.strides().end());
      swap(strides[dim0], strides[dim1]);
      swap(sizes[dim0], sizes[dim1]);
      self.as_strided_(sizes, strides);
      return self;
        */
}


pub fn transpose_c(
        self_: &Tensor,
        dim0:  i64,
        dim1:  i64) -> Tensor {
    
    todo!();
        /*
            auto ndims = self.dim();
      dim0 = maybe_wrap_dim(dim0, ndims);
      dim1 = maybe_wrap_dim(dim1, ndims);
      if (dim0 == dim1) {
        return self;
      }

      if (self.is_sparse()) {
        Tensor self_clone = self.clone();  // yes, this is what THS does
        return sparse_transpose_(self_clone, dim0, dim1);
      }

      if (self.is_mkldnn()) {
        return _mkldnn_transpose(self, dim0, dim1);
      }

      DimVector sizes(self.sizes().begin(), self.sizes().end());
      DimVector strides(self.strides().begin(), self.strides().end());
      swap(strides[dim0], strides[dim1]);
      swap(sizes[dim0], sizes[dim1]);
      auto result = self.as_strided(sizes, strides);
      propagate_transposed_names(result, self, dim0, dim1);
      return result;
        */
}


pub fn check_t(
        self_: &Tensor,
        fn_:   *const u8)  {
    
    todo!();
        /*
            if (self.is_sparse()) {
        i64 sparse_dim = self.sparse_dim();
        i64 dense_dim = self.dense_dim();
        TORCH_CHECK(sparse_dim <= 2 && dense_dim == 0,
                 fn, " expects a tensor with <= 2 sparse and 0 dense dimensions, but got ",
                 sparse_dim, " sparse and ", dense_dim, " dense dimensions");
      } else {
        TORCH_CHECK(self.dim() <= 2,
                 fn, " expects a tensor with <= 2 dimensions, but self is ", self.dim(), "D");
      }
        */
}


pub fn t_a(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            check_t(self, "t()");
      return self.transpose(0, self.dim() < 2 ? 0 : 1);
        */
}


pub fn t_b(self_: &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            check_t(self, "t_()");
      return self.transpose_(0, self.dim() < 2 ? 0 : 1);
        */
}


pub fn infer_squeeze_geometry_a(tensor: &Tensor) -> (DimVector,DimVector) {
    
    todo!();
        /*
            DimVector sizes;
      DimVector strides;

      for(const auto d : irange(tensor.dim())) {
        if(tensor.sizes()[d] != 1) {
          sizes.push_back(tensor.sizes()[d]);
          strides.push_back(tensor.strides()[d]);
        }
      }

      return make_tuple(move(sizes), move(strides));
        */
}

pub fn infer_squeeze_geometry_b(
    tensor: &Tensor,
    dim:    i64) -> (DimVector,DimVector) {
    
    todo!();
        /*
            DimVector sizes;
      DimVector strides;

      for(const auto d : irange(tensor.dim())) {
        if(d != dim || tensor.sizes()[dim] != 1) {
          sizes.push_back(tensor.sizes()[d]);
          strides.push_back(tensor.strides()[d]);
        }
      }
      return make_tuple(move(sizes), move(strides));
        */
}

/**
  | Named type instead of a pair/tuple so that we
  | can be sure to construct the vectors in place
  | and get NRVO.
  |
  */
pub struct InferUnsqueezeGeometryResult {
    sizes:   DimVector,
    strides: DimVector,
}

impl InferUnsqueezeGeometryResult {
    
    pub fn new(
        tensor_sizes:   &[i32],
        tensor_strides: &[i32]) -> Self {
    
        todo!();
        /*


            : sizes(tensor_sizes.begin(), tensor_sizes.end())
          , strides(tensor_strides.begin(), tensor_strides.end())
        */
    }
}

pub fn infer_unsqueeze_geometry(
    tensor: &Tensor,
    dim:    i64) -> InferUnsqueezeGeometryResult {

    todo!();
        /*
            InferUnsqueezeGeometryResult result(tensor.sizes(), tensor.strides());
      i64 new_stride = dim >= tensor.dim() ? 1 : result.sizes[dim] * result.strides[dim];
      result.sizes.insert(result.sizes.begin() + dim, 1);
      result.strides.insert(result.strides.begin() + dim, new_stride);

      return result;
        */
}

pub fn squeeze_qtensor_a(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            auto quantizer = get_qtensorimpl(self)->quantizer();
      DimVector sizes;
      DimVector strides;
      tie(sizes, strides) = inferSqueezeGeometry(self);
      if (quantizer->qscheme() == QScheme::PER_CHANNEL_AFFINE) {
        const auto* per_channel_quantizer = static_cast<PerChannelAffineQuantizer*>(quantizer.get());
        auto axis = per_channel_quantizer->axis();
        i64 shift = 0;
        for (const auto d : irange(self.dim())) {
          if (self.sizes()[d] == 1) {
            TORCH_CHECK(axis != d, "Squeeze is only possible on non-axis dimension for Per-Channel Quantized Tensors.");
            if (d < axis) {
              shift += 1;
            }
          }
        }
        axis = axis - shift;
        quantizer = make_per_channel_affine_quantizer(per_channel_quantizer->scales(),
                                                      per_channel_quantizer->zero_points(),
                                                      axis,
                                                      quantizer->scalar_type());
      }
      return make_qtensor(self, sizes, strides, quantizer);
        */
}

pub fn squeeze_qtensor_b(
    self_: &Tensor,
    dim:   i64) -> Tensor {

    todo!();
        /*
            auto quantizer = get_qtensorimpl(self)->quantizer();
      DimVector sizes;
      DimVector strides;
      tie(sizes, strides) = inferSqueezeGeometry(self, dim);
      if (quantizer->qscheme() == QScheme::PER_CHANNEL_AFFINE) {
        const auto* per_channel_quantizer = static_cast<PerChannelAffineQuantizer*>(quantizer.get());
        auto axis = per_channel_quantizer->axis();
        TORCH_CHECK(axis != dim, "Squeeze is only possible on non-axis dimension for Per-Channel Quantized Tensors.");
        if (axis >= dim) {
          axis -= 1;
        }
        quantizer = make_per_channel_affine_quantizer(per_channel_quantizer->scales(),
                                                      per_channel_quantizer->zero_points(),
                                                      axis,
                                                      quantizer->scalar_type());
      }
      if (self.dim() == 0 || self.sizes()[dim] != 1) {
        sizes = self.sizes().vec();
        strides = self.strides().vec();
      }
      auto result = make_qtensor(self, sizes, strides, quantizer);
      namedinference::propagate_names_except(result, self, {dim});
      return result;
        */
}

pub fn squeeze_a(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            auto g = inferSqueezeGeometry(self);
      Tensor result;
      if (self.is_quantized()) {
        result = squeeze_qtensor(self);
      } else {
        result = self.as_strided(get<0>(g), get<1>(g));
      }
      auto maybe_outnames = namedinference::compute_squeeze_outnames(self);
      namedinference::propagate_names_if_nonempty(result, maybe_outnames);
      return result;
        */
}

pub fn squeeze_b(
        self_: &Tensor,
        dim:   i64) -> Tensor {
    
    todo!();
        /*
            i64 dims = self.dim();
      dim = maybe_wrap_dim(dim, dims);

      if (self.is_quantized()) {
        return squeeze_qtensor(self, dim);
      }
      if (dims == 0 || self.sizes()[dim] != 1) {
        return self.as_strided(self.sizes(), self.strides());
      }
      auto g = inferSqueezeGeometry(self, dim);
      auto result = self.as_strided(get<0>(g), get<1>(g));
      namedinference::propagate_names_except(result, self, {dim});
      return result;
        */
}

pub fn squeeze_c(self_: &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            auto g = inferSqueezeGeometry(self);
      self.as_strided_(get<0>(g), get<1>(g));
      return self;
        */
}

pub fn squeeze_d(
        self_: &mut Tensor,
        dim:   i64) -> &mut Tensor {
    
    todo!();
        /*
            i64 dims = self.dim();
      dim = maybe_wrap_dim(dim, self.dim());

      if (dims == 0 || self.sizes()[dim] != 1) {
        self.as_strided_(self.sizes(), self.strides());
        return self;
      }
      auto g = inferSqueezeGeometry(self, dim);
      self.as_strided_(get<0>(g), get<1>(g));
      return self;
        */
}

/**
  | _unsafe_view() differs from view() in that the
  | returned tensor isn't treated as a view for the
  | purposes of automatic differentiation. (It's
  | not listed in VIEW_FUNCTIONS in
  | gen_autograd.py).
  |
  | It's only safe to use if the `self` tensor is
  | temporary. For example, the viewed tensor here
  | (a + b) is discarded immediately after viewing:
  |
  |  res = _unsafe_view(a + b, size);
  |
  | This is a hack because in-place operations on
  | tensors treated like views can be much more
  | expensive than the same operations on non-view
  | tensors.
  */
pub fn unsafe_view(
        self_: &Tensor,
        size:  &[i32]) -> Tensor {
    
    todo!();
        /*
            return self.view(size);
        */
}

pub fn unsqueeze_sparse(
    self_: &Tensor,
    /* should already be wrapped */
    dim:   i64) -> Tensor {
    
    todo!();
        /*
            i64 sparse_dim = self.sparse_dim();
      i64 dense_dim = self.dense_dim();
      auto indices = self._indices();
      auto sizes = self.sizes().vec();
      sizes.insert(sizes.begin() + dim, 1);
      if (dim <= sparse_dim) {
        auto new_indices = native::cat(
            {indices.narrow(0, 0, dim),
             native::zeros(
                 {1, indices.size(1)},
                 kLong,
                 indices.options().layout_opt(),
                 indices.options().device_opt(),
                 indices.options().pinned_memory_opt()),
             indices.narrow(0, dim, indices.size(0) - dim)});
        return _sparse_coo_tensor_with_dims_and_tensors(
            sparse_dim + 1, dense_dim, sizes, new_indices, self._values(), self.options());
      } else {
        return _sparse_coo_tensor_with_dims_and_tensors(
            sparse_dim, dense_dim + 1, sizes, indices, self._values().unsqueeze(dim - sparse_dim + 1), self.options());
      }
        */
}

pub fn unsqueeze_qtensor(
        self_: &Tensor,
        dim:   i64) -> Tensor {
    
    todo!();
        /*
            dim = maybe_wrap_dim(dim, self.dim() + 1);
      auto g = inferUnsqueezeGeometry(self, dim);
      auto quantizer = get_qtensorimpl(self)->quantizer();
      if (quantizer->qscheme() == QScheme::PER_CHANNEL_AFFINE) {
        const auto* per_channel_quantizer = static_cast<PerChannelAffineQuantizer*>(quantizer.get());
        auto axis = per_channel_quantizer->axis();
        if (axis >= dim) {
          axis += 1;
        }
        quantizer = make_per_channel_affine_quantizer(per_channel_quantizer->scales(),
                                                      per_channel_quantizer->zero_points(),
                                                      axis,
                                                      quantizer->scalar_type());
      }
      return make_qtensor(self, g.sizes, g.strides, quantizer);
        */
}

pub fn unsqueeze_a(
        self_: &Tensor,
        dim:   i64) -> Tensor {
    
    todo!();
        /*
            dim = maybe_wrap_dim(dim, self.dim() + 1);

      if (self.is_sparse()) {
        return unsqueeze_sparse(self, dim);
      } else if (self.is_quantized()) {
        return unsqueeze_qtensor(self, dim);
      } else {
        auto g = inferUnsqueezeGeometry(self, dim);
        return self.as_strided(g.sizes, g.strides);
      }
        */
}

pub fn unsqueeze_b(
        self_: &mut Tensor,
        dim:   i64) -> &mut Tensor {
    
    todo!();
        /*
            dim = maybe_wrap_dim(dim, self.dim() + 1);

      auto g = inferUnsqueezeGeometry(self, dim);
      self.as_strided_(g.sizes, g.strides);
      return self;
        */
}

pub fn flatten_a(
        self_:     &Tensor,
        start_dim: i64,
        end_dim:   i64) -> Tensor {
    
    todo!();
        /*
            start_dim = maybe_wrap_dim(start_dim, self.dim());
      end_dim = maybe_wrap_dim(end_dim, self.dim());
      TORCH_CHECK(start_dim <= end_dim, "flatten() has invalid args: start_dim cannot come after end_dim");

      if (self.dim() == 0) {
        return self.reshape({1});
      }
      if (start_dim == end_dim) {
        return self;
      }

      // We don't want to infer_size on the entire shape, because that can give us an extra degree
      // of freedom we don't want; for example, consider shape [0, 1, 3, 0], with start_dim=1, end_dim=2.
      // It's clear we want result shape [0, 3, 0] but passing [0, -1, 0] to infer_size means the -1
      // can take on any value and satisfy the constraints.
      auto slice_numel = multiply_integers(self.sizes().slice(start_dim, end_dim - start_dim + 1));
      vector<i64> shape;
      shape.reserve(self.dim() - end_dim + start_dim);
      for (const auto i : irange(start_dim)) {
        shape.push_back(self.sizes()[i]);
      }
      shape.push_back(slice_numel);
      for (const auto i : irange(end_dim + 1, self.dim())) {
        shape.push_back(self.sizes()[i]);
      }

      return native::reshape(self, shape);
        */
}

pub fn flatten_b(
        self_:     &Tensor,
        start_dim: i64,
        end_dim:   i64,
        out_dim:   Dimname) -> Tensor {
    
    todo!();
        /*
            auto outnames = self.names().vec();
      outnames.erase(outnames.begin() + start_dim, outnames.begin() + end_dim + 1);
      outnames.insert(outnames.begin() + start_dim, out_dim);

      Tensor result;
      {
        NoNamesGuard guard;
        result = native::flatten(self, start_dim, end_dim);
      }
      internal_set_names_inplace(result, outnames);
      return result;
        */
}

pub fn flatten_c(
        self_:     &Tensor,
        start_dim: Dimname,
        end_dim:   Dimname,
        out_dim:   Dimname) -> Tensor {
    
    todo!();
        /*
            auto start_pos = dimname_to_position(self, start_dim);
      auto end_pos  = dimname_to_position(self, end_dim);
      return native::flatten(self, start_pos, end_pos, out_dim);
        */
}

pub fn flatten_d(
        self_:   &Tensor,
        dims:    DimnameList,
        out_dim: Dimname) -> Tensor {
    
    todo!();
        /*
            auto positions = dimnames_to_positions(self, dims);
      for (const auto i : irange(positions.size() - 1)) {
        if (positions[i] + 1 == positions[i + 1]) continue;
        TORCH_CHECK(positions[i] + 1 == positions[i + 1],
            "flatten(tensor, dims, out_dim): dims ", dims, " must be consecutive ",
            "in Tensor", self.names());
      }
      return native::flatten(self, *dims.begin(), *(dims.end() - 1), out_dim);
        */
}

pub fn ravel(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            return self.reshape(-1);
        */
}

#[inline] pub fn handle_unflatten_exception(
        e:     &RuntimeError,
        self_: &Tensor,
        dim:   i64,
        sizes: &[i32],
        names: Option<DimnameList>)  {
    
    todo!();
        /*
            if (!strstr(e.what(), "is invalid for input of size")) {
        TORCH_CHECK(false, "unflatten got an unexpected error:\n", e.what());
      }

      if (self.has_names()) {
        TORCH_CHECK(false,
                    "unflatten: Provided sizes ", sizes, " don't multiply up to the size of dim ",
                    dim, " (", self.names()[dim], ": ", self.size(dim), ") in Tensor", self.names());

      } else {
        TORCH_CHECK(false,
                    "unflatten: Provided sizes ", sizes, " don't multiply up to the size of dim ",
                    dim, " (", self.size(dim), ") in the input tensor");
      }
        */
}

pub fn unflatten_a(
        self_: &Tensor,
        dim:   i64,
        sizes: &[i32],
        names: Option<DimnameList>) -> Tensor {
    
    todo!();
        /*
            dim = maybe_wrap_dim(dim, self.dim());

      TORCH_CHECK(sizes.size() > 0, "unflatten: sizes must be non-empty");
      TORCH_INTERNAL_ASSERT(!names || names->size() == sizes.size());
      if (self.has_names()) {
        TORCH_CHECK(names, "unflatten: input is a named tensor but no names were given for unflattened sizes");
      }

      DimVector inferred_size;
      try {
        inferred_size = infer_size_dv(sizes, self.size(dim));
      } catch (const runtime_error& e) {
        // infer_size would throw runtime_error for invalid size,
        // catch the runtime_error and display the error message in a more user-friendly way
        // for both tensors and named tensors
        handle_unflatten_exception(e, self, dim, sizes, names);
      }

      DimVector shape(self.sizes().begin(), self.sizes().end());
      shape.erase(shape.begin() + dim);
      shape.insert(shape.begin() + dim, inferred_size.begin(), inferred_size.end());

      Tensor result;
      {
        NoNamesGuard guard;
        result = self.view(shape);
      }

      if (names) {
        auto outnames = self.names().vec();
        outnames.erase(outnames.begin() + dim);
        outnames.insert(outnames.begin() + dim, names->begin(), names->end());
        internal_set_names_inplace(result, outnames);
      }

      return result;
        */
}

pub fn unflatten_b(
        self_: &Tensor,
        dim:   Dimname,
        sizes: &[i32],
        names: DimnameList) -> Tensor {
    
    todo!();
        /*
            return native::unflatten(self, dimname_to_position(self, dim), sizes, names);
        */
}

pub fn view_as(
        self_: &Tensor,
        other: &Tensor) -> Tensor {
    
    todo!();
        /*
            return self.view(other.sizes());
        */
}

pub fn numel(self_: &Tensor) -> i64 {
    
    todo!();
        /*
            return self.unsafeGetTensorImpl()->numel();
        */
}

pub fn unbind_a(
        self_: &Tensor,
        dim:   i64) -> Vec<Tensor> {
    
    todo!();
        /*
            dim = maybe_wrap_dim(dim, self.dim());
      i64 size = self.size(dim);
      vector<Tensor> tensors(size);
      for (const auto i : irange(size)) {
        tensors[i] = self.select(dim, i);
      }
      return tensors;
        */
}

pub fn unbind_b(
        self_: &Tensor,
        dim:   Dimname) -> Vec<Tensor> {
    
    todo!();
        /*
            return unbind(self, dimname_to_position(self, dim));
        */
}

pub fn meshgrid(tensors: TensorList) -> Vec<Tensor> {
    
    todo!();
        /*
            i64 size = tensors.size();
      TORCH_CHECK(size > 0, "meshgrid expects a non-empty TensorList");
      vector<i64> shape(size);
      for(const auto i: irange(size)){
        switch (tensors[i].dim()) {
        case 0:
          shape[i] = 1;
          break;
        case 1:
          shape[i] = tensors[i].size(0);
          break;
        default:
          AT_ERROR("Expected scalar or 1D tensor in the tensor list but got: ", tensors[i]);
        }
      }
      for(const auto i: irange(size - 1)){
          TORCH_CHECK(tensors[i].dtype() == tensors[i+1].dtype(), "meshgrid expects all tensors to have the same dtype");
          TORCH_CHECK(tensors[i].device() == tensors[i+1].device(), "meshgrid expects all tensors to have the same device");
      }
      vector<Tensor> grids;
      for(const auto i: irange(size)){
        vector<i64> view_shape(size, 1);
        view_shape[i] = -1;
        grids.push_back(tensors[i].view(view_shape).expand(shape));
      }
      return grids;
        */
}

/**
  | Numpy-style `a.T`: returns the tensor
  | with dims reversed
  |
  */
pub fn numpy_t(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            i64 n = self.dim();
      DimVector transpose_dims;
      for (i64 i = n - 1; i >= 0; --i) {
        transpose_dims.push_back(i);
      }
      return self.permute(transpose_dims);
        */
}

pub fn view(
        self_: &Tensor,
        size:  &[i32]) -> Tensor {
    
    todo!();
        /*
            DimVector inferred_size = infer_size_dv(size, self.numel());
      auto stride = computeStride(self.sizes(),
                                              self.strides(),
                                              inferred_size);
      TORCH_CHECK(stride.has_value(), "view size is "
        "not compatible with input tensor's size and stride (at least one dimension"
        " spans across two contiguous subspaces). Use .reshape(...) instead.");
      auto stride_value = *stride;
      return alias_with_sizes_and_strides(self, inferred_size, stride_value);
        */
}

pub fn alias(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            return alias_with_sizes_and_strides(self, self.sizes(), self.strides());
        */
}

pub fn detach(self_: &Tensor) -> Tensor {
    
    todo!();
        /*
            // this just exists to give us a hook in VariableType and an entry in Declarations.yaml
      //AT_ERROR("detach is not implemented for Tensor");
      return native::alias(self);
        */
}


pub fn unfold(
        self_:     &Tensor,
        dimension: i64,
        size:      i64,
        step:      i64) -> Tensor {
    
    todo!();
        /*
            // some special handling to deal with allow dimension == 0 when self.dim() == 0
      dimension = maybe_wrap_dim(dimension, self.dim(), /*wrap_scalar=*/true);

      const auto sizes = self.sizes();
      const auto strides = self.strides();
      i64 max_size = self.dim() == 0 ? 1 : sizes[dimension];
      TORCH_CHECK(size <= max_size, "maximum size for tensor at dimension ", dimension,
                                    " is ", max_size, " but size is ", size);
      TORCH_CHECK(step > 0, "step is ", step, " but must be > 0");

      DimVector new_size(self.dim() + 1);
      DimVector new_stride(self.dim() + 1);

      new_size[self.dim()] = size;
      new_stride[self.dim()] = self.dim() == 0 ? 1 : strides[dimension];
      for(const auto d : irange(self.dim())) {
        const auto self_size = sizes[d];
        const auto self_stride = strides[d];
        if(d == dimension) {
          new_size[d] = (self_size - size) / step + 1;
          new_stride[d] = step*self_stride;
        } else {
          new_size[d] = self_size;
          new_stride[d] = self_stride;
        }
      }

      return self.as_strided(new_size, new_stride);
        */
}

pub fn apply_diag<Scalar>(
        result:    &mut Tensor,
        self_:     &Tensor,
        dimension: i64)  {

    todo!();
        /*
            TORCH_CHECK(self.dim() == 1 || self.dim() == 2, "matrix or a vector expected");

      auto self_data = self.data_ptr<Scalar>();
      if (self.dim() == 1) {
        auto self_size = self.size(0);
        auto self_stride = self.stride(0);
        i64 sz = self_size + abs(dimension);

        native::resize_output(result, {sz, sz});
        result.zero_();
        auto r_data = result.data_ptr<Scalar>();
        auto r_stride_0 = result.stride(0);
        auto r_stride_1 = result.stride(1);
        r_data += (dimension >= 0 ? dimension*r_stride_1 : -dimension*r_stride_0);

        for (const auto i : irange(self_size)) {
          r_data[i * (r_stride_0 + r_stride_1)] = self_data[i * self_stride];
        }
      } else {
        auto self_stride_0 = self.stride(0);
        auto self_stride_1 = self.stride(1);

        i64 sz;
        if (dimension >= 0) {
          sz = min(self.size(0), self.size(1) - dimension);
        } else {
          sz = min(self.size(0) + dimension, self.size(1));
        }

        native::resize_output(result, {sz});
        result.zero_();
        auto r_data = result.data_ptr<Scalar>();
        auto r_stride_0 = result.stride(0);
        self_data += (dimension >= 0 ? dimension * self_stride_1 : -dimension * self_stride_0);
        for (const auto i : irange(sz)) {
          r_data[i * r_stride_0] = self_data[i * (self_stride_0 + self_stride_1)];
        }
      }
        */
}

pub fn diag(
        self_:     &Tensor,
        dimension: i64) -> Tensor {
    
    todo!();
        /*
            Tensor result = empty({0}, self.options());
      diag_out(result, self, dimension);
      return result;
        */
}

pub fn diag_cpu_out(
        self_:     &Tensor,
        dimension: i64,
        result:    &mut Tensor) -> &mut Tensor {
    
    todo!();
        /*
            AT_DISPATCH_ALL_TYPES_AND_COMPLEX_AND(ScalarType::Bool, self.scalar_type(), "diag", [&] {
        apply_diag<Scalar>(result, self, dimension);
      });
      return result;
        */
}

pub fn diag_backward(
        grad:        &Tensor,
        input_sizes: &[i32],
        diagonal:    i64) -> Tensor {
    
    todo!();
        /*
            auto ndimension = input_sizes.size();
      AT_ASSERT(ndimension == 1 || ndimension == 2);

      if (ndimension == 1 || input_sizes[0] == input_sizes[1]) {
        return grad.diag(diagonal);
      }

      // Input was a matrix but was not square
      auto grad_input = zeros(input_sizes, grad.options());
      auto diag = grad_input.diagonal(diagonal);
      diag.copy_(grad);
      return grad_input;
        */
}


pub fn diagonal_backward(
        grad:        &Tensor,
        input_sizes: &[i32],
        offset:      i64,
        dim1:        i64,
        dim2:        i64) -> Tensor {
    
    todo!();
        /*
            auto grad_input = zeros(input_sizes, grad.options());
      auto diag = grad_input.diagonal(offset, dim1, dim2);
      diag.copy_(grad);
      return grad_input;
        */
}

pub fn movedim_b(
        self_: &Tensor,
        src:   &[i32],
        dst:   &[i32]) -> Tensor {
    
    todo!();
        /*
            TORCH_CHECK(src.size() == dst.size(), "movedim: Invalid source or destination dims: source (",
                  src, " dims ) should contain the same number of dims as destination (", dst, " dims)");

      usize self_dim = self.dim();
      DimVector normalized_src(src.size());
      DimVector normalized_dst(dst.size());

      auto wrap_dims = [&self_dim](const IntArrayRef& vec, DimVector& normalized_vec) {
        for (const auto i : irange(vec.size())) {
          normalized_vec[i] = maybe_wrap_dim(vec[i], self_dim);
        }
      };

      wrap_dims(src, normalized_src);
      wrap_dims(dst, normalized_dst);

      auto all_unique = [](const DimVector& dims) {
        DimVector copy = dims;
        sort(copy.begin(), copy.end());
        auto duplicate = adjacent_find(copy.begin(), copy.end());
        return duplicate == copy.end();
      };
      TORCH_CHECK(all_unique(normalized_src), "movedim: repeated dim in `source` (", src, ")");
      TORCH_CHECK(all_unique(normalized_dst), "movedim: repeated dim in `destination` (", dst, ")");

      // TODO: The algorithm below can probably be optimized.
      // Reference: https://github.com/pytorch/pytorch/pull/41480#discussion_r456100505

      // Algorithm Walkthrough
      // Example Input
      // Variable State:
      //     normalized_src = 0, 1
      //     normalized_dst = 2, 4
      //     self_dim = 5
      DimVector order(self_dim);
      DimVector source_dims(self_dim);
      DimVector destination_dims(self_dim);

      // We initialize two vectors to track update to the dims
      // `order` contains the final order of the dim positions.
      // Variable State:
      //     order = NA, NA, NA, NA, NA
      //     source_dims = 0, 1, 2, 3, 4
      //     destination_dims = 0, 1, 2, 3, 4
      iota(source_dims.begin(), source_dims.end(), 0);
      iota(destination_dims.begin(), destination_dims.end(), 0);

      // We mark and update position for the dim provided by user
      // i.e. `normalized_src` and `normalized_dims`
      // Variable State:
      //     order = NA, NA, 0, NA, 1
      //     source_dims = -1, -1, 2, 3, 4
      //     destination_dims = 0, 1, -1, 3, -1
      for (const auto i : irange(src.size())) {
          order[normalized_dst[i]] = normalized_src[i];
          source_dims[normalized_src[i]] = -1;
          destination_dims[normalized_dst[i]] = -1;
      }

      // Remove the dims whose position we already know,
      // the ones marked with -1 in previous step
      // Variable State:
      //     source_dims = 2, 3, 4
      //     destination_dims = 0, 1, 3
      auto source_iter = remove(source_dims.begin(), source_dims.end(), -1);
      auto destination_iter = remove(destination_dims.begin(), destination_dims.end(), -1);

      i64 rest_dim = self.dim() - src.size();
      TORCH_INTERNAL_ASSERT(distance(source_dims.begin(), source_iter)  == rest_dim);
      TORCH_INTERNAL_ASSERT(distance(destination_dims.begin(), destination_iter)  == rest_dim);

      // Update the position of the remaining dimensions.
      // `source_dims` now contains the original position
      // `destination_dims` contains the new position it will shifted to
      // after considering the user inputs.
      // Variable State:
      //     order = 2, 3, 0, 4, 1
      for (const auto i : irange(rest_dim)) {
          order[destination_dims[i]] = source_dims[i];
      }

      return self.permute(order);
        */
}


pub fn movedim_a(
        self_: &Tensor,
        src:   i64,
        dst:   i64) -> Tensor {
    
    todo!();
        /*
            return movedim(self, IntArrayRef{src}, IntArrayRef{dst});
        */
}


pub fn moveaxis_a(
        self_: &Tensor,
        src:   &[i32],
        dst:   &[i32]) -> Tensor {
    
    todo!();
        /*
            return movedim(self, src, dst);
        */
}

pub fn moveaxis_b(
        self_: &Tensor,
        src:   i64,
        dst:   i64) -> Tensor {
    
    todo!();
        /*
            return movedim(self, IntArrayRef{src}, IntArrayRef{dst});
        */
}


pub fn swapaxes_a(
        self_: &Tensor,
        axis0: i64,
        axis1: i64) -> Tensor {
    
    todo!();
        /*
            return self.transpose(axis0, axis1);
        */
}

pub fn swapaxes_b(
        self_: &mut Tensor,
        axis0: i64,
        axis1: i64) -> &mut Tensor {
    
    todo!();
        /*
            return self.transpose_(axis0, axis1);
        */
}


pub fn swapdims_a(
        self_: &Tensor,
        dim0:  i64,
        dim1:  i64) -> Tensor {
    
    todo!();
        /*
            return self.transpose(dim0, dim1);
        */
}

pub fn swapdims_b(
        self_: &mut Tensor,
        dim0:  i64,
        dim1:  i64) -> &mut Tensor {
    
    todo!();
        /*
            return self.transpose_(dim0, dim1);
        */
}


pub fn flatten_dense_tensors(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            static auto flatten = [](const Tensor &t) { return t.contiguous().view({-1}); };
      if (tensors.size() == 1)
        return flatten(tensors[0]);
      return cat(fmap(tensors, flatten));
        */
}


pub fn unflatten_dense_tensors(
        flat:    &Tensor,
        tensors: TensorList) -> Vec<Tensor> {
    
    todo!();
        /*
            vector<Tensor> outputs;
      outputs.reserve(tensors.size());
      usize offset = 0;
      for (const auto & tensor : tensors) {
        auto numel = tensor.numel();
        // If unflatten an empty tensor, create a new empty tensor using
        // flat tensor Options.
        // This can avoid the unflattened empty tensor to share the same storage
        // with other unflatten tensors.
        if (numel == 0) {
          outputs.push_back(empty({0}, flat.options()));
        } else {
          outputs.push_back(flat.narrow(0, offset, numel).view(tensor.sizes()));
          offset += numel;
        }
      }
      return outputs;
        */
}
