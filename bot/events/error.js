
module.exports = {
    name: 'error',
	once: false,
	async execute(args) {
        console.error(args)
    }
}