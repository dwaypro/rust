// const wasmJs = import("./pkg/helloworld.js");
// wasmJs.then( js => {
//     console.log('js', js);
//     js.greet("Hello From Wasm!");
// })

// var importObject = { imports: { imported_func: arg => console.log(arg) } };
// WebAssembly.instantiateStreaming(fetch('./helloworld.wasm'), importObject).then(function(res){
//     console.log('res', res);
//     res.greet("Hello from wasm!");
// })

fetch('helloworld.wasm').then(response =>
    response.arrayBuffer()
  ).then(bytes =>
    WebAssembly.instantiate(bytes, importObject)
  ).then(results => {
    // Do something with the results!
    results.greet("Hello From Wasm!");
  });