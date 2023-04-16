
const { removeSend } = require('../common/bot')

module.exports = {
	name: 'clear',
	command_class: 'Admin',
	commandCategory: 'administrative',
	title: 'Clear Channel',
	description: `Delete up to 100 messages.`,
	usage: '[command name]',
	url: '',
	author: {
		name: '',
		url: '',
	},
	execute(message, args, client) {
		const user = message.mentions.users.first();
		let amount = Number(args.find(arg => !/<@!?\d+>/g.test(arg))) + 1;
		amount === 101 ? amount = 100 : null
		if (amount > 0 && amount <= 100) {
			message.channel.messages.fetch({
				limit: amount,
			}).then((messages) => {
				message.channel.bulkDelete(messages).catch(error => {
					console.log(error.stack)
					removeSend("**Error:** Almost certainly because you are tying to delete messages older than 14 days old. (Admin can check log.)", message)
				});
			});
		} else {
			message.channel.messages.fetch().then((results) => {
			message.channel.bulkDelete(results).catch(error => {
				console.log(error.stack)
				removeSend("**Error:** Almost certainly because you are tying to delete messages older than 14 days old. (Admin can check log.)", message)
			});
		})
		}
	},
};