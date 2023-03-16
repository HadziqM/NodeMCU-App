<script lang="ts">
  import Title from "./lib/Title.svelte"
  import Notif from "./lib/Notif.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { notifications } from "./lib/notification";
  let state = true
  let msg = "Press This button bellow to connect to database <br/> (Make sure you have internet access)"
  async function connect(){
    let message = await invoke("database") as string
    if (message=="success"){
      notifications.success(message,1000)
      state = false
    }else{
      notifications.danger(message,2000)
    }
  }
</script>
  <Title/>
  <Notif/>
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
