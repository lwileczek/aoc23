const fs = require('fs');

const filePath = 'data.txt';
const data = fs.readFileSync(filePath, 'utf8');

const paragraphs = data.split("\n\n");
const seedLine = paragraphs[0].split(': ')[1];
const maps = paragraphs.slice(1);
const seeds = seedLine.split(' ').map(s => +s);

for (const map of maps) {
  const lines = map.split('\n');
  const ref = [];
  //Make maping arrays
  for (let x=1; x< lines.length; x++) {
    ref.push(
      lines[x].split(' ').map(x => +x)
    )
  }

  for (let k=0; k < seeds.length; k++) {
    for (const r of ref) {
      const diff = seeds[k] - r[1]
      if (diff <= (r[2] - 1) && diff > 0) {
        seeds[k] = r[0] + diff;
        break;
      }
    }
  }
}

console.log(Math.min(...seeds));
