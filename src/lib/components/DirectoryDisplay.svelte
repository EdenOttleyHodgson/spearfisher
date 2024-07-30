<script lang="ts">
  import FileSelector from "./FileSelector.svelte";

  export let current_location: FileData;
  export let current_files: FileData[];
</script>

{#each current_files as file}
  <div id="filebox">
    <FileSelector {file} />
  </div>
{/each}

<style>
  #filebox {
    display: flex;
    flex-direction: column;

    align-items: center;
    justify-content: center;
  }
</style>
