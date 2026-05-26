# JRV Language Specification — Formal EBNF Grammar (Rev 8.0)

> Project JARVIS — Total Substrate Purity
> Language version: 8.0.0 | EBNF: ISO/IEC 14977

## 1. Lexical Structure

```
letter           = 'A'..'Z' | 'a'..'z' ;
digit            = '0'..'9' ;
hex_digit        = digit | 'A'..'F' | 'a'..'f' ;
underscore       = '_' ;
identifier_char  = letter | digit | underscore ;
identifier       = (letter | underscore), { identifier_char } ;
integer_literal  = digit, { digit } | '0x', hex_digit, { hex_digit } ;
float_literal    = digit, { digit }, '.', digit, { digit } ;
number_literal   = integer_literal | float_literal ;
string_literal   = '"', { character }, '"' ;
big_o            = 'O', '(', identifier_char, { identifier_char }, ')' ;
comment          = '//', { character }, newline ;
```

## 2. Module Structure

```
program          = { import_decl }, module_declaration ;
import_decl      = 'import', ( string_literal | identifier ) ;
module_decl      = 'module', identifier, complexity_block ;

complexity_block = 'complexity', big_o, '{', { module_item }, '}' ;
module_item      = budget_decl | struct_decl | static_decl 
                 | func_decl | render_decl | allocator_decl 
                 | hologram_decl | post_process_decl | neuro_adapt_decl ;
```

## 3. Declarations

```
budget_decl      = 'budget', '{', 'power', ':', number_literal, { statement }, '}' ;
struct_decl      = 'struct', identifier, '{', field, { ',', field }, '}' ;
field            = identifier, ':', type_spec ;
static_decl      = 'static', identifier, '[', address, ']', '{', 'size', ':', integer_literal, '}' ;
address          = integer_literal ;

func_decl        = attribute, { attribute }, 'func', identifier, '(', [ param_list ], ')', [ '->', type_spec ], '{', { statement }, '}' ;
param_list       = param, { ',', param } ;
param            = identifier, ':', type_spec ;
attribute        = '@interrupt', '(', identifier, ')' | '@no_mangle' | '@section', '(', string_literal, ')' ;

render_decl      = 'render', identifier, '(', [ param_list ], ')', '{', { node_item }, '}' ;
node_item        = layout_block | component_stmt ;

allocator_decl   = 'allocator', identifier, '{', { statement }, '}' ;
hologram_decl    = 'hologram', identifier, '{', { statement }, '}' ;
post_process_decl = 'post_process', identifier, '{', { statement }, '}' ;
neuro_adapt_decl = 'neuro_adapt', identifier, '{', { statement }, '}' ;
```

## 4. Statement Grammar

```
statement        = let_stmt | return_stmt | if_stmt | while_stmt | for_stmt 
                 | memory_stmt | evolve_stmt | budget_stmt | prob_block 
                 | sync_block | gossip_stmt | contract_block | knowledge_stmt 
                 | publish_stmt | window_stmt | event_block | assert_stmt 
                 | layout_stmt | component_stmt | poll_stmt | print_stmt 
                 | capture_frame_stmt | capture_stream_stmt | asm_block 
                 | volatile_op | port_op | atomic_op | hologram_stmt 
                 | post_process_stmt | neuro_adapt_stmt | expression_stmt ;

let_stmt         = 'let', identifier, [ ':', type_spec ], '=', expression, ';' ;
return_stmt      = 'return', [ expression ], ';' ;
if_stmt          = 'if', expression, '{', { statement }, '}', [ 'else', '{', { statement }, '}' ] ;
while_stmt       = 'while', expression, '{', { statement }, '}' ;
for_stmt         = 'for', identifier, 'in', expression, '{', { statement }, '}' ;

expression_stmt  = expression, ';' ;

memory_stmt      = 'memory', identifier, ':', type_spec, [ '[', integer_literal, ']' ] ;
evolve_stmt      = 'evolve', '{', { statement }, '}' ;

prob_block       = 'prob', '{', { number_literal, '=>', ( '{', { statement }, '}' | statement ) }, '}' ;
sync_block       = 'sync', [ '(', 'protocol', ':', identifier, ')' ], '{', { statement }, '}' ;
gossip_stmt      = 'gossip', '(', identifier, ')', ';' ;
contract_block   = 'contract', '{', text, '}' ;
knowledge_stmt   = 'knowledge', identifier, ':', 'Vector', '[', integer_literal, ']' ;
publish_stmt     = 'publish', '(', identifier, ')', ';' ;

window_stmt      = 'window', string_literal, '[', integer_literal, ',', integer_literal, ']', ';' ;
event_block      = 'event', identifier, '{', { statement }, '}' ;
assert_stmt      = 'assert', '(', expression, ')', ';' ;

layout_stmt      = 'layout', identifier, '{', { statement }, '}' ;
component_stmt   = 'component', identifier, '(', [ expression, { ',', expression } ], ')', ';' ;
poll_stmt        = 'poll', ';' ;
print_stmt       = 'print', '(', expression, ',', expression, ',', expression, ',', expression, ')', ';' ;
capture_frame_stmt = 'capture_frame', ';' ;
capture_stream_stmt = 'capture_stream', ';' ;

asm_block        = 'asm', '{', text, '}' ;
volatile_op      = 'volatile', ( 'write', '(', expression, ',', expression, ')' | 'read', '(', expression, ')' '->', identifier ), ';' ;
port_op          = 'port', ( 'write', '(', expression, ',', expression, ')' | 'read', '(', expression, ')' '->', identifier ), ';' ;
atomic_op        = 'atomic', identifier, '(', [ expression, { ',', expression } ], ')', ';' ;

hologram_stmt    = 'hologram', expression, ';' ;
post_process_stmt = 'post_process', expression, ';' ;
neuro_adapt_stmt = 'neuro_adapt', expression, ';' ;
```

