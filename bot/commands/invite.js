const config = require('../config');
const { removeSend } = require('../common/bot')
module.exports = {
	name: 'invite',
	command_class: 'Admin',
	description: '',
	usage: '',
	async execute(message, args, client) {
		removeSend(`Link to this server: https://discord.gg/${config.guild.inviteCode}`, message);
		removeSend(`Invite bot to another server: https://discord.com/oauth2/authorize?client_id=${config.bot.botId}&scope=bot+applications.commands`, message);
	},
};