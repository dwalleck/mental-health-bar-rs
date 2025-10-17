/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        mood: {
          'very-bad': '#EF4444',    // red-500
          'bad': '#F97316',         // orange-500
          'neutral': '#FCD34D',     // amber-300
          'good': '#84CC16',        // lime-500
          'very-good': '#22C55E',   // green-500
        },
        assessment: {
          'minimal': '#10B981',     // emerald-500
          'mild': '#F59E0B',        // amber-500
          'moderate': '#F97316',    // orange-500
          'severe': '#EF4444',      // red-500
        }
      }
    },
  },
  plugins: [],
}
