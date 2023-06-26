from rust_exercise import PyLexer, PyParser, Operator, NumberExpression, PrefixExpression, InfixExpression

def evaluate(expr):
    if isinstance(expr, NumberExpression):
        return expr.val
    elif isinstance(expr, PrefixExpression):
        operator = expr.operator
        right = evaluate(expr.right)
        if operator == Operator.Minus:
            return -right
    elif isinstance(expr, InfixExpression):
        operator = expr.operator
        left = evaluate(expr.left)
        right = evaluate(expr.right)
        if operator == Operator.Plus:
            return left + right
        elif operator == Operator.Minus:
            return left - right
        elif operator == Operator.Asterisk:
            return left * right
        elif operator == Operator.Slash:
            return left / right


def main():
    i = input("[python]>> ")
    if i == "exit":
        return
    lexer = PyLexer(i)
    token = lexer.token()
    print("**** Lexer Result ****")
    while token != None:
        print(token)
        token = lexer.token()
    parser = PyParser(i)
    expr = parser.parse()
    print("")
    print("**** Evaluate Result ****")
    print(evaluate(expr))

    main()

main()
