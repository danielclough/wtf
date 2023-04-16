const config = require('../config');
const command_prefix = config.bot.command_prefix;

// Community Roles
const communityRole = "**Community Roles**";
const communityRoleAllowed = Object.keys(config.roles.community);
const communityRoleAllowedForMsg = Object.keys(config.roles.community).join("\`, \`");

// Human Langs
const humanRole = "**Human Lanugages**";
const humanRoleAllowed = Object.keys(config.roles.human);
const humanRoleAllowedForMsg = Object.keys(config.roles.human).join("\`, \`");

// Bot Roles
const programmingRole = "**Programming Lanugages**";
const programmingRoleAllowed = Object.keys(config.roles.programming);
const programmingRoleAllowedForMsg = Object.keys(config.roles.programming).join("\`, \`");

// Exact Match
const escapeRegExpMatch = function (s) {
	return s.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&').replace(/^[,\s]+|[,\s]+$/g, '').replace(/,[,\s]*,/g, ',');
};
const isExactMatch = (str, match) => {
	let v = false;
	for (let s of str) {
		s === match
			? v = true
			: null
	}
	return (new RegExp(`\\b${escapeRegExpMatch(match)}\\b`).test(str)
		|| v)
};

module.exports = {
	name: 'roles',
	command_class: 'Admin',
	guildOnly: true,
	usage: '[command name]',
	author: {
		url: '',
	},
	execute(message, args, client) {
		const icon = `${message.guild.iconURL()}`;
		const name = `${message.guild.name}`;
		const botRoom = client.channels.cache.get(config.guild.botRoom);
		if (!args.length) {
			return message.channel.send({
				embeds: [
					{
						color: 0x0099ff,
						title: `${name}`,
						author: {
							name: this.author.name,
							icon_url: icon,
							url: this.author.url,
						},
						description: `**Curious explorers** use: \n \`${command_prefix}roles curious\`  
						\n
						**English speakers** use: \n \`${command_prefix}roles english\`

					${communityRole}:
					\`${communityRoleAllowedForMsg}\`

					${humanRole}:
					\`${humanRoleAllowedForMsg}\`

					${programmingRole}:
					\`${programmingRoleAllowedForMsg}\`

					To add a role use the \`${command_prefix}roles\` command followed by the roles you want added!
					
					${config.welcome.exampleRoleMessage}\n`,
						thumbnail: {
							url: icon,
						},
						image: {
							url: `${config.guild.mainImage}`,
						},
						timestamp: new Date(),
						footer: {
							text: `The future of ${name} starts`,
							icon_url: icon,
						},
					}],
			});
		} else {

			const { author } = message;
			const roleAddedMessage = `${message.author} 
	If you ever need bot help you can just type \`${command_prefix}help\`. \n\n
							Use \`${command_prefix}roles\` at any time to add more roles! \n
							`;
			const humanRoleToAdd = [];
			const programmingRoleToAdd = [];
			const communityRoleToAdd = [];
			const messagesToSend = [];

			for (const arg of args) {
				const content = arg.toLowerCase();
				for (let i = 0; i < humanRoleAllowed.length; i++) {
					if (content.includes(humanRoleAllowed[i])) {
						if (isExactMatch(humanRoleAllowed, content) == true) {
							humanRoleToAdd.push(humanRoleAllowed[i]);
						}
					}
				}
				for (let i = 0; i < programmingRoleAllowed.length; i++) {
					if (content.includes(programmingRoleAllowed[i])) {
						if (isExactMatch(programmingRoleAllowed, content) == true) {
							programmingRoleToAdd.push(programmingRoleAllowed[i]);
						}
					}
				}
				for (let i = 0; i < communityRoleAllowed.length; i++) {
					if (content.includes(communityRoleAllowed[i])) {
						if (isExactMatch(communityRoleAllowed, content) == true) {
							communityRoleToAdd.push(communityRoleAllowed[i]);
						}
					}
				}
			}
			if (!humanRoleToAdd.length && !programmingRoleToAdd.length && !communityRoleToAdd.length) {
				message.channel.send({
					embeds: [{
						color: Math.floor(Math.random() * (0xFFFFFF + 1)),
						description: `Add a list of roles after \`${command_prefix}roles\`. \n
				${humanRole} Roles are \`${humanRoleAllowed.join("\`, \`")}\`. \n
				${programmingRole} Roles are \`${programmingRoleAllowed.join("\`, \`")}\`. \n
				${communityRole} Roles are \`${communityRoleAllowed.join("\`, \`")}\`. \n`,
					}],
				});
			} else {
				// remove welcome
				message.member.roles.remove(config.welcome.isolationRole);

				// HUMAN LANGS
				for (let i = 0; i < humanRoleToAdd.length; i++) {
					const role = humanRoleToAdd[i];
					message.member.roles.add(config.roles.human[role]) // ensure this is a string in the roles ("")
						.then(console.log(`${humanRoleToAdd[i]} (${config.roles.human[role]}) added to member ${author.username} (${author.id})`))
						.catch(console.error);
					// send verification message
					const outgoingMessage = botRoom.send({
						embeds: [{
							color: Math.floor(Math.random() * (0xFFFFFF + 1)),
							description: roleAddedMessage,
							timestamp: new Date(),
							footer: {
								text: `${humanRoleToAdd[i]} added!`,
							}
						}],
					});
					messagesToSend.push(outgoingMessage);
				}

				// PROGRAMMING LANGS
				for (let i = 0; i < programmingRoleToAdd.length; i++) {
					const role = programmingRoleToAdd[i];
					message.member.roles.add(config.roles.programming[role]) // ensure this is a string in the roles ("")
						.then(console.log(`${programmingRoleToAdd[i]} (${config.roles.programming[role]}) added to member ${author.username} (${author.id})`))
						.catch(console.error);
					// send verification message
					const outgoingMessage = botRoom.send({
						embeds: [{
							color: Math.floor(Math.random() * (0xFFFFFF + 1)),
							description: roleAddedMessage,
							timestamp: new Date(),
							footer: {
								text: `${programmingRoleToAdd[i]} added!`,
							}
						}],
					});
					messagesToSend.push(outgoingMessage);
				}

				// COMMUNITY LANGS
				for (let i = 0; i < communityRoleToAdd.length; i++) {
					const role = communityRoleToAdd[i];
					message.member.roles.add(config.roles.community[role]) // ensure this is a string in the roles ("")
						.then(console.log(`${communityRoleToAdd[i]} (${config.roles.community[role]}) added to member ${author.username} (${author.id})`))
						.catch(console.error);
					// send verification message
					const outgoingMessage = botRoom.send({
						embeds: [{
							color: Math.floor(Math.random() * (0xFFFFFF + 1)),
							description: roleAddedMessage,
							timestamp: new Date(),
							footer: {
								text: `${communityRoleToAdd[i]} added!`,
							}
						}],
					});
					messagesToSend.push(outgoingMessage);
				}

				for (let i = 0; i < messagesToSend.length; i++) {
					return messagesToSend[i];
				}
			}
		}
	},
};
