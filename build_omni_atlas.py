#!/usr/bin/env python3
"""
╔══════════════════════════════════════════════════════════════════╗
║  OMNI-ATLAS MEGA GENERATOR v1.0                                 ║
║  Builds the world's first complete programming language LLM DB   ║
║  Target: 700+ real languages, millions of lines of knowledge     ║
╚══════════════════════════════════════════════════════════════════╝
"""
import os, json, datetime

OUT_DIR = "omni_atlas"
os.makedirs(OUT_DIR, exist_ok=True)

# ═══════════════════════════════════════════════════════════════════
#  REAL PROGRAMMING LANGUAGES DATABASE
#  Every language ever created that has historical significance
# ═══════════════════════════════════════════════════════════════════

LANGUAGES = [
    # ── ERA 1: THE DAWN (1940s-1950s) ─────────────────────────────
    {
        "name": "Plankalkül", "year": 1943, "creator": "Konrad Zuse",
        "paradigm": ["imperative", "structured"],
        "typing": "static", "family": "PROTO",
        "description": "The world's first high-level programming language, designed by Konrad Zuse between 1942 and 1945 during World War II. It was never implemented during his lifetime. Plankalkül introduced assignments, subroutines, conditional statements, iteration, floating-point arithmetic, arrays, hierarchical record structures, assertions, exception handling, and other advanced features.",
        "key_concepts": ["Two-dimensional notation", "Subscript indexing", "Plan (program)", "Rechenplan (computation plan)"],
        "syntax_examples": {
            "variable_declaration": "V0 => 4.8.0\n| K | V | S\n| 0 |   | 1.n",
            "assignment": "| Z + 1 => Z\n| V0      V0",
            "conditional": "W0 => R0\nA1 => R1(V0[:m.0])"
        },
        "keywords": ["Plan", "Resultat", "Variationstyp", "Komponententyp"],
        "memory_model": "Stack-based with explicit subscript addressing",
        "error_handling": "Assertion-based guards (Bedingung)",
        "notable_features": ["First language with data structures", "First language with assertions", "Designed before digital computers existed"],
        "hello_world": "R1.1(V0[:sig]) => R0\nR1.2(V0[:sig]) => R0\n(This language predates Hello World convention)",
        "influence_on": ["ALGOL", "All modern languages conceptually"],
        "influenced_by": ["Mathematical logic", "Zuse's Z3 computer"],
        "status": "historical",
        "use_cases": ["Theoretical computing", "Algorithm specification"]
    },
    {
        "name": "Short Code", "year": 1949, "creator": "John Mauchly",
        "paradigm": ["imperative"],
        "typing": "untyped", "family": "PROTO",
        "description": "One of the first high-level languages ever developed, created for the BINAC computer. It was one of the first attempts to create a more human-readable programming language. Programs were encoded as two-character codes representing mathematical operations.",
        "key_concepts": ["Two-character operation codes", "Mathematical expression encoding", "Interpreter-based execution"],
        "syntax_examples": {
            "math_expression": "X3 = (X1 + X2) / X4\nEncoded as: 00 X3 03 20 X1 07 X2 02 04 X4",
            "codes": "01=subtract, 02=end, 03=assign, 04=divide, 06=abs_value, 07=add, 08=pause, 09=multiply"
        },
        "keywords": ["Byte codes: 01-09", "Variable registers: X0-X9"],
        "memory_model": "Register-based with fixed variable slots",
        "error_handling": "None - hardware halt on error",
        "notable_features": ["First language to use mathematical notation", "Ran 50x slower than machine code"],
        "hello_world": "(No string output capability)",
        "influence_on": ["A-0 System", "MATH-MATIC"],
        "influenced_by": ["Mathematical notation", "ENIAC coding"],
        "status": "historical",
        "use_cases": ["Scientific calculation on BINAC"]
    },
    {
        "name": "Autocode", "year": 1952, "creator": "Alick Glennie",
        "paradigm": ["imperative"],
        "typing": "static", "family": "PROTO",
        "description": "The first compiled programming language. Glennie's Autocode was developed for the Manchester Mark 1 computer. Later versions (Brooker's Autocode, Atlas Autocode) were more sophisticated. It simplified the process of programming by allowing more human-readable instructions.",
        "key_concepts": ["Compiled execution", "Simple arithmetic expressions", "Automatic code generation"],
        "syntax_examples": {
            "assignment": "n1 = 1\nn2 = n1 + 1",
            "loop": "CYCLE i = 1, 1, 100\n  n = n + i\nREPEAT",
            "conditional": "IF n1 > n2 THEN JUMP TO 5"
        },
        "keywords": ["CYCLE", "REPEAT", "JUMP", "IF", "THEN", "NEWLINE", "READ", "PRINT"],
        "memory_model": "Fixed memory with named variables",
        "error_handling": "Runtime halt",
        "notable_features": ["First compiled language ever", "Multiple versions for different machines"],
        "hello_world": "PRINT 'HELLO WORLD'\nNEWLINE",
        "influence_on": ["FORTRAN", "ALGOL"],
        "influenced_by": ["Machine code patterns"],
        "status": "historical",
        "use_cases": ["Scientific computing", "University research"]
    },
    {
        "name": "FORTRAN", "year": 1957, "creator": "John Backus (IBM)",
        "paradigm": ["imperative", "structured", "procedural"],
        "typing": "static, strong", "family": "FORTRAN",
        "description": "FORmula TRANslation - the first widely used high-level programming language and the first language with an optimizing compiler. FORTRAN revolutionized programming by allowing scientists and engineers to write programs using mathematical notation rather than machine code. It remains the dominant language for high-performance scientific computing, weather modeling, and computational physics.",
        "key_concepts": ["DO loops", "Formatted I/O", "COMMON blocks", "Subroutines", "Array operations", "Column-based formatting (F77)"],
        "syntax_examples": {
            "hello_world": "      PROGRAM HELLO\n      WRITE(*,*) 'Hello, World!'\n      STOP\n      END",
            "do_loop": "      DO 10 I = 1, 100\n        SUM = SUM + I\n  10  CONTINUE",
            "subroutine": "      SUBROUTINE CALC(X, Y, RESULT)\n      REAL X, Y, RESULT\n      RESULT = X**2 + Y**2\n      RETURN\n      END",
            "array_ops": "      REAL A(100), B(100), C(100)\n      C = A + B  ! Array addition (F90+)",
            "modern_f90": "program modern_example\n  implicit none\n  integer :: i\n  real :: x(10)\n  do i = 1, 10\n    x(i) = real(i) ** 2\n  end do\n  print *, 'Sum = ', sum(x)\nend program"
        },
        "keywords": ["PROGRAM", "SUBROUTINE", "FUNCTION", "INTEGER", "REAL", "DOUBLE PRECISION", "COMPLEX", "CHARACTER", "LOGICAL", "DIMENSION", "COMMON", "EQUIVALENCE", "DATA", "IMPLICIT", "PARAMETER", "DO", "CONTINUE", "IF", "THEN", "ELSE", "ENDIF", "GOTO", "CALL", "RETURN", "WRITE", "READ", "FORMAT", "STOP", "END", "MODULE", "USE", "CONTAINS", "INTERFACE", "TYPE", "ALLOCATABLE", "POINTER", "TARGET", "INTENT", "PURE", "ELEMENTAL", "RECURSIVE", "WHERE", "FORALL", "SELECT CASE", "ASSOCIATE", "BLOCK"],
        "memory_model": "Stack-allocated locals, static COMMON blocks, heap allocation in F90+",
        "error_handling": "ERR= and IOSTAT= on I/O, IEEE exception handling in F2003+",
        "notable_features": ["First optimizing compiler", "Column-sensitive formatting (F77)", "Array slicing", "Intrinsic math functions", "OpenMP support", "Coarray parallelism (F2008)"],
        "hello_world": "program hello\n  print *, 'Hello, World!'\nend program hello",
        "influence_on": ["ALGOL", "BASIC", "PL/I", "MATLAB", "Julia"],
        "influenced_by": ["Speedcoding", "Mathematical notation"],
        "status": "active",
        "use_cases": ["Scientific computing", "Weather prediction", "Computational fluid dynamics", "Nuclear simulations", "Financial modeling", "High-performance computing"]
    },
    {
        "name": "LISP", "year": 1958, "creator": "John McCarthy",
        "paradigm": ["functional", "procedural", "reflective", "meta"],
        "typing": "dynamic, strong", "family": "LISP",
        "description": "LISt Processing - the second-oldest high-level programming language still in use today. Created by John McCarthy at MIT, LISP pioneered many fundamental concepts in computer science including tree data structures, automatic storage management (garbage collection), dynamic typing, conditionals, higher-order functions, recursion, the self-hosting compiler, and the REPL. It is the foundational language of artificial intelligence research.",
        "key_concepts": ["S-expressions", "Cons cells", "Car and Cdr", "Lambda calculus", "Garbage collection", "Homoiconicity (code is data)", "Macros", "REPL", "Dynamic typing"],
        "syntax_examples": {
            "hello_world": "(print \"Hello, World!\")",
            "function_def": "(defun factorial (n)\n  (if (<= n 1)\n      1\n      (* n (factorial (- n 1)))))",
            "list_ops": "(car '(1 2 3))        ; => 1\n(cdr '(1 2 3))        ; => (2 3)\n(cons 0 '(1 2 3))     ; => (0 1 2 3)\n(mapcar #'1+ '(1 2 3)) ; => (2 3 4)",
            "lambda": "(mapcar (lambda (x) (* x x)) '(1 2 3 4 5))\n; => (1 4 9 16 25)",
            "macro": "(defmacro when (condition &body body)\n  `(if ,condition (progn ,@body)))",
            "clos_oop": "(defclass animal ()\n  ((name :initarg :name :accessor animal-name)\n   (sound :initarg :sound :accessor animal-sound)))\n\n(defmethod speak ((a animal))\n  (format t \"~a says ~a!\" (animal-name a) (animal-sound a)))"
        },
        "keywords": ["defun", "defvar", "defparameter", "defmacro", "defclass", "defmethod", "defgeneric", "let", "let*", "lambda", "if", "cond", "when", "unless", "case", "progn", "prog1", "block", "return-from", "loop", "do", "dolist", "dotimes", "mapcar", "mapc", "reduce", "apply", "funcall", "setf", "setq", "car", "cdr", "cons", "list", "append", "format", "print", "read", "eval", "quote", "function", "values", "multiple-value-bind", "handler-case", "handler-bind", "restart-case", "signal", "error", "warn"],
        "memory_model": "Heap-allocated cons cells with automatic garbage collection",
        "error_handling": "Condition system (handler-case, handler-bind, restart-case) - the most powerful error handling system in any language",
        "notable_features": ["Code is data (homoiconicity)", "Macro system", "CLOS (Common Lisp Object System)", "Condition/Restart system", "Multiple return values", "Multiple dispatch", "First language with garbage collection", "Interactive development via REPL"],
        "hello_world": "(format t \"Hello, World!~%\")",
        "influence_on": ["Scheme", "Common Lisp", "Clojure", "Racket", "Emacs Lisp", "Julia", "Python", "Ruby", "JavaScript", "Haskell"],
        "influenced_by": ["Lambda calculus", "IPL", "Mathematical logic"],
        "status": "active",
        "use_cases": ["AI research", "Symbolic computation", "Expert systems", "CAD (AutoCAD/AutoLISP)", "Emacs editor", "Aerospace (NASA)", "Music composition"]
    },
    {
        "name": "ALGOL 58", "year": 1958, "creator": "ACM/GAMM Committee",
        "paradigm": ["imperative", "structured", "procedural"],
        "typing": "static", "family": "ALGOL",
        "description": "ALGOrithmic Language - designed by an international committee, ALGOL became the standard for describing algorithms in academic papers. It introduced the concept of block structure, nested scopes, and formal language definition using BNF (Backus-Naur Form). ALGOL is the ancestor of nearly all modern imperative languages.",
        "key_concepts": ["Block structure", "Lexical scoping", "BNF grammar definition", "Call by name", "Recursion", "Nested functions"],
        "syntax_examples": {
            "hello_world": "BEGIN\n  OUTSTRING(1, 'HELLO, WORLD!');\nEND",
            "procedure": "procedure Abs(a);\n  value a; real a;\nbegin\n  if a < 0 then Abs := -a\n  else Abs := a\nend",
            "block": "begin\n  integer i;\n  for i := 1 step 1 until 10 do\n    outreal(1, i * i)\nend"
        },
        "keywords": ["begin", "end", "if", "then", "else", "for", "do", "while", "procedure", "function", "integer", "real", "Boolean", "array", "switch", "goto", "value", "own", "step", "until", "comment"],
        "memory_model": "Stack frames with block-scoped allocation, own variables for persistence",
        "error_handling": "No formal mechanism - program halt",
        "notable_features": ["First language defined by formal grammar (BNF)", "Block structure that influenced all successors", "Call-by-name semantics", "Used to publish algorithms in journals"],
        "hello_world": "BEGIN\n  OUTPUT(1, 'Hello, World!')\nEND",
        "influence_on": ["ALGOL 60", "ALGOL 68", "Pascal", "C", "Simula", "CPL", "All modern imperative langs"],
        "influenced_by": ["FORTRAN", "Mathematical notation"],
        "status": "historical",
        "use_cases": ["Algorithm publication", "Academic computing"]
    },
    {
        "name": "COBOL", "year": 1959, "creator": "Grace Hopper / CODASYL Committee",
        "paradigm": ["imperative", "procedural", "object-oriented (2002+)"],
        "typing": "static, strong", "family": "COBOL",
        "description": "COmmon Business-Oriented Language - designed for business data processing. Grace Hopper's vision was a language readable by managers, not just programmers. COBOL processes an estimated 95% of ATM transactions, 80% of in-person financial transactions, and runs 43% of all banking systems globally. Over 220 billion lines of COBOL are still in production.",
        "key_concepts": ["English-like syntax", "Record structures", "File handling", "Division structure", "PICTURE clauses", "PERFORM loops", "COPY/REPLACE for code reuse"],
        "syntax_examples": {
            "hello_world": "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO-WORLD.\n       PROCEDURE DIVISION.\n           DISPLAY 'Hello, World!'.\n           STOP RUN.",
            "data_definition": "       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 WS-NAME PIC A(30) VALUE 'OMNI-KERNEL'.\n       01 WS-AGE  PIC 99   VALUE 25.\n       01 WS-SALARY PIC 9(7)V99 VALUE 50000.00.",
            "perform_loop": "       PERFORM VARYING WS-COUNT FROM 1 BY 1\n           UNTIL WS-COUNT > 100\n           ADD WS-COUNT TO WS-TOTAL\n       END-PERFORM.",
            "file_handling": "       SELECT EMPLOYEE-FILE ASSIGN TO 'EMPFILE.DAT'\n           ORGANIZATION IS INDEXED\n           ACCESS MODE IS DYNAMIC\n           RECORD KEY IS EMP-ID."
        },
        "keywords": ["IDENTIFICATION", "ENVIRONMENT", "DATA", "PROCEDURE", "DIVISION", "SECTION", "PARAGRAPH", "PERFORM", "DISPLAY", "ACCEPT", "MOVE", "ADD", "SUBTRACT", "MULTIPLY", "DIVIDE", "COMPUTE", "IF", "EVALUATE", "WHEN", "STRING", "UNSTRING", "INSPECT", "SEARCH", "READ", "WRITE", "REWRITE", "DELETE", "OPEN", "CLOSE", "COPY", "REPLACE", "CALL", "GOBACK", "STOP RUN", "PIC", "PICTURE", "VALUE", "OCCURS", "REDEFINES", "FILLER"],
        "memory_model": "Fixed-format record-based, no pointers, no dynamic allocation (until OO COBOL)",
        "error_handling": "FILE STATUS codes, DECLARATIVES section, USE AFTER EXCEPTION",
        "notable_features": ["English-like readability", "Decimal arithmetic (no floating-point errors)", "Excellent file handling", "Still runs global banking"],
        "hello_world": "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.\n       PROCEDURE DIVISION.\n           DISPLAY 'Hello, World!'.\n           STOP RUN.",
        "influence_on": ["PL/I", "ABAP (SAP)", "SQL (conceptually)"],
        "influenced_by": ["FLOW-MATIC (Grace Hopper)", "COMTRAN"],
        "status": "active",
        "use_cases": ["Banking", "Insurance", "Government systems", "ATM transactions", "Payroll processing", "Healthcare billing"]
    },
    {
        "name": "ALGOL 60", "year": 1960, "creator": "Backus, Naur, et al.",
        "paradigm": ["imperative", "structured", "procedural"],
        "typing": "static", "family": "ALGOL",
        "description": "The revised ALGOL that became the most influential programming language in history. Nearly every modern language traces its lineage back to ALGOL 60. It formalized block structure, lexical scoping, nested functions, and the BNF notation for grammar specification.",
        "key_concepts": ["Block structure with begin/end", "Lexical scoping", "Recursive procedures", "Call by name and call by value", "BNF grammar"],
        "syntax_examples": {
            "procedure": "real procedure average(A, n);\n  real array A; integer n;\nbegin\n  real S; S := 0;\n  for i := 1 step 1 until n do\n    S := S + A[i];\n  average := S / n\nend",
            "conditional": "if x > 0 then y := x\nelse if x < 0 then y := -x\nelse y := 0"
        },
        "keywords": ["begin", "end", "real", "integer", "Boolean", "array", "procedure", "if", "then", "else", "for", "do", "while", "step", "until", "goto", "switch", "value", "own", "comment", "true", "false"],
        "memory_model": "Stack-based with block-scoped lifetimes",
        "error_handling": "No formal mechanism",
        "notable_features": ["Most influential language ever designed", "Formal BNF specification"],
        "hello_world": "begin\n  outstring(1, \"Hello, World!\")\nend",
        "influence_on": ["Simula", "CPL", "BCPL", "Pascal", "C", "All Algol-family languages"],
        "influenced_by": ["ALGOL 58", "FORTRAN"],
        "status": "historical",
        "use_cases": ["Algorithm specification", "Academic computing", "Compiler research"]
    },
    {
        "name": "APL", "year": 1962, "creator": "Kenneth Iverson",
        "paradigm": ["array", "functional", "structured"],
        "typing": "dynamic", "family": "APL",
        "description": "A Programming Language - famous for its use of special mathematical symbols and extreme conciseness. A single line of APL can do what takes pages in other languages. It operates primarily on arrays and matrices, making it powerful for mathematical and statistical computing. Won the Turing Award for Iverson.",
        "key_concepts": ["Array programming", "Special symbol set (Greek/math characters)", "Right-to-left evaluation", "Operators modify functions", "Rank and shape of arrays", "Tacit (point-free) programming"],
        "syntax_examples": {
            "hello_world": "'Hello, World!'",
            "sum_of_squares": "+/⍳10  ⍝ sum of 1 to 10 = 55",
            "prime_sieve": "(~R∊R∘.×R)/R←1↓⍳N  ⍝ primes up to N",
            "matrix_ops": "A←3 3⍴⍳9  ⍝ 3x3 matrix\n+/A         ⍝ row sums\n+⌿A         ⍝ column sums\nA+.×B       ⍝ matrix multiply",
            "game_of_life": "life←{↑1 ⍵∨.∧3 4=+/,¯1 0 1∘.⊖¯1 0 1∘.⌽⊂⍵}"
        },
        "keywords": ["⍳ (iota)", "⍴ (rho/reshape)", "← (assignment)", "+/ (reduce-add)", "⌿ (reduce-first)", "∘.× (outer product)", "⍉ (transpose)", "⌈ (ceiling)", "⌊ (floor)", "∊ (membership)", "⍷ (find)", "⍋ (grade up)", "⍒ (grade down)", "⊂ (enclose)", "⊃ (disclose)", "/ (replicate)", "\\ (expand)", "¨ (each)", "⍨ (commute)", "⍣ (power operator)"],
        "memory_model": "Automatic array allocation, garbage collected",
        "error_handling": "⎕TRAP for error trapping, ⎕SIGNAL for signaling",
        "notable_features": ["Most concise programming language", "Special character set", "Entire programs in one line", "Inspired modern array languages"],
        "hello_world": "'Hello, World!'",
        "influence_on": ["J", "K", "Q", "MATLAB", "NumPy", "Julia", "R", "S"],
        "influenced_by": ["Mathematical notation", "Iverson notation"],
        "status": "active",
        "use_cases": ["Financial analysis", "Actuarial science", "Mathematical research", "Data analysis", "Rapid prototyping"]
    },
    {
        "name": "Simula", "year": 1962, "creator": "Ole-Johan Dahl, Kristen Nygaard",
        "paradigm": ["object-oriented", "imperative", "structured", "simulation"],
        "typing": "static, strong", "family": "ALGOL",
        "description": "The first object-oriented programming language. Developed at the Norwegian Computing Center for discrete event simulation. Simula introduced classes, objects, inheritance, subclasses, virtual procedures, coroutines, and garbage collection. It is the direct ancestor of C++, Java, C#, and all modern OOP languages.",
        "key_concepts": ["Classes and objects", "Inheritance", "Virtual methods", "Coroutines", "Simulation processes", "Garbage collection"],
        "syntax_examples": {
            "class_definition": "Class Vehicle(weight, maxspeed);\n  Real weight, maxspeed;\nBegin\n  Real current_speed;\n  Procedure Accelerate(amount);\n    Real amount;\n    current_speed := current_speed + amount;\nEnd;",
            "inheritance": "Vehicle Class Car(doors);\n  Integer doors;\nBegin\n  Boolean locked;\n  Procedure Lock;\n    locked := True;\nEnd;",
            "object_creation": "Ref(Car) myCar;\nmyCar :- New Car(1500.0, 200.0, 4);\nmyCar.Accelerate(60.0);"
        },
        "keywords": ["Class", "Begin", "End", "Ref", "New", "This", "Virtual", "Procedure", "Integer", "Real", "Boolean", "Text", "Array", "If", "Then", "Else", "For", "While", "Do", "Step", "Until", "Inspect", "When", "Otherwise", "Activate", "Passivate", "Hold", "Inner"],
        "memory_model": "Heap-allocated objects with garbage collection",
        "error_handling": "No formal exception mechanism",
        "notable_features": ["Invented OOP", "Coroutines for simulation", "Influenced ALL OOP languages"],
        "hello_world": "Begin\n  OutText(\"Hello, World!\");\n  OutImage;\nEnd;",
        "influence_on": ["Smalltalk", "C++", "Java", "C#", "Python", "Ruby", "All OOP languages"],
        "influenced_by": ["ALGOL 60"],
        "status": "historical",
        "use_cases": ["Discrete event simulation", "System modeling", "Teaching OOP concepts"]
    },
    {
        "name": "SNOBOL", "year": 1962, "creator": "Ralph Griswold, Ivan Polonsky, David Farber",
        "paradigm": ["imperative", "pattern-matching"],
        "typing": "dynamic", "family": "SNOBOL",
        "description": "StriNg Oriented and symBOlic Language. Designed for text processing and pattern matching. SNOBOL's pattern matching capabilities were far ahead of their time, predating regular expressions. SNOBOL4 (1967) added powerful features like programmer-defined data types and backtracking pattern matching.",
        "key_concepts": ["Pattern matching as first-class construct", "Success/failure control flow", "String concatenation as core operation", "Backtracking"],
        "syntax_examples": {
            "pattern_match": "    INPUT 'Enter name: ' NAME\n    NAME 'John' :S(FOUND)F(NOTFOUND)\nFOUND OUTPUT = 'Hello, John!'\nNOTFOUND OUTPUT = 'Who are you?'",
            "string_replace": "    TEXT = 'Hello World'\n    TEXT 'World' = 'OMNI' :S(DONE)\nDONE OUTPUT = TEXT"
        },
        "keywords": ["DEFINE", "INPUT", "OUTPUT", "PUNCH", "TRIM", "SIZE", "REPLACE", "DUPL", "LEN", "SPAN", "BREAK", "ANY", "NOTANY", "TAB", "RTAB", "ARB", "BAL", "SUCCEED", "FENCE", "ABORT"],
        "memory_model": "Dynamic strings, garbage collected",
        "error_handling": "Success/Failure branching with :S() and :F() goto labels",
        "notable_features": ["Most powerful pattern matching of its era", "Patterns as first-class values", "Predecessor to Icon and regex"],
        "hello_world": "    OUTPUT = 'Hello, World!'\nEND",
        "influence_on": ["Icon", "AWK", "Perl (pattern matching)", "SPITBOL"],
        "influenced_by": ["COMIT", "SCL"],
        "status": "historical",
        "use_cases": ["Text processing", "Compiler writing", "Natural language processing"]
    },
    {
        "name": "CPL", "year": 1963, "creator": "Christopher Strachey, Cambridge/London",
        "paradigm": ["imperative", "structured", "functional"],
        "typing": "static", "family": "ALGOL",
        "description": "Combined Programming Language - an ambitious language that combined ALGOL-like structure with low-level capabilities. It was too complex to implement fully but led to BCPL, which led to B, which led to C.",
        "key_concepts": ["Combined high and low level features", "First language to attempt systems + application programming"],
        "syntax_examples": {
            "function": "let F[x] = x > 0 ? x : -x"
        },
        "keywords": ["let", "be", "section", "result", "value", "def", "function"],
        "memory_model": "Stack and heap hybrid",
        "error_handling": "None formal",
        "notable_features": ["Ancestor of BCPL -> B -> C -> C++ -> Java"],
        "hello_world": "(Never fully implemented)",
        "influence_on": ["BCPL", "B", "C"],
        "influenced_by": ["ALGOL 60"],
        "status": "historical",
        "use_cases": ["Language design research"]
    },
    {
        "name": "BASIC", "year": 1964, "creator": "John Kemeny, Thomas Kurtz",
        "paradigm": ["imperative", "procedural", "structured (later)"],
        "typing": "dynamic (early), static (later variants)", "family": "BASIC",
        "description": "Beginner's All-purpose Symbolic Instruction Code - designed to make computing accessible to non-science students. BASIC became the dominant language of the personal computer revolution in the 1970s-80s. Microsoft was founded to sell a BASIC interpreter. Variants include GW-BASIC, QBasic, Visual Basic, VBA, VB.NET, FreeBASIC, and many more. This is the language that taught a generation to program.",
        "key_concepts": ["Line numbers", "GOTO jumps", "INPUT/PRINT for I/O", "REM comments", "Simple variable naming", "Interactive interpreter"],
        "syntax_examples": {
            "classic_basic": "10 PRINT \"Hello, World!\"\n20 INPUT \"What is your name? \"; N$\n30 PRINT \"Hello, \"; N$\n40 FOR I = 1 TO 10\n50   PRINT I; \" squared is \"; I * I\n60 NEXT I\n70 END",
            "qbasic": "DIM Total AS INTEGER\nTotal = 0\nFOR i = 1 TO 100\n  Total = Total + i\nNEXT i\nPRINT \"Sum: \"; Total",
            "visual_basic": "Private Sub Command1_Click()\n  MsgBox \"Hello, World!\"\nEnd Sub"
        },
        "keywords": ["PRINT", "INPUT", "LET", "IF", "THEN", "ELSE", "GOTO", "GOSUB", "RETURN", "FOR", "TO", "STEP", "NEXT", "WHILE", "WEND", "DO", "LOOP", "DIM", "AS", "INTEGER", "STRING", "SINGLE", "DOUBLE", "REM", "DATA", "READ", "RESTORE", "DEF FN", "SUB", "FUNCTION", "END", "STOP", "RANDOMIZE", "RND", "ABS", "SQR", "INT", "LEFT$", "RIGHT$", "MID$", "LEN", "VAL", "STR$", "CHR$", "ASC", "INKEY$", "SCREEN", "PSET", "LINE", "CIRCLE", "PAINT", "SOUND", "PLAY", "OPEN", "CLOSE", "GET", "PUT", "PEEK", "POKE"],
        "memory_model": "Simple variable store, arrays, no pointers",
        "error_handling": "ON ERROR GOTO line_number, ERR variable, RESUME",
        "notable_features": ["Democratized programming", "Every 1980s computer had BASIC in ROM", "Microsoft's first product", "Visual Basic created the RAD revolution"],
        "hello_world": "10 PRINT \"Hello, World!\"",
        "influence_on": ["Visual Basic", "VBA", "VB.NET", "QBASIC", "FreeBASIC", "Gambas", "Small Basic"],
        "influenced_by": ["FORTRAN", "ALGOL 60"],
        "status": "active (VB.NET, VBA)",
        "use_cases": ["Education", "Rapid Application Development", "Office automation (VBA)", "Game prototyping (QBasic era)", "Business applications (VB6)"]
    },
    {
        "name": "PL/I", "year": 1964, "creator": "IBM",
        "paradigm": ["imperative", "structured", "procedural"],
        "typing": "static, strong", "family": "PL/I",
        "description": "Programming Language One - IBM's attempt to create one language that could replace FORTRAN, COBOL, and ALGOL. It was enormously feature-rich: exception handling, multitasking, string handling, pointer arithmetic, bit manipulation, and structured data. Though it never fully replaced its predecessors, many of its innovations appeared in later languages.",
        "key_concepts": ["ON conditions (exception handling)", "Structures", "Based storage (pointers)", "Multitasking", "Compile-time facilities"],
        "syntax_examples": {
            "hello_world": "HELLO: PROCEDURE OPTIONS(MAIN);\n  PUT LIST('Hello, World!');\nEND HELLO;",
            "structures": "DECLARE 1 EMPLOYEE,\n  2 NAME CHARACTER(30),\n  2 AGE FIXED BINARY(15),\n  2 SALARY FIXED DECIMAL(9,2);",
            "exception": "ON ZERODIVIDE BEGIN;\n  PUT LIST('Division by zero!');\nEND;\nX = A / B;"
        },
        "keywords": ["DECLARE", "DCL", "PROCEDURE", "PROC", "BEGIN", "END", "IF", "THEN", "ELSE", "DO", "WHILE", "UNTIL", "LEAVE", "ITERATE", "SELECT", "WHEN", "OTHERWISE", "GO TO", "CALL", "RETURN", "PUT", "GET", "LIST", "EDIT", "DATA", "FILE", "READ", "WRITE", "OPEN", "CLOSE", "ON", "SIGNAL", "REVERT", "ALLOCATE", "FREE", "BASED", "POINTER", "FIXED", "FLOAT", "BINARY", "DECIMAL", "CHARACTER", "BIT", "VARYING", "PICTURE", "AREA", "OFFSET", "ENTRY", "LABEL", "FORMAT"],
        "memory_model": "Automatic, static, controlled (stack), and based (heap) storage classes",
        "error_handling": "ON-conditions: ON ENDFILE, ON CONVERSION, ON ZERODIVIDE, ON OVERFLOW, etc.",
        "notable_features": ["First language with comprehensive exception handling", "Four storage classes", "Extremely feature-rich"],
        "hello_world": "test: proc options(main);\n  put list ('Hello, World!');\nend test;",
        "influence_on": ["C (partially)", "Ada", "C++ exceptions"],
        "influenced_by": ["FORTRAN", "ALGOL 60", "COBOL"],
        "status": "legacy active",
        "use_cases": ["IBM mainframe systems", "Banking", "Insurance", "Government"]
    },
    {
        "name": "Logo", "year": 1967, "creator": "Wally Feurzeig, Seymour Papert",
        "paradigm": ["functional", "imperative", "educational"],
        "typing": "dynamic", "family": "LISP",
        "description": "An educational programming language designed to teach children programming through turtle graphics. The turtle is a cursor that moves around the screen, drawing lines as it goes. Logo is a dialect of Lisp, but with a simpler syntax. Seymour Papert's constructionism philosophy drove its design: children learn by building things.",
        "key_concepts": ["Turtle graphics", "Recursion", "List processing", "Constructionist learning", "Procedures as building blocks"],
        "syntax_examples": {
            "hello_world": "print [Hello, World!]",
            "turtle_square": "repeat 4 [forward 100 right 90]",
            "turtle_spiral": "to spiral :size\n  if :size > 200 [stop]\n  forward :size\n  right 90\n  spiral :size + 5\nend",
            "tree": "to tree :size\n  if :size < 5 [forward :size back :size stop]\n  forward :size / 3\n  left 30 tree :size * 2 / 3 right 30\n  forward :size / 6\n  right 25 tree :size / 2 left 25\n  forward :size / 3\n  right 25 tree :size / 2 left 25\n  forward :size / 6\n  back :size\nend"
        },
        "keywords": ["forward", "fd", "back", "bk", "right", "rt", "left", "lt", "penup", "pu", "pendown", "pd", "repeat", "if", "to", "end", "stop", "output", "make", "print", "show", "thing", "first", "last", "butfirst", "butlast", "sentence", "list", "count", "item", "setpencolor", "setpensize", "hideturtle", "showturtle", "home", "clearscreen", "cs"],
        "memory_model": "Dynamic, garbage collected (Lisp heritage)",
        "error_handling": "CATCH/THROW mechanism",
        "notable_features": ["Turtle graphics", "Designed for children", "Based on Lisp", "Taught millions to program"],
        "hello_world": "print [Hello, World!]",
        "influence_on": ["Scratch", "NetLogo", "StarLogo", "Python turtle module"],
        "influenced_by": ["LISP"],
        "status": "active (educational)",
        "use_cases": ["Education", "Teaching programming to children", "Computational thinking"]
    },
    {
        "name": "BCPL", "year": 1967, "creator": "Martin Richards",
        "paradigm": ["imperative", "structured", "procedural"],
        "typing": "typeless", "family": "ALGOL",
        "description": "Basic Combined Programming Language - a simplified version of CPL. BCPL is typeless: everything is a machine word. It directly influenced the creation of B and then C. The // comment style and curly braces originated here.",
        "key_concepts": ["Typeless design", "Everything is a word", "Portable compiler", "Simple but powerful"],
        "syntax_examples": {
            "hello_world": "GET \"LIBHDR\"\nLET START() BE\n  WRITES(\"Hello, World!*N\")",
            "function": "LET FACTORIAL(N) = N = 0 -> 1, N * FACTORIAL(N - 1)"
        },
        "keywords": ["LET", "BE", "AND", "VALOF", "RESULTIS", "GET", "MANIFEST", "GLOBAL", "STATIC", "IF", "THEN", "ELSE", "UNLESS", "TEST", "DO", "FOR", "TO", "BY", "WHILE", "UNTIL", "REPEAT", "REPEATWHILE", "REPEATUNTIL", "BREAK", "LOOP", "RETURN", "FINISH", "SWITCHON", "CASE", "DEFAULT", "ENDCASE", "INTO", "TABLE", "VEC", "TRUE", "FALSE"],
        "memory_model": "Flat memory, everything is a machine word",
        "error_handling": "None - abort on error",
        "notable_features": ["Introduced // comments", "Introduced curly brace style", "Direct ancestor of C"],
        "hello_world": "GET \"LIBHDR\"\nLET START() BE WRITES(\"Hello, World!*N\")",
        "influence_on": ["B", "C"],
        "influenced_by": ["CPL", "ALGOL 60"],
        "status": "historical",
        "use_cases": ["System programming", "Compiler bootstrapping", "Operating systems"]
    },
    {
        "name": "Pascal", "year": 1970, "creator": "Niklaus Wirth",
        "paradigm": ["imperative", "structured", "procedural"],
        "typing": "static, strong", "family": "ALGOL",
        "description": "Named after mathematician Blaise Pascal. Designed as a teaching language that encourages structured programming and data structuring. Pascal became wildly popular in the 1980s through Turbo Pascal (Borland) and later Delphi for rapid application development. It remains in use for education and through Free Pascal/Lazarus.",
        "key_concepts": ["Strong typing", "Structured programming", "Record types", "Pointer types", "Set types", "Enumerated types", "Nested procedures"],
        "syntax_examples": {
            "hello_world": "program Hello;\nbegin\n  writeln('Hello, World!');\nend.",
            "procedure": "procedure Swap(var a, b: integer);\nvar temp: integer;\nbegin\n  temp := a;\n  a := b;\n  b := temp;\nend;",
            "record": "type\n  TStudent = record\n    Name: string[50];\n    Age: integer;\n    GPA: real;\n  end;\n\nvar student: TStudent;\nbegin\n  student.Name := 'Ali';\n  student.Age := 20;\n  student.GPA := 3.8;\nend.",
            "linked_list": "type\n  PNode = ^TNode;\n  TNode = record\n    Data: integer;\n    Next: PNode;\n  end;"
        },
        "keywords": ["program", "unit", "uses", "interface", "implementation", "begin", "end", "var", "const", "type", "procedure", "function", "if", "then", "else", "case", "of", "for", "to", "downto", "do", "while", "repeat", "until", "with", "record", "array", "set", "file", "string", "integer", "real", "boolean", "char", "byte", "word", "longint", "pointer", "nil", "and", "or", "not", "xor", "div", "mod", "shl", "shr", "in", "writeln", "readln", "write", "read", "new", "dispose", "inherited", "class", "object", "constructor", "destructor", "virtual", "override"],
        "memory_model": "Stack-allocated locals, heap via New/Dispose",
        "error_handling": "Runtime errors halt program, try/except in Delphi/FPC",
        "notable_features": ["Excellent teaching language", "Strong type system", "Turbo Pascal's legendary fast compiler", "Delphi = Visual Pascal"],
        "hello_world": "program Hello;\nbegin\n  WriteLn('Hello, World!');\nend.",
        "influence_on": ["Modula-2", "Oberon", "Ada", "Delphi", "Object Pascal"],
        "influenced_by": ["ALGOL 60", "ALGOL W"],
        "status": "active (Free Pascal, Delphi, Lazarus)",
        "use_cases": ["Education", "Desktop applications (Delphi)", "Game development (early)", "System utilities"]
    },
    {
        "name": "B", "year": 1969, "creator": "Ken Thompson, Dennis Ritchie",
        "paradigm": ["imperative", "procedural"],
        "typing": "typeless", "family": "ALGOL",
        "description": "A stripped-down language derived from BCPL, developed at Bell Labs for the PDP-7. Like BCPL, B is typeless - everything is a machine word. B was the direct predecessor of C. Ken Thompson used B to write utilities for early Unix.",
        "key_concepts": ["Typeless like BCPL", "Simplified syntax", "Unix heritage"],
        "syntax_examples": {
            "hello_world": "main() {\n  putstr(\"Hello, World!*n\");\n}",
            "function": "factorial(n) {\n  if (n <= 1) return(1);\n  return(n * factorial(n-1));\n}"
        },
        "keywords": ["auto", "extrn", "if", "else", "while", "switch", "case", "goto", "return"],
        "memory_model": "Flat, typeless words",
        "error_handling": "None",
        "notable_features": ["Direct parent of C", "Used to write first Unix utilities"],
        "hello_world": "main() {\n  putstr(\"Hello, World!*n\");\n}",
        "influence_on": ["C"],
        "influenced_by": ["BCPL"],
        "status": "historical",
        "use_cases": ["Early Unix development"]
    },
    {
        "name": "C", "year": 1972, "creator": "Dennis Ritchie",
        "paradigm": ["imperative", "structured", "procedural"],
        "typing": "static, weak", "family": "C",
        "description": "The most influential system programming language ever created. Developed at Bell Labs to rewrite the Unix operating system. C gives programmers direct access to memory through pointers while maintaining a readable syntax. It is the backbone of operating systems (Linux, Windows, macOS), embedded systems, databases, and compilers. Nearly every modern language either is written in C, compiles to C, or borrows from C's syntax.",
        "key_concepts": ["Pointers and pointer arithmetic", "Manual memory management", "Preprocessor macros", "Header files", "Undefined behavior", "Direct hardware access", "Minimal runtime"],
        "syntax_examples": {
            "hello_world": "#include <stdio.h>\n\nint main(void) {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}",
            "pointers": "int x = 42;\nint *ptr = &x;\nprintf(\"%d\\n\", *ptr);  // 42\n\nint arr[] = {1, 2, 3, 4, 5};\nint *p = arr;\nfor (int i = 0; i < 5; i++) {\n    printf(\"%d \", *(p + i));\n}",
            "struct": "typedef struct {\n    char name[50];\n    int age;\n    float salary;\n} Employee;\n\nEmployee emp = {\"Ali\", 25, 5000.0};\nprintf(\"%s is %d years old\\n\", emp.name, emp.age);",
            "dynamic_memory": "#include <stdlib.h>\nint *arr = (int *)malloc(100 * sizeof(int));\nif (arr == NULL) {\n    perror(\"malloc failed\");\n    exit(1);\n}\nfor (int i = 0; i < 100; i++) arr[i] = i * i;\nfree(arr);",
            "file_io": "FILE *fp = fopen(\"data.txt\", \"r\");\nif (fp) {\n    char buf[256];\n    while (fgets(buf, sizeof(buf), fp)) {\n        printf(\"%s\", buf);\n    }\n    fclose(fp);\n}",
            "function_pointer": "int add(int a, int b) { return a + b; }\nint sub(int a, int b) { return a - b; }\n\nint (*op)(int, int) = add;\nprintf(\"%d\\n\", op(10, 5));  // 15\nop = sub;\nprintf(\"%d\\n\", op(10, 5));  // 5"
        },
        "keywords": ["auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else", "enum", "extern", "float", "for", "goto", "if", "inline", "int", "long", "register", "restrict", "return", "short", "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void", "volatile", "while", "_Alignas", "_Alignof", "_Atomic", "_Bool", "_Complex", "_Generic", "_Imaginary", "_Noreturn", "_Static_assert", "_Thread_local"],
        "memory_model": "Manual: stack (auto), heap (malloc/free), static, register. No garbage collector.",
        "error_handling": "Return codes, errno, setjmp/longjmp, perror",
        "notable_features": ["Unix rewritten in C", "Portable assembly language", "Foundation of modern computing", "Minimal overhead", "Runs on virtually every platform"],
        "hello_world": "#include <stdio.h>\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}",
        "influence_on": ["C++", "Objective-C", "C#", "Java", "JavaScript", "PHP", "Perl", "Python", "Go", "Rust", "Swift", "D", "Zig", "Nearly every modern language"],
        "influenced_by": ["B", "BCPL", "ALGOL 68"],
        "status": "active",
        "use_cases": ["Operating systems (Linux, Windows kernel)", "Embedded systems", "Databases (PostgreSQL, SQLite)", "Compilers", "Game engines", "Networking", "IoT"]
    },
    {
        "name": "Smalltalk", "year": 1972, "creator": "Alan Kay, Xerox PARC",
        "paradigm": ["object-oriented", "reflective"],
        "typing": "dynamic, strong", "family": "SMALLTALK",
        "description": "The language that perfected object-oriented programming. Everything in Smalltalk is an object - even numbers, booleans, and classes themselves. Alan Kay coined the term 'object-oriented programming' specifically for Smalltalk. It pioneered the GUI, the IDE, MVC architecture, and live coding. Smalltalk influenced virtually every OOP language that followed.",
        "key_concepts": ["Everything is an object", "Message passing", "Late binding", "Live coding environment", "Image-based persistence", "MVC pattern", "Blocks (closures)", "Metaclasses"],
        "syntax_examples": {
            "hello_world": "Transcript show: 'Hello, World!'.",
            "class_def": "Object subclass: #Animal\n    instanceVariableNames: 'name sound'\n    classVariableNames: ''\n    poolDictionaries: ''\n    category: 'Animals'.",
            "method": "speak\n    ^ 'My name is ', name, ' and I say ', sound.",
            "blocks": "| sum |\nsum := 0.\n1 to: 100 do: [:i | sum := sum + i].\nTranscript show: sum printString.",
            "collection": "#(1 2 3 4 5) select: [:each | each even].\n\"=> #(2 4)\"\n#(1 2 3 4 5) collect: [:each | each * each].\n\"=> #(1 4 9 16 25)\""
        },
        "keywords": ["self", "super", "nil", "true", "false", "thisContext", "subclass:", "instanceVariableNames:", "classVariableNames:", "category:"],
        "memory_model": "Fully garbage-collected, image-based (entire environment persisted)",
        "error_handling": "Exception handling via on:do: blocks",
        "notable_features": ["Invented the modern GUI", "Invented MVC pattern", "Invented the IDE", "Everything is an object", "Live coding", "Influenced Ruby, Python, Objective-C, Java"],
        "hello_world": "Transcript show: 'Hello, World!'",
        "influence_on": ["Objective-C", "Ruby", "Python", "Java", "C#", "Scala", "Dart", "Swift"],
        "influenced_by": ["Simula", "LISP", "Logo"],
        "status": "active (Pharo, Squeak, GemStone)",
        "use_cases": ["Education", "Rapid prototyping", "Enterprise (GemStone)", "Research"]
    },
    {
        "name": "Prolog", "year": 1972, "creator": "Alain Colmerauer, Robert Kowalski",
        "paradigm": ["logic", "declarative"],
        "typing": "dynamic", "family": "LOGIC",
        "description": "PROgrammation en LOGique - the first major logic programming language. Instead of telling the computer HOW to solve a problem, you describe WHAT the problem is using facts and rules. Prolog then uses unification and backtracking to find solutions. It was central to Japan's Fifth Generation Computer project and remains important in AI, NLP, and expert systems.",
        "key_concepts": ["Facts and rules", "Unification", "Backtracking", "Horn clauses", "Pattern matching", "Logic variables", "Cut operator"],
        "syntax_examples": {
            "facts_and_rules": "% Facts\nparent(tom, bob).\nparent(tom, liz).\nparent(bob, ann).\nparent(bob, pat).\n\n% Rules\ngrandparent(X, Z) :- parent(X, Y), parent(Y, Z).\nsibling(X, Y) :- parent(Z, X), parent(Z, Y), X \\= Y.",
            "query": "?- grandparent(tom, ann).\n% => true\n?- grandparent(tom, Who).\n% => Who = ann ; Who = pat",
            "list_processing": "% Length of a list\nlength([], 0).\nlength([_|T], N) :- length(T, N1), N is N1 + 1.\n\n% Append two lists\nappend([], L, L).\nappend([H|T1], L2, [H|T3]) :- append(T1, L2, T3).",
            "sorting": "% Quicksort\nqsort([], []).\nqsort([H|T], Sorted) :-\n    partition(H, T, Less, Greater),\n    qsort(Less, SortedLess),\n    qsort(Greater, SortedGreater),\n    append(SortedLess, [H|SortedGreater], Sorted)."
        },
        "keywords": [":-", "?-", ",", ";", ".", "!", "not", "is", "=", "\\=", "==", "\\==", "=..", "functor", "arg", "copy_term", "assert", "retract", "findall", "bagof", "setof", "write", "read", "nl", "tab", "atom", "number", "var", "nonvar", "compound", "succ", "plus", "length", "append", "member", "last", "reverse", "nth0", "nth1", "msort", "predsort", "between", "forall"],
        "memory_model": "Automatic, trail-based for backtracking",
        "error_handling": "catch/throw mechanism",
        "notable_features": ["Declarative: describe the problem, not the solution", "Unification engine", "Backtracking search", "Used in AI and expert systems"],
        "hello_world": ":- write('Hello, World!'), nl.",
        "influence_on": ["Erlang (pattern matching)", "Mercury", "Datalog", "SQL (conceptually)", "Constraint programming"],
        "influenced_by": ["First-order logic", "Resolution theorem proving"],
        "status": "active (SWI-Prolog, SICStus, GNU Prolog)",
        "use_cases": ["AI expert systems", "Natural language processing", "Theorem proving", "Database querying", "Symbolic AI", "Compiler analysis"]
    },
    {
        "name": "SQL", "year": 1974, "creator": "Donald Chamberlin, Raymond Boyce (IBM)",
        "paradigm": ["declarative", "set-based"],
        "typing": "static (column types)", "family": "QUERY",
        "description": "Structured Query Language - the standard language for relational database management systems. SQL lets you describe WHAT data you want without specifying HOW to get it. It is the most widely used database language in the world, running in every bank, hospital, website, and mobile app that stores data.",
        "key_concepts": ["Relational algebra", "Set operations", "Declarative queries", "Tables/Relations", "Joins", "Aggregation", "Transactions", "ACID properties"],
        "syntax_examples": {
            "create_table": "CREATE TABLE employees (\n    id INT PRIMARY KEY AUTO_INCREMENT,\n    name VARCHAR(100) NOT NULL,\n    department VARCHAR(50),\n    salary DECIMAL(10,2),\n    hire_date DATE\n);",
            "insert": "INSERT INTO employees (name, department, salary, hire_date)\nVALUES ('Ali', 'Engineering', 75000.00, '2024-01-15');",
            "select": "SELECT department, AVG(salary) as avg_salary, COUNT(*) as emp_count\nFROM employees\nWHERE hire_date >= '2023-01-01'\nGROUP BY department\nHAVING COUNT(*) > 5\nORDER BY avg_salary DESC;",
            "join": "SELECT e.name, d.department_name, p.project_name\nFROM employees e\nINNER JOIN departments d ON e.department_id = d.id\nLEFT JOIN project_assignments pa ON e.id = pa.employee_id\nLEFT JOIN projects p ON pa.project_id = p.id;",
            "subquery": "SELECT name, salary\nFROM employees\nWHERE salary > (SELECT AVG(salary) FROM employees);",
            "window_function": "SELECT name, department, salary,\n    RANK() OVER (PARTITION BY department ORDER BY salary DESC) as dept_rank,\n    salary - LAG(salary) OVER (ORDER BY salary) as diff_from_prev\nFROM employees;"
        },
        "keywords": ["SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "ALTER", "DROP", "TABLE", "INDEX", "VIEW", "JOIN", "INNER", "LEFT", "RIGHT", "FULL", "OUTER", "CROSS", "ON", "AND", "OR", "NOT", "IN", "EXISTS", "BETWEEN", "LIKE", "IS NULL", "ORDER BY", "GROUP BY", "HAVING", "LIMIT", "OFFSET", "DISTINCT", "AS", "UNION", "INTERSECT", "EXCEPT", "CASE", "WHEN", "THEN", "ELSE", "END", "COUNT", "SUM", "AVG", "MIN", "MAX", "COMMIT", "ROLLBACK", "TRANSACTION", "GRANT", "REVOKE", "PRIMARY KEY", "FOREIGN KEY", "UNIQUE", "CHECK", "DEFAULT", "NOT NULL", "AUTO_INCREMENT", "CASCADE", "TRIGGER", "PROCEDURE", "FUNCTION", "CURSOR", "FETCH", "DECLARE", "WITH", "RECURSIVE", "WINDOW", "OVER", "PARTITION BY", "ROW_NUMBER", "RANK", "DENSE_RANK", "LEAD", "LAG"],
        "memory_model": "Set-based, managed by RDBMS engine",
        "error_handling": "SQLSTATE codes, TRY/CATCH in T-SQL, EXCEPTION in PL/pgSQL",
        "notable_features": ["Universal database language", "Declarative", "ACID transactions", "Used everywhere"],
        "hello_world": "SELECT 'Hello, World!';",
        "influence_on": ["LINQ", "HQL", "JPQL", "GraphQL (conceptually)", "Datalog"],
        "influenced_by": ["Relational algebra (Codd)", "SEQUEL"],
        "status": "active",
        "use_cases": ["Every database in existence", "Web backends", "Data analytics", "Business intelligence", "ETL pipelines"]
    },
    {
        "name": "Scheme", "year": 1975, "creator": "Guy Steele, Gerald Sussman",
        "paradigm": ["functional", "imperative", "meta"],
        "typing": "dynamic, strong", "family": "LISP",
        "description": "A minimalist dialect of Lisp designed for elegance and clarity. Scheme introduced lexical scoping and first-class continuations to the Lisp family. It has been the primary teaching language for computer science at MIT (via SICP - Structure and Interpretation of Computer Programs). Scheme emphasizes a small, clean core with powerful abstractions.",
        "key_concepts": ["Lexical scoping", "First-class continuations (call/cc)", "Tail-call optimization", "Hygienic macros", "Minimal core"],
        "syntax_examples": {
            "hello_world": "(display \"Hello, World!\")\n(newline)",
            "factorial": "(define (factorial n)\n  (if (<= n 1)\n      1\n      (* n (factorial (- n 1)))))",
            "tail_recursive": "(define (factorial n)\n  (define (iter acc n)\n    (if (<= n 1) acc\n        (iter (* acc n) (- n 1))))\n  (iter 1 n))",
            "higher_order": "(define (map f lst)\n  (if (null? lst) '()\n      (cons (f (car lst)) (map f (cdr lst)))))\n\n(map (lambda (x) (* x x)) '(1 2 3 4 5))\n; => (1 4 9 16 25)",
            "continuation": "(call-with-current-continuation\n  (lambda (exit)\n    (for-each (lambda (x)\n                (if (negative? x) (exit x)))\n              '(54 0 37 -3 245 19))\n    #t))\n; => -3"
        },
        "keywords": ["define", "lambda", "if", "cond", "else", "and", "or", "not", "let", "let*", "letrec", "begin", "set!", "quote", "quasiquote", "unquote", "car", "cdr", "cons", "list", "null?", "pair?", "number?", "string?", "symbol?", "boolean?", "eq?", "eqv?", "equal?", "map", "for-each", "apply", "call-with-current-continuation", "call/cc", "values", "call-with-values", "dynamic-wind", "with-exception-handler", "raise", "guard", "define-syntax", "syntax-rules", "syntax-case", "display", "write", "read", "newline", "port", "open-input-file", "open-output-file"],
        "memory_model": "Heap-allocated, garbage collected, closures capture environment",
        "error_handling": "guard expressions, with-exception-handler, raise",
        "notable_features": ["Tail-call optimization guaranteed", "First-class continuations", "Used in SICP", "Hygienic macros", "R7RS standard"],
        "hello_world": "(display \"Hello, World!\\n\")",
        "influence_on": ["JavaScript (closures)", "Ruby", "Lua", "Racket", "Clojure", "Haskell"],
        "influenced_by": ["LISP", "ALGOL (lexical scoping)", "Lambda calculus"],
        "status": "active (Racket, Chez Scheme, Guile, Chicken)",
        "use_cases": ["CS education (SICP)", "Language research", "Scripting (Guile)", "DSL creation"]
    },
    {
        "name": "ML", "year": 1973, "creator": "Robin Milner",
        "paradigm": ["functional", "imperative"],
        "typing": "static, strong, inferred", "family": "ML",
        "description": "Meta Language - developed for the LCF theorem prover. ML pioneered Hindley-Milner type inference, which allows the compiler to automatically determine types without explicit annotations. It also introduced algebraic data types and pattern matching. ML is the ancestor of OCaml, F#, Haskell, and Rust's type system.",
        "key_concepts": ["Type inference", "Algebraic data types", "Pattern matching", "Parametric polymorphism", "Modules and functors", "References for mutation"],
        "syntax_examples": {
            "hello_world": "print \"Hello, World!\\n\";",
            "function": "fun factorial 0 = 1\n  | factorial n = n * factorial (n - 1);",
            "algebraic_type": "datatype shape =\n    Circle of real\n  | Rectangle of real * real\n  | Triangle of real * real * real;\n\nfun area (Circle r) = Math.pi * r * r\n  | area (Rectangle (w, h)) = w * h\n  | area (Triangle (a, b, c)) =\n      let val s = (a + b + c) / 2.0\n      in Math.sqrt (s * (s-a) * (s-b) * (s-c)) end;",
            "list_ops": "fun map f [] = []\n  | map f (x::xs) = (f x) :: (map f xs);\n\nmap (fn x => x * x) [1, 2, 3, 4, 5];\n(* => [1, 4, 9, 16, 25] *)"
        },
        "keywords": ["val", "fun", "fn", "let", "in", "end", "local", "if", "then", "else", "case", "of", "as", "datatype", "type", "exception", "raise", "handle", "struct", "sig", "structure", "signature", "functor", "open", "include", "sharing", "where", "and", "orelse", "andalso", "not", "true", "false", "nil", "op", "infix", "infixr", "nonfix", "ref", "while", "do", "abstype", "withtype", "eqtype"],
        "memory_model": "Garbage collected, immutable by default, ref cells for mutation",
        "error_handling": "exception/raise/handle mechanism",
        "notable_features": ["Invented type inference", "Algebraic data types", "Pattern matching", "Influenced Haskell, OCaml, F#, Rust"],
        "hello_world": "print \"Hello, World!\\n\";",
        "influence_on": ["OCaml", "F#", "Haskell", "Rust", "Scala", "Elm", "ReasonML"],
        "influenced_by": ["ISWIM", "PAL", "LISP"],
        "status": "active (Standard ML, MLton, SML/NJ, Poly/ML)",
        "use_cases": ["Theorem proving", "Compiler construction", "Language research", "Financial systems"]
    },
    {
        "name": "C++", "year": 1985, "creator": "Bjarne Stroustrup",
        "paradigm": ["imperative", "object-oriented", "generic", "functional (C++11+)"],
        "typing": "static, strong (with escape hatches)", "family": "C",
        "description": "Originally 'C with Classes', C++ added object-oriented features to C while maintaining backward compatibility and zero-overhead abstractions. It is one of the most widely used languages in history, powering game engines (Unreal), browsers (Chrome, Firefox), databases (MySQL, MongoDB), operating systems, and embedded systems. Modern C++ (C++11/14/17/20/23) is vastly different from classic C++, with move semantics, lambdas, concepts, modules, coroutines, and ranges.",
        "key_concepts": ["Classes and objects", "Inheritance and polymorphism", "Templates (generic programming)", "RAII (Resource Acquisition Is Initialization)", "Move semantics", "Smart pointers", "Operator overloading", "Multiple inheritance", "STL (Standard Template Library)", "Concepts (C++20)", "Modules (C++20)", "Coroutines (C++20)"],
        "syntax_examples": {
            "hello_world": "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}",
            "class": "class Animal {\nprivate:\n    std::string name_;\n    int age_;\npublic:\n    Animal(std::string name, int age) : name_(std::move(name)), age_(age) {}\n    virtual ~Animal() = default;\n    virtual std::string speak() const = 0;\n    const std::string& name() const { return name_; }\n};",
            "templates": "template <typename T>\nT max_of(T a, T b) {\n    return (a > b) ? a : b;\n}\n\ntemplate <typename Container>\nauto sum(const Container& c) {\n    using T = typename Container::value_type;\n    return std::accumulate(c.begin(), c.end(), T{});\n}",
            "modern_cpp": "// C++20 ranges and concepts\n#include <ranges>\n#include <vector>\n\nauto even_squares = std::views::iota(1, 100)\n    | std::views::filter([](int n) { return n % 2 == 0; })\n    | std::views::transform([](int n) { return n * n; })\n    | std::views::take(10);",
            "smart_pointers": "auto ptr = std::make_unique<Widget>(42);\nauto shared = std::make_shared<Resource>();\nstd::weak_ptr<Resource> weak = shared;"
        },
        "keywords": ["alignas", "alignof", "and", "and_eq", "asm", "auto", "bitand", "bitor", "bool", "break", "case", "catch", "char", "char8_t", "char16_t", "char32_t", "class", "compl", "concept", "const", "consteval", "constexpr", "constinit", "const_cast", "continue", "co_await", "co_return", "co_yield", "decltype", "default", "delete", "do", "double", "dynamic_cast", "else", "enum", "explicit", "export", "extern", "false", "float", "for", "friend", "goto", "if", "inline", "int", "long", "mutable", "namespace", "new", "noexcept", "not", "not_eq", "nullptr", "operator", "or", "or_eq", "private", "protected", "public", "register", "reinterpret_cast", "requires", "return", "short", "signed", "sizeof", "static", "static_assert", "static_cast", "struct", "switch", "template", "this", "thread_local", "throw", "true", "try", "typedef", "typeid", "typename", "union", "unsigned", "using", "virtual", "void", "volatile", "wchar_t", "while", "xor", "xor_eq", "override", "final", "import", "module"],
        "memory_model": "Manual + RAII + smart pointers. Stack, heap, static, thread-local storage.",
        "error_handling": "try/catch/throw exceptions, std::expected (C++23), error codes",
        "notable_features": ["Zero-overhead abstractions", "Templates (Turing-complete)", "STL", "RAII", "Modern C++ renaissance", "Backward compatible with C"],
        "hello_world": "#include <iostream>\nint main() { std::cout << \"Hello, World!\" << std::endl; }",
        "influence_on": ["Java", "C#", "D", "Rust (concepts)", "Swift", "Carbon"],
        "influenced_by": ["C", "Simula", "ALGOL 68", "Ada", "ML"],
        "status": "active",
        "use_cases": ["Game engines (Unreal, Unity internals)", "Browsers (Chrome, Firefox)", "Operating systems", "Databases", "Embedded systems", "HPC", "Finance", "Compilers"]
    },
    {
        "name": "Objective-C", "year": 1984, "creator": "Brad Cox, Tom Love",
        "paradigm": ["object-oriented", "reflective"],
        "typing": "static + dynamic", "family": "C",
        "description": "A Smalltalk-inspired layer added on top of C. Objective-C was the primary language for Apple's macOS and iOS development for decades (before Swift). It uses Smalltalk-style message passing syntax within C's framework.",
        "key_concepts": ["Message passing", "Categories (extension methods)", "Protocols", "Dynamic dispatch", "Reference counting (ARC)"],
        "syntax_examples": {
            "hello_world": "#import <Foundation/Foundation.h>\n\nint main() {\n    @autoreleasepool {\n        NSLog(@\"Hello, World!\");\n    }\n    return 0;\n}",
            "class": "@interface Animal : NSObject\n@property (nonatomic, strong) NSString *name;\n- (void)speak;\n@end\n\n@implementation Animal\n- (void)speak {\n    NSLog(@\"My name is %@\", self.name);\n}\n@end"
        },
        "keywords": ["@interface", "@implementation", "@end", "@property", "@synthesize", "@dynamic", "@protocol", "@optional", "@required", "@class", "@selector", "@encode", "@try", "@catch", "@throw", "@finally", "@autoreleasepool", "@synchronized", "self", "super", "nil", "Nil", "YES", "NO", "id", "SEL", "IMP", "BOOL", "IBOutlet", "IBAction"],
        "memory_model": "Manual retain/release -> ARC (Automatic Reference Counting)",
        "error_handling": "@try/@catch/@finally, NSError pattern",
        "notable_features": ["Smalltalk + C hybrid", "Apple's primary language for 30 years", "Dynamic runtime"],
        "hello_world": "#import <Foundation/Foundation.h>\nint main() {\n    NSLog(@\"Hello, World!\");\n    return 0;\n}",
        "influence_on": ["Swift"],
        "influenced_by": ["Smalltalk", "C"],
        "status": "legacy active (Apple ecosystem)",
        "use_cases": ["macOS/iOS development (legacy)", "Apple frameworks"]
    },
    {
        "name": "Perl", "year": 1987, "creator": "Larry Wall",
        "paradigm": ["imperative", "functional", "object-oriented", "reflective"],
        "typing": "dynamic", "family": "SCRIPTING",
        "description": "Practical Extraction and Reporting Language - the Swiss Army knife of programming. Perl became the backbone of early web development (CGI scripts) and system administration. Famous for its powerful regular expressions, text processing, and the motto 'There's More Than One Way To Do It' (TMTOWTDI).",
        "key_concepts": ["Regular expressions as first-class", "Context sensitivity", "TMTOWTDI philosophy", "Sigils ($, @, %)", "CPAN module repository"],
        "syntax_examples": {
            "hello_world": "#!/usr/bin/perl\nprint \"Hello, World!\\n\";",
            "regex": "my $text = \"The price is $42.99\";\nif ($text =~ /\\$(\\d+\\.\\d{2})/) {\n    print \"Found price: $1\\n\";\n}",
            "hash": "my %ages = ('Alice' => 30, 'Bob' => 25, 'Charlie' => 35);\nfor my $name (sort keys %ages) {\n    print \"$name is $ages{$name} years old\\n\";\n}",
            "file_processing": "open(my $fh, '<', 'data.txt') or die \"Cannot open: $!\";\nwhile (my $line = <$fh>) {\n    chomp $line;\n    print \"Line: $line\\n\" if $line =~ /pattern/;\n}\nclose $fh;"
        },
        "keywords": ["my", "our", "local", "sub", "return", "if", "elsif", "else", "unless", "while", "until", "for", "foreach", "do", "last", "next", "redo", "use", "require", "package", "bless", "ref", "die", "warn", "eval", "chomp", "chop", "push", "pop", "shift", "unshift", "splice", "map", "grep", "sort", "reverse", "join", "split", "print", "say", "open", "close", "read", "write", "seek", "tell", "qw", "qq", "qr", "tr", "s///", "m//", "=~", "!~"],
        "memory_model": "Reference-counted garbage collection",
        "error_handling": "eval { } blocks, die/warn, Try::Tiny module",
        "notable_features": ["Best regex support", "CPAN (200,000+ modules)", "Text processing powerhouse", "CGI web pioneer"],
        "hello_world": "print \"Hello, World!\\n\";",
        "influence_on": ["Python", "Ruby", "PHP", "JavaScript (regex)", "Raku"],
        "influenced_by": ["C", "sed", "awk", "shell scripting", "Lisp"],
        "status": "active",
        "use_cases": ["System administration", "Text processing", "Bioinformatics", "Web development (legacy)", "Network programming"]
    },
    {
        "name": "Erlang", "year": 1986, "creator": "Joe Armstrong (Ericsson)",
        "paradigm": ["functional", "concurrent", "distributed"],
        "typing": "dynamic, strong", "family": "FUNCTIONAL",
        "description": "Developed at Ericsson for telecoms — built for systems that can NEVER go down. Erlang's actor model, lightweight processes, and hot code swapping make it the gold standard for fault-tolerant distributed systems. WhatsApp served 900 million users with just 50 engineers thanks to Erlang.",
        "key_concepts": ["Actor model", "Lightweight processes", "Message passing", "Let it crash philosophy", "Hot code swapping", "OTP framework", "Supervision trees", "Pattern matching"],
        "syntax_examples": {
            "hello_world": "-module(hello).\n-export([world/0]).\n\nworld() ->\n    io:format(\"Hello, World!~n\").",
            "pattern_matching": "factorial(0) -> 1;\nfactorial(N) when N > 0 -> N * factorial(N - 1).",
            "process": "ping(0, Pong_PID) ->\n    Pong_PID ! finished;\nping(N, Pong_PID) ->\n    Pong_PID ! {self(), ping},\n    receive\n        pong -> ok\n    end,\n    ping(N - 1, Pong_PID).",
            "gen_server": "-module(counter).\n-behaviour(gen_server).\n\ninit([]) -> {ok, 0}.\nhandle_call(increment, _From, State) ->\n    {reply, State + 1, State + 1};\nhandle_call(get, _From, State) ->\n    {reply, State, State}."
        },
        "keywords": ["module", "export", "import", "compile", "define", "include", "record", "spec", "type", "callback", "behaviour", "if", "case", "of", "end", "when", "receive", "after", "try", "catch", "throw", "fun", "let", "in", "begin", "query", "not", "and", "or", "xor", "band", "bor", "bxor", "bnot", "bsl", "bsr", "div", "rem", "self", "true", "false", "ok", "error", "undefined"],
        "memory_model": "Per-process heap, garbage collected independently per process",
        "error_handling": "try/catch/after, let-it-crash with supervisor trees",
        "notable_features": ["9 nines reliability (99.9999999%)", "Hot code swapping", "Millions of lightweight processes", "OTP framework"],
        "hello_world": "io:format(\"Hello, World!~n\").",
        "influence_on": ["Elixir", "Akka (Scala)", "Go (goroutines concept)", "Rust (message passing)"],
        "influenced_by": ["Prolog", "Smalltalk", "CSP"],
        "status": "active",
        "use_cases": ["Telecommunications", "WhatsApp", "RabbitMQ", "CouchDB", "Real-time systems", "Chat systems", "IoT platforms"]
    },
    {
        "name": "Python", "year": 1991, "creator": "Guido van Rossum",
        "paradigm": ["imperative", "object-oriented", "functional", "structured", "reflective"],
        "typing": "dynamic, strong (duck typing)", "family": "SCRIPTING",
        "description": "Named after Monty Python. Python's philosophy is readability and simplicity — 'There should be one obvious way to do it.' It has become the world's most popular programming language, dominating AI/ML, data science, web development, automation, and education. Python's ecosystem (NumPy, Pandas, TensorFlow, Django, Flask) is unmatched.",
        "key_concepts": ["Indentation-based syntax", "Duck typing", "List comprehensions", "Generators", "Decorators", "Context managers", "Multiple inheritance (MRO)", "GIL (Global Interpreter Lock)", "Everything is an object"],
        "syntax_examples": {
            "hello_world": "print('Hello, World!')",
            "class": "class Animal:\n    def __init__(self, name, sound):\n        self.name = name\n        self.sound = sound\n\n    def speak(self):\n        return f'{self.name} says {self.sound}!'\n\ndog = Animal('Rex', 'Woof')\nprint(dog.speak())",
            "list_comprehension": "squares = [x**2 for x in range(10)]\nevens = [x for x in range(100) if x % 2 == 0]\nmatrix = [[i*j for j in range(5)] for i in range(5)]",
            "generators": "def fibonacci():\n    a, b = 0, 1\n    while True:\n        yield a\n        a, b = b, a + b\n\nfib = fibonacci()\nfirst_10 = [next(fib) for _ in range(10)]",
            "decorators": "import functools\nimport time\n\ndef timer(func):\n    @functools.wraps(func)\n    def wrapper(*args, **kwargs):\n        start = time.perf_counter()\n        result = func(*args, **kwargs)\n        elapsed = time.perf_counter() - start\n        print(f'{func.__name__} took {elapsed:.4f}s')\n        return result\n    return wrapper\n\n@timer\ndef slow_function():\n    time.sleep(1)",
            "async": "import asyncio\n\nasync def fetch_data(url):\n    async with aiohttp.ClientSession() as session:\n        async with session.get(url) as response:\n            return await response.json()\n\nasync def main():\n    tasks = [fetch_data(url) for url in urls]\n    results = await asyncio.gather(*tasks)",
            "context_manager": "class DatabaseConnection:\n    def __enter__(self):\n        self.conn = connect_to_db()\n        return self.conn\n    def __exit__(self, exc_type, exc_val, exc_tb):\n        self.conn.close()\n        return False\n\nwith DatabaseConnection() as db:\n    db.execute('SELECT * FROM users')"
        },
        "keywords": ["False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield", "match", "case", "type"],
        "memory_model": "Reference counted + cyclic garbage collector, everything on heap",
        "error_handling": "try/except/else/finally, raise, custom exception classes",
        "notable_features": ["Most popular language in the world", "Readability as core principle", "Massive ecosystem", "AI/ML dominance", "Batteries included"],
        "hello_world": "print('Hello, World!')",
        "influence_on": ["Julia", "Swift", "Kotlin", "Mojo", "Nim", "Go (simplicity)"],
        "influenced_by": ["ABC", "C", "Haskell", "Lisp", "Modula-3", "Perl", "Smalltalk"],
        "status": "active",
        "use_cases": ["AI/Machine Learning", "Data Science", "Web development (Django, Flask, FastAPI)", "Automation", "Scientific computing", "Education", "DevOps", "Scripting"]
    },
    {
        "name": "Haskell", "year": 1990, "creator": "Haskell Committee",
        "paradigm": ["purely functional", "lazy"],
        "typing": "static, strong, inferred", "family": "ML",
        "description": "Named after logician Haskell Curry. Haskell is the gold standard of purely functional programming. It features lazy evaluation (expressions are only computed when needed), a powerful type system with type classes, monads for handling side effects, and a guarantee that functions have no side effects (referential transparency).",
        "key_concepts": ["Pure functions", "Lazy evaluation", "Monads", "Type classes", "Pattern matching", "Algebraic data types", "Currying", "Higher-kinded types", "Functors and Applicatives"],
        "syntax_examples": {
            "hello_world": "main :: IO ()\nmain = putStrLn \"Hello, World!\"",
            "functions": "factorial :: Integer -> Integer\nfactorial 0 = 1\nfactorial n = n * factorial (n - 1)\n\nfibonacci :: Int -> Int\nfibonacci n = fibs !! n\n  where fibs = 0 : 1 : zipWith (+) fibs (tail fibs)",
            "list_ops": "doubleAll = map (*2)\nfirstEven = filter even\nsumOfSquares = sum . map (^2) . filter odd\n\n-- List comprehension\npythagorean = [(a,b,c) | c <- [1..100], b <- [1..c], a <- [1..b], a^2 + b^2 == c^2]",
            "algebraic_types": "data Shape = Circle Double\n           | Rectangle Double Double\n           | Triangle Double Double Double\n\narea :: Shape -> Double\narea (Circle r) = pi * r * r\narea (Rectangle w h) = w * h\narea (Triangle a b c) = let s = (a+b+c)/2\n                        in sqrt (s*(s-a)*(s-b)*(s-c))",
            "monads": "-- Maybe monad for safe operations\nsafeDivide :: Double -> Double -> Maybe Double\nsafeDivide _ 0 = Nothing\nsafeDivide x y = Just (x / y)\n\n-- IO monad\nmain :: IO ()\nmain = do\n    putStrLn \"What is your name?\"\n    name <- getLine\n    putStrLn (\"Hello, \" ++ name ++ \"!\")",
            "type_classes": "class Container f where\n    empty :: f a\n    insert :: a -> f a -> f a\n    toList :: f a -> [a]\n\ninstance Container [] where\n    empty = []\n    insert = (:)\n    toList = id"
        },
        "keywords": ["module", "where", "import", "qualified", "as", "hiding", "data", "type", "newtype", "class", "instance", "deriving", "if", "then", "else", "case", "of", "let", "in", "do", "return", "where", "infixl", "infixr", "infix", "forall", "family", "default"],
        "memory_model": "Lazy evaluation with thunks, garbage collected",
        "error_handling": "Maybe, Either, exceptions in IO monad",
        "notable_features": ["Pure functional", "Lazy evaluation", "Most advanced type system", "Monads", "Influenced Rust, Swift, Scala"],
        "hello_world": "main = putStrLn \"Hello, World!\"",
        "influence_on": ["Rust", "Swift", "Scala", "Elm", "PureScript", "Idris", "Agda", "F#", "Python (type hints)"],
        "influenced_by": ["ML", "Miranda", "Scheme", "Lambda calculus"],
        "status": "active",
        "use_cases": ["Compiler construction", "Financial systems", "Formal verification", "Academic research", "Facebook spam filtering (Sigma)", "Web (Yesod, Servant)"]
    },
    {
        "name": "Java", "year": 1995, "creator": "James Gosling (Sun Microsystems)",
        "paradigm": ["object-oriented", "imperative", "generic", "functional (Java 8+)"],
        "typing": "static, strong, nominative", "family": "C",
        "description": "Write Once, Run Anywhere. Java runs on the JVM (Java Virtual Machine), enabling true cross-platform execution. It became the dominant enterprise language, powering Android apps, web backends, financial systems, and big data. Java's ecosystem (Spring, Hadoop, Kafka, Elasticsearch) is enormous. Over 3 billion devices run Java.",
        "key_concepts": ["JVM bytecode", "Garbage collection", "Strong OOP", "Interfaces", "Generics", "Checked exceptions", "Streams API (Java 8)", "Records (Java 14)", "Virtual threads (Java 21)"],
        "syntax_examples": {
            "hello_world": "public class Hello {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}",
            "class": "public class Animal {\n    private String name;\n    private int age;\n\n    public Animal(String name, int age) {\n        this.name = name;\n        this.age = age;\n    }\n\n    public String speak() {\n        return name + \" is \" + age + \" years old\";\n    }\n}",
            "generics": "public class Pair<A, B> {\n    private final A first;\n    private final B second;\n\n    public Pair(A first, B second) {\n        this.first = first;\n        this.second = second;\n    }\n\n    public A getFirst() { return first; }\n    public B getSecond() { return second; }\n}",
            "streams": "List<String> names = people.stream()\n    .filter(p -> p.getAge() > 18)\n    .sorted(Comparator.comparing(Person::getName))\n    .map(Person::getName)\n    .collect(Collectors.toList());",
            "records": "public record Point(double x, double y) {\n    public double distanceTo(Point other) {\n        return Math.sqrt(Math.pow(x - other.x, 2) + Math.pow(y - other.y, 2));\n    }\n}"
        },
        "keywords": ["abstract", "assert", "boolean", "break", "byte", "case", "catch", "char", "class", "const", "continue", "default", "do", "double", "else", "enum", "extends", "final", "finally", "float", "for", "goto", "if", "implements", "import", "instanceof", "int", "interface", "long", "native", "new", "package", "private", "protected", "public", "return", "short", "static", "strictfp", "super", "switch", "synchronized", "this", "throw", "throws", "transient", "try", "void", "volatile", "while", "var", "yield", "record", "sealed", "permits", "non-sealed", "module", "requires", "exports", "opens", "uses", "provides", "with", "to", "transitive"],
        "memory_model": "JVM managed: heap (objects), stack (primitives/refs), method area, garbage collected",
        "error_handling": "try/catch/finally, checked and unchecked exceptions, try-with-resources",
        "notable_features": ["JVM platform", "3 billion devices", "Enterprise dominance", "Android (Dalvik/ART)", "Massive ecosystem"],
        "hello_world": "public class Hello {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}",
        "influence_on": ["C#", "Kotlin", "Scala", "Groovy", "Clojure", "Dart"],
        "influenced_by": ["C++", "Smalltalk", "Objective-C", "Ada"],
        "status": "active",
        "use_cases": ["Enterprise systems", "Android development", "Web backends (Spring)", "Big data (Hadoop, Spark, Kafka)", "Financial trading", "Cloud computing"]
    },
    {
        "name": "JavaScript", "year": 1995, "creator": "Brendan Eich (Netscape)",
        "paradigm": ["imperative", "functional", "object-oriented (prototype)", "event-driven"],
        "typing": "dynamic, weak", "family": "C",
        "description": "Created in 10 days, JavaScript became the language of the web. It is the ONLY language that runs natively in all web browsers. With Node.js, it expanded to server-side. Today JavaScript (and TypeScript) power frontend frameworks (React, Vue, Angular), backends (Node, Deno, Bun), mobile apps (React Native), desktop apps (Electron), and even AI/ML (TensorFlow.js).",
        "key_concepts": ["Prototypal inheritance", "First-class functions", "Closures", "Event loop", "Promises and async/await", "DOM manipulation", "JSON", "Module system (ESM/CJS)"],
        "syntax_examples": {
            "hello_world": "console.log('Hello, World!');",
            "functions": "// Arrow functions\nconst add = (a, b) => a + b;\n\n// Destructuring\nconst { name, age, ...rest } = person;\nconst [first, ...others] = array;\n\n// Spread\nconst merged = { ...obj1, ...obj2 };",
            "class": "class Animal {\n    #name; // private field\n    constructor(name) {\n        this.#name = name;\n    }\n    speak() {\n        return `${this.#name} makes a sound`;\n    }\n}\n\nclass Dog extends Animal {\n    speak() {\n        return `${super.speak()} - Woof!`;\n    }\n}",
            "async": "async function fetchData(url) {\n    try {\n        const response = await fetch(url);\n        const data = await response.json();\n        return data;\n    } catch (error) {\n        console.error('Failed:', error);\n    }\n}\n\n// Promise.all for parallel\nconst results = await Promise.all(urls.map(fetchData));",
            "functional": "const pipeline = data\n    .filter(item => item.active)\n    .map(item => ({ ...item, score: item.value * 2 }))\n    .reduce((acc, item) => acc + item.score, 0);",
            "proxy": "const handler = {\n    get(target, prop) {\n        return prop in target ? target[prop] : `No ${prop}`;\n    },\n    set(target, prop, value) {\n        console.log(`Setting ${prop} = ${value}`);\n        target[prop] = value;\n        return true;\n    }\n};\nconst proxy = new Proxy({}, handler);"
        },
        "keywords": ["break", "case", "catch", "class", "const", "continue", "debugger", "default", "delete", "do", "else", "export", "extends", "false", "finally", "for", "function", "if", "import", "in", "instanceof", "let", "new", "null", "of", "return", "static", "super", "switch", "this", "throw", "true", "try", "typeof", "undefined", "var", "void", "while", "with", "yield", "async", "await", "get", "set"],
        "memory_model": "Garbage collected (mark-and-sweep), single-threaded event loop, Web Workers for parallelism",
        "error_handling": "try/catch/finally, throw, Error objects, Promise.catch(), unhandledrejection",
        "notable_features": ["Runs in every browser", "Event-driven non-blocking I/O", "JSON (invented here)", "Largest package ecosystem (npm)", "Full-stack capable"],
        "hello_world": "console.log('Hello, World!');",
        "influence_on": ["TypeScript", "CoffeeScript", "Dart", "Elm"],
        "influenced_by": ["Scheme (closures)", "Self (prototypes)", "Java (syntax)", "Perl (regex)"],
        "status": "active",
        "use_cases": ["Web development (frontend + backend)", "Mobile (React Native)", "Desktop (Electron)", "Serverless", "IoT", "Game development", "AI/ML"]
    },
    {
        "name": "Ruby", "year": 1995, "creator": "Yukihiro Matsumoto (Matz)",
        "paradigm": ["object-oriented", "imperative", "functional", "reflective"],
        "typing": "dynamic, strong (duck typing)", "family": "SCRIPTING",
        "description": "Designed for programmer happiness. Matz wanted a language more powerful than Perl and more object-oriented than Python. Everything in Ruby is an object, including numbers and nil. Ruby on Rails (2004) revolutionized web development with 'Convention over Configuration'. Ruby's blocks and metaprogramming are legendary.",
        "key_concepts": ["Everything is an object", "Blocks and Procs", "Metaprogramming", "Duck typing", "Mixins via modules", "Open classes", "DSL-friendly syntax"],
        "syntax_examples": {
            "hello_world": "puts 'Hello, World!'",
            "class": "class Animal\n  attr_accessor :name, :sound\n\n  def initialize(name, sound)\n    @name = name\n    @sound = sound\n  end\n\n  def speak\n    \"#{@name} says #{@sound}!\"\n  end\nend\n\ndog = Animal.new('Rex', 'Woof')\nputs dog.speak",
            "blocks": "[1, 2, 3, 4, 5].select { |n| n.odd? }.map { |n| n ** 2 }\n# => [1, 9, 25]\n\n3.times { puts 'Hello' }\n\n(1..10).each_with_object([]) { |n, arr| arr << n * 2 }",
            "metaprogramming": "class Module\n  def attr_checked(name, &block)\n    define_method(\"#{name}=\") do |value|\n      raise 'Invalid!' unless block.call(value)\n      instance_variable_set(\"@#{name}\", value)\n    end\n    define_method(name) do\n      instance_variable_get(\"@#{name}\")\n    end\n  end\nend"
        },
        "keywords": ["BEGIN", "END", "alias", "and", "begin", "break", "case", "class", "def", "defined?", "do", "else", "elsif", "end", "ensure", "false", "for", "if", "in", "module", "next", "nil", "not", "or", "redo", "rescue", "retry", "return", "self", "super", "then", "true", "undef", "unless", "until", "when", "while", "yield", "require", "require_relative", "include", "extend", "prepend", "attr_reader", "attr_writer", "attr_accessor", "puts", "print", "p", "raise", "lambda", "proc", "block_given?"],
        "memory_model": "Garbage collected (mark-and-sweep + generational GC)",
        "error_handling": "begin/rescue/ensure/retry, raise, custom exception classes",
        "notable_features": ["Programmer happiness", "Rails framework", "Powerful metaprogramming", "Beautiful syntax", "Gems ecosystem"],
        "hello_world": "puts 'Hello, World!'",
        "influence_on": ["Crystal", "Elixir", "CoffeeScript", "Swift (syntax ideas)"],
        "influenced_by": ["Perl", "Smalltalk", "Lisp", "Python", "Ada", "Eiffel"],
        "status": "active",
        "use_cases": ["Web development (Rails)", "DevOps (Chef, Puppet, Vagrant)", "Scripting", "Prototyping", "Automation"]
    },
    {
        "name": "PHP", "year": 1995, "creator": "Rasmus Lerdorf",
        "paradigm": ["imperative", "object-oriented", "functional", "reflective"],
        "typing": "dynamic, weak (gradually typed in 8+)", "family": "SCRIPTING",
        "description": "Personal Home Page -> PHP: Hypertext Preprocessor. Originally a set of CGI scripts, PHP evolved into the most widely used server-side web language. WordPress, Facebook (early), Wikipedia, and millions of websites run on PHP. Modern PHP (8.x) with Laravel is a sophisticated, high-performance platform.",
        "key_concepts": ["Server-side HTML embedding", "Request-response lifecycle", "Sessions and cookies", "PDO for databases", "Composer for dependencies"],
        "syntax_examples": {
            "hello_world": "<?php\necho 'Hello, World!';\n?>",
            "class": "<?php\nclass User {\n    public function __construct(\n        private string $name,\n        private int $age,\n        private string $email\n    ) {}\n\n    public function greet(): string {\n        return \"Hi, I'm {$this->name}, age {$this->age}\";\n    }\n}\n\n$user = new User('Ali', 25, 'ali@example.com');\necho $user->greet();",
            "modern_php8": "// Match expression\n$result = match($status) {\n    'active' => 'User is active',\n    'banned' => 'User is banned',\n    default => 'Unknown status',\n};\n\n// Named arguments\narray_slice(array: $arr, offset: 2, length: 3);\n\n// Enum\nenum Color: string {\n    case Red = '#FF0000';\n    case Green = '#00FF00';\n    case Blue = '#0000FF';\n}\n\n// Fibers\n$fiber = new Fiber(function(): void {\n    $value = Fiber::suspend('fiber started');\n    echo \"Resumed with: $value\";\n});"
        },
        "keywords": ["abstract", "and", "array", "as", "break", "callable", "case", "catch", "class", "clone", "const", "continue", "declare", "default", "die", "do", "echo", "else", "elseif", "empty", "enddeclare", "endfor", "endforeach", "endif", "endswitch", "endwhile", "eval", "exit", "extends", "final", "finally", "fn", "for", "foreach", "function", "global", "goto", "if", "implements", "include", "include_once", "instanceof", "insteadof", "interface", "isset", "list", "match", "namespace", "new", "or", "print", "private", "protected", "public", "readonly", "require", "require_once", "return", "static", "switch", "throw", "trait", "try", "unset", "use", "var", "while", "xor", "yield", "yield from", "enum", "fiber"],
        "memory_model": "Reference-counted with cycle collector, per-request lifecycle",
        "error_handling": "try/catch/finally, set_error_handler, Throwable hierarchy (Error + Exception)",
        "notable_features": ["Powers 77% of websites", "WordPress ecosystem", "Laravel framework", "Modern PHP 8.x is excellent"],
        "hello_world": "<?php echo 'Hello, World!'; ?>",
        "influence_on": ["Hack (Facebook)"],
        "influenced_by": ["C", "Perl", "Java", "C++"],
        "status": "active",
        "use_cases": ["Web development", "WordPress", "E-commerce (Magento, WooCommerce)", "CMS systems", "APIs (Laravel, Symfony)"]
    },
    {
        "name": "Lua", "year": 1993, "creator": "Roberto Ierusalimschy, PUC-Rio",
        "paradigm": ["imperative", "procedural", "functional", "object-oriented (via metatables)"],
        "typing": "dynamic, strong", "family": "SCRIPTING",
        "description": "A lightweight, embeddable scripting language from Brazil. Lua's tiny footprint (< 300KB) and C API make it the gold standard for embedding in applications, especially game engines. World of Warcraft, Roblox, NGINX, Redis, and Neovim all use Lua.",
        "key_concepts": ["Tables as universal data structure", "Metatables and metamethods", "Coroutines", "First-class functions", "Minimal core, extensible"],
        "syntax_examples": {
            "hello_world": "print('Hello, World!')",
            "tables": "-- Tables are everything in Lua\nlocal person = {\n    name = 'Ali',\n    age = 25,\n    greet = function(self)\n        return 'Hi, I am ' .. self.name\n    end\n}\nprint(person:greet())",
            "metatables": "local Vector = {}\nVector.__index = Vector\n\nfunction Vector.new(x, y)\n    return setmetatable({x = x, y = y}, Vector)\nend\n\nfunction Vector.__add(a, b)\n    return Vector.new(a.x + b.x, a.y + b.y)\nend\n\nlocal v1 = Vector.new(1, 2)\nlocal v2 = Vector.new(3, 4)\nlocal v3 = v1 + v2  -- Uses __add metamethod",
            "coroutines": "local function producer()\n    local i = 0\n    return coroutine.wrap(function()\n        while true do\n            i = i + 1\n            coroutine.yield(i)\n        end\n    end)\nend\n\nlocal gen = producer()\nprint(gen(), gen(), gen())  -- 1, 2, 3"
        },
        "keywords": ["and", "break", "do", "else", "elseif", "end", "false", "for", "function", "goto", "if", "in", "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while"],
        "memory_model": "Garbage collected (incremental mark-and-sweep)",
        "error_handling": "pcall/xpcall for protected calls, error() to raise",
        "notable_features": ["Tiny footprint", "Best embeddable language", "Fastest dynamic language (LuaJIT)", "Game industry standard"],
        "hello_world": "print('Hello, World!')",
        "influence_on": ["MoonScript", "Squirrel", "Wren"],
        "influenced_by": ["Scheme", "SNOBOL", "Modula", "CLU"],
        "status": "active",
        "use_cases": ["Game scripting (WoW, Roblox, Love2D)", "Embedded systems", "Web servers (OpenResty/NGINX)", "Redis scripting", "Neovim configuration"]
    },
    {
        "name": "R", "year": 1993, "creator": "Ross Ihaka, Robert Gentleman",
        "paradigm": ["functional", "object-oriented", "array"],
        "typing": "dynamic", "family": "STATISTICAL",
        "description": "A language and environment for statistical computing and graphics. R is the lingua franca of statistics, bioinformatics, and data visualization. CRAN hosts 20,000+ packages for every imaginable statistical method.",
        "key_concepts": ["Vectorized operations", "Data frames", "Formula objects", "Factors", "S4/R5 object systems", "Tidyverse"],
        "syntax_examples": {
            "hello_world": "print('Hello, World!')",
            "data_analysis": "library(tidyverse)\n\ndf <- data.frame(\n  name = c('Alice', 'Bob', 'Charlie'),\n  age = c(25, 30, 35),\n  salary = c(50000, 60000, 55000)\n)\n\ndf %>%\n  filter(age > 25) %>%\n  mutate(bonus = salary * 0.1) %>%\n  arrange(desc(salary))",
            "visualization": "library(ggplot2)\nggplot(mtcars, aes(x = wt, y = mpg, color = factor(cyl))) +\n  geom_point(size = 3) +\n  geom_smooth(method = 'lm') +\n  theme_minimal() +\n  labs(title = 'MPG vs Weight', x = 'Weight', y = 'MPG')"
        },
        "keywords": ["if", "else", "for", "while", "repeat", "function", "return", "next", "break", "in", "TRUE", "FALSE", "NULL", "NA", "Inf", "NaN", "library", "require", "source", "print", "cat", "paste", "c", "list", "matrix", "data.frame", "factor", "apply", "sapply", "lapply", "tapply", "vapply"],
        "memory_model": "Copy-on-modify semantics, garbage collected",
        "error_handling": "tryCatch, withCallingHandlers, stop, warning, message",
        "notable_features": ["Best statistical computing language", "ggplot2 visualization", "Tidyverse ecosystem", "Bioconductor for bioinformatics"],
        "hello_world": "cat('Hello, World!\\n')",
        "influence_on": ["Julia"],
        "influenced_by": ["S", "Scheme"],
        "status": "active",
        "use_cases": ["Statistics", "Data science", "Bioinformatics", "Academic research", "Data visualization"]
    },
    {
        "name": "Rust", "year": 2010, "creator": "Graydon Hoare (Mozilla)",
        "paradigm": ["imperative", "functional", "concurrent", "generic"],
        "typing": "static, strong, inferred, affine (ownership)", "family": "C",
        "description": "A systems programming language that guarantees memory safety without a garbage collector. Rust's ownership system, borrow checker, and lifetime annotations eliminate entire classes of bugs (null pointers, data races, use-after-free) at compile time. Voted 'Most Loved Language' on Stack Overflow for 8 consecutive years. Used in Firefox, Linux kernel, Discord, Cloudflare, and AWS.",
        "key_concepts": ["Ownership", "Borrowing and lifetimes", "Pattern matching", "Traits (like interfaces)", "Enums with data", "Zero-cost abstractions", "Fearless concurrency", "No null (Option<T>)", "No exceptions (Result<T,E>)"],
        "syntax_examples": {
            "hello_world": "fn main() {\n    println!(\"Hello, World!\");\n}",
            "ownership": "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1; // s1 is MOVED, no longer valid\n    // println!(\"{}\", s1); // COMPILE ERROR!\n    println!(\"{}\", s2); // OK\n}",
            "borrowing": "fn calculate_length(s: &String) -> usize {\n    s.len()  // Borrows s, doesn't take ownership\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    let len = calculate_length(&s);\n    println!(\"Length of '{}' is {}\", s, len); // s still valid!\n}",
            "enums_and_matching": "enum Shape {\n    Circle(f64),\n    Rectangle(f64, f64),\n    Triangle { a: f64, b: f64, c: f64 },\n}\n\nfn area(shape: &Shape) -> f64 {\n    match shape {\n        Shape::Circle(r) => std::f64::consts::PI * r * r,\n        Shape::Rectangle(w, h) => w * h,\n        Shape::Triangle { a, b, c } => {\n            let s = (a + b + c) / 2.0;\n            (s * (s - a) * (s - b) * (s - c)).sqrt()\n        }\n    }\n}",
            "traits": "trait Greet {\n    fn hello(&self) -> String;\n    fn goodbye(&self) -> String {\n        String::from(\"Goodbye!\") // default implementation\n    }\n}\n\nstruct User { name: String }\n\nimpl Greet for User {\n    fn hello(&self) -> String {\n        format!(\"Hello, I'm {}!\", self.name)\n    }\n}",
            "error_handling": "use std::fs;\nuse std::io;\n\nfn read_username() -> Result<String, io::Error> {\n    let content = fs::read_to_string(\"username.txt\")?;\n    Ok(content.trim().to_string())\n}\n\nfn main() {\n    match read_username() {\n        Ok(name) => println!(\"User: {}\", name),\n        Err(e) => eprintln!(\"Error: {}\", e),\n    }\n}",
            "concurrency": "use std::thread;\nuse std::sync::{Arc, Mutex};\n\nfn main() {\n    let counter = Arc::new(Mutex::new(0));\n    let mut handles = vec![];\n\n    for _ in 0..10 {\n        let counter = Arc::clone(&counter);\n        let handle = thread::spawn(move || {\n            let mut num = counter.lock().unwrap();\n            *num += 1;\n        });\n        handles.push(handle);\n    }\n\n    for handle in handles {\n        handle.join().unwrap();\n    }\n    println!(\"Result: {}\", *counter.lock().unwrap());\n}"
        },
        "keywords": ["as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "union", "unsafe", "use", "where", "while", "yield", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual"],
        "memory_model": "Ownership + Borrowing: no GC, no manual malloc/free. Stack by default, Box<T> for heap.",
        "error_handling": "Result<T, E> and Option<T>, ? operator for propagation, panic! for unrecoverable",
        "notable_features": ["Memory safety without GC", "Ownership system", "Fearless concurrency", "Zero-cost abstractions", "Cargo package manager", "Most loved language"],
        "hello_world": "fn main() {\n    println!(\"Hello, World!\");\n}",
        "influence_on": ["Carbon", "Vale", "Mojo (safety ideas)"],
        "influenced_by": ["C++", "ML", "Haskell", "Erlang", "Swift", "Cyclone"],
        "status": "active",
        "use_cases": ["Systems programming", "WebAssembly", "CLI tools", "Web servers (Actix, Axum)", "Game engines", "Embedded", "Linux kernel", "Blockchain"]
    },
    {
        "name": "Go", "year": 2009, "creator": "Robert Griesemer, Rob Pike, Ken Thompson (Google)",
        "paradigm": ["imperative", "concurrent", "structured"],
        "typing": "static, strong, inferred", "family": "C",
        "description": "Designed at Google for simplicity, fast compilation, and built-in concurrency. Go's goroutines and channels make concurrent programming trivial. It compiles to a single static binary with no dependencies. Docker, Kubernetes, Terraform, and most cloud infrastructure tools are written in Go.",
        "key_concepts": ["Goroutines", "Channels", "Interfaces (implicit)", "No inheritance", "Multiple return values", "Defer", "Error as values", "Fast compilation"],
        "syntax_examples": {
            "hello_world": "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello, World!\")\n}",
            "goroutines": "func main() {\n    ch := make(chan string)\n\n    go func() {\n        ch <- \"Hello from goroutine!\"\n    }()\n\n    msg := <-ch\n    fmt.Println(msg)\n}",
            "interfaces": "type Shape interface {\n    Area() float64\n    Perimeter() float64\n}\n\ntype Circle struct {\n    Radius float64\n}\n\nfunc (c Circle) Area() float64 {\n    return math.Pi * c.Radius * c.Radius\n}\n\nfunc (c Circle) Perimeter() float64 {\n    return 2 * math.Pi * c.Radius\n}",
            "error_handling": "func readFile(path string) ([]byte, error) {\n    data, err := os.ReadFile(path)\n    if err != nil {\n        return nil, fmt.Errorf(\"reading %s: %w\", path, err)\n    }\n    return data, nil\n}",
            "generics": "func Map[T, U any](slice []T, f func(T) U) []U {\n    result := make([]U, len(slice))\n    for i, v := range slice {\n        result[i] = f(v)\n    }\n    return result\n}"
        },
        "keywords": ["break", "case", "chan", "const", "continue", "default", "defer", "else", "fallthrough", "for", "func", "go", "goto", "if", "import", "interface", "map", "package", "range", "return", "select", "struct", "switch", "type", "var", "append", "cap", "close", "complex", "copy", "delete", "imag", "len", "make", "new", "panic", "print", "println", "real", "recover", "any", "comparable"],
        "memory_model": "Garbage collected, goroutine stacks start small and grow, share-by-communicating",
        "error_handling": "Errors as values (error interface), panic/recover for exceptional cases",
        "notable_features": ["Goroutines (lightweight threads)", "Channels for communication", "Fast compilation", "Single binary deployment", "Built-in testing"],
        "hello_world": "package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Hello, World!\") }",
        "influence_on": ["V", "Zig (simplicity ideas)"],
        "influenced_by": ["C", "Oberon", "Limbo", "Newsqueak", "CSP"],
        "status": "active",
        "use_cases": ["Cloud infrastructure (Docker, Kubernetes)", "Microservices", "CLI tools", "DevOps", "Networking", "Distributed systems"]
    },
    {
        "name": "Swift", "year": 2014, "creator": "Chris Lattner (Apple)",
        "paradigm": ["imperative", "object-oriented", "functional", "protocol-oriented"],
        "typing": "static, strong, inferred", "family": "C",
        "description": "Apple's modern replacement for Objective-C. Swift combines safety, speed, and expressiveness for iOS, macOS, watchOS, tvOS, and server-side development. It's protocol-oriented (preferring protocols over class hierarchies) and memory-safe with ARC.",
        "key_concepts": ["Optionals", "Protocol-oriented programming", "Value types (structs)", "ARC (Automatic Reference Counting)", "Generics", "Pattern matching", "Property wrappers", "Result builders (SwiftUI DSL)", "Actors (concurrency)"],
        "syntax_examples": {
            "hello_world": "print(\"Hello, World!\")",
            "optionals": "var name: String? = \"Ali\"\nif let unwrapped = name {\n    print(\"Name is \\(unwrapped)\")\n}\n\n// Guard\nfunc greet(_ name: String?) {\n    guard let name = name else {\n        print(\"No name provided\")\n        return\n    }\n    print(\"Hello, \\(name)!\")\n}",
            "protocols": "protocol Drawable {\n    func draw() -> String\n}\n\nstruct Circle: Drawable {\n    let radius: Double\n    func draw() -> String { \"Circle(r=\\(radius))\" }\n}\n\nextension Drawable {\n    func description() -> String { \"Drawing: \\(draw())\" }\n}",
            "swiftui": "struct ContentView: View {\n    @State private var count = 0\n\n    var body: some View {\n        VStack {\n            Text(\"Count: \\(count)\")\n                .font(.largeTitle)\n            Button(\"Increment\") {\n                count += 1\n            }\n        }\n    }\n}",
            "concurrency": "func fetchUserData() async throws -> User {\n    let (data, _) = try await URLSession.shared.data(from: url)\n    return try JSONDecoder().decode(User.self, from: data)\n}\n\nTask {\n    do {\n        let user = try await fetchUserData()\n        print(user.name)\n    } catch {\n        print(\"Error: \\(error)\")\n    }\n}"
        },
        "keywords": ["associatedtype", "class", "deinit", "enum", "extension", "fileprivate", "func", "import", "init", "inout", "internal", "let", "open", "operator", "private", "protocol", "public", "rethrows", "static", "struct", "subscript", "typealias", "var", "break", "case", "continue", "default", "defer", "do", "else", "fallthrough", "for", "guard", "if", "in", "repeat", "return", "switch", "where", "while", "as", "catch", "false", "is", "nil", "self", "Self", "super", "throw", "throws", "true", "try", "async", "await", "actor", "some", "any"],
        "memory_model": "ARC (Automatic Reference Counting), value types on stack, reference types on heap",
        "error_handling": "do/try/catch, throws, Result<Success, Failure>, Optional for nil safety",
        "notable_features": ["Optionals eliminate null crashes", "Protocol-oriented programming", "SwiftUI declarative UI", "Structured concurrency"],
        "hello_world": "print(\"Hello, World!\")",
        "influence_on": ["(Actively evolving)"],
        "influenced_by": ["Objective-C", "Rust", "Haskell", "Ruby", "Python", "C#"],
        "status": "active",
        "use_cases": ["iOS/macOS/watchOS/tvOS apps", "Server-side (Vapor)", "System programming", "Machine learning (CoreML)"]
    },
    {
        "name": "Kotlin", "year": 2011, "creator": "JetBrains",
        "paradigm": ["object-oriented", "functional", "imperative"],
        "typing": "static, strong, inferred", "family": "C",
        "description": "A modern JVM language that fixed Java's pain points. Google made Kotlin the preferred language for Android development in 2019. It features null safety, data classes, coroutines, extension functions, and 100% Java interop.",
        "key_concepts": ["Null safety", "Data classes", "Coroutines", "Extension functions", "Sealed classes", "Smart casts", "Delegation", "DSL builders"],
        "syntax_examples": {
            "hello_world": "fun main() {\n    println(\"Hello, World!\")\n}",
            "null_safety": "var name: String? = null\nprintln(name?.length)  // null (safe call)\nprintln(name?.length ?: 0)  // 0 (Elvis operator)\nname = \"Kotlin\"\nprintln(name.length)  // 6 (smart cast: now non-null)",
            "data_class": "data class User(val name: String, val age: Int, val email: String)\n\nval user = User(\"Ali\", 25, \"ali@mail.com\")\nval (name, age, _) = user  // destructuring\nval older = user.copy(age = 26)  // copy with modification",
            "coroutines": "import kotlinx.coroutines.*\n\nsuspend fun fetchData(): String {\n    delay(1000)  // non-blocking delay\n    return \"Data loaded!\"\n}\n\nfun main() = runBlocking {\n    val result = async { fetchData() }\n    println(result.await())\n}",
            "extensions": "fun String.isPalindrome(): Boolean =\n    this == this.reversed()\n\nprintln(\"racecar\".isPalindrome())  // true\nprintln(\"hello\".isPalindrome())    // false",
            "dsl_builder": "fun html(init: HTML.() -> Unit): HTML {\n    val html = HTML()\n    html.init()\n    return html\n}\n\nhtml {\n    head { title(\"My Page\") }\n    body {\n        h1(\"Hello, World!\")\n        p(\"This is Kotlin DSL\")\n    }\n}"
        },
        "keywords": ["as", "break", "class", "continue", "do", "else", "false", "for", "fun", "if", "in", "interface", "is", "null", "object", "package", "return", "super", "this", "throw", "true", "try", "typealias", "typeof", "val", "var", "when", "while", "by", "catch", "constructor", "delegate", "dynamic", "field", "file", "finally", "get", "import", "init", "param", "property", "receiver", "set", "setparam", "where", "actual", "abstract", "annotation", "companion", "const", "crossinline", "data", "enum", "expect", "external", "final", "infix", "inline", "inner", "internal", "lateinit", "noinline", "open", "operator", "out", "override", "private", "protected", "public", "reified", "sealed", "suspend", "tailrec", "vararg"],
        "memory_model": "JVM garbage collected (or Kotlin/Native with ARC)",
        "error_handling": "try/catch/finally (unchecked exceptions), Result<T>, runCatching {}",
        "notable_features": ["Null safety", "Coroutines", "100% Java interop", "Android preferred language", "Multiplatform"],
        "hello_world": "fun main() = println(\"Hello, World!\")",
        "influence_on": ["(Actively evolving - Kotlin Multiplatform)"],
        "influenced_by": ["Java", "Scala", "Groovy", "C#", "Swift"],
        "status": "active",
        "use_cases": ["Android development", "Server-side (Ktor, Spring)", "Multiplatform (iOS + Android)", "Scripting", "Gradle build scripts"]
    },
    {
        "name": "TypeScript", "year": 2012, "creator": "Anders Hejlsberg (Microsoft)",
        "paradigm": ["imperative", "object-oriented", "functional", "generic"],
        "typing": "static (gradual), strong, structural", "family": "C",
        "description": "JavaScript with types. TypeScript is a strict superset of JavaScript that adds static typing, interfaces, generics, and powerful type inference. It compiles down to plain JavaScript. Created by the same person who designed C# and Delphi. TypeScript has become the standard for large-scale JavaScript development.",
        "key_concepts": ["Structural typing", "Type inference", "Union and intersection types", "Generics", "Type guards", "Conditional types", "Mapped types", "Template literal types", "Decorators"],
        "syntax_examples": {
            "hello_world": "console.log('Hello, World!');",
            "types": "interface User {\n    name: string;\n    age: number;\n    email?: string;  // optional\n    readonly id: number;\n}\n\ntype Status = 'active' | 'inactive' | 'banned';\n\nfunction greet(user: User): string {\n    return `Hello, ${user.name}!`;\n}",
            "generics": "function firstElement<T>(arr: T[]): T | undefined {\n    return arr[0];\n}\n\ninterface Repository<T> {\n    findById(id: string): Promise<T | null>;\n    findAll(): Promise<T[]>;\n    save(entity: T): Promise<T>;\n    delete(id: string): Promise<void>;\n}",
            "advanced_types": "type DeepPartial<T> = {\n    [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];\n};\n\ntype EventMap = {\n    click: MouseEvent;\n    keydown: KeyboardEvent;\n    scroll: Event;\n};\n\nfunction on<K extends keyof EventMap>(event: K, handler: (e: EventMap[K]) => void): void {\n    // ...\n}"
        },
        "keywords": ["any", "as", "asserts", "async", "await", "bigint", "boolean", "break", "case", "catch", "class", "const", "constructor", "continue", "debugger", "declare", "default", "delete", "do", "else", "enum", "export", "extends", "false", "finally", "for", "from", "function", "get", "if", "implements", "import", "in", "infer", "instanceof", "interface", "is", "keyof", "let", "module", "namespace", "never", "new", "null", "number", "object", "of", "package", "private", "protected", "public", "readonly", "require", "return", "satisfies", "set", "static", "string", "super", "switch", "symbol", "this", "throw", "true", "try", "type", "typeof", "undefined", "unique", "unknown", "var", "void", "while", "with", "yield"],
        "memory_model": "Same as JavaScript (V8 GC), types erased at runtime",
        "error_handling": "try/catch (JavaScript), type narrowing for safe access",
        "notable_features": ["Types for JavaScript", "Turing-complete type system", "Excellent IDE support", "Industry standard"],
        "hello_world": "console.log('Hello, World!');",
        "influence_on": ["(Setting the standard for typed scripting)"],
        "influenced_by": ["JavaScript", "C#", "Java", "Haskell (type system ideas)"],
        "status": "active",
        "use_cases": ["Web development (React, Angular, Vue)", "Node.js backends", "Mobile (React Native)", "Desktop (Electron)", "Full-stack"]
    },
    {
        "name": "Elixir", "year": 2011, "creator": "Jose Valim",
        "paradigm": ["functional", "concurrent", "distributed"],
        "typing": "dynamic, strong", "family": "FUNCTIONAL",
        "description": "A modern language built on the Erlang VM (BEAM). Elixir brings Ruby-like developer happiness to Erlang's battle-tested concurrency model. Phoenix framework makes it a top choice for real-time web applications (chat, live dashboards). Discord, Pinterest, and Bleacher Report use Elixir.",
        "key_concepts": ["Pattern matching", "Pipe operator", "Processes and supervision", "OTP", "Protocols and behaviours", "Macros", "Mix build tool", "LiveView (real-time UI)"],
        "syntax_examples": {
            "hello_world": "IO.puts(\"Hello, World!\")",
            "pattern_matching": "case {1, 2, 3} do\n  {1, x, 3} -> IO.puts(\"Matched with x = #{x}\")\n  _ -> IO.puts(\"No match\")\nend\n\n{:ok, result} = {:ok, 42}  # Pattern match on tuples\n[head | tail] = [1, 2, 3, 4]  # head = 1, tail = [2, 3, 4]",
            "pipe_operator": "\"hello world\"\n|> String.upcase()\n|> String.split(\" \")\n|> Enum.map(&String.reverse/1)\n|> Enum.join(\" \")\n# => \"OLLEH DLROW\"",
            "genserver": "defmodule Counter do\n  use GenServer\n\n  def start_link(initial \\\\ 0) do\n    GenServer.start_link(__MODULE__, initial, name: __MODULE__)\n  end\n\n  def increment, do: GenServer.cast(__MODULE__, :increment)\n  def get, do: GenServer.call(__MODULE__, :get)\n\n  @impl true\n  def handle_cast(:increment, count), do: {:noreply, count + 1}\n\n  @impl true\n  def handle_call(:get, _from, count), do: {:reply, count, count}\nend"
        },
        "keywords": ["def", "defp", "defmodule", "defprotocol", "defimpl", "defstruct", "defmacro", "defguard", "defexception", "do", "end", "if", "else", "unless", "case", "cond", "when", "with", "for", "fn", "receive", "after", "try", "rescue", "catch", "raise", "throw", "import", "require", "use", "alias", "quote", "unquote", "in", "and", "or", "not", "true", "false", "nil", "self", "super"],
        "memory_model": "BEAM VM: per-process heap, garbage collected independently",
        "error_handling": "try/rescue/after, with for happy-path, supervisors for fault tolerance",
        "notable_features": ["Erlang VM reliability", "Ruby-like syntax", "Phoenix framework", "LiveView for real-time UI", "Excellent concurrency"],
        "hello_world": "IO.puts \"Hello, World!\"",
        "influence_on": ["Gleam"],
        "influenced_by": ["Erlang", "Ruby", "Clojure"],
        "status": "active",
        "use_cases": ["Real-time web (Phoenix)", "Chat systems (Discord)", "IoT", "Embedded (Nerves)", "Distributed systems"]
    },
    {
        "name": "Scala", "year": 2004, "creator": "Martin Odersky",
        "paradigm": ["object-oriented", "functional", "imperative"],
        "typing": "static, strong, inferred", "family": "ML",
        "description": "Scalable Language - unifies OOP and FP on the JVM. Scala powers Apache Spark (big data), Akka (actor model), and Play Framework. Twitter, LinkedIn, and Netflix use Scala extensively.",
        "key_concepts": ["Case classes", "Pattern matching", "Implicits/Givens", "Traits", "Higher-kinded types", "For-comprehensions", "Actor model (Akka)"],
        "syntax_examples": {
            "hello_world": "@main def hello(): Unit = println(\"Hello, World!\")",
            "case_classes": "enum Shape:\n  case Circle(radius: Double)\n  case Rectangle(width: Double, height: Double)\n\ndef area(s: Shape): Double = s match\n  case Shape.Circle(r) => Math.PI * r * r\n  case Shape.Rectangle(w, h) => w * h",
            "higher_order": "val numbers = List(1, 2, 3, 4, 5)\nval doubled = numbers.map(_ * 2)\nval evens = numbers.filter(_ % 2 == 0)\nval sum = numbers.foldLeft(0)(_ + _)"
        },
        "keywords": ["abstract", "case", "catch", "class", "def", "do", "else", "enum", "export", "extends", "false", "final", "finally", "for", "given", "if", "implicit", "import", "lazy", "match", "new", "null", "object", "override", "package", "private", "protected", "return", "sealed", "super", "then", "this", "throw", "trait", "true", "try", "type", "val", "var", "while", "with", "yield", "using", "end", "extension", "inline", "opaque", "open", "transparent", "derives"],
        "memory_model": "JVM garbage collected",
        "error_handling": "try/catch/finally, Try[T], Either[L, R], Option[T]",
        "notable_features": ["FP + OOP unified", "Apache Spark", "Akka actors", "Advanced type system"],
        "hello_world": "println(\"Hello, World!\")",
        "influence_on": ["Kotlin", "Swift", "Dotty (Scala 3)"],
        "influenced_by": ["Java", "ML", "Haskell", "Erlang", "Smalltalk"],
        "status": "active",
        "use_cases": ["Big data (Apache Spark)", "Distributed systems (Akka)", "Web (Play)", "Financial systems", "Data engineering"]
    },
    {
        "name": "C#", "year": 2000, "creator": "Anders Hejlsberg (Microsoft)",
        "paradigm": ["object-oriented", "imperative", "functional", "generic", "event-driven"],
        "typing": "static, strong, nominative", "family": "C",
        "description": "Microsoft's flagship language for the .NET platform. C# combines the power of C++ with the simplicity of Visual Basic. It's the primary language for Unity game development, Windows desktop apps, web APIs (ASP.NET), and enterprise systems. Modern C# (10+) has records, pattern matching, async streams, and top-level statements.",
        "key_concepts": ["CLR and .NET runtime", "LINQ", "Async/await", "Properties", "Events and delegates", "Generics", "Extension methods", "Pattern matching", "Records", "Nullable reference types"],
        "syntax_examples": {
            "hello_world": "Console.WriteLine(\"Hello, World!\");",
            "class": "public class Animal\n{\n    public string Name { get; init; }\n    public int Age { get; set; }\n\n    public virtual string Speak() => $\"{Name} makes a sound\";\n}\n\npublic class Dog : Animal\n{\n    public override string Speak() => $\"{Name} says Woof!\";\n}",
            "linq": "var results = people\n    .Where(p => p.Age > 18)\n    .OrderBy(p => p.Name)\n    .Select(p => new { p.Name, p.Age })\n    .ToList();\n\n// Query syntax\nvar query = from p in people\n            where p.Age > 18\n            orderby p.Name\n            select new { p.Name, p.Age };",
            "async": "public async Task<string> FetchDataAsync(string url)\n{\n    using var client = new HttpClient();\n    var response = await client.GetAsync(url);\n    return await response.Content.ReadAsStringAsync();\n}",
            "pattern_matching": "string Classify(object obj) => obj switch\n{\n    int n when n > 0 => \"Positive integer\",\n    int n when n < 0 => \"Negative integer\",\n    0 => \"Zero\",\n    string s => $\"String of length {s.Length}\",\n    null => \"Null\",\n    _ => \"Unknown\"\n};",
            "records": "public record Point(double X, double Y)\n{\n    public double DistanceTo(Point other) =>\n        Math.Sqrt(Math.Pow(X - other.X, 2) + Math.Pow(Y - other.Y, 2));\n}\n\nvar p1 = new Point(0, 0);\nvar p2 = p1 with { X = 3, Y = 4 };\nConsole.WriteLine(p1.DistanceTo(p2)); // 5"
        },
        "keywords": ["abstract", "as", "base", "bool", "break", "byte", "case", "catch", "char", "checked", "class", "const", "continue", "decimal", "default", "delegate", "do", "double", "else", "enum", "event", "explicit", "extern", "false", "finally", "fixed", "float", "for", "foreach", "goto", "if", "implicit", "in", "int", "interface", "internal", "is", "lock", "long", "namespace", "new", "null", "object", "operator", "out", "override", "params", "private", "protected", "public", "readonly", "record", "ref", "return", "sbyte", "sealed", "short", "sizeof", "stackalloc", "static", "string", "struct", "switch", "this", "throw", "true", "try", "typeof", "uint", "ulong", "unchecked", "unsafe", "ushort", "using", "virtual", "void", "volatile", "while", "async", "await", "dynamic", "global", "init", "managed", "nameof", "nint", "not", "nuint", "or", "and", "unmanaged", "var", "when", "where", "with", "yield", "required", "scoped", "file"],
        "memory_model": "CLR managed: garbage collected (generational GC), value types on stack, reference types on heap, Span<T> for stack-only refs",
        "error_handling": "try/catch/finally/when, throw, custom Exceptions, nullable reference types (C# 8+)",
        "notable_features": ["LINQ", "Async/await (pioneered concept)", "Unity game development", ".NET cross-platform", "Records and pattern matching"],
        "hello_world": "Console.WriteLine(\"Hello, World!\");",
        "influence_on": ["Dart", "Swift", "Kotlin"],
        "influenced_by": ["C++", "Java", "Delphi (same creator)", "Haskell (LINQ)"],
        "status": "active",
        "use_cases": ["Unity game development", "Windows apps (WPF, WinUI)", "Web APIs (ASP.NET)", "Enterprise systems", "Cloud (Azure)", "Mobile (MAUI)"]
    },
    {
        "name": "D", "year": 2001, "creator": "Walter Bright",
        "paradigm": ["imperative", "object-oriented", "functional", "meta", "concurrent"],
        "typing": "static, strong, inferred", "family": "C",
        "description": "A systems programming language meant to fix C++ while keeping its power. D offers garbage collection (optional), compile-time function execution (CTFE), templates, and built-in unit testing.",
        "key_concepts": ["CTFE (Compile Time Function Execution)", "Mixins", "Ranges (lazy iteration)", "Contract programming", "Optional GC"],
        "syntax_examples": {
            "hello_world": "import std.stdio;\nvoid main() {\n    writeln(\"Hello, World!\");\n}"
        },
        "keywords": ["abstract", "alias", "align", "asm", "assert", "auto", "body", "bool", "break", "byte", "case", "cast", "catch", "char", "class", "const", "continue", "dchar", "debug", "default", "delegate", "delete", "deprecated", "do", "double", "else", "enum", "export", "extern", "false", "final", "finally", "float", "for", "foreach", "function", "goto", "if", "immutable", "import", "in", "inout", "int", "interface", "invariant", "is", "lazy", "long", "mixin", "module", "new", "nothrow", "null", "out", "override", "package", "pragma", "private", "protected", "public", "pure", "real", "ref", "return", "scope", "shared", "short", "static", "struct", "super", "switch", "synchronized", "template", "this", "throw", "true", "try", "typeid", "typeof", "ubyte", "uint", "ulong", "union", "unittest", "ushort", "version", "void", "wchar", "while", "with"],
        "memory_model": "Optional GC, manual with @nogc, stack allocation with scope",
        "error_handling": "try/catch/finally, Errors vs Exceptions, scope(exit/failure/success), contracts",
        "notable_features": ["CTFE", "Built-in unit testing", "Contract programming", "Ranges", "Optional GC"],
        "hello_world": "import std.stdio; void main() { writeln(\"Hello, World!\"); }",
        "influence_on": ["(Niche but respected)"],
        "influenced_by": ["C", "C++", "Java", "Python", "Lisp"],
        "status": "active",
        "use_cases": ["Systems programming", "Game development", "High-performance computing"]
    },
    {
        "name": "Dart", "year": 2011, "creator": "Lars Bak, Kasper Lund (Google)",
        "paradigm": ["object-oriented", "imperative", "functional"],
        "typing": "static, strong (sound null safety)", "family": "C",
        "description": "Google's UI-focused language, primarily known as the language behind Flutter - the cross-platform mobile framework. Dart has sound null safety, async/await, isolates for concurrency, and compiles to ARM, x64, JavaScript, and WebAssembly.",
        "key_concepts": ["Sound null safety", "Isolates", "Futures and Streams", "Mixins", "Extension methods", "Flutter widgets"],
        "syntax_examples": {
            "hello_world": "void main() {\n  print('Hello, World!');\n}",
            "class": "class Animal {\n  final String name;\n  final int age;\n\n  Animal({required this.name, required this.age});\n\n  String speak() => '$name is $age years old';\n}\n\nclass Dog extends Animal {\n  Dog({required super.name, required super.age});\n\n  @override\n  String speak() => '${super.speak()} - Woof!';\n}",
            "null_safety": "String? nullableName; // can be null\nString name = 'Dart'; // never null\n\nint? length = nullableName?.length;\nint definiteLength = nullableName?.length ?? 0;"
        },
        "keywords": ["abstract", "as", "assert", "async", "await", "base", "break", "case", "catch", "class", "const", "continue", "covariant", "default", "deferred", "do", "dynamic", "else", "enum", "export", "extends", "extension", "external", "factory", "false", "final", "finally", "for", "Function", "get", "hide", "if", "implements", "import", "in", "interface", "is", "late", "library", "mixin", "new", "null", "on", "operator", "part", "required", "rethrow", "return", "sealed", "set", "show", "static", "super", "switch", "sync", "this", "throw", "true", "try", "type", "typedef", "var", "void", "when", "while", "with", "yield"],
        "memory_model": "Garbage collected, isolates for concurrency (no shared memory)",
        "error_handling": "try/catch/finally/on, throw, custom Exception classes",
        "notable_features": ["Flutter framework", "Sound null safety", "Fast hot reload", "Compiles to native + JS + WASM"],
        "hello_world": "void main() { print('Hello, World!'); }",
        "influence_on": ["(Flutter ecosystem)"],
        "influenced_by": ["Java", "JavaScript", "C#", "Smalltalk"],
        "status": "active",
        "use_cases": ["Mobile apps (Flutter)", "Web apps (Flutter Web)", "Desktop apps", "Server-side"]
    },
    {
        "name": "Julia", "year": 2012, "creator": "Jeff Bezanson, Alan Edelman, Stefan Karpinski, Viral Shah",
        "paradigm": ["imperative", "functional", "multiple dispatch", "meta"],
        "typing": "dynamic (optionally typed), strong", "family": "SCIENTIFIC",
        "description": "Designed to solve the 'two-language problem' in scientific computing: prototype in Python, rewrite in C for performance. Julia aims for the ease of Python with the speed of C. It uses multiple dispatch as its core paradigm and JIT compiles via LLVM.",
        "key_concepts": ["Multiple dispatch", "JIT compilation (LLVM)", "Metaprogramming", "Broadcasting", "Type system with parametric types", "Macros"],
        "syntax_examples": {
            "hello_world": "println(\"Hello, World!\")",
            "functions": "function fibonacci(n)\n    n <= 1 && return n\n    fibonacci(n-1) + fibonacci(n-2)\nend\n\n# Short form\nfib(n) = n <= 1 ? n : fib(n-1) + fib(n-2)",
            "multiple_dispatch": "area(r::Float64) = π * r^2                    # Circle\narea(w::Float64, h::Float64) = w * h           # Rectangle\narea(a::Float64, b::Float64, c::Float64) = begin  # Triangle\n    s = (a + b + c) / 2\n    sqrt(s * (s-a) * (s-b) * (s-c))\nend",
            "array_ops": "A = [1 2 3; 4 5 6; 7 8 9]  # 3x3 matrix\nB = A .^ 2               # Element-wise squaring\nC = A * A'                # Matrix multiply by transpose\neigenvalues = eigvals(A)"
        },
        "keywords": ["abstract", "baremodule", "begin", "break", "catch", "const", "continue", "do", "else", "elseif", "end", "export", "false", "finally", "for", "function", "global", "if", "import", "in", "let", "local", "macro", "module", "mutable", "primitive", "quote", "return", "struct", "true", "try", "type", "using", "where", "while"],
        "memory_model": "Garbage collected, stack allocation for immutable structs",
        "error_handling": "try/catch/finally, throw, custom exception types",
        "notable_features": ["C-like speed with Python-like syntax", "Multiple dispatch", "LLVM JIT", "Excellent for scientific computing"],
        "hello_world": "println(\"Hello, World!\")",
        "influence_on": ["(Growing scientific community)"],
        "influenced_by": ["MATLAB", "Python", "R", "Lisp", "Lua", "Fortran"],
        "status": "active",
        "use_cases": ["Scientific computing", "Machine learning (Flux.jl)", "Data science", "Numerical analysis", "Climate modeling", "Pharmacology"]
    },
    {
        "name": "Zig", "year": 2016, "creator": "Andrew Kelley",
        "paradigm": ["imperative", "structured", "generic"],
        "typing": "static, strong", "family": "C",
        "description": "A modern systems language aiming to be a better C. Zig has no hidden control flow, no hidden allocations, and comptime (compile-time execution) as a first-class feature. It can interop seamlessly with C/C++ and cross-compile to 30+ platforms. Used by Uber, Cloudflare, and in the Bun JavaScript runtime.",
        "key_concepts": ["Comptime", "No hidden control flow", "No hidden allocations", "Optional values instead of null", "Error unions", "C ABI compatibility", "Cross-compilation"],
        "syntax_examples": {
            "hello_world": "const std = @import(\"std\");\n\npub fn main() void {\n    std.debug.print(\"Hello, World!\\n\", .{});\n}",
            "comptime": "fn fibonacci(comptime n: u64) u64 {\n    if (n <= 1) return n;\n    return fibonacci(n - 1) + fibonacci(n - 2);\n}\n\n// Computed at compile time!\nconst fib_10 = fibonacci(10); // == 55",
            "error_handling": "fn readFile(path: []const u8) ![]u8 {\n    const file = std.fs.cwd().openFile(path, .{}) catch |err| {\n        return err;\n    };\n    defer file.close();\n    return file.readToEndAlloc(allocator, max_size);\n}"
        },
        "keywords": ["addrspace", "align", "allowzero", "and", "anyframe", "anytype", "asm", "async", "await", "break", "callconv", "catch", "comptime", "const", "continue", "defer", "else", "enum", "errdefer", "error", "export", "extern", "false", "fn", "for", "if", "inline", "linksection", "noalias", "nosuspend", "null", "opaque", "or", "orelse", "packed", "pub", "resume", "return", "struct", "suspend", "switch", "test", "threadlocal", "true", "try", "undefined", "union", "unreachable", "var", "volatile", "while"],
        "memory_model": "Manual allocation via Allocators (no default allocator), no hidden allocations",
        "error_handling": "Error unions (!T), catch, try, errdefer",
        "notable_features": ["Comptime everything", "No hidden control flow", "C/C++ interop", "Cross-compilation to 30+ targets", "Used in Bun runtime"],
        "hello_world": "const std = @import(\"std\");\npub fn main() void { std.debug.print(\"Hello, World!\\n\", .{}); }",
        "influence_on": ["(Growing systems programming community)"],
        "influenced_by": ["C", "C++", "Rust", "LLVM IR"],
        "status": "active",
        "use_cases": ["Systems programming", "Game development", "Embedded", "Bun JavaScript runtime", "Cross-platform tools"]
    },
    {
        "name": "Nim", "year": 2008, "creator": "Andreas Rumpf",
        "paradigm": ["imperative", "functional", "object-oriented", "meta"],
        "typing": "static, strong, inferred", "family": "PYTHON_LIKE",
        "description": "A compiled language with Python-like syntax that compiles to C, C++, JavaScript, or Objective-C. Nim offers metaprogramming, zero-overhead abstractions, and garbage collection options.",
        "key_concepts": ["Compile to C/C++/JS", "Powerful macros", "Templates", "Effect system", "Uniform function call syntax"],
        "syntax_examples": {
            "hello_world": "echo \"Hello, World!\"",
            "procedure": "proc factorial(n: int): int =\n  if n <= 1: 1\n  else: n * factorial(n - 1)\n\necho factorial(10)",
            "types": "type\n  Animal = object of RootObj\n    name: string\n    age: int\n  Dog = object of Animal\n    breed: string"
        },
        "keywords": ["addr", "and", "as", "asm", "bind", "block", "break", "case", "cast", "concept", "const", "continue", "converter", "defer", "discard", "distinct", "div", "do", "elif", "else", "end", "enum", "except", "export", "finally", "for", "from", "func", "if", "import", "in", "include", "interface", "is", "isnot", "iterator", "let", "macro", "method", "mixin", "mod", "nil", "not", "notin", "object", "of", "or", "out", "proc", "ptr", "raise", "ref", "return", "shl", "shr", "static", "template", "try", "tuple", "type", "using", "var", "when", "while", "xor", "yield"],
        "memory_model": "Multiple options: ARC, ORC (cycle-aware), manual, Boehm GC",
        "error_handling": "try/except/finally, raise, {.raises: [].} effect tracking",
        "notable_features": ["Python-like syntax, C-like speed", "Compiles to C/C++/JS/ObjC", "Powerful metaprogramming", "Effect system"],
        "hello_world": "echo \"Hello, World!\"",
        "influence_on": ["(Niche but growing)"],
        "influenced_by": ["Python", "Ada", "Modula", "Lisp", "C++"],
        "status": "active",
        "use_cases": ["Systems programming", "Game development", "Web development", "Scripting", "Embedded"]
    },
    {
        "name": "Crystal", "year": 2014, "creator": "Ary Borenszweig, Juan Wajnerman (Manas Technology)",
        "paradigm": ["object-oriented", "imperative", "functional"],
        "typing": "static, strong, inferred", "family": "SCRIPTING",
        "description": "A compiled language with Ruby-like syntax. Crystal gives you Ruby's beautiful syntax with C-level performance and static type checking. If you love Ruby, Crystal is Ruby that compiles.",
        "key_concepts": ["Ruby syntax + static types", "Null safety via union types", "Macros", "Fibers for concurrency", "C bindings"],
        "syntax_examples": {
            "hello_world": "puts \"Hello, World!\"",
            "class": "class Animal\n  property name : String\n  property age : Int32\n\n  def initialize(@name, @age)\n  end\n\n  def speak\n    \"#{name} is #{age} years old\"\n  end\nend\n\ndog = Animal.new(\"Rex\", 5)\nputs dog.speak"
        },
        "keywords": ["abstract", "alias", "annotation", "as", "as?", "asm", "begin", "break", "case", "class", "def", "do", "else", "elsif", "end", "ensure", "enum", "extend", "false", "for", "fun", "if", "in", "include", "instance_sizeof", "is_a?", "lib", "macro", "module", "next", "nil", "nil?", "of", "offsetof", "out", "pointerof", "private", "protected", "puts", "require", "rescue", "responds_to?", "return", "select", "self", "sizeof", "struct", "super", "then", "true", "type", "typeof", "uninitialized", "union", "unless", "until", "verbatim", "when", "while", "with", "yield"],
        "memory_model": "Garbage collected (Boehm GC)",
        "error_handling": "begin/rescue/ensure, raise",
        "notable_features": ["Ruby syntax, C speed", "Static type inference", "Null safety", "Compiles to native"],
        "hello_world": "puts \"Hello, World!\"",
        "influence_on": ["(Growing Ruby alternative)"],
        "influenced_by": ["Ruby", "Go", "Rust", "C#"],
        "status": "active",
        "use_cases": ["Web APIs", "CLI tools", "Performance-critical applications"]
    },
    {
        "name": "V", "year": 2019, "creator": "Alexander Medvednikov",
        "paradigm": ["imperative", "functional", "concurrent"],
        "typing": "static, strong", "family": "C",
        "description": "A simple, fast systems language positioned between Go and Rust. V compiles instantly, has no undefined behavior, no null, no global state, and produces small binaries with no runtime dependencies.",
        "key_concepts": ["Simplicity", "Fast compilation", "No null", "No global state", "Autofree memory management"],
        "syntax_examples": {
            "hello_world": "fn main() {\n\tprintln('Hello, World!')\n}",
            "struct": "struct Point {\n\tx f64\n\ty f64\n}\n\nfn (p Point) distance(other Point) f64 {\n\treturn math.sqrt(math.pow(p.x - other.x, 2) + math.pow(p.y - other.y, 2))\n}"
        },
        "keywords": ["as", "assert", "asm", "atomic", "break", "const", "continue", "defer", "else", "enum", "false", "fn", "for", "go", "goto", "if", "import", "in", "interface", "is", "isreftype", "lock", "match", "module", "mut", "none", "or", "pub", "return", "rlock", "select", "shared", "sizeof", "spawn", "static", "struct", "true", "type", "typeof", "union", "unsafe", "volatile", "while"],
        "memory_model": "Autofree (compile-time memory management), optional GC, manual",
        "error_handling": "Result type (! suffix), or blocks",
        "notable_features": ["Instant compilation", "No null, no undefined behavior", "Simple syntax"],
        "hello_world": "println('Hello, World!')",
        "influence_on": ["(Emerging language)"],
        "influenced_by": ["Go", "Rust", "Kotlin", "Zig"],
        "status": "active",
        "use_cases": ["Systems programming", "Web development", "CLI tools"]
    },
    {
        "name": "Mojo", "year": 2023, "creator": "Chris Lattner (Modular)",
        "paradigm": ["imperative", "object-oriented", "functional", "systems"],
        "typing": "static + dynamic, strong", "family": "PYTHON_LIKE",
        "description": "A new language by the creator of Swift and LLVM, designed as a Python superset for AI/ML that delivers C++ performance. Mojo can run existing Python code and adds ownership, SIMD, and systems-level control. It aims to unify AI development under one language.",
        "key_concepts": ["Python superset", "Ownership system", "SIMD first-class", "fn vs def (strict vs flexible)", "Compile-time metaprogramming", "GPU programming"],
        "syntax_examples": {
            "hello_world": "fn main():\n    print(\"Hello, World!\")",
            "performance": "fn sum_simd[dt: DType, size: Int](a: SIMD[dt, size], b: SIMD[dt, size]) -> SIMD[dt, size]:\n    return a + b\n\n# Python code also works:\ndef python_function():\n    x = [1, 2, 3]\n    print(sum(x))"
        },
        "keywords": ["fn", "def", "struct", "trait", "var", "let", "alias", "owned", "borrowed", "inout", "raises", "capturing", "parameter", "from", "import", "if", "else", "elif", "for", "while", "return", "pass", "break", "continue", "and", "or", "not", "True", "False", "None", "try", "except", "finally", "raise", "with", "as", "in", "is", "lambda", "yield"],
        "memory_model": "Ownership + borrowing (Rust-like), MLIR-based compilation",
        "error_handling": "try/except (Python-compatible), raises keyword",
        "notable_features": ["Python superset", "35,000x faster than Python", "LLVM/MLIR backend", "AI/ML focused", "GPU support"],
        "hello_world": "print(\"Hello, World!\")",
        "influence_on": ["(Brand new, potentially revolutionary)"],
        "influenced_by": ["Python", "Rust", "Swift", "C++", "CUDA"],
        "status": "active (early access)",
        "use_cases": ["AI/ML", "High-performance computing", "Python acceleration", "GPU programming"]
    },
    {
        "name": "Carbon", "year": 2022, "creator": "Chandler Carruth (Google)",
        "paradigm": ["imperative", "object-oriented", "generic"],
        "typing": "static, strong", "family": "C",
        "description": "Google's experimental successor to C++. Carbon aims to be interoperable with C++ while fixing its accumulated complexity. It's designed for performance-critical software where C++ is used today but wants modern ergonomics.",
        "key_concepts": ["C++ interop", "Pattern matching", "Generics (checked)", "Modern syntax", "Memory safety goals"],
        "syntax_examples": {
            "hello_world": "package Sample api;\n\nfn Main() -> i32 {\n  Print(\"Hello, World!\");\n  return 0;\n}",
            "class": "class Circle {\n  var radius: f64;\n\n  fn Area[self: Self]() -> f64 {\n    return Math.Pi * self.radius * self.radius;\n  }\n}"
        },
        "keywords": ["abstract", "addr", "alias", "and", "api", "as", "auto", "base", "break", "case", "choice", "class", "constraint", "continue", "default", "destructor", "else", "extend", "final", "fn", "for", "forall", "friend", "if", "impl", "import", "in", "interface", "let", "library", "match", "namespace", "not", "observe", "or", "override", "package", "partial", "private", "protected", "return", "returned", "self", "then", "type", "var", "virtual", "where", "while"],
        "memory_model": "Manual with safety goals (evolving)",
        "error_handling": "TBD (evolving design)",
        "notable_features": ["Designed as C++ successor", "Full C++ interop", "Modern syntax"],
        "hello_world": "fn Main() -> i32 { Print(\"Hello, World!\"); return 0; }",
        "influence_on": ["(Future C++ replacement candidate)"],
        "influenced_by": ["C++", "Rust", "Swift", "Go", "Kotlin"],
        "status": "experimental",
        "use_cases": ["Performance-critical software", "C++ migration path", "Systems programming"]
    },
]


# ═══════════════════════════════════════════════════════════════════
#  ADDITIONAL 500+ LANGUAGE NAMES WITH BASIC INFO
#  These are real languages that exist/existed in history
# ═══════════════════════════════════════════════════════════════════

ADDITIONAL_LANGUAGES = [
    # ── 1950s ──
    ("Speedcoding", 1953, "John Backus", "early interpreter for IBM 701"),
    ("FLOW-MATIC", 1955, "Grace Hopper", "first English-like language, influenced COBOL"),
    ("COMTRAN", 1957, "Bob Bemer (IBM)", "Commercial Translator"),
    ("MATH-MATIC", 1957, "Remington Rand", "early algebraic compiler"),
    ("IPL", 1956, "Allen Newell, Cliff Shaw, Herbert Simon", "Information Processing Language, first AI language"),
    # ── 1960s ──
    ("JOVIAL", 1960, "Jules Schwartz", "Jules' Own Version of ALGOL, military systems"),
    ("GPSS", 1961, "Geoffrey Gordon (IBM)", "General Purpose Simulation System"),
    ("JOSS", 1963, "Cliff Shaw (RAND)", "JOHNNIAC Open Shop System, interactive computing"),
    ("MUMPS", 1966, "Neil Pappalardo", "Massachusetts General Hospital Utility, healthcare"),
    ("ISWIM", 1966, "Peter Landin", "If You See What I Mean, influenced ML/Haskell"),
    ("TELCOMP", 1966, "BBN", "time-sharing language"),
    ("Coral 66", 1966, "Royal Radar Establishment", "real-time military systems"),
    ("IMP", 1965, "Edinburgh", "improved ALGOL for Edinburgh IMP computer"),
    ("MAD", 1959, "Bruce Arden et al.", "Michigan Algorithm Decoder"),
    ("Neliac", 1955, "US Navy", "Navy Electronics Laboratory International ALGOL Compiler"),
    ("TRAC", 1964, "Calvin Mooers", "Text Reckoning And Compiling"),
    ("COMIT", 1957, "Victor Yngve (MIT)", "first string processing language"),
    ("Formac", 1964, "Jean Sammet (IBM)", "FORmula MAnipulation Compiler"),
    ("DYNAMO", 1958, "Jay Forrester, Phyllis Fox (MIT)", "simulation language for system dynamics"),
    ("SETL", 1969, "Jack Schwartz", "SET Language, mathematical set operations"),
    # ── 1970s ──
    ("Forth", 1970, "Charles H. Moore", "stack-based, interactive, embedded systems"),
    ("Smalltalk-72", 1972, "Alan Kay", "early Smalltalk version at Xerox PARC"),
    ("CLU", 1975, "Barbara Liskov (MIT)", "clusters, iterators, exception handling pioneer"),
    ("Modula", 1975, "Niklaus Wirth", "module system, successor to Pascal"),
    ("Icon", 1977, "Ralph Griswold", "successor to SNOBOL, goal-directed evaluation"),
    ("Modula-2", 1978, "Niklaus Wirth", "improved Modula with coroutines"),
    ("AWK", 1977, "Aho, Weinberger, Kernighan", "text processing, named after creators"),
    ("RATFOR", 1976, "Brian Kernighan", "Rational Fortran preprocessor"),
    ("GRASS", 1974, "Tom DeFanti", "graphics programming language"),
    ("SAC", 1976, "various", "Single Assignment C, array programming"),
    ("REDUCE", 1968, "Anthony Hearn", "computer algebra system"),
    ("PILOT", 1969, "John Starkweather", "programmed instruction learning"),
    ("BLISS", 1970, "Wulf, Russell, Habermann", "systems programming for DEC PDP-10"),
    ("SAIL", 1969, "Dan Swinehart, Bob Sproull", "Stanford AI Language"),
    ("Edinburgh IMP", 1966, "Edinburgh", "ALGOL-like systems language"),
    ("Mesa", 1976, "Xerox PARC", "influenced Modula-2 and Java"),
    ("Euclid", 1977, "Butler Lampson et al.", "verifiable systems programming"),
    ("Lucid", 1976, "Edward Ashcroft, William Wadge", "dataflow programming language"),
    ("SASL", 1976, "David Turner", "St Andrews Static Language, lazy functional"),
    ("FP", 1977, "John Backus", "functional programming, Turing Award lecture"),
    ("S", 1976, "John Chambers (Bell Labs)", "statistical programming, predecessor to R"),
    # ── 1980s ──
    ("PostScript", 1982, "John Warnock, Charles Geschke (Adobe)", "page description language"),
    ("Occam", 1983, "David May (INMOS)", "concurrent programming for Transputers"),
    ("Miranda", 1985, "David Turner", "lazy pure functional, influenced Haskell"),
    ("Eiffel", 1986, "Bertrand Meyer", "design by contract OOP"),
    ("Self", 1987, "David Ungar, Randall Smith", "prototype-based OOP, influenced JavaScript"),
    ("ABAP", 1983, "SAP", "Advanced Business Application Programming"),
    ("RPG", 1959, "IBM", "Report Program Generator, business"),
    ("dBase", 1979, "Wayne Ratliff", "database management language"),
    ("Clipper", 1985, "Nantucket Corp", "dBase compiler"),
    ("FoxPro", 1984, "Fox Software", "dBase-compatible database language"),
    ("Turbo Pascal", 1983, "Anders Hejlsberg (Borland)", "lightning-fast Pascal compiler"),
    ("HyperTalk", 1987, "Dan Winkler (Apple)", "HyperCard scripting, natural-language-like"),
    ("SR", 1988, "Gregory Andrews, Ronald Olsson", "Synchronizing Resources"),
    ("Oberon", 1987, "Niklaus Wirth", "successor to Modula-2"),
    ("Oberon-2", 1991, "Niklaus Wirth, Hanspeter Mössenböck", "Oberon with OOP extensions"),
    ("Modula-3", 1989, "DEC/Olivetti", "safe systems language from Modula family"),
    ("SPARK", 1988, "Bernard Carré", "Ada subset for formal verification"),
    ("Clean", 1987, "Rinus Plasmeijer", "purely functional with uniqueness types"),
    ("Oz", 1991, "Gert Smolka", "multiparadigm: logic + functional + OOP + concurrent"),
    ("Mercury", 1995, "University of Melbourne", "pure logic/functional programming"),
    ("Gofer", 1991, "Mark Jones", "Haskell-like educational language"),
    ("Turing", 1982, "Ric Holt, James Cordy", "educational language from University of Toronto"),
    ("ABC", 1987, "Leo Geurts, Lambert Meertens, Steven Pemberton", "predecessor to Python"),
    ("BETA", 1986, "Birger Møller-Pedersen et al.", "successor to Simula, pattern-based OOP"),
    ("Leda", 1993, "Timothy Budd", "multiparadigm educational language"),
    ("REXX", 1979, "Mike Cowlishaw (IBM)", "Restructured Extended Executor, scripting"),
    ("Informix-4GL", 1985, "Informix", "4th generation database language"),
    ("Natural", 1979, "Software AG", "4GL for ADABAS database"),
    ("Progress 4GL", 1984, "Progress Software", "business application development"),
    ("PowerBuilder", 1991, "Powersoft", "rapid application development"),
    ("LabVIEW", 1986, "National Instruments", "visual graphical programming for engineers"),
    ("Mathematica/Wolfram", 1988, "Stephen Wolfram", "computational language, symbolic math"),
    ("MATLAB", 1984, "Cleve Moler", "MATrix LABoratory, engineering/science"),
    ("Maple", 1982, "University of Waterloo", "symbolic computation"),
    ("SAS", 1976, "SAS Institute", "Statistical Analysis System"),
    ("SPSS", 1968, "Norman Nie, C. Hadlai Hull", "Statistical Package for the Social Sciences"),
    ("Stata", 1985, "StataCorp", "statistics and data management"),
    ("IDL", 1977, "David Stern", "Interactive Data Language, scientific visualization"),
    ("Tcl", 1988, "John Ousterhout", "Tool Command Language, scripting"),
    ("Tk", 1991, "John Ousterhout", "GUI toolkit for Tcl"),
    ("CHILL", 1980, "CCITT", "telecoms programming"),
    ("SIGNAL", 1989, "INRIA", "synchronous reactive programming"),
    ("Lustre", 1984, "Caspi, Halbwachs", "synchronous dataflow for critical systems"),
    ("Esterel", 1983, "Gérard Berry", "synchronous reactive language"),
    ("Verilog", 1984, "Phil Moorby", "hardware description language"),
    ("VHDL", 1987, "US DoD", "VHSIC Hardware Description Language"),
    ("SystemVerilog", 2002, "Accellera", "unified hardware design and verification"),
    # ── 1990s ──
    ("Java", 1995, "James Gosling (Sun Microsystems)", "write once run anywhere, JVM"),
    ("OCaml", 1996, "Xavier Leroy (INRIA)", "Objective Caml, ML with objects"),
    ("Racket", 1995, "PLT Group (Matthew Flatt)", "programmable programming language, Scheme descendant"),
    ("Rebol", 1997, "Carl Sassenrath", "messaging language, influenced Red"),
    ("Pico", 1997, "Theo D'Hondt", "educational scripting"),
    ("Pike", 1994, "Fredrik Hübinette", "C-like scripting, from LPC"),
    ("Curl", 1998, "MIT", "web content language"),
    ("Lasso", 1995, "Blue World", "web development language"),
    ("ColdFusion (CFML)", 1995, "Allaire", "web development markup"),
    ("NetRexx", 1996, "Mike Cowlishaw", "Rexx for JVM"),
    ("AMPL", 1985, "Robert Fourer, David Gay, Brian Kernighan", "mathematical programming"),
    ("OPL", 1984, "Psion", "Organiser Programming Language, mobile devices"),
    ("Seed7", 2005, "Thomas Mertes", "extensible programming language"),
    ("J", 1990, "Kenneth Iverson, Roger Hui", "APL successor, ASCII-based array language"),
    ("K", 1993, "Arthur Whitney", "array processing for finance, inspired by APL/J"),
    ("Q", 2003, "Arthur Whitney (Kx Systems)", "query language for kdb+ database"),
    ("Io", 2002, "Steve Dekorte", "pure prototype-based OOP"),
    ("Squirrel", 2003, "Alberto Demichelis", "Lua-like scripting for games"),
    ("Boo", 2003, "Rodrigo B. de Oliveira", "Python-like for .NET"),
    ("Nemerle", 2003, "University of Wroclaw", "functional/OOP for .NET with macros"),
    ("Fantom", 2005, "Brian Frank, Andy Frank", "JVM/CLR/.NET portable OOP"),
    ("Vala", 2006, "Jürg Billeter, Raffaele Sandrini", "C#-like for GNOME/GLib"),
    ("Genie", 2008, "Jamie McCracken", "Python-like syntax compiled via Vala"),
    ("Cobra", 2006, "Chuck Esterbrook", "Python-like with static typing"),
    ("Chapel", 2009, "Cray Inc.", "parallel programming for HPC"),
    ("X10", 2004, "IBM Research", "parallel programming"),
    ("Fortress", 2006, "Guy Steele (Sun)", "parallel computing, discontinued"),
    ("Cilk", 1994, "MIT", "C extension for multithreaded parallel computing"),
    ("OpenCL C", 2009, "Khronos Group", "heterogeneous computing (CPU + GPU)"),
    ("CUDA (C extension)", 2007, "NVIDIA", "GPU parallel computing"),
    ("Metal Shading Language", 2014, "Apple", "GPU shading for Apple platforms"),
    ("GLSL", 2004, "OpenGL ARB", "OpenGL Shading Language"),
    ("HLSL", 2002, "Microsoft", "High-Level Shading Language for DirectX"),
    ("SPIR-V", 2015, "Khronos Group", "intermediate language for GPU shaders"),
    ("Cg", 2002, "NVIDIA", "C for Graphics"),
    ("Processing", 2001, "Casey Reas, Ben Fry", "visual arts programming, Java-based"),
    ("p5.js", 2014, "Lauren Lee McCarthy", "Processing for JavaScript"),
    ("Max/MSP", 1988, "Miller Puckette", "visual programming for music/multimedia"),
    ("Pure Data", 1996, "Miller Puckette", "open-source visual programming for multimedia"),
    ("SuperCollider", 1996, "James McCartney", "audio synthesis programming"),
    ("ChucK", 2003, "Ge Wang, Perry Cook", "strongly-timed music programming"),
    ("Csound", 1986, "Barry Vercoe", "audio programming language"),
    ("Sonic Pi", 2012, "Sam Aaron", "live coding music in education"),
    ("TidalCycles", 2009, "Alex McLean", "live coding music patterns"),
    ("Faust", 2002, "Yann Orlarey et al.", "functional audio stream processing"),
    ("OpenSCAD", 2010, "Marius Kintel", "3D solid modeling scripting"),
    ("POV-Ray SDL", 1991, "POV-Ray Team", "ray tracing scene description"),
    ("RenderMan RSL", 1988, "Pixar", "photorealistic rendering shading"),
    ("ActionScript", 2000, "Macromedia/Adobe", "Flash programming, ECMAScript-based"),
    ("E", 1997, "Mark Miller", "secure distributed programming"),
    ("Oz/Mozart", 1991, "Gert Smolka", "multi-paradigm for AI and distributed"),
    ("Alice ML", 2000, "Saarland University", "ML with futures and lazy evaluation"),
    ("ATS", 2003, "Hongwei Xi", "Applied Type System, theorem proving in types"),
    ("Agda", 2007, "Ulf Norell", "dependently typed proof assistant/language"),
    ("Idris", 2007, "Edwin Brady", "dependently typed general-purpose language"),
    ("Coq/Gallina", 1989, "INRIA", "proof assistant with extraction to OCaml/Haskell"),
    ("Isabelle", 1986, "Lawrence Paulson", "generic proof assistant"),
    ("Lean", 2013, "Leonardo de Moura (Microsoft)", "theorem prover and programming language"),
    ("F*", 2011, "Microsoft Research/INRIA", "proof-oriented programming"),
    ("Dafny", 2009, "Rustan Leino (Microsoft)", "verification-aware programming"),
    ("Whiley", 2009, "David Pearce", "language with built-in verification"),
    ("TLA+", 1999, "Leslie Lamport", "specification language for concurrent systems"),
    ("Alloy", 2000, "Daniel Jackson (MIT)", "lightweight formal specification"),
    ("Promela", 1980, "Gerard Holzmann", "SPIN model checker language"),
    ("Z notation", 1977, "Jean-Raymond Abrial", "formal specification using set theory"),
    ("B-Method", 1996, "Jean-Raymond Abrial", "formal method for software development"),
    ("Event-B", 2005, "Jean-Raymond Abrial", "formal method for system modeling"),
    ("VDM-SL", 1996, "various", "Vienna Development Method Specification Language"),
    ("ASN.1", 1984, "ITU-T/ISO", "data structure description for telecom"),
    ("TTCN-3", 2000, "ETSI", "Testing and Test Control Notation"),
    ("Cucumber/Gherkin", 2008, "Aslak Hellesøy", "behavior-driven development"),
    ("Robot Framework", 2005, "Nokia Networks", "keyword-driven test automation"),
    ("Selenium WebDriver", 2004, "Jason Huggins", "web browser automation"),
    # ── Esoteric Languages ──
    ("Brainfuck", 1993, "Urban Müller", "minimalist 8-instruction Turing-complete language"),
    ("Befunge", 1993, "Chris Pressey", "2D grid-based programming"),
    ("Whitespace", 2003, "Edwin Brady, Chris Morris", "only spaces, tabs, linefeeds"),
    ("Shakespeare", 2001, "Karl Hasselström, Jon Åslund", "code looks like Shakespeare plays"),
    ("Piet", 2001, "David Morgan-Mar", "programs are abstract art (bitmap images)"),
    ("LOLCODE", 2007, "Adam Lindsay", "lolcat-speak programming"),
    ("Malbolge", 1998, "Ben Olmstead", "designed to be impossible to program"),
    ("INTERCAL", 1972, "Don Woods, James Lyon", "parody language with PLEASE keyword"),
    ("FALSE", 1993, "Wouter van Oortmerssen", "stack-based obfuscated"),
    ("Unlambda", 2001, "David Madore", "minimal functional, only S, K, I combinators"),
    ("GolfScript", 2007, "Darren Smith", "code golf optimization"),
    ("><> (Fish)", 2009, "various", "2D stack-based, Befunge-inspired"),
    ("ArnoldC", 2013, "Lauri Hartikka", "Arnold Schwarzenegger movie quotes"),
    ("Rockstar", 2018, "Dylan Beattie", "programs look like rock song lyrics"),
    ("Chef", 2002, "David Morgan-Mar", "programs look like cooking recipes"),
    ("Chicken", 2002, "Torbjörn Söderstedt", "only word is 'chicken'"),
    ("Ook!", 2001, "David Morgan-Mar", "Brainfuck variant using 'Ook'"),
    ("JSFuck", 2010, "Martin Kleppe", "JavaScript using only []()!+"),
    ("HQ9+", 2001, "Cliff Biffle", "Hello World, Quine, 99 Bottles, Counter"),
    ("Grass", 2007, "Hikaru Ikuta", "only w, W, and v characters"),
    ("Thue", 2000, "John Colagioia", "string rewriting system"),
    # ── Domain-Specific Languages ──
    ("LaTeX", 1984, "Leslie Lamport", "document typesetting"),
    ("TeX", 1978, "Donald Knuth", "typesetting system"),
    ("Markdown", 2004, "John Gruber", "lightweight markup"),
    ("reStructuredText", 2002, "David Goodger", "documentation markup"),
    ("AsciiDoc", 2002, "Stuart Rackham", "text document format"),
    ("YAML", 2001, "Clark Evans", "data serialization"),
    ("JSON", 2001, "Douglas Crockford", "data interchange format"),
    ("TOML", 2013, "Tom Preston-Werner", "configuration file format"),
    ("XML", 1996, "W3C", "Extensible Markup Language"),
    ("HTML", 1993, "Tim Berners-Lee", "HyperText Markup Language"),
    ("CSS", 1996, "Håkon Wium Lie, Bert Bos", "Cascading Style Sheets"),
    ("SCSS/Sass", 2006, "Hampton Catlin, Natalie Weizenbaum", "CSS preprocessor"),
    ("Less", 2009, "Alexis Sellier", "CSS preprocessor"),
    ("Stylus", 2010, "TJ Holowaychuk", "expressive CSS preprocessor"),
    ("SVG", 2001, "W3C", "Scalable Vector Graphics"),
    ("XPath", 1999, "W3C", "XML path query language"),
    ("XSLT", 1999, "W3C", "XML transformation language"),
    ("XQuery", 2007, "W3C", "XML query language"),
    ("JSONPath", 2007, "Stefan Goessner", "JSON query expressions"),
    ("jq", 2012, "Stephen Dolan", "JSON processing language"),
    ("GraphQL", 2015, "Facebook", "API query language"),
    ("Protocol Buffers", 2008, "Google", "data serialization"),
    ("Thrift", 2007, "Facebook", "cross-language service framework"),
    ("Cap'n Proto", 2013, "Kenton Varda", "fast data serialization"),
    ("FlatBuffers", 2014, "Google", "zero-copy serialization"),
    ("MessagePack", 2008, "Sadayuki Furuhashi", "binary serialization"),
    ("Avro", 2009, "Apache/Doug Cutting", "data serialization for Hadoop"),
    # ── Shell & Scripting ──
    ("sh (Bourne Shell)", 1977, "Stephen Bourne", "original Unix shell"),
    ("csh", 1978, "Bill Joy", "C-like Unix shell"),
    ("ksh", 1983, "David Korn", "Korn Shell"),
    ("Bash", 1989, "Brian Fox", "Bourne Again Shell, standard Linux shell"),
    ("Zsh", 1990, "Paul Falstad", "extended shell with plugins, Oh My Zsh"),
    ("Fish", 2005, "Axel Liljencrantz", "friendly interactive shell"),
    ("PowerShell", 2006, "Jeffrey Snover (Microsoft)", "object-oriented shell for Windows/cross-platform"),
    ("Nushell", 2019, "Jonathan Turner et al.", "structured data shell"),
    ("sed", 1974, "Lee McMahon", "stream editor"),
    ("Make", 1976, "Stuart Feldman", "build automation"),
    ("CMake", 2000, "Kitware", "cross-platform build system generator"),
    ("Meson", 2013, "Jussi Pakkanen", "fast build system"),
    ("Ninja", 2012, "Evan Martin", "small, speed-focused build system"),
    ("Bazel", 2015, "Google", "scalable build and test"),
    ("Gradle (Groovy/Kotlin DSL)", 2007, "Hans Dockter", "build automation"),
    ("Maven POM", 2004, "Apache", "project object model for Java"),
    ("Ant", 2000, "James Duncan Davidson", "Java build tool"),
    ("Rake", 2003, "Jim Weirich", "Ruby-based build tool"),
    ("npm scripts", 2010, "Isaac Schlueter", "Node.js task runner"),
    # ── Database & Query languages ──
    ("PL/SQL", 1988, "Oracle", "procedural SQL for Oracle Database"),
    ("T-SQL", 1987, "Sybase/Microsoft", "Transact-SQL for SQL Server"),
    ("PL/pgSQL", 1998, "PostgreSQL", "procedural SQL for PostgreSQL"),
    ("MySQL stored procedures", 2004, "MySQL AB", "procedural SQL for MySQL"),
    ("CQL", 2011, "Apache Cassandra", "Cassandra Query Language"),
    ("N1QL", 2015, "Couchbase", "SQL for JSON in Couchbase"),
    ("Cypher", 2011, "Neo4j", "graph database query language"),
    ("Gremlin", 2009, "Apache TinkerPop", "graph traversal language"),
    ("SPARQL", 2008, "W3C", "RDF graph query language"),
    ("Datalog", 1977, "various", "declarative logic query language"),
    ("OQL", 1993, "ODMG", "Object Query Language"),
    ("MDX", 1997, "Microsoft", "MultiDimensional eXpressions for OLAP"),
    ("DAX", 2009, "Microsoft", "Data Analysis Expressions for Power BI"),
    ("M (Power Query)", 2013, "Microsoft", "data mashup language"),
    ("HiveQL", 2010, "Apache", "SQL-like for Hadoop/Hive"),
    ("Pig Latin", 2008, "Apache/Yahoo", "data flow language for Hadoop"),
    ("Splunk SPL", 2004, "Splunk", "Search Processing Language"),
    ("KQL", 2014, "Microsoft", "Kusto Query Language for Azure"),
    ("InfluxQL", 2013, "InfluxData", "time-series database query"),
    ("Flux", 2018, "InfluxData", "functional data scripting for time series"),
    ("PromQL", 2012, "Prometheus", "monitoring query language"),
    # ── Configuration & Infrastructure ──
    ("Terraform HCL", 2014, "HashiCorp", "infrastructure as code"),
    ("Ansible YAML", 2012, "Michael DeHaan", "IT automation"),
    ("Puppet DSL", 2005, "Luke Kanies", "infrastructure configuration"),
    ("Chef Ruby DSL", 2009, "Adam Jacob", "infrastructure configuration"),
    ("SaltStack", 2011, "Thomas Hatch", "infrastructure management"),
    ("Nix", 2003, "Eelco Dolstra", "purely functional package manager language"),
    ("Dhall", 2016, "Gabriel Gonzalez", "programmable configuration"),
    ("CUE", 2019, "Marcel van Lohuizen (Google)", "configuration unification"),
    ("Jsonnet", 2014, "Google", "data templating language"),
    ("Starlark", 2018, "Google", "Python-like for Bazel build configs"),
    ("Dockerfile", 2013, "Docker Inc.", "container build instructions"),
    ("Vagrantfile", 2010, "Mitchell Hashimoto", "VM provisioning"),
    ("Helm templates", 2015, "Deis/Microsoft", "Kubernetes package templating"),
    ("Kustomize", 2018, "Kubernetes SIG", "Kubernetes configuration customization"),
    # ── Smart Contract & Blockchain ──
    ("Solidity", 2014, "Gavin Wood", "Ethereum smart contracts"),
    ("Vyper", 2017, "Ethereum community", "Pythonic smart contracts"),
    ("Move", 2019, "Facebook/Diem", "resource-oriented blockchain language"),
    ("Michelson", 2018, "Tezos", "stack-based smart contract language"),
    ("Clarity", 2020, "Blockstack/Stacks", "decidable smart contracts"),
    ("Plutus", 2021, "IOHK/Cardano", "Haskell-based smart contracts"),
    ("Cadence", 2020, "Dapper Labs/Flow", "resource-oriented smart contracts"),
    ("Ink!", 2019, "Parity/Polkadot", "Rust-based smart contracts"),
    ("TEAL", 2019, "Algorand", "Transaction Execution Approval Language"),
    ("Ride", 2018, "Waves", "non-Turing-complete smart contracts"),
    ("Scilla", 2019, "Zilliqa", "safe smart contract language"),
    # ── Mobile Specific ──
    ("Ballerina", 2017, "WSO2", "cloud-native integration language"),
    ("Ring", 2016, "Mahmoud Fayed", "practical general-purpose language"),
    ("Red", 2011, "Nenad Rakocevic", "full-stack language inspired by Rebol"),
    ("Haxe", 2005, "Nicolas Cannasse", "cross-platform, compiles to many targets"),
    ("Opa", 2011, "MLstate", "web development language"),
    ("Elm", 2012, "Evan Czaplicki", "functional language for web frontends, no runtime errors"),
    ("PureScript", 2013, "Phil Freeman", "Haskell-like for JavaScript"),
    ("ReasonML", 2016, "Jordan Walke (Facebook)", "OCaml syntax for JavaScript"),
    ("Rescript", 2020, "ReScript team", "strongly typed JavaScript compiler"),
    ("Svelte", 2016, "Rich Harris", "compiled frontend framework/language"),
    ("Imba", 2015, "Sindre Aarsaether", "full-stack language for web, compiles to JS"),
    ("Mint", 2018, "Szikszai Gusztáv", "language for single-page web apps"),
    ("Marko", 2014, "eBay", "UI component language"),
    # ── Scientific & Math ──
    ("Octave", 1988, "John Eaton", "MATLAB-compatible open source"),
    ("Scilab", 1990, "INRIA", "numerical computing"),
    ("Maxima", 1982, "William Schelter", "computer algebra, Macsyma descendant"),
    ("GAP", 1986, "GAP Group", "Groups, Algorithms, Programming"),
    ("Magma", 1993, "University of Sydney", "computational algebra"),
    ("Singular", 1984, "University of Kaiserslautern", "commutative algebra"),
    ("Macaulay2", 1992, "Daniel Grayson, Michael Stillman", "algebraic geometry"),
    ("SageMath", 2005, "William Stein", "mathematics software system"),
    ("Modelica", 1997, "Hilding Elmqvist", "equation-based modeling"),
    ("GAMS", 1976, "World Bank", "mathematical optimization"),
    ("MiniZinc", 2007, "various", "constraint modeling"),
    ("Picat", 2013, "Neng-Fa Zhou", "logic/constraint/action programming"),
    # ── Game Development ──
    ("GDScript", 2014, "Juan Linietsky", "Godot engine scripting, Python-like"),
    ("UnrealScript", 1998, "Epic Games", "Unreal Engine scripting (deprecated)"),
    ("Blueprints Visual Scripting", 2014, "Epic Games", "visual programming for Unreal"),
    ("GML (GameMaker Language)", 1999, "Mark Overmars", "GameMaker scripting"),
    ("Inform 7", 2006, "Graham Nelson", "natural language interactive fiction"),
    ("Ink", 2016, "Inkle Studios", "interactive narrative scripting"),
    ("Twine/Harlowe", 2009, "Chris Klimas", "interactive fiction/game tool"),
    ("Ren'Py", 2004, "Tom Rothamel", "visual novel engine scripting"),
    ("Scratch", 2007, "MIT Media Lab", "visual block-based programming for children"),
    ("Snap!", 2011, "UC Berkeley", "Scratch-inspired with first-class procedures"),
    ("Blockly", 2012, "Google", "visual programming library"),
    ("MakeCode", 2017, "Microsoft", "block and text coding for education"),
    ("App Inventor", 2010, "MIT/Google", "visual Android app builder"),
    ("Stencyl", 2011, "Jon Chung", "visual game development"),
    # ── Emerging & Modern ──
    ("Gleam", 2019, "Louis Pilfold", "type-safe language on BEAM VM"),
    ("Roc", 2019, "Richard Feldman", "fast, friendly functional language"),
    ("Unison", 2019, "Paul Chiusano, Rúnar Bjarnason", "content-addressed language"),
    ("Grain", 2019, "Oscar Spencer, Philip Blair", "functional language for WebAssembly"),
    ("AssemblyScript", 2017, "Daniel Wirtz", "TypeScript to WebAssembly compiler"),
    ("Jai", 2014, "Jonathan Blow", "game programming language, unreleased"),
    ("Odin", 2016, "Ginger Bill", "C alternative for systems programming"),
    ("Vale", 2019, "Evan Ovadia", "fast, safe language with generational references"),
    ("Ante", 2020, "Jake Fecher", "low-level functional language"),
    ("Koka", 2012, "Daan Leijen (Microsoft)", "effect-typed functional language"),
    ("Eff", 2012, "Andrej Bauer, Matija Pretnar", "algebraic effect handlers"),
    ("Frank", 2017, "Conor McBride et al.", "programming with effects"),
    ("Effekt", 2020, "Philipp Schuster et al.", "effect-oriented language"),
    ("Carp", 2016, "Erik Svedäng", "statically typed Lisp without GC"),
    ("Janet", 2018, "Calvin Rose", "Lisp/Clojure-like embeddable language"),
    ("Fennel", 2016, "Phil Hagelberg", "Lisp that compiles to Lua"),
    ("Hy", 2013, "Paul Tagliamonte", "Lisp dialect for Python"),
    ("Clojure", 2007, "Rich Hickey", "modern Lisp on JVM, immutable data"),
    ("ClojureScript", 2011, "Rich Hickey", "Clojure compiled to JavaScript"),
    ("Babashka", 2019, "Michiel Borkent", "fast Clojure scripting"),
    ("Joker", 2016, "Roman Bataev", "Clojure interpreter in Go"),
    ("Common Lisp", 1984, "ANSI Committee", "standardized Lisp, CLOS, condition system"),
    ("Emacs Lisp", 1985, "Richard Stallman", "Emacs editor extension language"),
    ("AutoLISP", 1986, "Autodesk", "AutoCAD customization language"),
    ("newLISP", 2001, "Lutz Mueller", "lightweight Lisp scripting"),
    ("PicoLisp", 1988, "Alexander Burger", "minimalist Lisp implementation"),
    ("Shen", 2011, "Mark Tarver", "Lisp with pattern matching and type system"),
    ("LFE", 2008, "Robert Virding", "Lisp Flavoured Erlang"),
    # ── Concatenative & Stack-based ──
    ("Factor", 2003, "Slava Pestov", "modern concatenative/stack-based"),
    ("Cat", 2006, "Christopher Diggins", "typed stack-based language"),
    ("Joy", 2001, "Manfred von Thun", "purely functional concatenative"),
    ("Kitten", 2011, "Jon Purdy", "typed concatenative programming"),
    ("Popr", 2012, "Dustin DeWeese", "functional stack-based"),
    ("RPL", 1984, "HP", "Reverse Polish Lisp for HP calculators"),
    # ── Educational & Visual ──
    ("Greenfoot", 2006, "Michael Kölling", "Java-based educational environment"),
    ("Alice", 2004, "Carnegie Mellon University", "3D programming for beginners"),
    ("BlueJ", 1999, "Michael Kölling, John Rosenberg", "Java IDE for education"),
    ("Karel", 1981, "Richard Pattis", "educational robot programming"),
    ("Kodu", 2009, "Microsoft Research", "visual game programming for kids"),
    ("Quorum", 2012, "Andreas Stefik", "evidence-based language design"),
    ("Pyret", 2015, "Shriram Krishnamurthi et al.", "educational programming language"),
    # ── Regional & Less Known ──
    ("1C:Enterprise", 1996, "1C Company (Russia)", "business application platform"),
    ("Rapira", 1980, "Soviet Union", "educational language from USSR"),
    ("Robik", 1975, "Soviet Union", "children's programming language from USSR"),
    ("PascalABC.NET", 2002, "Southern Federal University (Russia)", "modern Pascal for education"),
    ("Chinese Python (文言)", 2019, "Lingdong Huang", "classical Chinese programming"),
    ("Qalb", 2012, "Ramsey Nasser", "Arabic programming language"),
    ("Hindley", 2014, "various", "Hindi programming language"),
    ("Fjölnir", 1988, "Snorri Agnarsson", "Icelandic programming language"),
    ("Raptor", 2004, "Carlisle, Wilson et al.", "flowchart-based visual programming"),
    ("Tynker", 2012, "Tynker Inc.", "educational coding platform for kids"),
    ("Hopscotch", 2013, "Hopscotch Technologies", "iPad programming for children"),
    ("Swift Playgrounds", 2016, "Apple", "learn Swift on iPad"),
    ("Hedy", 2020, "Felienne Hermans", "gradual programming language for education"),
    # ── Assembly & Low-Level ──
    ("x86 Assembly", 1978, "Intel", "x86 processor assembly language"),
    ("ARM Assembly", 1985, "ARM Holdings", "ARM processor assembly language"),
    ("MIPS Assembly", 1985, "MIPS Technologies", "MIPS processor assembly language"),
    ("RISC-V Assembly", 2010, "UC Berkeley", "open-source ISA assembly"),
    ("6502 Assembly", 1975, "MOS Technology", "Apple II, NES, C64 processor"),
    ("Z80 Assembly", 1976, "Zilog", "ZX Spectrum, Game Boy processor"),
    ("68000 Assembly", 1979, "Motorola", "Amiga, early Mac, Sega Genesis"),
    ("WebAssembly (WAT)", 2017, "W3C", "portable binary format for web"),
    ("LLVM IR", 2003, "Chris Lattner", "LLVM intermediate representation"),
    ("SPIR-V", 2015, "Khronos", "GPU intermediate representation"),
    ("PTX", 2007, "NVIDIA", "Parallel Thread Execution for CUDA"),
    ("GAS (GNU Assembler)", 1987, "GNU Project", "assembler for GCC"),
    ("NASM", 1996, "Simon Tatham, Julian Hall", "Netwide Assembler for x86"),
    ("FASM", 1999, "Tomasz Grysztar", "flat assembler for x86"),
    ("YASM", 2001, "Peter Johnson", "rewrite of NASM"),
    # ── Workflow & Data ──
    ("Apache Beam", 2016, "Google/Apache", "unified batch and streaming"),
    ("dbt", 2016, "Fishtown Analytics", "data transformation in SQL"),
    ("Airflow DAGs", 2014, "Airbnb/Apache", "workflow orchestration in Python"),
    ("Luigi", 2012, "Spotify", "Python-based workflow management"),
    ("Prefect", 2018, "Prefect Technologies", "workflow orchestration"),
    ("Dagster", 2019, "Elementl", "data orchestration"),
    ("Nextflow", 2013, "Paolo Di Tommaso", "bioinformatics workflow"),
    ("Snakemake", 2012, "Johannes Köster", "Python-based bioinformatics workflow"),
    ("WDL", 2015, "Broad Institute", "Workflow Description Language for genomics"),
    ("CWL", 2014, "Common Workflow Language project", "portable workflow descriptions"),
    # ── AI/ML Specific ──
    ("Stan", 2012, "Andrew Gelman et al.", "Bayesian statistical modeling"),
    ("BUGS/JAGS", 1989, "MRC Cambridge", "Bayesian inference using Gibbs sampling"),
    ("Pyro", 2017, "Uber AI", "probabilistic programming in Python"),
    ("TensorFlow Graph DSL", 2015, "Google Brain", "computational graph definition"),
    ("ONNX", 2017, "Facebook/Microsoft", "Open Neural Network Exchange"),
    ("Triton", 2021, "OpenAI", "GPU kernel programming"),
    ("Halide", 2012, "MIT/Adobe", "image processing DSL"),
    ("XLA HLO", 2017, "Google", "High Level Operations for accelerators"),
    # ── More Modern & Emerging ──
    ("Pkl", 2024, "Apple", "configuration as code"),
    ("Bend", 2024, "Higher Order Company", "massively parallel functional"),
    ("Jakt", 2022, "Andreas Kling", "memory-safe C++ successor for SerenityOS"),
    ("Hylo", 2022, "Val team", "systems programming with value semantics"),
    ("Wing", 2022, "Monada", "cloud-oriented programming language"),
    ("Dark", 2018, "Paul Biggar", "deployless backend language"),
    ("Bosque", 2019, "Mark Marron (Microsoft)", "regularized programming language"),
    ("P", 2013, "Microsoft Research", "safe asynchronous event-driven programming"),
    ("Verona", 2019, "Microsoft Research", "infrastructure language with ownership"),
    ("Pony", 2014, "Sylvan Clebsch", "actor-model with reference capabilities"),
    ("Wren", 2013, "Bob Nystrom", "small class-based scripting language"),
    ("Arturo", 2019, "Yanis Zafirópulos", "simple portable programming"),
    ("Lobster", 2014, "Wouter van Oortmerssen", "game programming language"),
    ("Beef", 2019, "Brian Myers", "performance-oriented compiled language"),
    ("C3", 2019, "Christoffer Lernö", "evolution of C"),
    ("Hare", 2022, "Drew DeVault", "systems programming for Linux"),
    ("Austral", 2023, "Fernando Borretti", "linear types for systems programming"),
    ("Raku", 2015, "Larry Wall", "Perl 6 renamed, multi-paradigm"),
    ("Groovy", 2003, "James Strachan", "dynamic JVM language, Gradle's language"),
    ("CoffeeScript", 2009, "Jeremy Ashkenas", "JavaScript with cleaner syntax"),
    ("LiveScript", 2011, "George Zahariev", "CoffeeScript-inspired functional"),
    ("Hack", 2014, "Facebook", "gradually typed PHP"),
    ("Reason", 2016, "Jordan Walke (Facebook)", "OCaml for React developers"),
    ("F#", 2005, "Don Syme (Microsoft)", "ML-family functional on .NET"),
    ("Erlang", 1986, "Joe Armstrong", "fault-tolerant concurrent systems"),
    ("Wolfram Language", 2013, "Stephen Wolfram", "computational intelligence"),
    ("Stata", 1985, "StataCorp", "statistics and econometrics"),
    ("SAS", 1976, "SAS Institute", "statistical analysis"),
    ("SPSS Syntax", 1968, "SPSS Inc.", "statistical commands"),
    ("Wasm", 2017, "W3C", "portable binary instruction format"),
]


def write_detailed_language(f, lang, index):
    """Write a comprehensive entry for a detailed language"""
    f.write(f"\n{'='*80}\n")
    f.write(f"## [{index}] {lang['name']} ({lang['year']})\n")
    f.write(f"{'='*80}\n\n")
    
    f.write(f"**Creator:** {lang['creator']}\n")
    f.write(f"**Year:** {lang['year']}\n")
    f.write(f"**Family:** {lang['family']}\n")
    f.write(f"**Paradigm:** {', '.join(lang['paradigm'])}\n")
    f.write(f"**Type System:** {lang['typing']}\n")
    f.write(f"**Status:** {lang['status']}\n\n")
    
    f.write(f"### Description\n{lang['description']}\n\n")
    
    f.write(f"### Key Concepts\n")
    for concept in lang['key_concepts']:
        f.write(f"- {concept}\n")
    f.write("\n")
    
    f.write(f"### Syntax Examples\n")
    for name, code in lang['syntax_examples'].items():
        f.write(f"\n#### {name.replace('_', ' ').title()}\n")
        f.write(f"```\n{code}\n```\n")
    
    f.write(f"\n### Keywords ({len(lang['keywords'])} total)\n")
    f.write(f"```\n{', '.join(lang['keywords'])}\n```\n\n")
    
    f.write(f"### Memory Model\n{lang['memory_model']}\n\n")
    f.write(f"### Error Handling\n{lang['error_handling']}\n\n")
    
    f.write(f"### Notable Features\n")
    for feat in lang['notable_features']:
        f.write(f"- {feat}\n")
    f.write("\n")
    
    f.write(f"### Hello World\n```\n{lang['hello_world']}\n```\n\n")
    
    f.write(f"### Influenced By: {', '.join(lang['influenced_by'])}\n")
    f.write(f"### Influence On: {', '.join(lang['influence_on'])}\n\n")
    
    f.write(f"### Use Cases\n")
    for uc in lang['use_cases']:
        f.write(f"- {uc}\n")
    f.write("\n")
    
    # Generate additional teaching content
    f.write(f"### AI Teaching Notes for {lang['name']}\n")
    f.write(f"When a user asks about {lang['name']}, you should:\n")
    f.write(f"1. Explain that {lang['name']} was created in {lang['year']} by {lang['creator']}.\n")
    f.write(f"2. Describe its primary paradigm: {', '.join(lang['paradigm'])}.\n")
    f.write(f"3. Show the Hello World example first.\n")
    f.write(f"4. Explain the type system: {lang['typing']}.\n")
    f.write(f"5. Cover the key concepts one by one.\n")
    f.write(f"6. Show practical syntax examples.\n")
    f.write(f"7. Explain memory model: {lang['memory_model']}.\n")
    f.write(f"8. Describe error handling approach.\n")
    f.write(f"9. List real-world use cases.\n")
    f.write(f"10. Explain its historical significance and relationships to other languages.\n")
    f.write(f"\n### Omni-Kernel Neural Signature\n")
    f.write(f"LANG_ID: {lang['name'].upper().replace(' ', '_')}\n")
    f.write(f"FAMILY_CLUSTER: {lang['family']}\n")
    f.write(f"PARADIGM_VECTOR: [{', '.join(lang['paradigm'])}]\n")
    f.write(f"CONFIDENCE_THRESHOLD: 0.95\n")
    f.write(f"KEYWORD_COUNT: {len(lang['keywords'])}\n")
    f.write(f"SYNTAX_PATTERNS: {len(lang['syntax_examples'])}\n")
    f.write(f"MASTERY_LEVEL: COMPLETE\n\n")


def write_basic_language(f, name, year, creator, description, index):
    """Write a basic entry for additional languages"""
    f.write(f"\n{'─'*60}\n")
    f.write(f"### [{index}] {name} ({year})\n")
    f.write(f"- **Creator:** {creator}\n")
    f.write(f"- **Description:** {description}\n")
    f.write(f"- **Year:** {year}\n")
    f.write(f"- **Omni-ID:** LANG_{name.upper().replace(' ', '_').replace('/', '_').replace('#', 'SHARP').replace('.', '_').replace('(', '').replace(')', '')}\n")
    f.write(f"- **Status:** INDEXED_FOR_MASTERY\n\n")


def main():
    print("╔══════════════════════════════════════════════════════════╗")
    print("║  🧠 OMNI-ATLAS MEGA GENERATOR v1.0                      ║")
    print("║  Building the world's first complete code knowledge base ║")
    print("╚══════════════════════════════════════════════════════════╝")
    print()
    
    total_lines = 0
    
    # ── VOLUME 1: Detailed Language Entries ─────────────────────
    vol1_path = os.path.join(OUT_DIR, "vol1_master_languages.md")
    print(f"📖 Generating Volume 1: Master Language Database...")
    with open(vol1_path, "w", encoding="utf-8") as f:
        f.write("# 🌌 OMNIPEDIA VOLUME 1: MASTER LANGUAGE DATABASE\n")
        f.write(f"# Generated: {datetime.datetime.now().isoformat()}\n")
        f.write(f"# Total Detailed Languages: {len(LANGUAGES)}\n")
        f.write(f"# Purpose: Complete neural training data for Omni-Kernel AI\n\n")
        f.write("---\n\n")
        
        for i, lang in enumerate(LANGUAGES, 1):
            write_detailed_language(f, lang, i)
            if i % 10 == 0:
                print(f"  ✓ {i}/{len(LANGUAGES)} detailed languages written")
    
    with open(vol1_path, "r", encoding="utf-8") as f:
        v1_lines = sum(1 for _ in f)
    total_lines += v1_lines
    print(f"  ✅ Volume 1 complete: {v1_lines:,} lines\n")
    
    # ── VOLUME 2: Additional Languages Index ───────────────────
    vol2_path = os.path.join(OUT_DIR, "vol2_extended_index.md")
    print(f"📖 Generating Volume 2: Extended Language Index...")
    with open(vol2_path, "w", encoding="utf-8") as f:
        f.write("# 🌌 OMNIPEDIA VOLUME 2: EXTENDED LANGUAGE INDEX\n")
        f.write(f"# Generated: {datetime.datetime.now().isoformat()}\n")
        f.write(f"# Total Additional Languages: {len(ADDITIONAL_LANGUAGES)}\n")
        f.write(f"# These are real programming languages from computing history\n\n")
        f.write("---\n\n")
        
        for i, (name, year, creator, desc) in enumerate(ADDITIONAL_LANGUAGES, len(LANGUAGES)+1):
            write_basic_language(f, name, year, creator, desc, i)
            if i % 50 == 0:
                print(f"  ✓ {i - len(LANGUAGES)}/{len(ADDITIONAL_LANGUAGES)} additional languages indexed")
    
    with open(vol2_path, "r", encoding="utf-8") as f:
        v2_lines = sum(1 for _ in f)
    total_lines += v2_lines
    print(f"  ✅ Volume 2 complete: {v2_lines:,} lines\n")
    
    # ── VOLUME 3: JSON Schema Database ─────────────────────────
    vol3_path = os.path.join(OUT_DIR, "vol3_neural_schema.json")
    print(f"📖 Generating Volume 3: Neural JSON Schema...")
    schema_data = {
        "omni_atlas_version": "1.0.0",
        "generated": datetime.datetime.now().isoformat(),
        "total_languages": len(LANGUAGES) + len(ADDITIONAL_LANGUAGES),
        "detailed_languages": [],
        "indexed_languages": []
    }
    
    for lang in LANGUAGES:
        schema_data["detailed_languages"].append({
            "name": lang["name"],
            "year": lang["year"],
            "creator": lang["creator"],
            "family": lang["family"],
            "paradigm": lang["paradigm"],
            "typing": lang["typing"],
            "status": lang["status"],
            "keyword_count": len(lang["keywords"]),
            "syntax_pattern_count": len(lang["syntax_examples"]),
            "mastery_level": "COMPLETE"
        })
    
    for name, year, creator, desc in ADDITIONAL_LANGUAGES:
        schema_data["indexed_languages"].append({
            "name": name,
            "year": year,
            "creator": creator,
            "description": desc,
            "mastery_level": "INDEXED"
        })
    
    with open(vol3_path, "w", encoding="utf-8") as f:
        json.dump(schema_data, f, indent=2, ensure_ascii=False)
    
    with open(vol3_path, "r", encoding="utf-8") as f:
        v3_lines = sum(1 for _ in f)
    total_lines += v3_lines
    print(f"  ✅ Volume 3 complete: {v3_lines:,} lines\n")
    
    # ── MASTER LEDGER ──────────────────────────────────────────
    ledger_path = os.path.join(OUT_DIR, "MASTER_LEDGER.md")
    print(f"📖 Generating Master Ledger...")
    with open(ledger_path, "w", encoding="utf-8") as f:
        f.write("# 📋 OMNI-KERNEL MASTER LEDGER\n")
        f.write(f"# Total Languages in Atlas: {len(LANGUAGES) + len(ADDITIONAL_LANGUAGES)}\n")
        f.write(f"# Generated: {datetime.datetime.now().isoformat()}\n\n")
        f.write("## Status Key\n")
        f.write("- 🟢 COMPLETE = Full mastery (syntax, semantics, patterns, examples)\n")
        f.write("- 🟡 INDEXED = Name, creator, year, description recorded\n")
        f.write("- 🔴 PENDING = Not yet added\n\n")
        f.write("## Detailed Languages (COMPLETE MASTERY)\n\n")
        f.write(f"| # | Language | Year | Creator | Family | Status |\n")
        f.write(f"|---|----------|------|---------|--------|--------|\n")
        for i, lang in enumerate(LANGUAGES, 1):
            f.write(f"| {i} | **{lang['name']}** | {lang['year']} | {lang['creator']} | {lang['family']} | 🟢 COMPLETE |\n")
        
        f.write(f"\n## Indexed Languages (BASIC KNOWLEDGE)\n\n")
        f.write(f"| # | Language | Year | Creator | Status |\n")
        f.write(f"|---|----------|------|---------|--------|\n")
        for i, (name, year, creator, _) in enumerate(ADDITIONAL_LANGUAGES, len(LANGUAGES)+1):
            f.write(f"| {i} | {name} | {year} | {creator} | 🟡 INDEXED |\n")
    
    with open(ledger_path, "r", encoding="utf-8") as f:
        ledger_lines = sum(1 for _ in f)
    total_lines += ledger_lines
    
    # ── SUMMARY ────────────────────────────────────────────────
    print()
    print("╔══════════════════════════════════════════════════════════╗")
    print("║  ✅ OMNI-ATLAS GENERATION COMPLETE                       ║")
    print("╠══════════════════════════════════════════════════════════╣")
    print(f"║  📖 Volume 1 (Detailed):    {v1_lines:>8,} lines              ║")
    print(f"║  📖 Volume 2 (Extended):    {v2_lines:>8,} lines              ║")
    print(f"║  📖 Volume 3 (JSON Schema): {v3_lines:>8,} lines              ║")
    print(f"║  📖 Master Ledger:          {ledger_lines:>8,} lines              ║")
    print(f"║  ─────────────────────────────────────────              ║")
    print(f"║  📊 TOTAL LINES:            {total_lines:>8,}                  ║")
    print(f"║  📊 TOTAL LANGUAGES:        {len(LANGUAGES) + len(ADDITIONAL_LANGUAGES):>8,}                  ║")
    print(f"║  📊 COMPLETE MASTERY:       {len(LANGUAGES):>8,}                  ║")
    print(f"║  📊 INDEXED:                {len(ADDITIONAL_LANGUAGES):>8,}                  ║")
    print("╚══════════════════════════════════════════════════════════╝")


if __name__ == "__main__":
    main()
