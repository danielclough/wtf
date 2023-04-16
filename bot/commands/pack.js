var crypto = require('crypto');
const { config } = require('dotenv');
const { removeSend, warnRemoveSend } = require('../common/bot')
const {yamlToJson} = require('../common/bot.js')
const cfg = require('../config');
const branca = require("branca")(cfg.branca_key);

module.exports = {
	name: 'pack',
	command_class: 'Community',
	title: 'Pack Object',
	usage: '[]',
	async execute(message, args, client) {

        let outputJSON = yamlToJson(message, args, true)

        const token = branca.encode(outputJSON);
        const payload = branca.decode(token);

        warnRemoveSend(token, message, null, 500)

        console.log(token);
        console.log(JSON.parse(payload));
	},
};