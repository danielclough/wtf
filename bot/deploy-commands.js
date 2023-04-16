const fs = require('fs');
const { REST } = require('@discordjs/rest');
const { Routes } = require('discord-api-types/v9');
const config = require('./config');

const slashCommands = [];
const slashCommandFiles = fs.readdirSync('./slashCommands').filter(file => file.endsWith('.js'));

for (const file of slashCommandFiles) {
	const slashCommand = require(`./slashCommands/${file}`);
	slashCommands.push(slashCommand.data.toJSON());
}

const rest = new REST({ version: '9' }).setToken(config.bot.token);

rest.put(Routes.applicationGuildCommands(config.bot.botId, config.guild.id), { body: slashCommands })
	.then(() => console.log('Successfully registered application commands.'))
	.catch(console.error);

