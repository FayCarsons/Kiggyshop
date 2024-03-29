/** @type {import('tailwindcss').Config} */
export const content = ["./src/**/*.elm"];
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
export const plugins = [];

