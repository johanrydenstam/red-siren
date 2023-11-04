/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./red-siren-lib/src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    colors: {
      red: '#E30022',
      black: '#353839',
      gray: '#36454F',
    }
  },
  plugins: [],
};
