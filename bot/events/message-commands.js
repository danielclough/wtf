const config = require('../config/index.js');
const fs = require('fs');
const { Collection } = require('discord.js');
const path = require('path');
const commandDir = path.join(__dirname, '..', '/commands')

module.exports = {
    name: 'messageCreate',
	once: false,
	async execute(message, client) {

    const cooldowns = new Collection();

    // make command names commands
    client.commands = new Collection();
    const commandFiles = fs.readdirSync(`${commandDir}`).filter(file => file.endsWith('.js'));

    for (const file of commandFiles) {
    const command = require(`${commandDir}/${file}`);
    client.commands.set(command.name, command);
    }

    
        if (message.author.bot || message.member.roles.cache.has('1060401903420325980')) return;
    
    if (!message.content.startsWith(config.bot.command_prefix) ) return;
    
    const args = message.content.slice(config.bot.command_prefix.length).split(/ +/);
    const commandName = args.shift().toLowerCase();
    
    const command = client.commands.get(commandName)
    || client.commands.find(cmd => cmd.aliases && cmd.aliases.includes(commandName));
    
    
    if (!command) return;
    
    if (command.guildOnly && message.channel.type !== 'GUILD_TEXT') {
        return message.reply('I can\'t execute that command inside DMs!');
    }
    
    if (command.rootOnly && !message.member.roles.cache.has(config.guild.rootId)) {
        return message.reply('This command is for root users only!');
    }

    if (command.args && !args.length) {
        let reply = `You didn't provide any arguments, ${message.author}!`;

        if (command.usage) {
        reply += `\nThe proper usage would be: \`${config.bot.command_prefix}${command.name} ${command.usage}\``;
        }

        return message.channel.send(reply);
    }

    // prevent spam with "cooldowns"
    if (!cooldowns.has(command.name)) {
        cooldowns.set(command.name, new Collection());
    }

    const now = Date.now();
    const timestamps = cooldowns.get(command.name);
    const cooldownAmount = (command.cooldown || 3) * 1000;

    if (timestamps.has(message.author.id)) {
        const expirationTime = timestamps.get(message.author.id) + cooldownAmount;

        if (now < expirationTime) {
        const timeLeft = (expirationTime - now) / 1000;
        return message.reply(`please wait ${timeLeft.toFixed(1)} more second(s) before reusing the \`${command.name}\` command.`);
        }
    }

    timestamps.set(message.author.id, now);
    setTimeout(() => timestamps.delete(message.author.id), cooldownAmount);

    try {
        command.execute(message, args, client);
    } catch (error) {
        console.error(error);
        message.reply('there was an error trying to execute that command!');
    }
}}
