async function init(){
	const response =await  fetch("sum.wasm")
	const buffer = await response.arrayBuffer()
	const wasm = await WebAssembly.instantiate(buffer)
	const sum = wasm.instance.exports.addTwo
	console.log(sum(1,2))
}

init();