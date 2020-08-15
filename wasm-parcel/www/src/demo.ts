const imports = {js: {
  import1: () => console.log("hello,"),
  import2: () => console.log("world!")
}};

fetch('demo.wasm')
  .then(res => res.arrayBuffer())
  .then(buffer => WebAssembly.instantiate(buffer, imports))
  .then(({module, instance}) => {
    //instance.exports.f()
  })