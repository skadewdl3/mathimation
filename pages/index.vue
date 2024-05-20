<script setup lang="ts">
const init = await import("../wasm/pkg/wasm.js");

onMounted(() => {
  init.run();
});

</script>

<template>
  <button @click="">Run</button>
  <button @click="">Stop</button>
  <div id="wasm-example"></div>
</template>

<style></style>