## 5. Expression Grammar (Operator Precedence)

```
expression       = assignment_expr ;

assignment_expr  = identifier, '=', expression | logical_or_expr ;
logical_or_expr  = logical_and_expr, { '||', logical_and_expr } ;
logical_and_expr = comparison_expr, { '&&', comparison_expr } ;
comparison_expr  = additive_expr, { ('==' | '<' | '>' | '<=' | '>='), additive_expr } ;
additive_expr    = multiplicative_expr, { ('+' | '-'), multiplicative_expr } ;
multiplicative_expr = unary_expr, { ('*' | '/'), unary_expr } ;
unary_expr       = '!', unary_expr | '-' unary_expr | postfix_expr ;
postfix_expr     = primary_expr, { '(', [ expression, { ',', expression } ], ')' | '.', identifier } ;
primary_expr     = number_literal | string_literal | identifier 
                 | 'input', [ '(', ')' ] 
                 | '(', expression, ')' ;
```

## 6. Type System

```
type_spec        = 'i32' | 'I32' | 'f32' | 'F32' | 'Stream' | 'PixelStream' 
                 | 'FrameBuffer' | 'VectorCanvas' | identifier ;

builtin_types    = i32 | f32 | Stream | PixelStream | FrameBuffer | VectorCanvas ;
stream_type      = Stream | PixelStream ;
```

## 7. Verify Blocks (eTDD)

```
verify_block     = 'verify', '{', { test_decl }, '}' ;
test_decl        = 'test', string_literal, '{', { statement }, '}' ;
```

## 8. Core Standard Library (Built-in Functions)

```
builtin_functions = 
    // I/O
    'print(i32, i32, i32, i32)'     // Print string at (x,y) with color
    | 'input() -> i32'               // Read keyboard input
    
    // Stream operations
    | 'stream_read(Stream) -> i32'   // Read from stream
    | 'stream_write(Stream, i32)'    // Write to stream
    
    // Memory operations
    | 'mem_read(i32) -> i32'         // Read from address
    | 'mem_write(i32, i32)'          // Write to address
    
    // Type conversion
    | 'i32_to_f32(i32) -> f32'       // Convert i32 to f32
    | 'f32_to_i32(f32) -> i32'       // Convert f32 to i32
    
    // Crypto
    | 'sha256(Stream) -> Stream'     // SHA-256 hash
    
    // Math
    | 'sin(f32) -> f32'
    | 'cos(f32) -> f32' ;
```

## 9. Semantics

### Complexity Verification
Every function MUST have a parent `complexity` block declaring its Big-O bound.
The compiler SHALL statically verify that loops and recursions match the declared bound.

### Energy Budget
Every `budget` block declares a nanojoule (nJ) limit.
The compiler SHALL verify that the contained operations' cumulative energy does not exceed the limit.

### Memory Safety
- `i32` and `f32` types are pass-by-value on stack
- `Stream` types are pass-by-reference (linear ownership)
- No implicit conversions between types
- Struct fields are accessed with `.` notation

### Determinism
- All operations are deterministically ordered
- No race conditions by design (single-threaded execution)
- Stream operations are lock-free (atomic ring buffers)
- Every state transition is SHA-256 identified