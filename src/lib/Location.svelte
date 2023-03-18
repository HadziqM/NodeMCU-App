<script lang="ts">
  export let degree = 0
  export let speed = 0
  let getDegree = (x:number) => {
    const deg = x - 45;
    return `transform: rotate(${deg} deg)`;
  };
</script>
<div class="bg-black text-white flex w-[200px] h-[100px] col-span-2 justify-center items-center gap-2">
  <div class="rounded-lg h-s9 w-s9 flex flex-col justify-between bg-gray-800 p-1">
    <p class="text-xs">Speed</p>
    <h2 class="text-center text-lg font-bold">{speed}</h2>
    <p class="text-xs">Knot</p>
  </div>
  <div class="rounded-lg h-s9 w-s9 flex flex-col justify-between bg-gray-800 item-center">
    <p class="text-lg text-center py-1 translate-y-1">U</p>
    <div class="flex border-white border mx-2 mb-2 h-32 justify-center items-center test">
      <img
        src="/arrow.svg"
        class="w-8 h-8 translate-y-1"
        style={getDegree(degree)}
        alt=""
      />
    </div>
  </div>
</div>
