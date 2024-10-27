import infernal from "eslint-config-infernal";

export default [
	...infernal,
	{
		ignores: ["packages/eslint-config-infernal/*", "packages/prettier-config-infernal/*"],
	},
	{
		languageOptions: {
			parserOptions: {
				projectService: true,
				tsconfigRootDir: import.meta.dirname,
			},
		},
	},
];
