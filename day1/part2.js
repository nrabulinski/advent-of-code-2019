const { readFileSync } = require('fs');
let modules = readFileSync('input').toString().split('\n').map(a => parseInt(a)).filter(a => !isNaN(a));
let fuels = modules.map(module => Math.floor(module / 3) - 2);
fuels = fuels.map(fuel => {
	let total = fuel;
	while((fuel = Math.floor(fuel / 3) - 2) >= 0) total += fuel;
	return total;
});
console.log(fuels.reduce((a, b) => a + b));

