<script lang="ts">
  export let flow1 = false
  export let flow2 = true
    let styles = (arg: boolean) => {
    let some = arg === true ? `background: green` : `background: red`;
    return some;
  };
</script>
<div class="flex w-[150px] h-[200px] bg-black row-span-2 flex-col text-white p-1 gap-1">
    <div class="w-full bg-gray-800 rounded-lg flex items-center text-left px-2">
      <h2>Flow Sensor</h2>
    </div>
    <div class="flex-col flex bg-gray-800 rounded-lg w-full h-full status">
      <p class="w-full px-2 m-1 font-bold">Connection</p>
      <div class="flex items-center justify-between w-full pl-2 pr-4">
        <p>Flow In</p>
        <div
          class="border rounded-full h-2 w-2"
          style={styles(flow1)}
        ></div>
      </div>
      <div class="flex items-center justify-between w-full pl-2 pr-4">
        <p>Flow Out</p>
        <div
          class="border rounded-full h-2 w-2"
          style={styles(flow2)}
        ></div>
      </div>
    </div>
  </div>
  <style>
  h2{
  font-size: 1rem;
  }
  p{
  font-size: 0.8rem;
  }
</style>
