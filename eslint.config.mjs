import eslint from "@eslint/js"
import tseslint from "typescript-eslint"
import globals from "globals"

export default tseslint.config(
    {
        ignores: [
            "node_modules/",
            "dashboard/public/",
            "dist/",
            "scratch/",
            ".devenv/",
        ],
    },
    eslint.configs.recommended,
    ...tseslint.configs.recommended,
    {
        languageOptions: {
            globals: {
                ...globals.node,
                ...globals.browser,
            },
        },
        rules: {
            "no-undef": "off",
            "no-empty": "off",
            "@typescript-eslint/no-explicit-any": "off",
        },
    }
)
