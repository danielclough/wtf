module.exports = {
	name: 'join',
	command_class: 'Admin',
	description: 'Fake join for testing and debug.',
	usage: '[command name]',
	async execute(message, args, client) {
		client.emit('guildMemberAdd', message.member)
		await message.delete()
	},
};