module.exports = {
	"*.{css,scss,md,json,yml,yaml}": "prettier --write",
	"*.m?{ts,js}x?": ["prettier --write", "eslint --fix"],
	"*.rs": (files) => [
		`cargo fmt -- ${files.join(" ")}`,
		"cargo clippy --fix --allow-dirty --allow-staged --",
	],
	"*.tf": "terraform fmt -write=true -diff=true",
};
