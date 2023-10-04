/** @type {import('tailwindcss').Config} */
const config = {
  mode: "jit",
  content: ["*.html", "./app/src/{pages,components}/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
}

module.exports = config;
