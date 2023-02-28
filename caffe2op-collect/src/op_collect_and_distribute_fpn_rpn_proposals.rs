crate::ix!();

/**
  | Merge RPN proposals generated at multiple
  | FPN levels and then distribute those
  | proposals to their appropriate FPN
  | levels for Faster RCNN.
  | 
  | An anchor at one FPN level may predict
  | an RoI that will map to another level,
  | hence the need to redistribute the proposals.
  | 
  | Only inference is supported. To train,
  | please use the original Python operator
  | in Detectron.
  | 
  | Inputs and outputs are examples only;
  | if min/max levels change, the number
  | of inputs and outputs, as well as their
  | level numbering, will change.
  | 
  | C++ implementation of
  | 
  | CollectAndDistributeFpnRpnProposalsOp
  | Merge RPN proposals generated at multiple
  | FPN levels and then distribute those
  | proposals to their appropriate FPN
  | levels for Faster RCNN. An anchor at
  | one FPN level may predict an RoI that
  | will map to another level, hence the
  | need to redistribute the proposals.
  | 
  | Reference: facebookresearch/Detectron/detectron/ops/collect_and_distribute_fpn_rpn_proposals.py
  |
  */
pub struct CollectAndDistributeFpnRpnProposalsOp<Context> {
    //USE_OPERATOR_CONTEXT_FUNCTIONS;
    storage: OperatorStorage,
    context: Context,

    /// ROI_CANONICAL_SCALE
    roi_canonical_scale:    i32, // {224};

    /// ROI_CANONICAL_LEVEL
    roi_canonical_level:    i32, // {4};

    /// ROI_MAX_LEVEL
    roi_max_level:          i32, // {5};

    /// ROI_MIN_LEVEL
    roi_min_level:          i32, // {2};

    /// RPN_MAX_LEVEL
    rpn_max_level:          i32, // {6};

    /// RPN_MIN_LEVEL
    rpn_min_level:          i32, // {2};

    /// RPN_POST_NMS_TOP_N
    rpn_post_nms_top_n:     i32, // {2000};

    /**
      | The infamous "+ 1" for box width and height
      | dating back to the DPM days
      |
      */
    legacy_plus_one:        bool, // {true};
}

num_inputs!{CollectAndDistributeFpnRpnProposals, (2,INT_MAX)}

num_outputs!{CollectAndDistributeFpnRpnProposals, (3,INT_MAX)}

