const { readFileSync } = require('fs');
let modules = readFileSync('input').toString().split('\n').map(Number);
let fuels = modules.map(module => Math.floor(module / 3) - 2);
console.log(fuels.reduce((a, b) => a + b));
