// Add Canvas dependencies
// sudo apt-get install build-essential libcairo2-dev libpango1.0-dev libjpeg-dev libgif-dev librsvg2-dev
const { MessageAttachment } = require('discord.js');
const Canvas = require('canvas');
const path = require('path');

// Pass Canvas Object
function applyText(canvas, ctx, text, size, color, style, widthDivisor, heightDivisor, pad) {
    do {
        // Assign the font to the context and decrement it so it can be measured again
		ctx.font = `${size}px ${style}`;
        size -= size * 0.01
		// Compare pixel width of the text to the canvas minus the approximate avatar size
	} while (ctx.measureText(text).width > canvas.width - pad);
    
    
    ctx.fillStyle = color;
    ctx.fillText(text, canvas.width / widthDivisor, canvas.height / heightDivisor);

	return ctx
};

async function joinInstructionMaker(msg) {
    const imageDir = path.join(__dirname, '..', '/images');
    const canvas = Canvas.createCanvas(700, 400);
    let ctx = canvas.getContext('2d');
    
    // Number of images
    const n = Math.ceil(Math.random() * 56);
    
    const backgroundImage = await Canvas.loadImage(`${imageDir}/jpg/background${n}.jpg`);
    
    // use the canvas dimensions to stretch the image onto the entire canvas
    ctx.drawImage(backgroundImage, 0, 0, canvas.width, canvas.height);
    // transparent overlay
    ctx.rect(0, 0, canvas.width, canvas.height)
    ctx.fillStyle = "rgba(0, 0, 0, .3)"
    ctx.fill()
    
    // Border
    ctx.strokeStyle = '#74037b';
    ctx.strokeRect(0, 0, canvas.width, canvas.height);
    
    // Fontconfig error? -> `sudo apt install -y fontconfig`
    const msgArr = msg.split('\n')

    for (let [i, m] of msgArr.entries()) {
        const text = msgArr[msgArr.length - i-1]
        const widthDivisor = 10
        const heightDivisor = i == 0 ? 1.2 : (i + msgArr.length) / (msgArr.length - i)
        ctx = applyText(canvas, ctx, text, 42, '#ffffff', 'sans-serif', widthDivisor, heightDivisor, 100);
    }
        
    return new MessageAttachment(canvas.toBuffer(), 'image.png');
}


async function joinCanvasMaker(member, config) {
    const imageDir = path.join(__dirname, '..', '/images');
    const canvas = Canvas.createCanvas(700, 250);
    let ctx = canvas.getContext('2d');

    // Number of images
    const n = Math.ceil(Math.random() * 56);

    const backgroundImage = await Canvas.loadImage(`${imageDir}/jpg/background${n}.jpg`);
    
    // use the canvas dimensions to stretch the image onto the entire canvas
    ctx.drawImage(backgroundImage, 0, 0, canvas.width, canvas.height);
    // transparent overlay
    ctx.rect(0, 0, canvas.width, canvas.height)
    ctx.fillStyle = "rgba(0, 0, 0, .2)"
    ctx.fill()

    // Border
    ctx.strokeStyle = '#74037b';
    ctx.strokeRect(0, 0, canvas.width, canvas.height);

    // Fontconfig error? -> `sudo apt install -y fontconfig`
    // Text above username
    const guildMsg = `Welcome to ${config.guild.name},`
    let widthDivisor = 2.5
    let heightDivisor = 3.5
    ctx = applyText(canvas, ctx, guildMsg, 28, '#ffffff', 'sans-serif', widthDivisor, heightDivisor, 300);

    // Username
    widthDivisor = 2.5
    heightDivisor = 1.5
    ctx = applyText(canvas, ctx, member.displayName, 70, '#ffffff', 'sans-serif', widthDivisor, heightDivisor, 300);
    
    // Add avatar circle
    ctx.beginPath();
    ctx.arc(125, 125, 100, 0, Math.PI * 2, true);
    ctx.closePath();
    // Clip off the region you drew on
    ctx.clip();

    const avatar = await Canvas.loadImage(member.user.displayAvatarURL({ format: 'jpg' }));
    ctx.drawImage(avatar, 25, 25, 200, 200);

    return new MessageAttachment(canvas.toBuffer(), 'image.png');
}

module.exports = {joinCanvasMaker, joinInstructionMaker}