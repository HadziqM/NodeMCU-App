<script lang="ts">
  export let x = 0
  export let y = 0
  let getSlant = (x:number) => {
    return `transform: rotate(${x}deg)`;
  };
</script>
<div class="bg-black text-white flex w-[200px] h-[100px] justify-center col-span-2 items-center gap-2">
  <div class="rounded-lg h-s9 w-s9 flex flex-col justify-between bg-gray-800 item-center">
    <p class="text-xs text-center py-1">Slant Y</p>
    <div class="flex border-white border mx-2 mb-2 h-32 justify-center items-center test">
      <img
        src="/sideship.svg"
        class="w-16 h-8 translate-y-1"
        style={getSlant(y)}
        alt=""
      />
    </div>
  </div>
  <div class="rounded-lg h-s9 w-s9 flex flex-col justify-between bg-gray-800 item-center">
    <p class="text-xs text-center py-1">Slant X</p>
    <div class="flex border-white border mx-2 mb-2 h-32 justify-center items-center test">
      <img
        src="/frontship.svg"
        class="w-10 h-10 translate-y-1"
        style={getSlant(x)}
        alt=""
      />
    </div>
  </div>
</div>
