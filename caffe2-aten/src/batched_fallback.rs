crate::ix!();

//-------------------------------------------[.cpp/pytorch/aten/src/ATen/BatchedFallback.h]
//-------------------------------------------[.cpp/pytorch/aten/src/ATen/BatchedFallback.cpp]

/**
  | Given a linear index, return the actual index.
  |
  | Example: Given linear_idx = 3, sizes = [5, 2],
  | we would return [1, 0]
  */
pub fn compute_index(
        linear_idx: i64,
        sizes:      &[i32]) -> SmallVector<TensorIndex,kVmapStaticDimVecSize> {
    
    todo!();
        /*
            SmallVector<indexing::TensorIndex,kVmapStaticDimVecSize> result;
      result.reserve(sizes.size());
      for (auto it = sizes.rbegin(); it != sizes.rend(); it++) {
        auto remainder = linear_idx % *it;
        result.push_back(remainder);
        linear_idx -= remainder;
        linear_idx /= *it;
      }
      std::reverse(std::begin(result), std::end(result));
      return result;
        */
}

pub fn are_all_returns_tensors(schema: &FunctionSchema) -> bool {
    
    todo!();
        /*
            return std::all_of(
          schema.returns().begin(),
          schema.returns().end(),
          [] (const Argument& arg) { return arg.type() == TensorType::get(); });
        */
}

pub fn are_any_arguments_tensor_list(schema: &FunctionSchema) -> bool {
    
    todo!();
        /*
            return std::any_of(
          schema.arguments().begin(),
          schema.arguments().end(),
          [] (const Argument& arg) { return arg.type()->isSubtypeOf(ListType::ofTensors()); });
        */
}

/**
  | Returns if an operator is in-place. An operator
  | is inplace if:
  |
  | 1. The first argument is a Tensor and it is
  | being written to
  |
  | 2. The first argument is being returned
  |
  | 3. No other arguments are aliased
  |
  | Here is an example of an in-place operator:
  |
  | add_(Tensor(a!) self, Tensor other, *, Scalar
  | alpha=1) -> Tensor(a!)
  */
pub fn is_inplace_op(schema: &FunctionSchema) -> bool {
    
    todo!();
        /*
            if (!schema.is_mutable() || schema.returns().size() != 1) {
        return false;
      }
      // Check that the first argument is being written to
      const auto& first_arg_alias_info = schema.arguments().begin()->alias_info();
      if (!first_arg_alias_info || !first_arg_alias_info.value().isWrite()) {
        return false;
      }
      // Check that none of the other args are being aliased
      for (auto it = schema.arguments().begin() + 1; it != schema.arguments().end(); ++it) {
        const auto& alias_info = it->alias_info();
        if (alias_info) {
          return false;
        }
      }
      // Check that the first tensor is being returned (i.e., output has a (a!))
      const auto& return_alias_info = schema.returns()[0].alias_info();
      return return_alias_info && return_alias_info.value().isWrite();
        */
}

pub fn warn_fallback(
        schema:     &FunctionSchema,
        is_inplace: bool)  {
    
    todo!();
        /*
            if (!globalContext().areVmapFallbackWarningsEnabled()) {
        return;
      }
      auto uses_stack = is_inplace ? "" : " and stack";
      TORCH_WARN("Batching rule not implemented for ", schema.operator_name(), " falling back "
                 "to slow (for loop", uses_stack, ") implementation");
        */
}

/**
  | The general flow of the algorithm is
  | as follows.
  | 
  | - First, we figure out which arguments
  | are BatchedTensors and save them to
  | a vector. We also store a vector of which
  | index of the arguments list each BatchedTensor
  | appears in. This will be useful for bookkeeping
  | later.
  | 
  | - Next, we apply the MultiBatchVmapTransform
  | to all of the BatchedTensors.
  | 
  | This returns a vector of VmapPhysicalView
  | that hold tensors that contain all of
  | the collective batch dimensions at
  | the front of the tensors.
  | 
  | - Then, we attempt to call `op` once per
  | slice of the inputs. To do this, we repeatedly
  | we slice the input arguments (if they
  | are BatchedTensors), put the sliced
  | (or a not-sliced) version of the input
  | onto the stack, invoke the operator,
  | and then pop the results off the stack.
  |
  */
