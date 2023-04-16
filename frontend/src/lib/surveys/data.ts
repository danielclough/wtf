const results = [
	[1,3,5,1,5,5,5,1,3,5,1,3,5,5,5,3,1,3,1,1,1,1,3,1],
	[1,2,4,2,4,4,4,2,4,2,2,3,4,2,2,3,2,2,3,2,3,2,2,2],
	[2,2,4,2,4,3,5,2,4,2,2,4,3,5,2,2,4,2,2,1,1,1,4,2],
	[1,1,3,1,5,4,5,1,5,2,4,3,4,3,5,5,2,3,3,3,1,1,4,1],
	[2,4,3,2,4,2,5,1,5,3,3,3,5,1,4,3,3,3,2,1,2,2,4,2],
	[1,3,2,2,5,3,5,1,5,3,1,2,5,5,3,3,3,1,1,1,1,1,1,1]
];
// like up dislike down

interface Points {
	x: number;
	y: number;
}

let scat: Points[] = [];


const averageArr: number[] = Array.from({length: results[0].length}).fill(0)
for (let [i, result] of results.entries()) {
	result
		.forEach((entry,j) => {
			averageArr[j] += entry
		});
	}

for (let [i, result] of results.entries()) {
	result
		.forEach((entry, j) => {
			let y = (averageArr[j] / results.length);
			scat.push({ x: j, y });
		});
	}

export const data = {
	labels: ['Scatter'],
	datasets: [
		{
			borderColor: 'rgba(99,0,125, .2)',
			backgroundColor: 'rgba(99,0,125, .5)',
			label: 'Results',
			data: scat
		}
	]
};
