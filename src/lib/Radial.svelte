<script lang="ts">
  export let value = 10
  export let max = 200
  export let name = "Torque"
  export let unit = "Rpm"
  let style = () => {
    const stroke = Math.round((value / max) * 100);
    const color = stroke > 30 ? "yellow" : "red";
    return `--value: ${stroke} ;--size: 60px ;color: ${color}`;
  };
  $: percent = String(Math.round((value / max) * 100));
</script>


<div class="bg-black text-white flex w-[200px] h-[100px] justify-center gap-2 items-center m-0 col-span-2">
  <div class="rounded-lg h-s9 w-s9 flex flex-col justify-between bg-gray-800 p-1">
    <p class="text-xs">{name}</p>
    <h2 class="text-center text-lg font-bold">{value}</h2>
    <p class="text-xs">{unit}</p>
  </div>
  <div class="rounded-lg h-s9 w-s9 flex flex-col justify-between bg-gray-800 item-center">
    <p class="text-xs text-center py-1">Max {max}</p>
    <div
      class="radial-progress center translate-x-2.5 -translate-y-1"
      style={style()}
    >
      {percent} %
    </div>
  </div>
</div>
