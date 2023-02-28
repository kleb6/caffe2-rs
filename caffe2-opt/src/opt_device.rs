crate::ix!();

use crate::{
    NNGraph_NodeRef,
    NNGraph_SubgraphType,
    NNGraph_EdgeRef,
    NNGraph,
    NNModule
};

#[inline] pub fn get_input_edges(
    sg: &NNGraph_SubgraphType, 
    g:  &NNGraph) -> Vec<NNGraph_EdgeRef> 
{
    todo!();
    /*
        std::vector<NNGraph_EdgeRef> inputTensorEdges;
      for (const auto& node : sg.getNodes()) {
        NOM_REQUIRE_OR_CONT(nn::is<NeuralNetOperator>(node));
        NOM_REQUIRE_OR_CONT(nn::hasInputs(node));

        // Check if tensor's parents are in the sg
        for (const auto& input : nn::getInputs(node)) {
          NOM_REQUIRE_OR_CONT(
              !nn::hasProducer(input) || !sg.hasNode(nn::getProducer(input)));
          inputTensorEdges.emplace_back(g.getEdge(input, node));
        }
      }
      return inputTensorEdges;
    */
}

#[inline] pub fn get_output_edges(
    sg: &NNGraph_SubgraphType, 
    g: &NNGraph) -> Vec<NNGraph_EdgeRef> 
{
    todo!();
    /*
        std::vector<NNGraph_EdgeRef> outputTensorEdges;
      for (const auto& node : sg.getNodes()) {
        NOM_REQUIRE_OR_CONT(nn::is<NeuralNetOperator>(node));

        for (const auto& output : nn::getOutputs(node)) {
          auto consumers = nn::getConsumers(output);
          for (const auto& consumer : consumers) {
            NOM_REQUIRE_OR_CONT(!sg.hasNode(consumer));
            outputTensorEdges.emplace_back(g.getEdge(node, output));
          }
          NOM_REQUIRE_OR_CONT(consumers.size() == 0);
          outputTensorEdges.emplace_back(g.getEdge(node, output));
        }
      }
      return outputTensorEdges;
    */
}

#[inline] pub fn insert_copies<T,U>(
    nn:           *mut NNModule<T,U>,
    supported:    fn(_u0: NNGraph_NodeRef) -> bool,
    copy_to_fn:   fn(_u0: &mut NNGraph) -> NNGraph_NodeRef,
    copy_from_fn: fn(_u0: &mut NNGraph) -> NNGraph_NodeRef)  {
    
    todo!();
    /*
        auto matches = nom::algorithm::binaryMatch(&nn->dataFlow, supported);

      // We're doing a lot of inplace mutation so this is necessary.
      std::set<NNGraph_EdgeRef> changedEdges;

      for (const auto& match : matches) {
        for (const auto& edge : getInputEdges(match, nn->dataFlow)) {
          NOM_REQUIRE_OR_CONT(changedEdges.count(edge) == 0);
          auto input = edge->tail();
          NNGraph_NodeRef newInput = nullptr;

          // First we check if there already is a copyNode that we can reuse.
          auto copyNode = copyToFn(nn->dataFlow);
          auto copyOp = nn::get<NeuralNetOperator>(copyNode);

          // Rectify redudancies.
          for (const auto& consumer : nn::getConsumers(input)) {
            auto consumerOp = nn::get<NeuralNetOperator>(consumer);
            // We already have a copy node, let's reuse it.
            if (consumerOp->getKind() == copyOp->getKind()) {
              nn->dataFlow.deleteNode(copyNode);
              copyNode = consumer;
              newInput = nn::getOutputs(copyNode).front();
              break;
            }
          }

          // Second, we may have found the out-edge of a previous match.
          auto copyFromNode = copyFromFn(nn->dataFlow);
          auto copyFromOp = nn::get<NeuralNetOperator>(copyFromNode);
          do {
            NOM_REQUIRE_OR_CONT(nn::hasProducer(input));
            const auto& producer = nn::getProducer(input);
            const auto& producerOp = nn::get<NeuralNetOperator>(producer);
            NOM_REQUIRE_OR_CONT(producerOp->getKind() == copyFromOp->getKind());
            NOM_REQUIRE_OR_CONT(nn::hasInputs(producer));
            auto oldInputs = nn::getInputs(producer);
            NOM_REQUIRE_OR_CONT(oldInputs.size() == 1);
            nn->dataFlow.deleteNode(copyNode);
            newInput = oldInputs.front();
          } while (false);
          nn->dataFlow.deleteNode(copyFromNode);

          // Third, we may have to insert a copy operation
          // if the above checks failed.
          if (!newInput) {
            auto data = nn::get<NeuralNetData>(input);
            newInput = nn->dataFlow.createNode(
                std::make_unique<repr::Tensor>(data->getName() + "_opencl_0"));
            nn->dataFlow.createEdge(input, copyNode);
            nn->dataFlow.createEdge(copyNode, newInput);
          }
          // Finally, swap our input node to reflect a tensor already
          // on the device.
          input->removeOutEdge(edge);
          edge->setTail(newInput);
          newInput->addOutEdge(edge);

          changedEdges.insert(edge);
        }

        for (const auto& edge : getOutputEdges(match, nn->dataFlow)) {
          NOM_REQUIRE_OR_CONT(changedEdges.count(edge) == 0);
          auto output = edge->head();

          auto copyNode = copyFromFn(nn->dataFlow);
          auto data = nn::get<NeuralNetData>(output);

          auto newOutput = nn->dataFlow.createNode(
              std::make_unique<repr::Tensor>(data->getName() + "_opencl_0"));

          output->removeInEdge(edge);
          edge->setHead(newOutput);

          changedEdges.insert(edge);

          nn->dataFlow.createEdge(newOutput, copyNode);
          nn->dataFlow.createEdge(copyNode, output);

          // We may have broken some consumers that are actually in the match.
          for (auto consumer : nn::getConsumers(output)) {
            if (match.getNodes().count(consumer)) {
              auto brokenEdge = nn->dataFlow.getEdge(output, consumer);
              output->removeOutEdge(brokenEdge);
              brokenEdge->setTail(newOutput);
              newOutput->addOutEdge(brokenEdge);
            }
          }
        }
      }
    */
}
