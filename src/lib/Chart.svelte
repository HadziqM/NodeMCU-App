<script lang="ts">
  import "@google-web-components/google-chart";
  import { flowState } from "../global";
  let fall:boolean;
  flowState.subscribe(e=>{fall=e})
  export let data: (String[] | (string | number)[])[] = [];
  export let title: String = "";
  export let colors: String[] = [];
  $: options = {
    width:450,
    height:200,
    title,
    legend: "none",
    backgroundColor: "#222",
    colors: colors.length > 0 ? colors : undefined,
    titleTextStyle: { fontSize: 14, color: "#aaa" },
    hAxis: { textStyle:{color: "#aaa"}},
    wAxis: { textStyle:{color: "#aaa"}}
  };
</script>
<div class="text-white w-[450px] h-[200px] bg-gray-800 row-span-2 col-span-3 relative">
  <button on:click={()=>{
    flowState.update(e=>!e)
  }} class="py-1 px-2 absolute top-1 left-1 z-30 text-sm bg-red-600">{fall?"Fall":"Rise"}</button>
  <google-chart {data} options={{ ...options }} />
</div>
