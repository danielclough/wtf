const fs = require('fs');
const { Collection } = require('discord.js');
const path = require('path');
const slashCommandsDir = path.join(__dirname, '..', '/slashCommands')

module.exports = {
    name: 'interactionCreate',
	once: false,
	async execute(interaction, client) {
        client.slashCommands = new Collection();
        const slashCommandFiles = fs.readdirSync(slashCommandsDir).filter(file => file.endsWith('.js'));
        
        for (const file of slashCommandFiles) {
            const slashCommand = require(`${slashCommandsDir}/${file}`);
            client.slashCommands.set(slashCommand.data.name, slashCommand);
        }
        
        
            if (!interaction.isCommand()) return;
            
            const slashCommand = client.slashCommands.get(interaction.commandName);
            
            if (!slashCommand) return;
            
            try {
                await slashCommand.execute(interaction, client);
            } catch (error) {
                console.error(error);
                return interaction.reply({ content: 'There was an error while executing this slashCommand!', ephemeral: true });
            }
    }
}