initSidebarItems({"enum":[["AssignmentLHS","A type of expression that can be on the left of an assignment: either a variable, an identifier, or an indexed expr"],["Expr",""],["IfTail","A possible additional clause on an if statement"],["Literal","Any kind of literal written in code. Each variant represents a different kind of literal"],["PPOVariant",""],["Statement",""]],"struct":[["Assignment",""],["AttrLookup","An attribute expression without a function call (i.e., not a method call): `person.name`"],["CaseOf",""],["DictLiteral","A literal dictionary given in code: `{\"a\": true, \"b\": 123}`"],["ForLoop",""],["FormatString","A string preceded by `~`, used to substitute variables into a string"],["FuncCall","A function call, not attached to a particular parent (i.e., not a method call): `print(\"123\")`"],["FuncDefinition",""],["Identifier","A name given as part of an expression: `print`"],["IfStatement",""],["IndexedExpr","An expression indexed with square brackets: `my_list[i + 2]`"],["ListLiteral","A list of expressions surrounded by square brackets in code: `[1, 2, \"hello\"]`"],["MethodCall","A method call on an expression: `[1, 2].length()`"],["PostPreOp","Some kind of post/pre-increment/decrement attached to a expr: `x++` or `--a[0]`"],["ReturnStatement",""],["SetLiteral","A list of expressions surrounded by brackets in code: `{1, 2, \"hello\"}`"],["ShStatement","A line beginning in `$` to make a shell statement"],["SlicedExpr","An expression with a slice attached to it: `hello[1:3]` or `hello[10::-1]`"],["StatementList",""],["TupleLiteral","A list of expressions surrounded by parenthesis in code: `(1, 2, \"hello\")`"],["WhileLoop",""]],"trait":[["AstExpr","Common functionality for AST expression nodes"]]});