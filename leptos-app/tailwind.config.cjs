/** @type {import('tailwindcss').Config} */
const config = {
  mode: "jit",
  content: ["*.html", "./app/src/{components,layouts,pages}/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
}

module.exports = config;
