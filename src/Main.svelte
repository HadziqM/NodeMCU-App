<script lang="ts">
  import Chart from "./lib/Chart.svelte"
  import Card from "./lib/Card.svelte";
  import Gyro from "./lib/Gyro.svelte";
  import Radial from "./lib/Radial.svelte";
  import Location from "./lib/Location.svelte";
  import { flowState } from "./global";
  import  type {FlowData} from "./type"
  import {onMount} from "svelte"
  import { invoke } from "@tauri-apps/api/tauri";
  import Status from "./lib/Status.svelte";

  const label = ["date","mL"] 
    
  let state:boolean //fall or rise
  flowState.subscribe(e=>{
    state=e
  })
  let data_fall: (String[] | (string | number)[])[] = [label,["place",0],["lace2",2]];
  let data_rise: (String[] | (string | number)[])[] = [label,["place",0],["lace2",2]];
  let title: String = "Flow Sensor";
  let colors: String[] = ["#fff"];
  let colors2: String[] = ["#aaf"];
  let rate = 0
  let total = 0
  let flow1 = false
  let flow2 = false

  async function get_data() {
    let x = await invoke("data") as string
    let flow = JSON.parse(x) as FlowData
    rate = flow.rate.rate
    total = flow.rate.total
    flow1 = flow.status.fall
    flow2 = flow.status.rise
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

<div class="grid grid-cols-5 gap-2">
  <Card 
    title = "flowrate"
    value = {rate}
    unit = "mL/min"
  />
  <Chart 
      title = {title}
      data = {state?data_fall:data_rise}
      colors = {state?colors:colors2}
  />
    <Status flow1 = {flow1} flow2={flow2}/>
  <Card 
    title = "Total (since app open)"
    value = {total}
    unit = "mL"
  />
</div>
<div class="grid grid-cols-6 gap-2 mt-2 bg-gray-600 p-4 relative">
  <div class="flex justify-center items-center absolute top-0 left-0 w-full h-full bg-[rgba(200,200,200,0.5)]">
    <h1 class="lock z-30 font-bold">LOCKED</h1>
  </div>
  <Gyro/>
  <Location/>
  <Radial/>
</div>
<style>
  .lock{
    font-size: 3rem;
  }
</style>
