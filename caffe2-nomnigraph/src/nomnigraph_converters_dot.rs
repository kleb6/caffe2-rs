crate::ix!();

pub struct DotGenerator<G: GraphType> {
    node_printer: NodePrinter<G>,
    edge_printer: EdgePrinter<G>,
}

pub type NodePrinter<G: GraphType> = fn(_u0: <G as GraphType>::NodeRef) -> HashMap<String,String>;
pub type EdgePrinter<G: GraphType> = fn(_u0: <G as GraphType>::EdgeRef) -> HashMap<String,String>;

impl<G: GraphType> DotGenerator<G> {

    #[inline] pub fn default_edge_printer(edge: <G as GraphType>::EdgeRef) -> HashMap<String,String> {
        
        todo!();
        /*
            std::map<std::string, std::string> labelMap;
        return labelMap;
        */
    }
    
    pub fn new(node_printer: NodePrinter<G>, edge_printer: EdgePrinter<G>) -> Self {
        todo!();
        /*
            : nodePrinter_(nodePrinter), edgePrinter_(edgePrinter)
        */
    }

    /// Convert a graph (with optional subgraphs cluster) to dot.
    #[inline] pub fn convert_with_optional_subgraphs_cluster(
        &self, 
        sg: &<G as GraphType>::SubgraphType, 
        subgraphs: &Vec<*mut <G as GraphType>::SubgraphType>) -> String 
    {
        todo!();
        /*
            std::ostringstream output;
        output << "digraph G {\nrankdir=LR\n";
        for (const auto& node : sg.getNodes()) {
          generateNode(node, sg, output);
        }
        for (size_t i = 0; i < subgraphs.size(); ++i) {
          const auto& subgraph = subgraphs[i];
          output << "subgraph cluster" << i << " {\n";
          output << "style=dotted;\n";
          for (const auto& node : subgraph->getNodes()) {
            output << node;
            output << ";\n";
          }
          output << "}\n";
        }
        output << "}";
        return output.str();
        */
    }

    /// Convert a subgraph to dot.
    #[inline] pub fn convert_subgraph_to_dot(
        &self, 
        sg: &<G as GraphType>::SubgraphType) -> String {
        
        todo!();
        /*
            std::ostringstream output;
        output << "digraph G {\nrankdir=LR\n";
        for (const auto& node : sg.getNodes()) {
          generateNode(node, sg, output);
        }
        output << "}";
        return output.str();
        */
    }
    
    /**
      | NOTE No subgraph support
      | 
      | Converts given graph into DOT string
      | w/operator input-order preserved
      | 
      | Assumes graph is acyclic, nodes are
      | unique_ptr
      | 
      | (1) Get & print input nodes (nodes w/o
      | parents)
      | 
      | - Node: <p0>[shape=record, label="{{Data
      | In}|{<p0>*}}"]
      | 
      | (2) Find operators w/BFS from input
      | nodes
      | 
      | (3) Print operator records & incoming
      | edges
      | 
      | - Node: op_ptr[shape=record, label="{{<i0>*|<i1>*|...}|{op}|{<o0>*}"]
      | 
      | - Edge: <parent_node_ptr>:<ref>:s
      | -> <this_node_ptr>:<ref>:n
      |
      */
    #[inline] pub fn convert_struct(&self, sg: &<G as GraphType>::SubgraphType) -> String {
        
        todo!();
        /*
            std::ostringstream output;
        output << "digraph G {\nrankdir=LR\n";

        // Get input nodes (nodes w/o parents)
        std::unordered_map<typename GraphT::NodeRef, int>
            nodeDepthMap; // Touched nodes for BFS
        std::queue<typename GraphT::NodeRef> workList; // Init w/parentless nodes
        for (const auto& node : sg.getNodes()) {
          if (node->getInEdges().size() == 0 && node->getOutEdges().size() > 0) {
            // Add input node to dot string
            output << (uint64_t)node << "[shape=record, label=\"{{Data In}|{<"
                   << (uint64_t)node << ">";
            for (const auto& attr : nodePrinter_(node)) {
              output << attr.second;
            }
            output << "}}\"]\n";

            // Track input node
            nodeDepthMap[node] = 0;
            workList.push(node);
          }
        }

        // BFS to get operator nodes
        std::vector<typename GraphT::NodeRef> ops;
        while (workList.size() > 0) {
          const auto& node = workList.front();
          for (const auto& edge : node->getOutEdges()) {
            // Enqueue child iff not touched yet
            const auto& child = edge->head();
            if (!nodeDepthMap.count(child)) {
              nodeDepthMap[child] = nodeDepthMap[node] + 1;
              workList.push(child);
              if (nodeDepthMap[child] % 2 == 1) { // "odd" ==> operator
                ops.emplace_back(child);
              }
            } else {
            }
          }
          workList.pop();
        }

        // Finalize output
        output << getOperatorSubtreeDotString(ops) << "}\n";
        return output.str();
        */
    }
    
    /**
      | Get DOT string record of given operator
      | and DOT string of its input edges
      | 
      | -----------
      | @param op
      | 
      | operator to parse
      | ----------
      | @param nodePrinter
      | 
      | node attribute extractor
      | 
      | 
      | -----------
      | @return
      | 
      | '\n' sep string of operator & input edges
      |
      */
    #[inline] fn get_operator_dot_string(&self, op: <G as GraphType>::NodeRef) -> String {
        
        todo!();
        /*
            std::ostringstream output;
        std::ostringstream record; // Operator node record
        record << (uint64_t)op << "[shape=record, label=\"{{";

        // Input refs
        std::string sep = "";
        for (const auto& opInEdge : op->getInEdges()) {
          // Draw edge between prev. op output to cur. op input
          const auto& input = opInEdge->tail();
          int inputInEdgeCt = input->getInEdges().size();
          if (inputInEdgeCt == 0) { // Node @ top of subgraph
            output << (uint64_t)input;
          } else { // Node between operators
            assert(inputInEdgeCt == 1);
            output << (uint64_t)input->getInEdges().at(0)->tail();
          }
          output << ":" << (uint64_t)input << ":s -> " << (uint64_t)op << ":"
                 << (uint64_t)input << ":n\n";

          // Add input to operator record
          record << sep << "<" << (uint64_t)input << ">";
          for (const auto& attr : nodePrinter_(input)) {
            record << attr.second;
          }
          sep = "|";
        }

        // Extract operator name
        record << "}|{";
        for (const auto& attr : nodePrinter_(op)) {
          record << attr.second;
        }
        record << "}|{";

        // Output refs
        sep = "";
        for (const auto& edge : op->getOutEdges()) {
          const auto& child = edge->head();
          record << sep << "<" << (uint64_t)child << ">";
          for (const auto& attr : nodePrinter_(child)) {
            record << attr.second;
          }
          sep = "|";
        }

        // Append record to output string
        output << record.str() << "}}\"]\n";
        return output.str();
        */
    }

    /**
      | Prints DOT string of given operator
      | subgraph
      | 
      | -----------
      | @param ops
      | 
      | operators in a given subgraph
      | ----------
      | @param nodePrinter
      | 
      | node attribute extractor
      | 
      | 
      | -----------
      | @return
      | 
      | DOT string that renders operators subgraph
      |
      */
    #[inline] fn get_operator_subtree_dot_string(&self, ops: Vec<<G as GraphType>::NodeRef>) -> String {
        
        todo!();
        /*
            std::ostringstream output;
        for (const auto& op : ops) {
          output << getOperatorDotString(op);
        }
        return output.str();
        */
    }
    
    /// Generate dot string for a node.
    #[inline] pub fn generate_node(&self, 
        node:   <G as GraphType>::NodeRef,
        sg:     &<G as GraphType>::SubgraphType,
        output: &mut String)  {

        todo!();
        /*
            output << (uint64_t)node; // dot doesn't like hex
        output << "[";
        for (const auto& attrib : nodePrinter_(node)) {
          output << attrib.first << "=\"" << attrib.second << "\",";
        }
        output << "];\n";
        for (const auto& edge : node->getOutEdges()) {
          if (!sg.hasEdge(edge)) {
            continue;
          }
          output << (uint64_t)edge->tail() << " -> " << (uint64_t)edge->head();
          output << "[";
          for (const auto& attrib : edgePrinter_(edge)) {
            output << attrib.first << "=\"" << attrib.second << "\",";
          }
          output << "];\n";
        }
        */
    }
}

