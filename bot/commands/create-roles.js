const { ChannelType, PermissionsBitField } = require('discord.js');
const config = require('../config');
const { removeSend, warnRemoveSend } = require('../common/bot')

module.exports = {
	name: 'create-roles',
	command_class: 'Admin',
	description: 'Create roles.',
	aliases: ['commands'],
	usage: '[command name]',
	cooldown: 5,
	execute(message, args, client) {

        // createRole("test", '#37FF3F', `Indicate interest in test!`, message)

        // Object.keys(config.roles.community).forEach(role => {
        //     createRole(role, '#37FF3F', `Indicate interest in ${role}!`, message)
        // });
        // Object.keys(config.roles.human).forEach(role => {
        //     createRole(role, '#A2FF00', `${role.charAt(0).toUpperCase() + role.slice(1)} speakers are important to our community!`, message)
        // });
        // Object.keys(config.roles.programming).forEach(role => {
        //     createRole(role, '#A27100', `Help organize development in ${role}!`, message)
        // });
        
        const roles = message.guild.roles.cache.map(role => `${role.name}: ${role.id},`);
        // const roles = message.guild.roles.cache.map(role => role.name);

        console.log(roles)
        message.reply("...")
    }
}

const createRole = (role, color, reason, message) => {
    message.guild.roles.create({
        name: role,
        color: color,
        reason: reason,
    })
    .then(console.log)
    .catch(console.error);
}