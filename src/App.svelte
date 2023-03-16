<script lang="ts">
  import Title from "./lib/Title.svelte"
  import { invoke } from "@tauri-apps/api/tauri";
  let state = true
  let msg = "Press This button bellow to connect to database <br/> (Make sure you have internet access)"
  async function connect(){
    msg = await invoke("database") as string
    if (msg=="success"){
      state = false
    }
  }
</script>
  <Title/>
{#if state}
  <div class="main-box">
    <p>{@html msg}</p>
    <button on:click={async () => await connect()}>Connect</button>
  </div>
  {:else}
  <button>Disconnect</button>
{/if}
<style>
  .main-box{
  color: #fff;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 50vw;
  flex-direction: column;
  }
  p{
  text-align: center;
  }
</style>
