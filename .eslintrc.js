module.exports = {
	extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
	parserOptions: {
		project: "./tsconfig.json",
	},
	parser: "@typescript-eslint/parser",
	plugins: ["@typescript-eslint"],
	root: true,
	// ignore javascript files
	ignorePatterns: ["*.js"],
};
