<script setup lang="ts">
import { ref } from "vue";
import { useVueFlow, VueFlow } from "@vue-flow/core";
import { MiniMap } from "@vue-flow/minimap";
import { Background } from "@vue-flow/background";
import { ControlButton, Controls } from "@vue-flow/controls";
import { initialEdges, initialNodes } from "./initial-canvas";
import SpecialNode from "./node.vue";
import SpecialEdge from "./edge.vue";
import { IconTerminal } from "@tabler/icons-vue";

const { onInit, onNodeDragStop, onConnect, addEdges, toObject } = useVueFlow();

const nodes = ref(initialNodes);
const edges = ref(initialEdges);

onInit((vueFlowInstance) => {
  // instance is the same as the return of `useVueFlow`
  vueFlowInstance.fitView();
});

onNodeDragStop(({ event, nodes, node }) => {
  console.log("Node Drag Stop", { event, nodes, node });
});

onConnect((connection) => {
  addEdges(connection);
});

function logToObject() {
  console.log(toObject());
}
</script>

<template>
  <VueFlow :nodes="nodes" :edges="edges">
    <Background pattern-color="#aaa" :gap="16" />

    <MiniMap />

    <Controls position="bottom-left">
      <ControlButton title="Shuffle Node Positions" @click="logToObject">
        <IconTerminal name="update" />
      </ControlButton>
    </Controls>
    <!-- bind your custom node type to a component by using slots, slot names are always `node-<type>` -->
    <template #node-special="specialNodeProps">
      <SpecialNode v-bind="specialNodeProps" />
    </template>

    <!-- bind your custom edge type to a component by using slots, slot names are always `edge-<type>` -->
    <template #edge-special="specialEdgeProps">
      <SpecialEdge v-bind="specialEdgeProps" />
    </template>
  </VueFlow>
</template>
