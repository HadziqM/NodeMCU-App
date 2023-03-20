<script lang="ts">
  export let value = 0
  export let  title="Flow Rate"
  export let  unit="mL/min"
  $: val = Number(value.toFixed(2)).toLocaleString('en-US')
</script>
<div class="bg-black text-white flex w-[150px] h-[80px] justify-center items-center gap-1">
  <div class="rounded-lg h-f9 w-f9 flex flex-col justify-between bg-gray-800 p-1">
    <p class="text-xs">{title}</p>
    <h2 class="text-center text-lg font-bold">{val}</h2>
    <p class="text-xs">{unit}</p>
  </div>
</div>

