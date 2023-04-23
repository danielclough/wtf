export const returnColor = (test: number | string) => {
	let x =
		test === 1 || test === 'Strongly disagree'
			? 'red'
			: test === 2 || test === 'Disagree'
			? 'orange'
			: test === 3 || test === 'Neutral'
			? 'aqua'
			: test === 4 || test === 'Agree'
			? 'forestgreen'
			: 'lawngreen';
	return `color:${x}`;
};



const calculateType = () => {};