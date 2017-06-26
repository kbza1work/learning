const wasm = require('./main.rs');

wasm.initialize().then(module => {
	const add = module.cwrap('add', 'number', ['number', 'number']);

	console.log(`from Rust: add(1, 2) = `);
	console.log(add(1,2));
});
