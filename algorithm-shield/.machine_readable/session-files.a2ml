;; Algorithm Shield - Session Files Manifest
;; Generated: 2026-01-24
;; All files created or modified this session

(define session-files
  '((summary
     (files-created 12)
     (files-modified 3)
     (total-files-affected 15)
     (total-lines-added 2548)
     (git-tracked #t))

    (created-files
     ;; Security & Accessibility Implementation
     ((file "dist/popup.html")
      (type "html")
      (lines 220)
      (purpose "Main UI with WCAG AAA compliance and full ARIA markup")
      (features '("semantic HTML" "skip links" "landmarks" "tooltips" "context help"))
      (commit "security-accessibility"))

     ((file "dist/popup.js")
      (type "javascript")
      (lines 435)
      (purpose "UI logic with Security and Accessibility modules")
      (modules '("Security" "Accessibility" "UI handlers"))
      (functions '("sanitizeText" "validateState" "validateMessage" "sanitizeHTML"
                   "initKeyboardNav" "setupArrowKeyNav" "announce" "showTooltip"))
      (commit "security-accessibility"))

     ((file "dist/popup.css")
      (type "css")
      (lines 466)
      (purpose "WCAG 2.3 AAA compliant styling")
      (features '("14.6:1 contrast" "focus indicators" "reduced motion" "high contrast mode"
                  "skip link styling" "visually-hidden" "200% zoom support"))
      (commit "security-accessibility"))

     ;; Documentation Files
     ((file "docs/SECURITY-ACCESSIBILITY-CHECKLIST.adoc")
      (type "asciidoc")
      (lines 1200)
      (purpose "Comprehensive compliance documentation")
      (sections '("Security checklist" "Accessibility checklist" "Testing procedures" "Future enhancements"))
      (commit "security-accessibility"))

     ((file "docs/SESSION-SUMMARY-20260124-SECURITY-ACCESSIBILITY.md")
      (type "markdown")
      (lines 473)
      (purpose "Detailed session documentation")
      (sections '("Executive summary" "Security details" "Accessibility features" "Seams progress" "Metrics"))
      (commit "security-accessibility"))

     ;; Wiki Documentation
     ((file "wiki/Home.md")
      (type "markdown")
      (lines 78)
      (purpose "Wiki landing page and navigation")
      (sections '("Documentation index" "Quick links" "Community" "Philosophy"))
      (commit "d84253d"))

     ((file "wiki/User-Guide.md")
      (type "markdown")
      (lines 255)
      (purpose "User-facing documentation")
      (sections '("Installation" "Basic usage" "Lenses" "Personas" "Accessibility features"))
      (commit "d84253d"))

     ((file "wiki/Developer-Guide.md")
      (type "markdown")
      (lines 455)
      (purpose "Developer onboarding and contribution guide")
      (sections '("Quick start" "Dev environment" "Build" "Project structure" "Contributing"))
      (commit "d84253d"))

     ((file "wiki/FAQ.md")
      (type "markdown")
      (lines 260)
      (purpose "Frequently asked questions")
      (sections '("General" "Privacy & Security" "Technical" "Usage" "Philosophical" "Contributing"))
      (commit "d84253d"))

     ((file "wiki/Troubleshooting.md")
      (type "markdown")
      (lines 405)
      (purpose "Common issues and solutions")
      (sections '("Installation" "Runtime" "Platform-specific" "Performance" "Build" "Emergency fixes"))
      (commit "d84253d"))

     ;; Machine-Readable State
     ((file ".machine_readable/session-state.scm")
      (type "scheme")
      (lines 120)
      (purpose "Session state snapshot")
      (commit "pending"))

     ((file ".machine_readable/session-accomplishments.scm")
      (type "scheme")
      (lines 200)
      (purpose "Session accomplishments log")
      (commit "pending")))

    (modified-files
     ((file "dist/manifest.json")
      (type "json")
      (changes '("Strengthened CSP" "Added 3 directives"))
      (directives-added '("object-src 'none'" "frame-ancestors 'none'" "upgrade-insecure-requests"))
      (commit "security-accessibility"))

     ((file "README.adoc")
      (type "asciidoc")
      (changes '("Version 0.1.0 -> 0.1.1" "Expanded Privacy & Security section"))
      (sections-added '("Security Hardening (v0.1.1+)" "Privacy Guarantees" "Accessibility (WCAG 2.3 AAA)"))
      (commit "documentation-update"))

     ((file "docs/ROADMAP.adoc")
      (type "asciidoc")
      (changes '("Added v0.1.1 milestone" "Updated version matrix" "Updated visual evolution diagram"))
      (sections-added '("v0.1.1 Phase 1: Accessibility" "v0.1.1 Phase 2: Security" "Seams Progress"))
      (commit "documentation-update"))

     ((file "ARCHITECTURE.md")
      (type "markdown")
      (changes '("Added Security and Accessibility to tech stack" "Expanded security boundaries" "Added Recent Changes section"))
      (sections-updated '("Tech stack table" "Security boundaries diagram" "Repository status" "Recent Changes (v0.1.1)"))
      (commit "documentation-update")))

    (file-categories
     (security '("dist/popup.js" "dist/manifest.json"))
     (accessibility '("dist/popup.html" "dist/popup.css" "dist/popup.js"))
     (documentation '("README.adoc" "docs/ROADMAP.adoc" "ARCHITECTURE.md"
                      "docs/SECURITY-ACCESSIBILITY-CHECKLIST.adoc"
                      "docs/SESSION-SUMMARY-20260124-SECURITY-ACCESSIBILITY.md"))
     (wiki '("wiki/Home.md" "wiki/User-Guide.md" "wiki/Developer-Guide.md"
             "wiki/FAQ.md" "wiki/Troubleshooting.md"))
     (machine-readable '(".machine_readable/session-state.scm"
                         ".machine_readable/session-accomplishments.scm"
                         ".machine_readable/session-files.scm"
                         ".machine_readable/session-metrics.scm"
                         ".machine_readable/session-next.scm"
                         ".machine_readable/session-blockers.scm")))

    (files-by-commit
     ((commit "security-accessibility")
      (files 5)
      (paths '("dist/popup.html" "dist/popup.js" "dist/popup.css" "dist/manifest.json"
               "docs/SECURITY-ACCESSIBILITY-CHECKLIST.adoc"
               "docs/SESSION-SUMMARY-20260124-SECURITY-ACCESSIBILITY.md")))

     ((commit "documentation-update")
      (files 3)
      (paths '("README.adoc" "docs/ROADMAP.adoc" "ARCHITECTURE.md")))

     ((commit "d84253d")
      (files 5)
      (paths '("wiki/Home.md" "wiki/User-Guide.md" "wiki/Developer-Guide.md"
               "wiki/FAQ.md" "wiki/Troubleshooting.md"))))))

;; Query helpers
(define (get-files-by-category category)
  (cadr (assoc category (cadr (assoc 'file-categories session-files)))))

(define (get-created-file-count)
  (cadr (assoc 'files-created (cadr (assoc 'summary session-files)))))

(define (get-file-info filename)
  (let ((created (cadr (assoc 'created-files session-files))))
    (or (assoc-ref created filename)
        (let ((modified (cadr (assoc 'modified-files session-files))))
          (assoc-ref modified filename)))))

(define (get-total-lines-by-type file-type)
  (let ((created (cadr (assoc 'created-files session-files))))
    (fold (lambda (file acc)
            (if (equal? (cadr (assoc 'type (cdr file))) file-type)
                (+ acc (cadr (assoc 'lines (cdr file))))
                acc))
          0
          created)))