inputs!{CollectAndDistributeFpnRpnProposals, 
    0 => ("rpn_rois_fpn2",         "RPN proposals for FPN level 2, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    1 => ("rpn_rois_fpn3",         "RPN proposals for FPN level 3, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    2 => ("rpn_rois_fpn4",         "RPN proposals for FPN level 4, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    3 => ("rpn_rois_fpn5",         "RPN proposals for FPN level 5, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    4 => ("rpn_rois_fpn6",         "RPN proposals for FPN level 6, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    5 => ("rpn_roi_probs_fpn2",    "RPN objectness probabilities for FPN level 2. See rpn_roi_probs documentation from GenerateProposals."),
    6 => ("rpn_roi_probs_fpn3",    "RPN objectness probabilities for FPN level 3. See rpn_roi_probs documentation from GenerateProposals."),
    7 => ("rpn_roi_probs_fpn4",    "RPN objectness probabilities for FPN level 4. See rpn_roi_probs documentation from GenerateProposals."),
    8 => ("rpn_roi_probs_fpn5",    "RPN objectness probabilities for FPN level 5. See rpn_roi_probs documentation from GenerateProposals."),
    9 => ("rpn_roi_probs_fpn6",    "RPN objectness probabilities for FPN level 6. See rpn_roi_probs documentation from GenerateProposals.")
}

outputs!{CollectAndDistributeFpnRpnProposals, 
    0 => ("rois",                  "Top proposals limited to rpn_post_nms_topN total, format (image_index, x1, y1, x2, y2)"),
    1 => ("rois_fpn2",             "RPN proposals for ROI level 2, format (image_index, x1, y1, x2, y2)"),
    2 => ("rois_fpn3",             "RPN proposals for ROI level 3, format (image_index, x1, y1, x2, y2)"),
    3 => ("rois_fpn4",             "RPN proposals for ROI level 4, format (image_index, x1, y1, x2, y2)"),
    4 => ("rois_fpn5",             "RPN proposals for ROI level 5, format (image_index, x1, y1, x2, y2)"),
    5 => ("rois_idx_restore",      "Permutation on the concatenation of all rois_fpni, i=min...max, such that when applied the RPN RoIs are restored to their original order in the input blobs.")
}

args!{CollectAndDistributeFpnRpnProposals, 
    0 => ("roi_canonical_scale",   "(int) ROI_CANONICAL_SCALE"),
    1 => ("roi_canonical_level",   "(int) ROI_CANONICAL_LEVEL"),
    2 => ("roi_max_level",         "(int) ROI_MAX_LEVEL"),
    3 => ("roi_min_level",         "(int) ROI_MIN_LEVEL"),
    4 => ("rpn_max_level",         "(int) RPN_MAX_LEVEL"),
    5 => ("rpn_min_level",         "(int) RPN_MIN_LEVEL"),
    6 => ("rpn_post_nms_topN",     "(int) RPN_POST_NMS_TOP_N")
}

should_not_do_gradient!{CollectAndDistributeFpnRpnProposals}

impl<Context> CollectAndDistributeFpnRpnProposalsOp<Context> {
    
    pub fn new<Args>(args: Args) -> Self {
        todo!();
        /*
            : Operator<Context>(std::forward<Args>(args)...),
            roi_canonical_scale_(
                this->template GetSingleArgument<int>("roi_canonical_scale", 224)),
            roi_canonical_level_(
                this->template GetSingleArgument<int>("roi_canonical_level", 4)),
            roi_max_level_(
                this->template GetSingleArgument<int>("roi_max_level", 5)),
            roi_min_level_(
                this->template GetSingleArgument<int>("roi_min_level", 2)),
            rpn_max_level_(
                this->template GetSingleArgument<int>("rpn_max_level", 6)),
            rpn_min_level_(
                this->template GetSingleArgument<int>("rpn_min_level", 2)),
            rpn_post_nms_topN_(
                this->template GetSingleArgument<int>("rpn_post_nms_topN", 2000)),
            legacy_plus_one_(
                this->template GetSingleArgument<bool>("legacy_plus_one", true)) 

        CAFFE_ENFORCE_GE(
            roi_max_level_,
            roi_min_level_,
            "roi_max_level " + c10::to_string(roi_max_level_) +
                " must be greater than or equal to roi_min_level " +
                c10::to_string(roi_min_level_) + ".");
        CAFFE_ENFORCE_GE(
            rpn_max_level_,
            rpn_min_level_,
            "rpn_max_level " + c10::to_string(rpn_max_level_) +
                " must be greater than or equal to rpn_min_level " +
                c10::to_string(rpn_min_level_) + ".");
        */
    }
}

impl CollectAndDistributeFpnRpnProposalsOp<CPUContext> {

    #[inline] pub fn run_on_device(&mut self) -> bool {
        
        todo!();
        /*
            int num_rpn_lvls = rpn_max_level_ - rpn_min_level_ + 1;
      CAFFE_ENFORCE_EQ(InputSize(), 2 * num_rpn_lvls);

      int num_roi_lvls = roi_max_level_ - roi_min_level_ + 1;
      CAFFE_ENFORCE_EQ(OutputSize(), num_roi_lvls + 2);

      // Collect rois and scores in Eigen
      // rois are in [[batch_idx, x0, y0, x1, y2], ...] format
      // Combine predictions across all levels and retain the top scoring
      //
      // equivalent to python code
      //   roi_inputs = inputs[:num_rpn_lvls]
      //   score_inputs = inputs[num_rpn_lvls:]
      //   rois = np.concatenate([blob.data for blob in roi_inputs])
      //   scores = np.concatenate([blob.data for blob in score_inputs]).squeeze()
      int proposal_num = 0;
      for (int i = 0; i < num_rpn_lvls; i++) {
        const auto& roi_in = Input(i);
        proposal_num += roi_in.size(0);
      }
      ERArrXXf rois(proposal_num, 5);
      EArrXf scores(proposal_num);
      int len = 0;
      for (int i = 0; i < num_rpn_lvls; i++) {
        const auto& roi_in = Input(i);
        const int n = roi_in.size(0);

        Eigen::Map<const ERArrXXf> roi(roi_in.data<float>(), n, 5);
        rois.block(len, 0, n, 5) = roi;

        const auto& score_in = Input(num_rpn_lvls + i);
        CAFFE_ENFORCE_EQ(score_in.size(0), n);

        // No need to squeeze, since we are reshaping when converting to Eigen
        // https://docs.scipy.org/doc/numpy/reference/generated/numpy.squeeze.html
        Eigen::Map<const EArrXf> score(score_in.data<float>(), n);
        scores.segment(len, n) = score;

        len += n;
      }

      // Grab only top rpn_post_nms_topN rois
      // equivalent to python code
      //   inds = np.argsort(-scores)[:rpn_post_nms_topN]
      //   rois = rois[inds, :]
      utils::SortAndLimitRoIsByScores(scores, rpn_post_nms_topN_, rois);

      // Distribute
      // equivalent to python code
      //   lvl_min = cfg.FPN.ROI_MIN_LEVEL
      //   lvl_max = cfg.FPN.ROI_MAX_LEVEL
      //   lvls = fpn.map_rois_to_fpn_levels(rois[:, 1:5], lvl_min, lvl_max)
      const int lvl_min = roi_min_level_;
      const int lvl_max = roi_max_level_;
      const int canon_scale = roi_canonical_scale_;
      const int canon_level = roi_canonical_level_;
      auto rois_block = rois.block(0, 1, rois.rows(), 4);
      auto lvls = utils::MapRoIsToFpnLevels(
          rois_block, lvl_min, lvl_max, canon_scale, canon_level, legacy_plus_one_);

      // equivalent to python code
      //   outputs[0].reshape(rois.shape)
      //   outputs[0].data[...] = rois

      auto* rois_out = Output(0, {rois.rows(), rois.cols()}, at::dtype<float>());
      Eigen::Map<ERArrXXf> rois_out_mat(
          rois_out->template mutable_data<float>(), rois.rows(), rois.cols());
      rois_out_mat = rois;

      // Create new roi blobs for each FPN level
      // (See: modeling.FPN.add_multilevel_roi_blobs which is similar but annoying
      // to generalize to support this particular case.)
      //
      // equivalent to python code
      //   rois_idx_order = np.empty((0, ))
      //   for (output_idx, lvl in enumerate(range(lvl_min, lvl_max + 1)))
      //       idx_lvl = np.where(lvls == lvl)[0]
      //       blob_roi_level = rois[idx_lvl, :]
      //       outputs[output_idx + 1].reshape(blob_roi_level.shape)
      //       outputs[output_idx + 1].data[...] = blob_roi_level
      //       rois_idx_order = np.concatenate((rois_idx_order, idx_lvl))
      //   rois_idx_restore = np.argsort(rois_idx_order)
      //   blob_utils.py_op_copy_blob(rois_idx_restore.astype(np.int32),
      //   outputs[-1])
      EArrXi rois_idx_restore;
      for (int i = 0, lvl = lvl_min; i < num_roi_lvls; i++, lvl++) {
        ERArrXXf blob_roi_level;
        EArrXi idx_lvl;
        utils::RowsWhereRoILevelEquals(rois, lvls, lvl, &blob_roi_level, &idx_lvl);

        // Output blob_roi_level

        auto* roi_out = Output(
            i + 1,
            {blob_roi_level.rows(), blob_roi_level.cols()},
            at::dtype<float>());
        Eigen::Map<ERArrXXf> roi_out_mat(
            roi_out->template mutable_data<float>(),
            blob_roi_level.rows(),
            blob_roi_level.cols());
        roi_out_mat = blob_roi_level;

        // Append indices from idx_lvl to rois_idx_restore
        rois_idx_restore.conservativeResize(
            rois_idx_restore.size() + idx_lvl.size());
        rois_idx_restore.tail(idx_lvl.size()) = idx_lvl;
      }
      utils::ArgSort(rois_idx_restore);

      auto* rois_idx_restore_out =
          Output(OutputSize() - 1, {rois_idx_restore.size()}, at::dtype<int>());
      Eigen::Map<EArrXi> rois_idx_restore_out_mat(
          rois_idx_restore_out->template mutable_data<int>(),
          rois_idx_restore.size());
      rois_idx_restore_out_mat = rois_idx_restore;

      return true;
        */
    }
}
///-----------------------------------------------
pub struct CollectRpnProposalsOp<Context> {
    //USE_OPERATOR_CONTEXT_FUNCTIONS;
    storage: OperatorStorage,
    context: Context,

    rpn_max_level_:     i32, // {6};
    rpn_min_level_:     i32, // {2};
    rpn_post_nms_topN_: i32, // {2000};
}

num_inputs!{CollectRpnProposals, (2,INT_MAX)}

num_outputs!{CollectRpnProposals, 1}

inputs!{CollectRpnProposals, 
    0 => ("rpn_rois_fpn2",         "RPN proposals for FPN level 2, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    1 => ("rpn_rois_fpn3",         "RPN proposals for FPN level 3, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    2 => ("rpn_rois_fpn4",         "RPN proposals for FPN level 4, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    3 => ("rpn_rois_fpn5",         "RPN proposals for FPN level 5, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    4 => ("rpn_rois_fpn6",         "RPN proposals for FPN level 6, format (image_index, x1, y1, x2, y2). See rpn_rois documentation from GenerateProposals."),
    5 => ("rpn_roi_probs_fpn2",    "RPN objectness probabilities for FPN level 2. See rpn_roi_probs documentation from GenerateProposals."),
    6 => ("rpn_roi_probs_fpn3",    "RPN objectness probabilities for FPN level 3. See rpn_roi_probs documentation from GenerateProposals."),
    7 => ("rpn_roi_probs_fpn4",    "RPN objectness probabilities for FPN level 4. See rpn_roi_probs documentation from GenerateProposals."),
    8 => ("rpn_roi_probs_fpn5",    "RPN objectness probabilities for FPN level 5. See rpn_roi_probs documentation from GenerateProposals."),
    9 => ("rpn_roi_probs_fpn6",    "RPN objectness probabilities for FPN level 6. See rpn_roi_probs documentation from GenerateProposals.")
}

outputs!{CollectRpnProposals, 
    0 => ("rois",                  "Top proposals limited to rpn_post_nms_topN total, format (image_index, x1, y1, x2, y2)")
}

args!{CollectRpnProposals, 
    0 => ("rpn_max_level",         "(int) RPN_MAX_LEVEL"),
    1 => ("rpn_min_level",         "(int) RPN_MIN_LEVEL"),
    2 => ("rpn_post_nms_topN",     "(int) RPN_POST_NMS_TOP_N")
}

should_not_do_gradient!{CollectRpnProposals}

impl<Context> CollectRpnProposalsOp<Context> {
    
    pub fn new<Args>(args: Args) -> Self {
        todo!();
        /*
            : Operator<Context>(std::forward<Args>(args)...),
            rpn_max_level_(
                this->template GetSingleArgument<int>("rpn_max_level", 6)),
            rpn_min_level_(
                this->template GetSingleArgument<int>("rpn_min_level", 2)),
            rpn_post_nms_topN_(
                this->template GetSingleArgument<int>("rpn_post_nms_topN", 2000)) 

        CAFFE_ENFORCE_GE(
            rpn_max_level_,
            rpn_min_level_,
            "rpn_max_level " + c10::to_string(rpn_max_level_) +
                " must be greater than or equal to rpn_min_level " +
                c10::to_string(rpn_min_level_) + ".");
        */
    }
}

impl<Context> CollectRpnProposalsOp<Context> {

    #[inline] pub fn run_on_cpu_device(&mut self) -> bool {
        
        todo!();
        /*
            int num_rpn_lvls = rpn_max_level_ - rpn_min_level_ + 1;
      CAFFE_ENFORCE_EQ(InputSize(), 2 * num_rpn_lvls);

      // Collect rois and scores in Eigen
      // rois are in [[batch_idx, x0, y0, x1, y2], ...] format
      // Combine predictions across all levels and retain the top scoring
      //
      // equivalent to python code
      //   roi_inputs = inputs[:num_rpn_lvls]
      //   score_inputs = inputs[num_rpn_lvls:]
      //   rois = np.concatenate([blob.data for blob in roi_inputs])
      //   scores = np.concatenate([blob.data for blob in score_inputs]).squeeze()
      int proposal_num = 0;
      for (int i = 0; i < num_rpn_lvls; i++) {
        const auto& roi_in = Input(i);
        proposal_num += roi_in.size(0);
      }
      ERArrXXf rois(proposal_num, 5);
      EArrXf scores(proposal_num);
      int len = 0;
      for (int i = 0; i < num_rpn_lvls; i++) {
        const auto& roi_in = Input(i);
        const int n = roi_in.size(0);

        Eigen::Map<const ERArrXXf> roi(roi_in.data<float>(), n, 5);
        rois.block(len, 0, n, 5) = roi;

        const auto& score_in = Input(num_rpn_lvls + i);
        CAFFE_ENFORCE_EQ(score_in.size(0), n);

        // No need to squeeze, since we are reshaping when converting to Eigen
        // https://docs.scipy.org/doc/numpy/reference/generated/numpy.squeeze.html
        Eigen::Map<const EArrXf> score(score_in.data<float>(), n);
        scores.segment(len, n) = score;

        len += n;
      }

      // Grab only top rpn_post_nms_topN rois
      // equivalent to python code
      //   inds = np.argsort(-scores)[:rpn_post_nms_topN]
      //   rois = rois[inds, :]
      utils::SortAndLimitRoIsByScores(scores, rpn_post_nms_topN_, rois);

      // equivalent to python code
      //   outputs[0].reshape(rois.shape)
      //   outputs[0].data[...] = rois

      auto* rois_out = Output(0, {rois.rows(), rois.cols()}, at::dtype<float>());
      Eigen::Map<ERArrXXf> rois_out_mat(
          rois_out->template mutable_data<float>(), rois.rows(), rois.cols());
      rois_out_mat = rois;

      return true;
        */
    }
}

pub struct DistributeFpnProposalsOp<Context> {

    //USE_OPERATOR_CONTEXT_FUNCTIONS;
    storage: OperatorStorage,
    context: Context,

    roi_canonical_scale: i32, // {224};
    roi_canonical_level: i32, // {4};
    roi_max_level:       i32, // {5};
    roi_min_level:       i32, // {2};

    /**
      | The infamous "+ 1" for box width and height
      | dating back to the DPM days
      |
      */
    legacy_plus_one: bool, // {true};
}

num_inputs!{DistributeFpnProposals, 1}

num_outputs!{DistributeFpnProposals, (2,INT_MAX)}

should_not_do_gradient!{DistributeFpnProposals}

inputs!{DistributeFpnProposals, 
    0 => ("rois", "Top proposals limited to rpn_post_nms_topN total, format (image_index, x1, y1, x2, y2)")
}

outputs!{DistributeFpnProposals, 
    0 => ("rois_fpn2", "RPN proposals for ROI level 2, format (image_index, x1, y1, x2, y2)"),
    1 => ("rois_fpn3", "RPN proposals for ROI level 3, format (image_index, x1, y1, x2, y2)"),
    2 => ("rois_fpn4", "RPN proposals for ROI level 4, format (image_index, x1, y1, x2, y2)"),
    3 => ("rois_fpn5", "RPN proposals for ROI level 5, format (image_index, x1, y1, x2, y2)"),
    4 => ("rois_idx_restore", "Permutation on the concatenation of all rois_fpni, i=min...max, such that when applied the RPN RoIs are restored to their original order in the input blobs.")
}

args!{DistributeFpnProposals, 
    0 => ("roi_canonical_scale", "(int) ROI_CANONICAL_SCALE"),
    1 => ("roi_canonical_level", "(int) ROI_CANONICAL_LEVEL"),
    2 => ("roi_max_level", "(int) ROI_MAX_LEVEL"),
    3 => ("roi_min_level", "(int) ROI_MIN_LEVEL")
}

impl<Context> DistributeFpnProposalsOp<Context> {
    
    pub fn new<Args>(args: Args) -> Self {
        todo!();
        /*
            : Operator<Context>(std::forward<Args>(args)...),
            roi_canonical_scale_(
                this->template GetSingleArgument<int>("roi_canonical_scale", 224)),
            roi_canonical_level_(
                this->template GetSingleArgument<int>("roi_canonical_level", 4)),
            roi_max_level_(
                this->template GetSingleArgument<int>("roi_max_level", 5)),
            roi_min_level_(
                this->template GetSingleArgument<int>("roi_min_level", 2)),
            legacy_plus_one_(
                this->template GetSingleArgument<bool>("legacy_plus_one", true)) 

        CAFFE_ENFORCE_GE(
            roi_max_level_,
            roi_min_level_,
            "roi_max_level " + c10::to_string(roi_max_level_) +
                " must be greater than or equal to roi_min_level " +
                c10::to_string(roi_min_level_) + ".");
        */
    }
}

impl DistributeFpnProposalsOp<CPUContext> {

    #[inline] pub fn run_on_device(&mut self) -> bool {
        
        todo!();
        /*
            int num_roi_lvls = roi_max_level_ - roi_min_level_ + 1;
      CAFFE_ENFORCE_EQ(OutputSize(), num_roi_lvls + 1);

      // Load Input(0) to rois
      const auto& rois_in = Input(0);
      const int num_rois = rois_in.size(0);
      const int dim_rois = rois_in.size(1);
      CAFFE_ENFORCE(dim_rois == 4 || dim_rois == 5);
      Eigen::Map<const ERArrXXf> rois_4or5(
          rois_in.data<float>(), num_rois, dim_rois);
      ERArrXXf rois = ERArrXXf::Zero(num_rois, 5);
      rois.rightCols(dim_rois) = rois_4or5;

      // Distribute
      // equivalent to python code
      //   lvl_min = cfg.FPN.ROI_MIN_LEVEL
      //   lvl_max = cfg.FPN.ROI_MAX_LEVEL
      //   lvls = fpn.map_rois_to_fpn_levels(rois[:, 1:5], lvl_min, lvl_max)
      const int lvl_min = roi_min_level_;
      const int lvl_max = roi_max_level_;
      const int canon_scale = roi_canonical_scale_;
      const int canon_level = roi_canonical_level_;
      auto rois_block = rois.block(0, 1, rois.rows(), 4);
      auto lvls = utils::MapRoIsToFpnLevels(
          rois_block, lvl_min, lvl_max, canon_scale, canon_level, legacy_plus_one_);

      // Create new roi blobs for each FPN level
      // (See: modeling.FPN.add_multilevel_roi_blobs which is similar but annoying
      // to generalize to support this particular case.)
      //
      // equivalent to python code
      //   rois_idx_order = np.empty((0, ))
      //   for (output_idx, lvl in enumerate(range(lvl_min, lvl_max + 1)))
      //       idx_lvl = np.where(lvls == lvl)[0]
      //       blob_roi_level = rois[idx_lvl, :]
      //       outputs[output_idx + 1].reshape(blob_roi_level.shape)
      //       outputs[output_idx + 1].data[...] = blob_roi_level
      //       rois_idx_order = np.concatenate((rois_idx_order, idx_lvl))
      //   rois_idx_restore = np.argsort(rois_idx_order)
      //   blob_utils.py_op_copy_blob(rois_idx_restore.astype(np.int32),
      //   outputs[-1])
      EArrXi rois_idx_restore;
      for (int i = 0, lvl = lvl_min; i < num_roi_lvls; i++, lvl++) {
        ERArrXXf blob_roi_level;
        EArrXi idx_lvl;
        utils::RowsWhereRoILevelEquals(rois, lvls, lvl, &blob_roi_level, &idx_lvl);

        // Output blob_roi_level

        auto* roi_out = Output(
            i + 0,
            {blob_roi_level.rows(), blob_roi_level.cols()},
            at::dtype<float>());
        Eigen::Map<ERArrXXf> roi_out_mat(
            roi_out->template mutable_data<float>(),
            blob_roi_level.rows(),
            blob_roi_level.cols());
        roi_out_mat = blob_roi_level;

        // Append indices from idx_lvl to rois_idx_restore
        rois_idx_restore.conservativeResize(
            rois_idx_restore.size() + idx_lvl.size());
        rois_idx_restore.tail(idx_lvl.size()) = idx_lvl;
      }
      utils::ArgSort(rois_idx_restore);

      auto* rois_idx_restore_out =
          Output(OutputSize() - 1, {rois_idx_restore.size()}, at::dtype<int>());
      Eigen::Map<EArrXi> rois_idx_restore_out_mat(
          rois_idx_restore_out->template mutable_data<int>(),
          rois_idx_restore.size());
      rois_idx_restore_out_mat = rois_idx_restore;

      return true;
        */
    }
}

/// Compute the area of an array of boxes.
#[inline] pub fn boxes_area(boxes: &ERArrXXf, legacy_plus_one: Option<bool>) -> ERArrXXf {

    let legacy_plus_one: bool = legacy_plus_one.unwrap_or(false);
    
    todo!();
    /*
        // equivalent to python code
      //   w = (boxes[:, 2] - boxes[:, 0] + 1)
      //   h = (boxes[:, 3] - boxes[:, 1] + 1)
      //   areas = w * h
      //   assert np.all(areas >= 0), 'Negative areas founds'
      const auto w = boxes.col(2) - boxes.col(0) + int(legacy_plus_one);
      const auto h = boxes.col(3) - boxes.col(1) + int(legacy_plus_one);
      const ERArrXXf areas = w * h;
      CAFFE_ENFORCE((areas >= 0).all(), "Negative areas founds: ", boxes);
      return areas;
    */
}

/**
  | Determine which FPN level each RoI in
  | a set of RoIs should map to based on the 
  | heuristic in the FPN paper.
  |
  */
#[inline] pub fn map_ro_is_to_fpn_levels(
    rois:            &ERArrXXf,
    k_min:           f32,
    k_max:           f32,
    s0:              f32,
    lvl0:            f32,
    legacy_plus_one: Option<bool>) -> ERArrXXf {

    let legacy_plus_one: bool = legacy_plus_one.unwrap_or(false);
    
    todo!();
    /*
        // Compute level ids
      ERArrXXf s = BoxesArea(rois, legacy_plus_one).sqrt();
      // s0 = cfg.FPN.ROI_CANONICAL_SCALE  # default: 224
      // lvl0 = cfg.FPN.ROI_CANONICAL_LEVEL  # default: 4

      // Eqn.(1) in FPN paper
      // equivalent to python code
      //   target_lvls = np.floor(lvl0 + np.log2(s / s0 + 1e-6))
      //   target_lvls = np.clip(target_lvls, k_min, k_max)
      auto target_lvls = (lvl0 + (s / s0 + 1e-6).log() / log(2)).floor();
      auto target_lvls_clipped = target_lvls.min(k_max).max(k_min);
      return target_lvls_clipped;
    */
}

