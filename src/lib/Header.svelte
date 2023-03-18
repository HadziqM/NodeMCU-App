<script lang="ts">
  import { onMount } from "svelte";
  let date = {
    date:"today",
    time:"now"
  }
  let add_zero = (num:number) => num<10 ? `0${num}` : num;
  onMount(()=>{
    let clock = setInterval(() => {
      let tick = new Date();
      date.time = `${add_zero(tick.getHours())}:${add_zero(tick.getMinutes())}:${add_zero(tick.getSeconds())}`;
      date.date = tick.toDateString()
    }, 1000);
    return () => clearInterval(clock)
  })
</script>
    <div class="flex flex-col w-screen absolute top-[20px]">
      <div class="flex justify-between  items-center p-4 bg-blue-50">
        <img src="/logo.svg" alt="" class="max-h-12"/>
        <div>
          <p class="text-xs">
            {date.date} | {date.time}
          </p>
          <h1 class="text-2xl">SHIP MONITORING</h1>
        </div>
      </div>
      <div class="bg-blue-800 flex justify-between items-center px-4 py-2 text-white">
        <div class="flex gap-4">
          <img src="/ship.svg" alt="" class="h-6 w-6 invert"/>
          <h2>Beranda</h2>
          <h2>Setting</h2>
        </div>
        <div class="flex gap-2">
          <h2>Hi,Brother</h2>
          <img src="/user.svg" alt="" class="h-4 w-4 invert"/>
        </div>
      </div>
    </div>
