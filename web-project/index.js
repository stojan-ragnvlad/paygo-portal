import init, { greet } from "./compile_time_tools.js";

init().then(() => {
  console.log(greet());
});

//fetch('compile_time_tools_bg.wasm').then(wasm => wasm.arrayBuffer())
//  .then(bytes => WebAssembly.instantiate(bytes, { wbg: {}}))
//  .then(({ instance }) => console.log(instance.exports.greet()));

