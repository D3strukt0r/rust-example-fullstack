import daisyui from 'daisyui'

/** @type {import('tailwindcss').Config} */
export default {
  mode: 'all',
  content: [
    // include all rust, html and css files in the src directory
    './frontend/src/**/*.{rs,html,css}',
    // include all html files in the output (dist) directory
    './frontend/dist/**/*.html',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    daisyui,
  ],
  daisyui: {
    themes: [
      'cupcake',
    ],
  },
}

