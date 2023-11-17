/** @type {import('tailwindcss').Config} */
export const content = ["./src/**/*.{html,rs}", "index.html", "./dist/**/*.{html, css}"];
export const theme = {
	extend: {
		colors: {
			kiggypink: "#ff8ead",
			kiggyred: "#e44342",
			kiggygreen: "#b8cc4b",
		},
		fontFamily: {
			'bubble': ['BUBBLE'],
			'bubbleup': ['BUBBLEUP'],
			'bubblegum': ['BUBBLEGUM']
		}
	},
};
export const plugins = [require("@tailwindcss/typography"), require("@tailwindcss/forms")];
