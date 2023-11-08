# Tailwind info

## Start a watcher
`./tailwindcss -i styles/input.css -o styles/output.css --watch`

## Compile and minify your CSS for production
`./tailwindcss -i styles/input.css -o styles/output.css --minify`

If you already had an HTML file before the tailwind build, the link to the css file has been added for you. 

If not, an HTML file has already been created for you with the proper link.

```html
<head>
  <link data-trunk rel="css" href="/styles/output.css"/>
</head>
```

The latest versions of all of the tailwind plugins are already bundled, so all you need to do is require them in the plugins array of your `tailwind.config.js`.

The [@tailwindcss/typography](https://tailwindcss.com/docs/typography-plugin) and [@tailwindcss/forms](https://github.com/tailwindlabs/tailwindcss-forms) plugins have been added. 

More [plugins](https://tailwindcss.com/docs/plugins#official-plugins)


