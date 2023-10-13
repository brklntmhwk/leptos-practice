/** @type {import('tailwindcss').Config} */
const config = {
  mode: "jit",
  content: ["*.html", "./app/src/{components,layouts,pages}/*.rs"],
  theme: {
    extend: {
      animation: {
        toggle: "show ease-in-out duration-300"
      },
      keyframes: {
        show: {
          "100%": { opacity: 1.0, transform: "none" },
        }
      }
    },
  },
  plugins: [],
}

module.exports = config;
