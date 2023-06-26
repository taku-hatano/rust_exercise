const { NodeLexer, NodeParser } = require("../../pkg");
const readline = require("readline");

const evaluate = (ast) => {
	if ("Number" in ast) {
		return ast.Number;
	}
	if ("PrefixExpression" in ast) {
		const { operator, right } = ast.PrefixExpression;
		if (operator === "Minus") {
			return -evaluate(right);
		} else if (operator === "Plus") {
			return evaluate(right);
		} else {
			throw new Error(`unknown operator: ${operator}`);
		}
	}
	if ("InfixExpression" in ast) {
		const { operator, left, right } = ast.InfixExpression;
		if (operator === "Plus") {
			return evaluate(left) + evaluate(right);
		} else if (operator === "Minus") {
			return evaluate(left) - evaluate(right);
		} else if (operator === "Asterisk") {
			return evaluate(left) * evaluate(right);
		} else if (operator === "Slash") {
			return evaluate(left) / evaluate(right);
		} else {
			throw new Error(`unknown operator: ${operator}`);
		}
	}
};

const prompt = async (input) => {
	const rl = readline.createInterface({
		input: process.stdin,
		output: process.stdout,
	});
	return new Promise((resolve) => {
		rl.question(input, (answer) => {
			resolve(answer);
			rl.close();
		});
	});
};

const main = async () => {
	const input = await prompt("[nodejs]>> ");
	if (input === "exit") {
		return;
	}

	const lexer = new NodeLexer(input);
	let token = lexer.token();
	console.log("**** Lexer Result ****");
	while (token) {
		console.log(token);
		token = lexer.token();
	}
	const parser = new NodeParser(input);
	const ast = parser.parse();
	console.log("");
	console.log("**** Evaluate Result ****");
	console.log(evaluate(ast));

	await main();
};

main();