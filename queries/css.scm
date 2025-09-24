;; CSS Classes
(class_selector) @css_class

;; CSS IDs
(id_selector) @css_id

;; CSS Element selectors
(tag_name) @css_element

;; CSS Keyframes
(keyframes_statement
  (keyframes_name) @keyframe_name)

;; CSS Custom Properties (CSS Variables)
(property_name) @css_property
  (#match? @css_property "^--")

;; CSS At-rules (@media, @import, etc)
(at_rule
  (at_keyword) @at_rule_name)