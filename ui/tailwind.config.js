module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
        container: {
            center: true,
        },
    },
    variants: {},
    plugins: [
        require('@tailwindcss/forms'),
    ],
};
