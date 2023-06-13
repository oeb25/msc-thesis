---
tags: appendix
---

```{.ungram .noNames}
SourceFile =
  Item*

Item =
  Const
| Fn
| Struct
| TypeInvariant

Name = 'ident' | 'self'

NameRef = 'ident' | 'self'

Const =
  Attrs
  'const' Name ':' Type
  ('=' Expr)?
  ';'

Fn =
  Attrs
 'fn' Name ParamList
 ('->' Type)?
 Condition*
 Decreases?
 (BlockExpr | ';')

Attr =
  'ghost'
| 'pure'

Attrs = Attr*

ParamList =
 '(' Param* ')'

Param = Attrs Name (':' Type)? ','?

Condition =
  Requires
| Ensures

Requires =
  ('requires' | 'req') CommaExpr*
Ensures =
  ('ensures'  | 'ens') CommaExpr*

CommaExpr =
  Expr ','?

Decreases =
  ('decreases' | 'dec') ('_' | Expr)

Struct =
  Attrs
  'struct' Name GenericArgList? '{' StructField* '}'

StructField = Attrs Name ':' Type ','?

TypeInvariant =
  'invariant' NameRef GenericArgList? BlockExpr

BlockExpr =
 '{'
   Stmt*
   Expr?
 '}'

Type =
  NamedType
| Primitive
| Optional
| ListType
| GhostType
| RefType

NamedType =
  Name GenericArgList?

GenericArgList =
  '<' GenericArg* '>'

GenericArg =
  Type ','?

Primitive =
  'int'
| 'bool'

Optional =
  '?' Type

ListType =
  '[' Type ']'

GhostType =
  'ghost' Type

RefType =
  '&' 'mut'? Type

Stmt =
  LetStmt
| Item
| ExprStmt
| AssertStmt
| AssumeStmt

LetStmt =
 'let' Name (':' Type)?
 ('=' Expr)?
 ';'

ExprStmt =
  Expr ';'

AssertStmt =
 'assert' CommaExpr* ';'

AssumeStmt =
 'assume' CommaExpr* ';'

ReturnExpr =
 'return' Expr ';'

Invariant =
 'invariant' CommaExpr*

Expr =
  Literal
| IfExpr
| ReturnExpr
| WhileExpr
| ForExpr
| PrefixExpr
| BinExpr
| BlockExpr
| RangeExpr
| CallExpr
| ListExpr
| IndexExpr
| NotNullExpr
| FieldExpr
| StructExpr
| ParenExpr
| RefExpr
| IdentExpr
| NullExpr
| ResultExpr
| QuantifierExpr

IfExpr =
  'if' Expr BlockExpr
  ('else' IfExprElse)?

IfExprElse =
  IfExpr | BlockExpr

WhileExpr =
  'while' Expr
  Invariant*
  Decreases?
  BlockExpr

ForExpr =
  'for' NameInExpr
  Invariant*
  BlockExpr

PrefixExpr =
  ('-' | '!') Expr

BinExpr =
  Expr
  (
    '||' | '&&'
  | '==' | '!=' | '<=' | '>=' | '<' | '>'
  | '+' | '*' | '-' | '/' | '%' | '<<' | '>>' | '^' | '|' | '&'
  | '=' | '+=' | '/=' | '*=' | '%=' | '>>=' | '<<=' | '-=' | '|=' | '&=' | '^='
  )
  Expr

RangeExpr =
  Expr?
  '..'
  Expr?

CallExpr =
  Expr ArgList

ArgList =
  '(' Arg* ')'

Arg =
  Expr ','?

ListExpr =
  '[' CommaExpr* ']'

IndexExpr =
  base:Expr '[' index:Expr ']'

NotNullExpr =
  Expr '!'

FieldExpr =
  Expr '.' field:NameRef

StructExpr =
  NameRef '{' fields:StructExprField* '}'

StructExprField =
  NameRef ':' Expr ','?

ParenExpr =
  '(' Expr ')'

RefExpr =
  '&' 'mut'? Expr

IdentExpr =
  NameRef

NullExpr = 'null'

ResultExpr = 'result'

QuantifierExpr = Quantifier QuantifierOver Expr

QuantifierOver = ParamList | NameInExpr

NameInExpr = Name 'in' Expr

Quantifier =
  'forall' | 'exists'

Literal =
    'int_number'
  | 'true' | 'false'
```
