const config = require('../config/index.js');
const fs = require('fs');
const { Collection } = require('discord.js');
const path = require('path');
const commandDir = path.join(__dirname, '..', '/commands')

module.exports = {
    name: 'messageReactionAdd',
    once: false,
    async execute(reaction_orig, user) {

        console.log(reaction_orig, user);
    }
}