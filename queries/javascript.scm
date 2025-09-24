;; Functions
(function_declaration
  name: (identifier) @fname
  parameters: (formal_parameters) @fparams)

;; Methods
(method_definition
  name: (property_identifier) @mname
  parameters: (formal_parameters) @mparams)

;; Classes
(class_declaration
  name: (identifier) @cname)

;; Variable assigned arrow function
(lexical_declaration
  (variable_declarator
    name: (identifier) @vname
    value: (arrow_function
      parameters: (formal_parameters) @vparams) @is_arrow))

;; Variable assigned function expression
(lexical_declaration
  (variable_declarator
    name: (identifier) @vname
    value: (function
      (formal_parameters) @vparams)))

