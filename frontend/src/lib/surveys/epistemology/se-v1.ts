export default {
	title: 'Street Epistemology Survey v1',
	image: ['/images/logo-se-white.svg', 'Street Epistemology Logo'],
	copyright: {
		title: 'Street Epistemology Survey and Associated Branding',
		date: '2023',
		rights: 'Fair Use (Clarification Requested)',
		cite: [
			'Link to Original',
			'https://docs.google.com/spreadsheets/d/1V8VGPgKpy-U7Gdf_PouahqCBsEQY1DgIhEoyZ0zNd7s/edit#gid=217309315'
		],
		alt: []
	},
	api: 'https://formspree.io/f/mwkjwogv',
	responses: ['Strongly disagree', 'Disagree', 'Neutral', 'Agree', 'Strongly agree'],
	questions: [
		{
			topic: 'WHAT IS TRUTH?',
			questions: [
				{
					id: 1,
					text: 'A statement is true when it corresponds to reality.'
				},
				{
					id: 2,
					text: 'We all share the same reality and only interpret it differently.'
				},
				{
					id: 3,
					text: 'Truth depends on the opinions and beliefs of people.'
				},
				{
					id: 4,
					text: 'People create words and define their meaning.'
				},
				{
					id: 5,
					text: 'A statement is true if everyone agrees.'
				},
				{
					id: 6,
					text: 'Strong belief, even without action, can change external reality.'
				}
			]
		},
		{
			topic: 'HOW DOES BELIEF WORK?',
			questions: [
				{
					id: 7,
					text: 'Some beliefs should never be questioned.'
				},
				{
					id: 8,
					text: 'Someone can be certain something is true yet still be mistaken.'
				},
				{
					id: 9,
					text: 'We should be satisfied with a test that can only confirm our claim.'
				},
				{
					id: 10,
					text: 'If all members of a society share a belief, they are justified to hold that belief.'
				},
				{
					id: 11,
					text: 'Believing something that is false feels just like believing something that is true.'
				},
				{
					id: 12,
					text: 'Feelings are a reliable way to discover truth.'
				}
			]
		},
		{
			topic: 'WHEN SHOULD WE BELIEVE?',
			questions: [
				{
					id: 13,
					text: 'Believing something without evidence is admirable.'
				},
				{
					id: 14,
					text: 'Without an explanation for something, any answer is better than no answer.'
				},
				{
					id: 15,
					text: 'It is ok to accept something is true because it is comforting.'
				},
				{
					id: 16,
					text: 'I give all claims the benefit of the doubt when I first encounter them.'
				},
				{
					id: 17,
					text: 'Someone is justified in their beliefs until they are proven wrong.'
				},
				{
					id: 18,
					text: 'The most important criteria for my beliefs is that they match reality.'
				}
			]
		},
		{
			topic: 'WHEN SHOULD WE CHANGE OUR MINDS?',
			questions: [
				{
					id: 19,
					text: 'I often investigate beliefs that do not match my own.'
				},
				{
					id: 20,
					text: 'I am comfortable with saying: "I don\'t know".'
				},
				{
					id: 21,
					text: 'It is beneficial to find out when I am wrong about something.'
				},
				{
					id: 22,
					text: 'I will abandon a belief if I discover reliable information that falsifies it.'
				},
				{
					id: 23,
					text: 'The more unusual the statement, the stronger the evidence needs to be.'
				},
				{
					id: 24,
					text: 'It is possible that some of my beliefs are not true.'
				}
			]
		}
	]
};
