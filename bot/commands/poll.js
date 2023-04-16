// const optChars = ['⓪', '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪', '⑫', '⑬', '⑭', '⑮', '⑯', '⑰', '⑱', '⑲', '⑳', '㉑', '㉒', '㉓', '㉔', '㉕', '㉖', '㉗', '㉘', '㉙', '㉚', '㉛', '㉜', '㉝', '㉞', '㉟', '㊱', '㊲', '㊳', '㊴', '㊵', '㊶', '㊷', '㊸', '㊹', '㊺', '㊻', '㊼', '㊽', '㊾', '㊿']
const optEmojis = ['0️⃣', '1️⃣', '2️⃣', '3️⃣', '4️⃣', '5️⃣', '6️⃣', '7️⃣', '8️⃣', '9️⃣', '🔟']
const addReactionsUpDown = (message) => {
	message.react('👍')
	setTimeout(() => {
		message.react('👎')
	}, 350)
}
const addReactionsOpts = (message, optsArr) => {
	optsArr.forEach((opt, i) => {
		setTimeout(() => {
			message.react(optEmojis[i])
		}, 350)
	})
}

module.exports = {
	name: 'poll',
	command_class: 'Community',
	title: 'Community Poll',
	usage: 'Question {options; optional} [ending message; optional]',
	url: '',
	author: {
		name: '',
		url: '',
	},
	async execute(message, args, client) {
		let optsArr = []
		pureContent =  args.join(' ')
		
		args.forEach(element => {
			/\{*.*\}/.test(element) && optsArr.push(element)
		});
		let addendum
		if (/\[*.*\]/.test(message.content)) {
			let clip = message.content.split("[")[1]
			addendum = clip.split("]")[0]
		}

		if (!optsArr.length) {
			// has no bracketed options
			message.channel.send(pureContent).then(msg => {
				addReactionsUpDown(msg)
			})
		} else {
			// has bracketed options
			let question = pureContent.split("{")[0]
			message.channel.send(`**${question}**`).then(msg => {
				optsArr.forEach((element, i) => {
					let last = element.length - 1
					let option = element.substring(1, last)
					message.channel.send(`${optEmojis[i]}:    **${option}**`)
				});
				if (!addendum) addendum = "Choose wisely..." 
				message.channel.send(addendum).then(x => {
					addReactionsOpts(x, optsArr)
				})
			})
		}

		await message.delete()
	},
};
// // has bracketed options
// let question = pureContent.split("{")[0]
// message.channel.send(`**${question}:**`)
// optsArr.forEach((element, i) => {
// 	let last = element.length - 1
// 	let elem = element.substring(1,	last)
// 	message.channel.send(`**${elem}**`).then(msg => {
// 		addReactionsUpDown(msg)
// 	})
// });