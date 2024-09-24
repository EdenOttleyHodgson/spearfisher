<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let file: FileData;
  export let isSelected: boolean;
  let dispatch = createEventDispatcher();
  function file_selected() {
    console.log("File selector: File selected");
    dispatch("file-selected", file);
  }
</script>

{#if isSelected}
  <button on:click={file_selected} class="file-selector selected"
    >{file.name}</button
  >
{:else}
  <button on:click={file_selected} class="file-selector unselected"
    >{file.name}</button
  >
{/if}

<style>
  .file-selector {
    min-width: 100%;
    min-height: 100%;
    width: 100%;
    height: 100%;
    max-width: 100%;
    max-height: 100%;
  }
  .unselected {
    color: black;

    background-color: white;
  }
  .selected {
    color: white;

    background-color: blue;
  }
</style>