/**
  | Sort RoIs from highest to lowest individual
  | RoI score based on values from scores
  | array and limit to n results
  |
  */
#[inline] pub fn sort_and_limit_ro_is_by_scores(
    scores: &EArrXf,
    n:      i32,
    rois:   &mut ERArrXXf)  {
    
    todo!();
    /*
        CAFFE_ENFORCE(rois.rows() == scores.size(), "RoIs and scores count mismatch");
      // Create index array with 0, 1, ... N
      std::vector<int> idxs(rois.rows());
      std::iota(idxs.begin(), idxs.end(), 0);
      // Reuse a comparator based on scores and store a copy of RoIs that
      // will be truncated and manipulated below
      auto comp = [&scores](int lhs, int rhs) {
        if (scores(lhs) > scores(rhs)) {
          return true;
        }
        if (scores(lhs) < scores(rhs)) {
          return false;
        }
        // To ensure the sort is stable
        return lhs < rhs;
      };
      ERArrXXf rois_copy = rois;
      // Note that people have found nth_element + sort to be much faster
      // than partial_sort so we use it here
      if (n > 0 && n < rois.rows()) {
        std::nth_element(idxs.begin(), idxs.begin() + n, idxs.end(), comp);
        rois.resize(n, rois.cols());
      } else {
        n = rois.rows();
      }
      std::sort(idxs.begin(), idxs.begin() + n, comp);
      // Update RoIs based on new order
      for (int i = 0; i < n; i++) {
        rois.row(i) = rois_copy.row(idxs[i]);
      }
    */
}

