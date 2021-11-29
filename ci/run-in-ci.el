;;; A simple CI hook to run the slide builder function with all
;;; required packages loaded

(add-to-list 'load-path "/usr/share/emacs/26.3/lisp/")
(print load-path)

(require 'ox-reveal)
(require 'htmlize)
(require 'export-course)

(export-course)
