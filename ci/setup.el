;; Evaluate this file to set-up emacs in CI for building slides

(require 'package)
(add-to-list 'package-archives '("melpa" . "http://melpa.org/packages/"))
(package-initialize)
(setq package-selected-packages
      '(org-mode
        ox-reveal
        htmlify))
(package-install-selected-packages)

(package-refresh-contents)

(save-buffers-kill-emacs)