/**
  | Updates arr to be indices that would
  | sort the array. Implementation of https://docs.scipy.org/doc/numpy/reference/generated/numpy.argsort.html
  |
  */
#[inline] pub fn arg_sort(arr: &mut EArrXi)  {
    
    todo!();
    /*
        // Create index array with 0, 1, ... N and sort based on array values
      std::vector<int> idxs(arr.size());
      std::iota(std::begin(idxs), std::end(idxs), 0);
      std::sort(idxs.begin(), idxs.end(), [&arr](int lhs, int rhs) {
        return arr(lhs) < arr(rhs);
      });
      // Update array to match new order
      for (int i = 0; i < arr.size(); i++) {
        arr(i) = idxs[i];
      }
    */
}

/**
  | Update out_filtered and out_indices
  | with rows from rois where lvl matches
  | value in lvls passed in.
  |
  */
#[inline] pub fn rows_where_roi_level_equals(
    rois:         &ERArrXXf,
    lvls:         &ERArrXXf,
    lvl:          i32,
    out_filtered: *mut ERArrXXf,
    out_indices:  *mut EArrXi)  {
    
    todo!();
    /*
        CAFFE_ENFORCE(out_filtered != nullptr, "Output filtered required");
      CAFFE_ENFORCE(out_indices != nullptr, "Output indices required");
      CAFFE_ENFORCE(rois.rows() == lvls.rows(), "RoIs and lvls count mismatch");
      // Calculate how many rows we need
      int filtered_size = (lvls == lvl).rowwise().any().count();
      // Fill in the rows and indices
      out_filtered->resize(filtered_size, rois.cols());
      out_indices->resize(filtered_size);
      for (int i = 0, filtered_idx = 0; i < rois.rows(); i++) {
        auto lvl_row = lvls.row(i);
        if ((lvl_row == lvl).any()) {
          out_filtered->row(filtered_idx) = rois.row(i);
          (*out_indices)(filtered_idx) = i;
          filtered_idx++;
        }
      }
    */
}

