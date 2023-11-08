/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,rs}', 'index.html','./dist/**/*.{html, css}'],

	theme: {
		extend: {
			colors: {
			kiggypink: '#ff8ead',
			kiggyred: '#e44342',
			kiggygreen: '#b8cc4b'
		}
	},
	},
	plugins: [
		require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
	],
};
