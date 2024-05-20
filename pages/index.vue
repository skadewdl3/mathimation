<script setup lang="ts">
const init = await import("../wasm/pkg/wasm.js");

onMounted(() => {
  init.init();
});

</script>

<template>
  <button @click="init.run">Run</button>
  <button @click="init.pause">Stop</button>
  <div id="wasm-example"></div>
</template>

<style></style>
