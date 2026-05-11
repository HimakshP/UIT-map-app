/** @type {import('tailwindcss').Config} */
export const content = [
  "./src/**/*.rs",
  "./index.html",
];
export const theme = {
  extend: {
    colors: {
      // High-end dark theme colors
      'campus-navy': '#0f172a',
      'campus-accent': '#38bdf8', // Sky Blue for paths
    },
    backdropBlur: {
      xs: '2px',
    }
  },
};
export const plugins = [
  require('@tailwindcss/forms'),
  require('@tailwindcss/typography'),
];