pub fn batched_tensor_inplace_for_loop_fallback(
        op:    &OperatorHandle,
        stack: *mut JitStack)  {
    
    todo!();
        /*
            const auto& schema = op.schema();
      warnFallback(schema, /*in_place*/true);

      const auto num_arguments = schema.arguments().size();
      const auto arguments = torch::jit::last(stack, num_arguments);
      const auto arguments_begin = stack->size() - num_arguments;

      // `self` is the Tensor being modified in-place
      Tensor self = arguments[0].toTensor();
      const auto* self_impl = maybeGetBatchedImpl(self);
      std::bitset<kVmapMaxTensorDims> self_vmap_levels;
      if (self_impl) {
        self_vmap_levels = createVmapLevelsBitset(self_impl->bdims());
      }

      // Figure out which arguments are BatchedTensor. Save them to a vector.
      // For each BatchedTensor, also record what position of `arguments` they came from.
      SmallVector<Tensor,kVmapTransformStaticInputSize> batched_tensor_inputs;
      VmapDimVector batched_tensor_inputs_position;
      for (i64 idx = 0; idx < arguments.size(); ++idx) {
        const auto& ivalue = arguments[idx];
        if (!ivalue.isTensor()) {
          continue;
        }
        const auto& tensor = ivalue.toTensor();
        if (!tensor.defined()) {
          continue;
        }
        const auto* batched = maybeGetBatchedImpl(tensor);
        if (!batched) {
          continue;
        }

        // NOTE: [vmap-incompatible in-place operations]
        // In-place operations on `self` are not possible if there exists some vmap
        // level `l` such that `self` is not being vmapped on that level but another
        // argument is. For example, let B0 be a batch dim inside vmap and consider
        // vmap(Tensor.add_, in_dims=(None, 0))(torch.ones(3), torch.ones(B0, 3))
        // - self is torch.ones(3) and does not participate in this vmap
        // - other is BatchedTensor(torch.ones(B0, 3))
        // There's no way to do self.add_(other) because `other` has more elements
        // elements than `self` due to being vmapped over.
        //
        // In the vmap fallback, we should error out when we detect this.
        auto other_vmap_levels = createVmapLevelsBitset(batched->bdims());
        if (self_vmap_levels != (self_vmap_levels | other_vmap_levels)) {
          // Find one vmap level to complain about
          auto additional_bdims = (self_vmap_levels | other_vmap_levels) ^ self_vmap_levels;
          auto offending_level = llvm::findLastSet(additional_bdims.to_ulong());
          // The following prints out "vmap: aten::add_(tensor, ...) is not possible",
          // but it would be better to print out "tensor.add_(...) is not possible".
          // Afaict there's no official way to get the add_ and there is no way to
          // tell if an operator has method or function variants.
          TORCH_CHECK(false,
            "vmap: ", schema.name(), "(self, *extra_args) is not possible because ",
            "there exists a Tensor `other` in extra_args that has more elements ",
            "than `self`. This happened due to `other` being vmapped over but ",
            "`self` not being vmapped over at level ", offending_level, ". ",
            "Please try to use out-of-place operators instead of ", schema.name(), ". ",
            "If said operator is being called inside the PyTorch framework, ",
            "please file a bug report instead.");
        }
        batched_tensor_inputs.push_back(tensor);
        batched_tensor_inputs_position.push_back(idx);
      }
      TORCH_INTERNAL_ASSERT(batched_tensor_inputs.size() > 0);

      // MultiBatchVmapTransform the BatchedTensor arguments. This returns
      // VmapPhysicalViews that contain all of the batch dimensions.
      const auto input_physical_views = MultiBatchVmapTransform::logicalToPhysical(
          batched_tensor_inputs);

      // Compute the total number of batches
      auto num_batch_dims = input_physical_views.front().numBatchDims();
      auto first_physical_view_sizes = input_physical_views.front().tensor().sizes();
      auto batch_sizes = ArrayRef<i64>(
          first_physical_view_sizes.begin(), first_physical_view_sizes.begin() + num_batch_dims);
      const auto num_batches = c10::multiply_integers(batch_sizes);
      // Without a shape-checking API, we're unable to compute the correct shape of
      // the output so we just error out.
      TORCH_CHECK(num_batches > 0,
          "Batching rule not implemented for ", schema.operator_name(), ". ",
          "The fallback path does not support vmap over dims of size 0.");

      // Strategy: For each batch, we are going to push slices (where applicable)
      // of the arguments onto `stack`, and call `op`.
      for (i64 linear_idx = 0; linear_idx < num_batches; ++linear_idx) {
        auto index = computeIndex(linear_idx, batch_sizes);
        auto batched_tensor_inputs_pos_iter = batched_tensor_inputs_position.begin();
        auto input_physical_views_iter = input_physical_views.begin();
        for (i64 arg_idx = 0; arg_idx < num_arguments; ++arg_idx) {
          // We assume that torch::jit::Stack is backed by vector<IValue> for
          // simplicity. When that is not the case, this code should be updated.
          const auto& argument = (*stack)[arguments_begin + arg_idx];
          if (batched_tensor_inputs_pos_iter == batched_tensor_inputs_position.end()
              || arg_idx != *batched_tensor_inputs_pos_iter) {
            // argument isn't a BatchedTensor
            torch::jit::push(stack, argument);
            continue;
          }
          // argument is a BatchedTensor
          TORCH_INTERNAL_ASSERT(input_physical_views_iter != input_physical_views.end());
          const auto& physical_view_for_argument = *input_physical_views_iter;
          torch::jit::push(stack, physical_view_for_argument.tensor().index(index));
          batched_tensor_inputs_pos_iter++;
          input_physical_views_iter++;
        }

        op.callBoxed(stack);
        torch::jit::drop(stack, 1);
      }

      // Return the tensor that was written to in-place
      torch::jit::drop(stack, num_arguments);
      torch::jit::push(stack, self);
        */
}

