module.exports = {
	extends: "infernal",
	parserOptions: {
		project: "./tsconfig.json",
	},
	parser: "@typescript-eslint/parser",
	plugins: ["@typescript-eslint"],
	root: true,
	// ignore javascript files
	ignorePatterns: ["*.js"],
};
