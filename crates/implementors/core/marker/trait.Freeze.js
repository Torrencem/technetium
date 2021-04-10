(function() {var implementors = {};
implementors["compile"] = [{"text":"impl Freeze for CompileContext","synthetic":true,"types":[]},{"text":"impl Freeze for CompileManager","synthetic":true,"types":[]},{"text":"impl Freeze for NameLookupResult","synthetic":true,"types":[]},{"text":"impl Freeze for ExprParser","synthetic":true,"types":[]},{"text":"impl Freeze for ProgramParser","synthetic":true,"types":[]},{"text":"impl Freeze for CompileError","synthetic":true,"types":[]},{"text":"impl Freeze for CompileErrorType","synthetic":true,"types":[]},{"text":"impl Freeze for ListLiteral","synthetic":true,"types":[]},{"text":"impl Freeze for TupleLiteral","synthetic":true,"types":[]},{"text":"impl Freeze for SetLiteral","synthetic":true,"types":[]},{"text":"impl Freeze for DictLiteral","synthetic":true,"types":[]},{"text":"impl Freeze for FuncCall","synthetic":true,"types":[]},{"text":"impl Freeze for AttrLookup","synthetic":true,"types":[]},{"text":"impl Freeze for MethodCall","synthetic":true,"types":[]},{"text":"impl Freeze for IndexedExpr","synthetic":true,"types":[]},{"text":"impl Freeze for SlicedExpr","synthetic":true,"types":[]},{"text":"impl Freeze for PostPreOp","synthetic":true,"types":[]},{"text":"impl Freeze for AnonFuncDefinition","synthetic":true,"types":[]},{"text":"impl Freeze for ForLoop","synthetic":true,"types":[]},{"text":"impl Freeze for WhileLoop","synthetic":true,"types":[]},{"text":"impl Freeze for IfStatement","synthetic":true,"types":[]},{"text":"impl Freeze for CaseOf","synthetic":true,"types":[]},{"text":"impl Freeze for FuncDefinition","synthetic":true,"types":[]},{"text":"impl Freeze for ReturnStatement","synthetic":true,"types":[]},{"text":"impl Freeze for ShStatement","synthetic":true,"types":[]},{"text":"impl Freeze for FormatString","synthetic":true,"types":[]},{"text":"impl Freeze for Assignment","synthetic":true,"types":[]},{"text":"impl Freeze for StatementList","synthetic":true,"types":[]},{"text":"impl Freeze for Identifier","synthetic":true,"types":[]},{"text":"impl Freeze for Literal","synthetic":true,"types":[]},{"text":"impl Freeze for Expr","synthetic":true,"types":[]},{"text":"impl Freeze for PPOVariant","synthetic":true,"types":[]},{"text":"impl Freeze for IfTail","synthetic":true,"types":[]},{"text":"impl Freeze for AssignmentLHS","synthetic":true,"types":[]},{"text":"impl Freeze for Statement","synthetic":true,"types":[]}];
implementors["lexer"] = [{"text":"impl&lt;'input&gt; Freeze for Lexer&lt;'input&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for Tok","synthetic":true,"types":[]},{"text":"impl Freeze for LexError","synthetic":true,"types":[]},{"text":"impl Freeze for MiscParseError","synthetic":true,"types":[]}];
implementors["mlrefcell"] = [{"text":"impl Freeze for AlreadyLockedError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !Freeze for MLRefCell&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for BorrowMutError","synthetic":true,"types":[]}];
implementors["runtime"] = [{"text":"impl Freeze for ObjectRef","synthetic":true,"types":[]},{"text":"impl Freeze for HashableObjectRef","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for ObjectCell&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for RuntimeContext&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for FrameIdGen","synthetic":true,"types":[]},{"text":"impl Freeze for FRAME_ID_GEN","synthetic":true,"types":[]},{"text":"impl Freeze for DebugSymbol","synthetic":true,"types":[]},{"text":"impl Freeze for GlobalContext","synthetic":true,"types":[]},{"text":"impl&lt;'code&gt; Freeze for Frame&lt;'code&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for Op","synthetic":true,"types":[]},{"text":"impl Freeze for RuntimeError","synthetic":true,"types":[]},{"text":"impl Freeze for RuntimeErrorType","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for MemoryBacking&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for MemoryManager","synthetic":true,"types":[]},{"text":"impl Freeze for String_","synthetic":true,"types":[]},{"text":"impl Freeze for Bool","synthetic":true,"types":[]},{"text":"impl Freeze for Int","synthetic":true,"types":[]},{"text":"impl Freeze for Float","synthetic":true,"types":[]},{"text":"impl Freeze for Char","synthetic":true,"types":[]},{"text":"impl Freeze for List_","synthetic":true,"types":[]},{"text":"impl Freeze for Set_","synthetic":true,"types":[]},{"text":"impl Freeze for Dict_","synthetic":true,"types":[]},{"text":"impl Freeze for Sin","synthetic":true,"types":[]},{"text":"impl Freeze for Cos","synthetic":true,"types":[]},{"text":"impl Freeze for Tan","synthetic":true,"types":[]},{"text":"impl Freeze for Abs","synthetic":true,"types":[]},{"text":"impl Freeze for Sqrt","synthetic":true,"types":[]},{"text":"impl Freeze for Exp","synthetic":true,"types":[]},{"text":"impl Freeze for Ln","synthetic":true,"types":[]},{"text":"impl Freeze for Arcsin","synthetic":true,"types":[]},{"text":"impl Freeze for Arccos","synthetic":true,"types":[]},{"text":"impl Freeze for Arctan","synthetic":true,"types":[]},{"text":"impl Freeze for ShObject","synthetic":true,"types":[]},{"text":"impl Freeze for Sh","synthetic":true,"types":[]},{"text":"impl Freeze for Cd","synthetic":true,"types":[]},{"text":"impl Freeze for Open","synthetic":true,"types":[]},{"text":"impl Freeze for Os","synthetic":true,"types":[]},{"text":"impl Freeze for LinuxDistro","synthetic":true,"types":[]},{"text":"impl Freeze for Args","synthetic":true,"types":[]},{"text":"impl Freeze for Which","synthetic":true,"types":[]},{"text":"impl Freeze for Exists","synthetic":true,"types":[]},{"text":"impl Freeze for IsDirectory","synthetic":true,"types":[]},{"text":"impl Freeze for Canonicalize","synthetic":true,"types":[]},{"text":"impl Freeze for Hostname","synthetic":true,"types":[]},{"text":"impl Freeze for Devicename","synthetic":true,"types":[]},{"text":"impl Freeze for Realname","synthetic":true,"types":[]},{"text":"impl Freeze for Username","synthetic":true,"types":[]},{"text":"impl Freeze for Langs","synthetic":true,"types":[]},{"text":"impl Freeze for DesktopEnv","synthetic":true,"types":[]},{"text":"impl Freeze for ShObjectState","synthetic":true,"types":[]},{"text":"impl Freeze for Print","synthetic":true,"types":[]},{"text":"impl Freeze for Printr","synthetic":true,"types":[]},{"text":"impl Freeze for Println","synthetic":true,"types":[]},{"text":"impl Freeze for Eprint","synthetic":true,"types":[]},{"text":"impl Freeze for Eprintr","synthetic":true,"types":[]},{"text":"impl Freeze for Eprintln","synthetic":true,"types":[]},{"text":"impl Freeze for Exit","synthetic":true,"types":[]},{"text":"impl Freeze for Type","synthetic":true,"types":[]},{"text":"impl Freeze for Hash","synthetic":true,"types":[]},{"text":"impl Freeze for Lock","synthetic":true,"types":[]},{"text":"impl Freeze for Clone_","synthetic":true,"types":[]},{"text":"impl Freeze for Assert","synthetic":true,"types":[]},{"text":"impl Freeze for Version","synthetic":true,"types":[]},{"text":"impl Freeze for Range","synthetic":true,"types":[]},{"text":"impl Freeze for RangeIterator","synthetic":true,"types":[]},{"text":"impl Freeze for RangeFunc","synthetic":true,"types":[]},{"text":"impl Freeze for Stale","synthetic":true,"types":[]},{"text":"impl Freeze for Lines","synthetic":true,"types":[]},{"text":"impl Freeze for LinesIterator","synthetic":true,"types":[]},{"text":"impl Freeze for Chars","synthetic":true,"types":[]},{"text":"impl Freeze for CharsIterator","synthetic":true,"types":[]},{"text":"impl Freeze for Map","synthetic":true,"types":[]},{"text":"impl Freeze for MapFunc","synthetic":true,"types":[]},{"text":"impl Freeze for MapIterator","synthetic":true,"types":[]},{"text":"impl Freeze for Filter","synthetic":true,"types":[]},{"text":"impl Freeze for FilterFunc","synthetic":true,"types":[]},{"text":"impl Freeze for FilterIterator","synthetic":true,"types":[]},{"text":"impl Freeze for BoolObject","synthetic":true,"types":[]},{"text":"impl Freeze for IntObject","synthetic":true,"types":[]},{"text":"impl Freeze for FloatObject","synthetic":true,"types":[]},{"text":"impl Freeze for CharObject","synthetic":true,"types":[]},{"text":"impl Freeze for StringObject","synthetic":true,"types":[]},{"text":"impl !Freeze for Function","synthetic":true,"types":[]},{"text":"impl Freeze for List","synthetic":true,"types":[]},{"text":"impl Freeze for ListIterator","synthetic":true,"types":[]},{"text":"impl Freeze for Slice","synthetic":true,"types":[]},{"text":"impl !Freeze for SliceIterator","synthetic":true,"types":[]},{"text":"impl Freeze for Tuple","synthetic":true,"types":[]},{"text":"impl Freeze for UnitObject","synthetic":true,"types":[]},{"text":"impl Freeze for Set","synthetic":true,"types":[]},{"text":"impl Freeze for Dictionary","synthetic":true,"types":[]}];
implementors["tech"] = [{"text":"impl Freeze for Logger","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()