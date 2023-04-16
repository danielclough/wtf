var crypto = require('crypto');
const { removeSend, warnRemoveSend } = require('../common/bot')
const {yamlToJson} = require('../common/bot.js')
const cfg = require('../config');
const branca = require("branca")(cfg.branca_key);

module.exports = {
	name: 'unpack',
	command_class: 'Community',
	title: 'Pack Object',
	usage: '[]',
	async execute(message, args, client) {
        let token = args[0]
        
        const payload = branca.decode(token).toString();

        warnRemoveSend(payload, message, null, 500)
	},
};