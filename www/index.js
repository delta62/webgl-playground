import("webgl").then((wasm) => {
  let c = document.getElementById("canvas");
  c.width = window.innerWidth;
  c.height = window.innerHeight;
  wasm.start();
});