/// Convert a graph to dot string.
#[inline] pub fn convert_graph_to_dot_string<G: GraphType>(
    g:            *mut G,
    node_printer: NodePrinter<G>,
    edge_printer: Option<EdgePrinter<G>>) -> String {


    todo!();
    /*
    let edge_printer: EdgePrinter<G> =
             edge_printer.unwrap_or(DotGenerator<G>::default_edge_printer);

        auto d = DotGenerator<GraphT>(nodePrinter, edgePrinter);
      return d.convert(algorithm::createSubgraph(g), {});
    */
}

/**
  | Convert a graph to dot string and annotate
  | subgraph clusters.
  |
  */
#[inline] pub fn convert_graph_to_dot_string_and_annotate_subgraph_clusters<G: GraphType>(
    g:            *mut G,
    subgraphs:    &Vec<*mut <G as GraphType>::SubgraphType>,
    node_printer: NodePrinter<G>,
    edge_printer: EdgePrinter<G>) -> String {

    todo!();
    /*

    let edge_printer: EdgePrinter<G> =
             edge_printer.unwrap_or(DotGenerator<G>::default_edge_printer);

        auto d = DotGenerator<GraphT>(nodePrinter, edgePrinter);
      return d.convert(algorithm::createSubgraph(g), subgraphs);
    */
}

/// Convert a subgraph to dot string.
#[inline] pub fn convert_subgraph_to_dot_string<G: GraphType>(
    sg:           &<G as GraphType>::SubgraphType,
    node_printer: NodePrinter<G>,
    edge_printer: EdgePrinter<G>) -> String {


    todo!();
    /*
        let edge_printer: DotGenerator<G>::EdgePrinter =
                 edge_printer.unwrap_or(DotGenerator::<G>::default_edge_printer);

        auto d = DotGenerator<GraphT>(nodePrinter, edgePrinter);
      return d.convert(sg);
    */
}


#[inline] pub fn convert_to_dot_record_string<G: GraphType>(
    g:            *mut G,
    node_printer: NodePrinter<G>,
    edge_printer: EdgePrinter<G>) -> String {

    todo!();
    /*
        let edge_printer: DotGenerator<G>::EdgePrinter =
                 edge_printer.unwrap_or(DotGenerator::<G>::default_edge_printer);

        auto d = DotGenerator<GraphT>(nodePrinter, edgePrinter);
      return d.convertStruct(algorithm::createSubgraph(g));
    */
}
