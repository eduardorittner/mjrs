# Mini-Java rs

Mini java compiler written in rust


# Grammar Specification

The syntax of MiniJava in Backus-Naur Form.

The MiniJava grammar was adapted from this [Reference Manual](https://www.cs.tufts.edu/~sguyer/classes/comp181-2006/minijava.html).

```
<program> ::= {<class_declaration>}+
<class_declaration> ::= "class" <identifier> {<extends_expression>}? "{" {<compound_declaration>}* {<method_declaration>}* "}"
<extends_expression> ::= "extends" <id>
<method_declaration> ::= <regular_method_declaration>
                       | <main_method_declaration>

regular_method_declaration ::= "public" <type_specifier> <identifier> "(" <parameter_list>? ")" <compound_statement>
<main_method_declaration> ::= "public" "static" "void" "main" "(" "String" "[" "]" <identifier> ")" <compound_statement>
<compound_declaration> ::= <type_specifier> <init_declarator_list> ";"
<init_declarator_list> ::= <init_declarator>
                         | <init_declarator_list> "," <init_declarator>
<init_declarator> ::= <declarator>
                    | <declarator> "=" <initializer>
<initializer> ::= <assignment_expression>
                | "{" {<initializer_list>}? "}"
                | "{" <initializer_list> "," "}"
<initializer_list> ::= <initializer>
                     | <initializer_list> "," <initializer>
<declarator> ::= <identifier>
               | "(" <declarator> ")"
<parameter_declaration> ::= <type_specifier> <declarator>
<parameter_list> ::= <parameter_declaration>
                   | <parameter_list> "," <parameter_declaration>
<type_specifier> ::= "void"
                   | "boolean"
                   | "char"
                   | "int"
                   | "String"
                   | "char" "[" "]"
                   | "int" "[" "]"
                   | <identifier>

<expression> ::= <assignment_expression>
               | <expression> "," <assignment_expression>

<assignment_expression> ::= <binary_expression>
                          | <unary_expression> "=" <assignment_expression>
<binary_expression> ::= <unary_expression>
                      | <binary_expression> "*" <binary_expression>
                      | <binary_expression> "/" <binary_expression>
                      | <binary_expression> "%" <binary_expression>
                      | <binary_expression> "+" <binary_expression>
                      | <binary_expression> "-" <binary_expression>
                      | <binary_expression> "<" <binary_expression>
                      | <binary_expression> "<=" <binary_expression>
                      | <binary_expression> ">" <binary_expression>
                      | <binary_expression> ">=" <binary_expression>
                      | <binary_expression> "==" <binary_expression>
                      | <binary_expression> "!=" <binary_expression>
                      | <binary_expression> "&&" <binary_expression>
                      | <binary_expression> "||" <binary_expression>
<unary_expression> ::= <postfix_expression>
                     | <unary_operator> <unary_expression>
<unary_operator> ::= "+"
                   | "-"
                   | "!"
<postfix_expression> ::= <primary_expression>
                       | <postfix_expression> "." "length"
                       | <postfix_expression> "." <identifier>
                       | <postfix_expression> "." <identifier> "(" {<argument_expression>}? ")"
                       | <postfix_expression> "[" <expression> "]"
<primary_expression> ::= <identifier>
                       | <constant>
                       | <this_expression>
                       | <new_expression>
                       | "(" <expression> ")"
<argument_expression> ::= <assignment_expression>
                        | <argument_expression> "," <assignment_expression>
<constant> ::= <boolean_literal>
             | <CHAR_LITERAL>
             | <INT_LITERAL>
             | <STRING_LITERAL>
<this_expression> ::= "this"
<new_expression> ::= "new" "char" "[" <expression> "]"
                   | "new" "int" "[" <expression> "]"
                   | "new" <identifier> "(" ")"
<boolean_literal> ::= "true"
                    | "false"
<identifier> ::= <ID>
<statement> ::= <compound_statement>
              | <expression_statement>
              | <if_statement>
              | <while_statement>
              | <for_statement>
              | <assert_statement>
              | <print_statement>
              | <jump_statement>
<compound_statement> ::= "{" {<compound_declaration>}* {<statement>}* "}"
<expression_statement> ::= <expression> ";"
<if_statement> ::= "if" "(" <expression> ")" <statement>
                 | "if" "(" <expression> ")" <statement> "else" <statement>
<while_statement> ::= "while" "(" <expression> ")" <statement>
<for_statement> ::= "for" "(" {<expression>}? ";" {<expression>}? ";" {<expression>}? ")" <statement>
                  | "for" "(" <compound_declaration> {<expression>}? ";" {<expression>}? ")" <statement>
<assert_statement> ::= "assert" <expression> ";"
<print_statement> ::= "print" "(" {<expression>}? ")" ";"
<jump_statement> ::= "break" ";"
                   | "return" {<expression>}? ";"
```
