<script lang="ts">
  import { get_current_directory_data } from "$lib/commands";
  import AddressBar from "$lib/components/AddressBar.svelte";
  import ButtonBar from "$lib/components/ButtonBar.svelte";
  import DirectoryDisplay from "$lib/components/DirectoryDisplay.svelte";
  import DirectoryTreeArea from "$lib/components/DirectoryTreeArea.svelte";
  export let data: { path: string };

  let current_directory_data = get_current_directory_data(data.path);
</script>

{#await current_directory_data}
  <!-- promise is pending -->
{:then directory_data}
  <div id="outer-area" class="vertical-flex centered">
    <div id="navbar" class="horizontal-flex centered">
      <div id="button-bar-div" class=" debug-border">
        <ButtonBar></ButtonBar>
      </div>
      <div id="address-bar-div" class="debug-border">
        <AddressBar></AddressBar>
      </div>
    </div>
    <div id="lower-area" class="horizontal-flex centered">
      <div id="directory-tree-div" class="debug-border">
        <DirectoryTreeArea></DirectoryTreeArea>
      </div>
      <div id="directory-display-div" class=" debug-border">
        <DirectoryDisplay
          current_location={directory_data.current_location}
          current_files={directory_data.current_files}
        ></DirectoryDisplay>
      </div>
    </div>
  </div>
{/await}

<style>
  #outer-area {
    height: 98vh;
    width: 99vw;
    border: 3px solid green;
  }
  .horizontal-flex {
    display: flex;
    flex-direction: row;
  }
  .vertical-flex {
    display: flex;
    flex-direction: column;
  }
  .centered {
    align-items: center;
    justify-content: center;
  }
  #navbar {
    width: 100%;
    border: 3px solid blue;
    height: 6%;
  }
  #lower-area {
    width: 100%;
    height: 94%;
    border: 3px solid black;
  }

  .debug-border {
    border: 2px dotted black;
  }

  #button-bar-div {
    height: 100%;
    width: 30%;
  }
  #address-bar-div {
    height: 100%;
    width: 70%;
  }
  #directory-tree-div {
    height: 100%;
    width: 30%;
  }
  #directory-display-div {
    height: 100%;
    width: 70%;
  }
</style>