pub fn safe_stack(tensors: TensorList) -> Tensor {
    
    todo!();
        /*
            auto is_defined = [](const Tensor& t) { return t.defined(); };
      if (std::all_of(tensors.begin(), tensors.end(), is_defined)) {
        return at::stack(tensors);
      }
      // NOTE [vmap through backward and undefined grad]
      // While vmapping through backward functions (to compute batched grad), it
      // is possible for the backward function to return an undefined grad for some
      // grad_input for each example. In that case, we return an undefined grad.
      //
      // It is theoretically posssible for *some* of the examples to produce an
      // undefined grad (a kernel could peek at the gradient values and return an
      // undefined tensor if it determines the gradient is full of zeros). We
      // could handle this by treating the undefined grad as a zero-filled tensor
      // of the correct shape while stacking the tensors together. However I expect
      // this to happen very rarely (I have not been able to find an example in our
      // codebase) so we just error out in this case.
      if (std::none_of(tensors.begin(), tensors.end(), is_defined)) {
        return Tensor();
      }
      TORCH_CHECK(false,
          "vmap: slow fallback received a mix of undefined and defined tensors ",
          "as the result of an operation. This is not supported, please file us ",
          "an issue on github.");
        */
}

/**
  | If an operator doesn't have a batching rule
  | implemented then we fallback to this
  | implementation. The fallback only works on
  | out-of-place operators that return only tensors
  | with new memory. (e.g., no in-place operators,
  | no view operations).
  |
  | The fallback effectively takes all of the
  | BatchedTensors in `stack`, slices them, and
  | runs `op` on all of the corresponding slices to
  | produce slices of the outputs. The output
  | slices then get `torch.stack`ed to create the
  | final returns.
  |
  | The performance of the fallback is not very
  | good because it introduces an extra copy from
  | stacking the sliced outputs. Because of this,
  | we prefer to write batching rules for operators
  | whenever possible.
  |
  | The general flow of the algorithm is
  | as follows.
  | 
  | - First, we figure out which arguments
  | are BatchedTensors and save them to
  | a vector. We also store a vector of which
  | index of the arguments list each BatchedTensor
  | appears in. This will be useful for bookkeeping
  | later.
  | 
  | - Next, we apply the MultiBatchVmapTransform
  | to all of the BatchedTensors.
  | 
  | This returns a vector of VmapPhysicalView
  | that hold tensors that contain all of
  | the collective batch dimensions at
  | the front of the tensors.
  | 
  | - Then, we attempt to call `op` once per
  | slice of the inputs. To do this, we repeatedly
  | we slice the input arguments (if they
  | are BatchedTensors), put the sliced
  | (or a not-sliced) version of the input
  | onto the stack, invoke the operator,
  | and then pop the results off the stack.
  | 
  | - Each result obtained from the previous
  | step is a slice of the total result, so
  | we stack those tensors together to form
  | the final result.
  |
  */
