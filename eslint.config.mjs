import infernal from "eslint-config-infernal";

export default [
	...infernal,
	{
		languageOptions: {
			parserOptions: {
				projectService: true,
				tsconfigRootDir: import.meta.dirname,
			},
		},
	},
];
