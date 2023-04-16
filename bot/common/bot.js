const yaml = require('js-yaml');
const config = require('../config');

const channelCheck = (message, channel) => {
	if (!channel) channel = message.client.channels.cache.get(config.guild.botRoom)
	if (message.channel !== channel) {
		message.delete()
	}
	return channel
}
const parseTime = (time) => {
	let minutes = parseInt(time / 60)
	let seconds = time % 60
	return durationStr = seconds !== 0
		? `${minutes} minutes and ${seconds} seconds`
		: `${minutes} minutes`
}

// 2 required
const removeSend = (content, message, channel, waitToDelete, suppressEmbeds) => {
	if (!waitToDelete) waitToDelete = 45
	channel = channelCheck(message, channel)
	channel.send(content).then(
		(message) => {
			setTimeout(() => {
				message.delete()
			}, 1000 * waitToDelete)
		}
	)
}

// 2 required
const joinRemoveSend = (content, channel, waitToDelete) => {
	if (!waitToDelete) waitToDelete = 90
	channel.send(content).then((message) => {
		if (message) {
			let mWait = waitToDelete / 3
			setTimeout(() => {
			}, mWait)
			setTimeout(() => {
				message.delete()
			}, waitToDelete * 1000)
		}
	});
}

// 2 required
const warnRemoveSend = (content, message, channel, waitToDelete) => {
	const alertTarget = message.author
	if (!waitToDelete) waitToDelete = 90
	channel = channelCheck(message, channel)
	channel.send(content).then((message) => {
		if (message) {
			let durationStr = parseTime(waitToDelete)
			let mWait = waitToDelete / 3
			setTimeout(() => {
			}, mWait)
			channel.send(`${alertTarget}, This message will self destruct in ${durationStr}.`).then((nextMessage) => {
				setTimeout(() => {
					message.delete()
					nextMessage.delete()
				}, waitToDelete * 1000)
			});
		}
	});
}
// 3 required
const delaySend = (waitToSend, content, message, channel, waitToDelete) => {
	channel = channelCheck(message, channel)
	setTimeout(() => {
		console.log("waitToSend", waitToSend)
		warnRemoveSend(content, message, channel, waitToDelete)
	}, waitToSend * 1000)
}

const yamlToJson = (message, args, stringify) => {
	let prefix = 'yaml'
	const removePrefix = (str, prefix) => {
		let len = prefix.length;
		return str.slice(len)
	}

	if (args[0].includes(`\`\`\`${prefix}`)) {
		let str = message.content.split("```")[1]

		const inputYML = removePrefix(str, prefix)

		let out = yaml.load(inputYML);
		if (stringify === true) {
			return JSON.stringify(out);
		} else {
			return out;
		}

	}
}

module.exports = { removeSend, joinRemoveSend, warnRemoveSend, delaySend, yamlToJson }