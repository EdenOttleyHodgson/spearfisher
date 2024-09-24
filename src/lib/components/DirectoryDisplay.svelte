<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import FileSelector from "./FileSelector.svelte";

  export let current_location: FileData;
  export let current_files: FileData[];
  let selectedFile: FileData | null = null;
  let dispatch = createEventDispatcher();
  function on_file_selected(new_selected: CustomEvent<FileData>) {
    console.log("File selected!");
    selectedFile = new_selected.detail;
    dispatch("file-selected", selectedFile);
  }
  function deselect_file(_: CustomEvent<FileData>) {
    selectedFile = null;
    dispatch("deselect-file");
  }
</script>

{#each current_files as file}
  <div id="filebox" class="debug_border">
    {#if file == selectedFile}
      <FileSelector {file} isSelected={true} on:file-selected={deselect_file} />
    {:else}
      <FileSelector
        {file}
        isSelected={false}
        on:file-selected={on_file_selected}
      />
    {/if}
  </div>
{/each}

<style>
  #filebox {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }
  .debug_border {
    border: 1px solid black;
  }
</style>
