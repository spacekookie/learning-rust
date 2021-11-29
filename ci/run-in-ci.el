(add-to-list 'load-path "/usr/share/emacs/26.3/lisp/")
(print load-path)

(require 'ox-reveal)
(require 'export-course)

(export-course)
