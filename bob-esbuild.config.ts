import proto from 'rollup-plugin-gproto'

export const config: import('bob-esbuild').BobConfig = {
	tsc: {
		dirs: ['packages/*'],
	},
	verbose: true,
	clean: true,
	plugins: [proto()],
}
