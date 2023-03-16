<script lang="ts">
  import Chart from "./lib/Chart.svelte"
  import  type {FlowData} from "./type"
  import {onMount} from "svelte"
  import { invoke } from "@tauri-apps/api/tauri";

  const label = ["Date","Flowed(mL)"] 
    
  let state = true //fall or rise
  let data_fall: (String[] | (string | number)[])[] = [label];
  let data_rise: (String[] | (string | number)[])[] = [];
  let title: String = "Flow Sensor";
  let colors: String[] = ["#fff"];

  async function get_data() {
    let x = await invoke("flow") as string
    let flow = JSON.parse(x) as FlowData
    let rise = flow.rise.map(e=>{
      let date = new Date(e.created_at * 1000).toString()
      let value = `${e.value} mL`
      return [date,value]
    })
    let fall = flow.fall.map(e=>{
      let date = new Date(e.created_at * 1000).toString()
      let value = `${e.value} mL`
      return [date,value]
    })
    data_fall = [label,...fall]
    data_rise = [label,...rise]
  }

  onMount(async ()=>{
    let interval = setInterval(async () => {
      await get_data()
    }, 5000);
    return () => clearInterval(interval)
  })
</script>