pub fn batched_tensor_for_loop_fallback(
        op:    &OperatorHandle,
        stack: *mut JitStack)  {
    
    todo!();
        /*
            const auto& schema = op.schema();
      const auto num_returns = schema.returns().size();

      if (isInplaceOp(schema)) {
        batchedTensorInplaceForLoopFallback(op, stack);
        return;
      }
      TORCH_CHECK(!schema.is_mutable() && !schema.hasAnyAliasInfo(),
                  "Batching rule not implemented for ", schema.operator_name(), "; ",
                  "the fallback path doesn't work on out= or view ops.");
      TORCH_CHECK(areAllReturnsTensors(schema) && !areAnyArgumentsTensorList(schema),
                  "Batching rule not implemented for ", schema.operator_name(), ". ",
                  "We could not generate a fallback.");
      TORCH_CHECK(num_returns >= 1,
                  "Batching rule not implemented for ", schema.operator_name(), ". ",
                  "The fallback path does not support operations with no returns.");
      warnFallback(schema, /*in_place*/false);

      const auto num_arguments = schema.arguments().size();
      const auto arguments = torch::jit::last(stack, num_arguments);
      const auto arguments_begin = stack->size() - num_arguments;

      // Figure out which arguments are BatchedTensor. Save them to a vector.
      // For each BatchedTensor, also record what position of `arguments` they came from.
      SmallVector<Tensor,kVmapTransformStaticInputSize> batched_tensor_inputs;
      VmapDimVector batched_tensor_inputs_position;
      for (i64 idx = 0; idx < arguments.size(); ++idx) {
        const auto& ivalue = arguments[idx];
        if (!ivalue.isTensor()) {
          continue;
        }
        const auto& tensor = ivalue.toTensor();
        if (!tensor.defined()) {
          continue;
        }
        const auto* batched = maybeGetBatchedImpl(tensor);
        if (!batched) {
          continue;
        }
        batched_tensor_inputs.push_back(tensor);
        batched_tensor_inputs_position.push_back(idx);
      }
      TORCH_INTERNAL_ASSERT(batched_tensor_inputs.size() > 0);

      // MultiBatchVmapTransform the BatchedTensor arguments. This returns
      // VmapPhysicalViews that contain all of the batch dimensions.
      const auto input_physical_views = MultiBatchVmapTransform::logicalToPhysical(
          batched_tensor_inputs);

      // Compute the total number of batches
      auto num_batch_dims = input_physical_views.front().numBatchDims();
      auto some_sizes = input_physical_views.front().tensor().sizes();
      auto batch_sizes = ArrayRef<i64>(some_sizes.begin(), some_sizes.begin() + num_batch_dims);
      const auto num_batches = c10::multiply_integers(batch_sizes);
      // Without a shape-checking API, we're unable to compute the correct shape of
      // the output so we just error out.
      TORCH_CHECK(num_batches > 0,
          "Batching rule not implemented for ", schema.operator_name(), ". ",
          "The fallback path does not support vmap over dims of size 0.");

      // Strategy: For each batch, we are going to push slices (where applicable)
      // of the arguments onto `stack`, call `op`, and store the result in
      // `output_shards`.
      //
      // NOTE: [Output shards layout]
      // Assume that the operator has three outputs: a, b, c.
      // The layout of output_shards is as follows:
      // [ a0, a1, a2, a3, b0, b1, b2, b3, c0, c1, c2, c3]
      // This is so that we can call at::stack([a0...a3]), at::stack([b0...b3])
      // more easily in the next step.
      std::vector<Tensor> output_shards(num_batches * num_returns);

      for (i64 linear_idx = 0; linear_idx < num_batches; ++linear_idx) {
        auto index = computeIndex(linear_idx, batch_sizes);
        auto batched_tensor_inputs_pos_iter = batched_tensor_inputs_position.begin();
        auto input_physical_views_iter = input_physical_views.begin();
        for (i64 arg_idx = 0; arg_idx < num_arguments; ++arg_idx) {
          // We assume that torch::jit::Stack is backed by vector<IValue> for
          // simplicity. When that is not the case, this code should be updated.
          const auto& argument = (*stack)[arguments_begin + arg_idx];
          if (batched_tensor_inputs_pos_iter == batched_tensor_inputs_position.end()
              || arg_idx != *batched_tensor_inputs_pos_iter) {
            // argument isn't a BatchedTensor
            torch::jit::push(stack, argument);
            continue;
          }
          // argument is a BatchedTensor
          TORCH_INTERNAL_ASSERT(input_physical_views_iter != input_physical_views.end());
          const auto& physical_view_for_argument = *input_physical_views_iter;
          torch::jit::push(stack, physical_view_for_argument.tensor().index(index));
          batched_tensor_inputs_pos_iter++;
          input_physical_views_iter++;
        }

        op.callBoxed(stack);

        // Store the result into `output_shards`. See NOTE: [Output shards layout]
        // to learn about the details of how we store the shards.
        const auto returns = torch::jit::last(stack, num_returns);
        for (i64 return_idx = 0; return_idx < returns.size(); ++return_idx) {
          output_shards[num_batches * return_idx + linear_idx] = returns[return_idx].toTensor();
        }
        torch::jit::drop(stack, num_returns);
      }

      // For each output Tensor, stack the shards of the tensor together to form a return
      torch::jit::drop(stack, num_arguments);
      auto output_shards_chunks = MatrixRef<Tensor>(output_shards, num_batches);
      for (i64 return_idx = 0; return_idx < num_returns; ++return_idx) {
        auto shards = output_shards_chunks[return_idx];
        auto flat_output = safeStack(shards);
        // See NOTE [vmap through backward and undefined grad]
        if (!flat_output.defined()) {
          torch::jit::push(stack, flat_output);
          continue;
        }
        VmapDimVector output_sizes(batch_sizes);
        output_sizes.insert(
            output_sizes.end(),
            flat_output.sizes().begin() + 1,
            flat_output.sizes().end());
        torch::jit::push(
            stack,
            input_physical_views.front().getPhysicalToLogicalMap().apply(flat_output.view(output_sizes)));
      }
        */
}
