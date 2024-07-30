<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let file: FileData;
  let dispatch = createEventDispatcher();
  function file_selected() {
    dispatch("file-selected", file);
  }
</script>

<button on:click={file_selected} class="file-selector">{file.name}</button>

<style>
  button.file-selector {
    background-color: black;
    color: white;
    width: 100%;
    height: 100%;
  }
</style>
