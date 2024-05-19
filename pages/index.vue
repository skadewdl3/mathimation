<script setup lang="ts">
const init = await import("../wasm/pkg/wasm.js");

onMounted(() => {
  init.run();
});

const stop = ref(false);
const loop = () => {
  console.log("this ran")
  if (!stop.value) requestAnimationFrame(loop);
};

const run = () => {
  stop.value = false;
  requestAnimationFrame(loop);
};

const pause = () => {
  stop.value = true;
};
</script>

<template>
  <button @click="run">Run</button>
  <button @click="pause">Stop</button>
  <div id="wasm-example"></div>
</template>

<style></style>