register_cpu_operator!{CollectAndDistributeFpnRpnProposals, CollectAndDistributeFpnRpnProposalsOp<CPUContext>}
register_cpu_operator!{CollectRpnProposals,                 CollectRpnProposalsOp<CPUContext>}
register_cpu_operator!{DistributeFpnProposals,              DistributeFpnProposalsOp<CPUContext>}

export_caffe2_op_to_c10_cpu!{CollectAndDistributeFpnRpnProposals,
    "_caffe2::CollectAndDistributeFpnRpnProposals(
        Tensor[] input_list, 
        int roi_canonical_scale, 
        int roi_canonical_level, 
        int roi_max_level, 
        int roi_min_level, 
        int rpn_max_level, 
        int rpn_min_level, 
        int rpn_post_nms_topN, 
        bool legacy_plus_one) 
        -> (
            Tensor rois, 
            Tensor rois_fpn2, 
            Tensor rois_fpn3, 
            Tensor rois_fpn4, 
            Tensor rois_fpn5, 
            Tensor rois_idx_restore_int32)",
    CollectAndDistributeFpnRpnProposalsOp<CPUContext>}

export_caffe2_op_to_c10_cpu!{CollectRpnProposals,
    "_caffe2::CollectRpnProposals(
        Tensor[] input_list, 
        int rpn_max_level, 
        int rpn_min_level, 
        int rpn_post_nms_topN) -> (Tensor rois)",
    CollectRpnProposalsOp<CPUContext>}

export_caffe2_op_to_c10_cpu!{DistributeFpnProposals,
    "_caffe2::DistributeFpnProposals(
        Tensor rois, 
        int roi_canonical_scale, 
        int roi_canonical_level, 
        int roi_max_level, 
        int roi_min_level, 
        bool legacy_plus_one) 
        -> (
        Tensor rois_fpn2, 
        Tensor rois_fpn3, 
        Tensor rois_fpn4, 
        Tensor rois_fpn5, 
        Tensor rois_idx_restore_int32)",
        DistributeFpnProposalsOp<CPUContext>}
