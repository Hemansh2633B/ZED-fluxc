/// Syntax highlighting query for Flux
pub const HIGHLIGHTS: &str = r#"
; Keywords
(keyword) @keyword

; Type keywords
(type_keyword) @type

; Storage class keywords
(storage_class) @storageclass

; Constants
(constant) @constant

; Built-in types
(builtin_type) @type.builtin

; Functions
(function_definition
  name: (identifier) @function)

(function_call
  function: (identifier) @function.call)

; Methods
(method_definition
  name: (identifier) @function.method)

; Variables
(variable_declaration
  name: (identifier) @variable)

; Constants
(constant_declaration
  name: (identifier) @constant)

; Parameters
(parameter
  name: (identifier) @parameter)

; Fields
(field_declaration
  name: (identifier) @property)

; Types
(type_identifier) @type

; Structs
(struct_definition
  name: (type_identifier) @type.struct)

; Enums
(enum_definition
  name: (type_identifier) @type.enum)

; Contracts
(contract_definition
  name: (type_identifier) @type.contract)

; Templates
(template_definition
  name: (type_identifier) @type.template)

; Operators
(operator_definition
  name: (operator_name) @operator)

; Operators (custom infix)
(infix_operator) @operator

; Numbers
(number_literal) @number

; Integer literals with width suffix
(width_integer_literal) @number

; Float literals
(float_literal) @number.float

; Binary literals
(binary_literal) @number.binary

; Hex literals
(hex_literal) @number.hex

; Bit literals
(bit_literal) @number.bit

; Strings
(string_literal) @string

; Raw strings
(raw_string_literal) @string

; Characters
(char_literal) @character

; Comments
(comment) @comment

; Documentation comments
(doc_comment) @comment.documentation

; Preprocessor directives
(preproc_def) @keyword.directive
(preproc_if) @keyword.directive
(preproc_else) @keyword.directive
(preproc_endif) @keyword.directive

; Punctuation
(punctuation) @punctuation

; Brackets
( "(" ) @punctuation.bracket
( ")" ) @punctuation.bracket
( "[" ) @punctuation.bracket
( "]" ) @punctuation.bracket
( "{" ) @punctuation.bracket
( "}" ) @punctuation.bracket

; Operators
(operator) @operator

; Assignment
(assignment_operator) @operator

; Comparison
(comparison_operator) @operator

; Arithmetic
(arithmetic_operator) @operator

; Bitwise
(bitwise_operator) @operator

; Logical
(logical_operator) @operator

; Attributes
(attribute) @attribute

; Macros
(macro_invocation) @function.macro

; Comptime blocks
(comptime_block) @keyword.comptime

; Emitflux blocks
(emitflux_block) @keyword.emitflux

; Fluxvm blocks
(fluxvm_block) @keyword.fluxvm

; Inline assembly
(inline_asm) @keyword.asm

; From expressions (zero-copy reinterpretation)
(from_expression) @keyword.from

; Cast expressions
(cast_expression) @keyword.cast

; Bit slice expressions
(bit_slice_expression) @keyword.bitslice

; Tied/move marker
(tied_marker) @keyword.move

; Data type declarations
(data_type_declaration) @type.data

; Width specifications
(width_specification) @number.width

; Alignment specifications
(alignment_specification) @number.alignment

; Endianness specifications
(endianness_specification) @keyword.endianness

; Labels
(label) @label

; Goto statements
(goto_statement) @keyword.goto
"#;

/// Indentation query for Flux
pub const INDENTS: &str = r#"
; Increase indent after these constructs
[
  (function_definition)
  (struct_definition)
  (enum_definition)
  (contract_definition)
  (template_definition)
  (comptime_block)
  (emitflux_block)
  (fluxvm_block)
  (if_statement)
  (while_statement)
  (for_statement)
  (block)
] @indent.begin

; Decrease indent before these constructs
[
  ( "}" )
  ( "]" )
  ( ")" )
] @indent.end

; Indent branches
(else_clause) @indent.branch
(else_if_clause) @indent.branch
(catch_clause) @indent.branch

; No indent for these
(identifier) @indent.none
(number_literal) @indent.none
(string_literal) @indent.none
"#;

/// Folding query for Flux
pub const FOLDS: &str = r#"
; Function definitions
(function_definition) @fold

; Struct definitions
(struct_definition) @fold

; Enum definitions
(enum_definition) @fold

; Contract definitions
(contract_definition) @fold

; Template definitions
(template_definition) @fold

; Block statements
(block) @fold

; Comptime blocks
(comptime_block) @fold

; Emitflux blocks
(emitflux_block) @fold

; Fluxvm blocks
(fluxvm_block) @fold

; If statements
(if_statement) @fold

; While loops
(while_statement) @fold

; For loops
(for_statement) @fold

; Match statements
(match_statement) @fold

; Inline assembly
(inline_asm) @fold
"#;

/// Local variable scoping query for Flux
pub const LOCALS: &str = r#"
; Function parameters
(function_definition
  parameters: (parameter_list
    (parameter name: (identifier) @local.definition.parameter)))

; Variable declarations
(variable_declaration
  name: (identifier) @local.definition.var)

; Constant declarations
(constant_declaration
  name: (identifier) @local.definition.constant)

; Struct fields
(struct_definition
  body: (field_declaration_list
    (field_declaration
      name: (identifier) @local.definition.field)))

; Enum variants
(enum_definition
  body: (variant_list
    (variant
      name: (identifier) @local.definition.variant)))

; Loop variables
(for_statement
  initializer: (variable_declaration
    name: (identifier) @local.definition.var))

; Catch clause variables
(catch_clause
  parameter: (parameter
    name: (identifier) @local.definition.var))

; Comptime block variables
(comptime_block
  body: (block
    (variable_declaration
      name: (identifier) @local.definition.var)))

; References to locals
(identifier) @local.reference

; Function calls (to distinguish from variable references)
(function_call
  function: (identifier) @local.reference)

; Method calls
(method_call
  method: (identifier) @local.reference)

; Field access
(field_access
  field: (identifier) @local.reference)
"#;