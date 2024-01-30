import { defineConfig, loadEnv } from "vite";
import elmPlugin from "vite-plugin-elm";

export default ({ mode }) => {
    process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };
    const config = {
        plugins: [
            elmPlugin({
              debug: true,
            }),
          ],
          css: {
            postcss: './postcss.config.js'
          },
          server: {
            proxy: {
                '/api': {
                    target: 'http://localhost:8080',
                    changeOrigin: true,
                    secure: false
                }
            }
          }
    }
    return defineConfig(config)
}
