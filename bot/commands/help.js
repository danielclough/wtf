const config = require('../config');
const { removeSend, warnRemoveSend } = require('../common/bot')

module.exports = {
	name: 'help',
	command_class: 'Bot Help',
	description: 'List all of my commands or info about a specific command.',
	aliases: ['commands'],
	usage: '[command name]',
	cooldown: 5,
	execute(message, args, client) {
		const botRoom = client.channels.cache.get(config.guild.botRoom)
		let data = "";
		const { commands } = message.client;
		
		let commandClassList = commands.map(command => command.command_class)
		commandClassList = commandClassList.filter((v, i, a) => a.indexOf(v) === i);
		let commandsListedByClass = ""
		for (i=0;i<commandClassList.length;i++) {
			commandsListedByClass += `\n  🤖  ${commandClassList[i]}: \n`
			commandsListedByClass += commands.filter(command => command.command_class == commandClassList[i]).map(command => command.name).join('\`, \`')
		}

		if (!args.length) {
			helpMessage = `Here's a list of all my commands: \`${commandsListedByClass}\`
			
			You can send \`${config.bot.command_prefix}help [command name]\` to get info on a specific command!
			
			
			`;

			return removeSend(`${message.author}\n` + helpMessage, message)
		}

		const name = args[0].toLowerCase();
		const command = commands.get(name) || commands.find(c => c.aliases && c.aliases.includes(name));

		if (!command) {
			return  removeSend(`${message.author}\n` + helpMessage, message);
		}

		data += `Name: **${command.name}**\n`;

		if (command.aliases) data += `Aliases: **${command.aliases.join(', ')}**\n`;
		if (command.description) data += `Description: **${command.description}**\n`;
		if (command.usage) data += `Usage: **${config.bot.command_prefix}${command.name} ${command.args}**
		${command.usage}\n`;

		data += `Cooldown: **${command.cooldown || 3} second(s)**`;
		
		return warnRemoveSend(`${message.author}\n` + data, { split: true }, message, botRoom, (60 * 10));
	},
};