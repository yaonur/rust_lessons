/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: ["class"],
    content: ['./src/**/*.{html,js,svelte,ts}', './src/**/*.stories.svelte'],
    theme: {
        extend: {
            screens: {
                xl: {max: "1279px"},
                // => @media (max-width: 1279px) { ... }

                lg: {max: "1023px"},
                // => @media (max-width: 1023px) { ... }

                md: {max: "767px"},
                // => @media (max-width: 767px) { ... }

                sm: {max: "639px"},
                // => @media (max-width: 639px) { ... }
            },
            colors: {
                border: "hsl(var(--border))",
                input: "hsl(var(--input))",
                ring: "hsl(var(--ring))",
                background: "hsl(var(--background))",
                foreground: "hsl(var(--foreground))",
                primary: {
                    '200': '#FF8EA8',
                    '300': '#FF5E86',
                    '400': '#FF376B',
                    '500': '#FF004F',
                    '600': '#EE0050',
                    '700': '#D9004D',
                    '800': '#C5004B',
                    'DEFAULT': '#FF004F',
                },
                secondary: {
                    '200': '#7FD7F8',
                    '300': '#43BDF1',
                    '400': '#00A1EC',
                    '500': '#00A1EC',
                    '600': '#0094DE',
                    '700': '#0081CA',
                    '800': '#0070B6',
                    'DEFAULT': '#43BDF1',
                },
                tertiary: {
                    '50': '#FAFAFA',
                    '100': '#F5F5F5',
                    '200': '#EEEEEE',
                    '300': '#E0E0E0',
                    '400': '#BDBDBD',
                    '500': '#9E9E9E',
                    '600': '#757575',
                    '700': '#616161',
                    '800': '#424242',
                    'DEFAULT': '#9E9E9E',
                },
                destructive: {
                    DEFAULT: "hsl(var(--destructive))",
                    foreground: "hsl(var(--destructive-foreground))"
                },
                muted: {
                    DEFAULT: "hsl(var(--muted))",
                    foreground: "hsl(var(--muted-foreground))"
                },
                accent: {
                    DEFAULT: "hsl(var(--accent))",
                    foreground: "hsl(var(--accent-foreground))"
                },
                popover: {
                    DEFAULT: "hsl(var(--popover))",
                    foreground: "hsl(var(--popover-foreground))"
                },
                card: {
                    DEFAULT: "hsl(var(--card))",
                    foreground: "hsl(var(--card-foreground))"
                }
            },
            borderRadius: {
                lg: "var(--radius)",
                md: "calc(var(--radius) - 2px)",
                sm: "calc(var(--radius) - 4px)"
            },
        }
    },
    plugins: [require("tailwindcss-animate")]
};
