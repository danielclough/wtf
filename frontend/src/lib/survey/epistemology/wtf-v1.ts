export default {
	title: 'Epistemology Survey v1',
	image: ['/images/wtf.svg', 'Epistemology Logo'],
	copyright: {
		title: 'WTF Epistemology Survey',
		date: '2023',
		rights: 'Original Work',
		cite: ['', ''],
		alt: [
			'Epistemology - "theory of knowledge," 1856... literally "overstand" (from epi "over, near" + histasthai "to stand")',
			'https://www.etymonline.com/word/Epistemology'
		]
	},
	api: 'https://formspree.io/f/mwkjwogv',
	responses: ['Strongly disagree', 'Disagree', 'Neutral', 'Agree', 'Strongly agree'],
	isms: ['mysticism', 'empiricism', 'rationalism', 'idealism', 'reductionism'],
	questions: [
		{
			topic: 'HOW DO WE KNOW?',
			questions: [
				{
					id: 1,
					text: 'Truth can be known by logic alone.',
					type: ['rationalism', 'idealism']
				},
				{
					id: 2,
					text: 'Prophecy is valid knowledge.',
					type: ['mysticism']
				},
				{
					id: 3,
					text: 'Seeing, touching, feeling, smelling, or hearing give the most certain knowledge.',
					type: ['empiricism']
				},
				{
					id: 4,
					text: 'Clairvoyance, Divination, Dowsing, and/or Telepathy are valid forms of knowledge.',
					type: ['mysticism']
				},
				{
					id: 5,
					text: 'Intuition is valid knowledge.',
					type: ['mysticism']
				},
				{
					id: 6,
					text: 'Faith is valid knowledge.',
					type: ['mysticism']
				}
			]
		},
		{
			topic: 'WHAT IS KNOWLEDGE?',
			questions: [
				{
					id: 7,
					text: 'Facts represented in text and facts in personal memory are equally valid knowledge.',
					type: ['reductionism']
				},
				{
					id: 8,
					text: 'Knowledge is just information stored in the brain.',
					type: ['reductionism']
				},
				{
					id: 9,
					text: 'True knowledge is in some way beyond the physical world.',
					type: ['mysticism', 'idealism']
				},
				{
					id: 10,
					text: 'Knowledge is nothing more than electrochemical signals in the brain.',
					type: ['reductionism']
				},
				{
					id: 11,
					text: 'Scientific fact is the most true knowledge.',
					type: ['empiricism', 'reductionism', 'rationalism']
				},
				{
					id: 12,
					text: 'Knowledge is merely true belief.',
					type: ['reductionism']
				}
			]
		},
		{
			topic: 'WHAT CAN BE KNOWN?',
			questions: [
				{
					id: 13,
					text: 'Only what we see, touch, feel, smell, or hear.',
					type: ['empiricism']
				},
				{
					id: 14,
					text: 'A world of ideas apart from the physical world.',
					type: ['idealism', 'mysticism']
				},
				{
					id: 15,
					text: "Other people's thoughts and/or feelings.",
					type: ['mysticism']
				},
				{
					id: 16,
					text: 'Our own motivations.',
					type: ['idealism']
				},
				{
					id: 17,
					text: 'The motivations of others.',
					type: ['idealism']
				},
				{
					id: 18,
					text: 'Supernatural events and entities.',
					type: ['mysticism']
				}
			]
		},
		{
			topic: 'WHY SHOULD WE CHANGE OUR MINDS?',
			questions: [
				{
					id: 19,
					text: 'When presented with scientific arguments.',
					type: ['rationalism', 'empiricism']
				},
				{
					id: 20,
					text: 'When an expert tells you to.',
					type: ['rationalism']
				},
				{
					id: 21,
					text: 'When presented with factual arguments.',
					type: ['rationalism']
				},
				{
					id: 22,
					text: 'When we feel strongly about something.',
					type: ['idealism']
				},
				{
					id: 23,
					text: 'When presented with logical arguments.',
					type: ['rationalism']
				},
				{
					id: 24,
					text: 'Never.',
					type: ['idealism', 'mysticism']
				}
			]
		}
	]
};
