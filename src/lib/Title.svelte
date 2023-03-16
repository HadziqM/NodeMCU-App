<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";

  let close = () => {
    appWindow.close()
  }
  let minimize = () => {
    appWindow.minimize()
  }
  let maximize =() => {
      appWindow.toggleMaximize()
  }
</script>

<div data-tauri-drag-region class="titlebar">
  <div class="titlebar-button" on:click={()=>minimize()}>
    <img src="/minimize.svg" alt="idk">
  </div>
  <div class="titlebar-button" on:click={()=>maximize()}>
    <img src="/maximize.svg" alt="idk">
  </div>
  <div class="titlebar-button" on:click={()=>close()}>
    <img src="/close.svg" alt="idk">
  </div>
</div>
