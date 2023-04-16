
module.exports = {
    name: 'warn',
	once: false,
	async execute(args) {
        console.warn(args)
    }
}