const config = require('../config/index.js');

module.exports = {
    name: 'ready',
    once: true,
    async execute(client) {
        client.user.setActivity(config.bot.watching, { type: 'config.bot.watching' })
        
        
        console.log(`[WTF_BOT] Connected as ${client.user.username}#${client.user.discriminator} ${client.user.id}`)
        console.log(`
            command_prefix: ${config.bot.command_prefix}
            watching: ${config.bot.watching}
        `)
    },
};