**Changelog**
===============
**v0.2.8 - 2025-08-04**
------------------------------------
* **Added**:
   + little optimizations
   + command to replace `[]` for ""
   + goto and hover for super class
* **Fixed**: 
   + support en version "к" keyword
* **Changed**:
   + workspace initialization and logs

**v0.2.7 - 2025-07-09**
------------------------------------
* **Fixed**: 
   + crash where hovering element end of file
* **Changed**:
   + downgrade ubuntu version for linux binaries
   + keybinding for psql toggle has been replaced with ctrl+shift+r


**v0.2.6 - 2025-07-02**
------------------------------------
* **Added**:
   + constructors for class information
   + code lens for report sections (report name ) and class methods ( class name )
* **Fixed**: 
   + convert to lowercase keywords: `ложь`, `истина` and `нуль` 
   + parsing nested folders from stack.ini path only with `**` pattern
   + removed parsing `.hdl` files as startup


**v0.2.5 - 2025-06-27**
------------------------------------
* **Added**:
   + class information for class member definition
* **Fixed**: 
   + insertion of  parentheses when formatting. for example `x & 2 == 0`


**v0.2.4 - 2025-06-18**
------------------------------------
* **Added**:
   + support report files ( parsing, formatting, etc )
* **Fixed**: 
   + insertion of parentheses for the initializer has been removed. for exanple: `var x = x1 = x2 = x3 = 0;`
   + search for an identifier that has leading comments


**v0.2.3 - 2025-05-21**
------------------------------------
* **Fixed**: 
   + go to definition for functions and classes


**v0.2.2 - 2025-05-20**
------------------------------------
* **Added**:
   + some code snippets
   + normlizing keyword cases when formatting
   + go to definition for methods

* **Changed**:
   + change the line width from 120 to 150

* **Fixed**: 
   + formatting comments


**v0.2.0 - 2025-05-07**
------------------------------------
* **Added**:
   + document and range formatting

* **Changed**:
   + moved status bar on left side

* **Fixed**: 
   + parsing a time literal like `24:00`
   + parsing a function declaration like `func Object.Subject_API() {}`


**v0.1.5 - 2025-03-20**
------------------------------------

* **Fixed**: 
   + start linux server
   + selection range for definition
   + remove parsing for report files

**v0.1.4 - 2025-03-18**
------------------------------------

* **Fixed**: 
   + go to some definitions
   + styling documentation

* **Feat**: 
   + add support "входитв" keyword

**v0.1.2 - 2025-03-17**
------------------------------------

* **Fixed**: 
   + moving on to definitions
   + hover description 

**v0.1.1 - 2025-03-14**
------------------------------------

* **Fixed**: 
   + fix server

**v0.1.0 - 2025-03-14**
------------------------------------

* **Improved**: 
   + improve parsing performance
* **Added**: 
   + displaying all diagnostics

**v0.0.6 - 2024-10-30**
------------------------------------

* **Added**: 
   + linux support


**v0.0.5 - 2024-10-28**
------------------------------------

* **Improved**: 
   + definitions parsing speed


**v0.0.4 - 2024-10-17**
------------------------------------

* **Fixed**: 
   + `int~[]~` converted to `int[]` 


**v0.0.3 - 2024-10-02**
------------------------------------

* **Added**: 
   + show parameters functions
   + added escape # for function descriptions