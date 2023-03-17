<script lang="ts">
  import Chart from "./lib/Chart.svelte"
  import  type {FlowData} from "./type"
  import {onMount} from "svelte"
  import { invoke } from "@tauri-apps/api/tauri";

  const label = ["date","mL"] 
    
  let state = true //fall or rise
  let data_fall: (String[] | (string | number)[])[] = [label,["place",0],["lace2",2]];
  let data_rise: (String[] | (string | number)[])[] = [];
  let title: String = "Flow Sensor";
  let colors: String[] = ["#fff","#aaf","#faa"];
  let rate = 0
  let flow1 = false

  async function get_data() {
    let x = await invoke("data") as string
    let flow = JSON.parse(x) as FlowData
    rate = flow.rate.rate
    flow1 = flow.status.fall
    let rise = flow.rise.vec.map(e=>{
      let date = new Date( e.date* 1000).toString()
      let value = e.data
      return [date,value]
    })
    let fall = flow.fall.vec.map(e=>{
      let date = new Date(e.date* 1000).toString()
      let value = e.data
      return [date,value]
    })
    data_fall = [label,...fall]
    data_rise = [label,...rise]
  }

  onMount(()=>{
    let interval = setInterval(async () => {
      await get_data()
    }, 6000);
    return () => clearInterval(interval)
  })
</script>
  <Chart 
    title = {title}
    data = {data_fall}
    colors = {colors}
/>
  <div>
  <p>{`Flow Rate = ${rate} mL/minute`}</p>
  <p>{`Flow sensor is ${flow1 ? "active" : "inactive"}`}</p>
</div>

<style>
  p{
    color: #fff;
  }
</style>
