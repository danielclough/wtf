const { joinRemoveSend } = require('../common/bot.js')
const config = require('../config/index.js');
const {joinCanvasMaker,joinInstructionMaker} = require('../common/canvas.js')

module.exports = {
	name: 'guildMemberAdd',
	once: false,
	async execute(member) {
		// Act on join
		if (member.user.bot || member.guild.id !== config.guild.id) return;

		const usernameAttachment = await joinCanvasMaker(member, config)
		const instructionAttachment = await joinInstructionMaker(config.welcome.welcomeMsg)
		
		member.roles.add(config.welcome.isolationRole);
		
		const welcomeRoom = member.guild.channels.cache.get(config.welcome.welcomeRoom);
		const inRoomMsg = `Welcome ${member}!`;
		joinRemoveSend(inRoomMsg, welcomeRoom, 7200)
		joinRemoveSend(config.welcome.welcomeMsg2, welcomeRoom, 64800)
		joinRemoveSend({ files: [instructionAttachment] }, welcomeRoom, 64800)
		joinRemoveSend({ files: [usernameAttachment] }, welcomeRoom, 64800)
	},
};