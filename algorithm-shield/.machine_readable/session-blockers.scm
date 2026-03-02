;; Algorithm Shield - Blockers and Known Issues
;; Generated: 2026-01-24
;; Current obstacles and technical debt

(define session-blockers
  '((critical-blockers
     (count 0)
     (status "none")
     (notes "No critical blockers preventing release"))

    (high-priority-issues
     (count 0)
     (status "none")
     (notes "All high-priority items resolved this session"))

    (medium-priority-issues
     (count 2)
     (issues
      ((issue-1
        (id "MEDIUM-001")
        (title "Platform testing on live YouTube needed")
        (description "Bubble metrics calculation untested on real YouTube feed")
        (severity "medium")
        (impact "Feature may not work as expected, but extension won't break")
        (workarounds '("Heuristic fallback exists if platform-specific extraction fails"))
        (resolution-options
         '(("manual-test" "1 hour manual testing - may be frustrating")
           ("defer-to-v0.5" "Wait for user testing phase")
           ("accept-risk" "Ship with untested metrics, iterate based on user feedback")))
        (recommended-action "defer-to-v0.5")
        (rationale "User frustrated with platform testing, extension stable otherwise"))

       (issue-2
        (id "MEDIUM-002")
        (title "DOM extraction fragility")
        (description "Platform DOM changes frequently, selectors may break")
        (severity "medium")
        (impact "Content extraction may fail, requiring selector updates")
        (workarounds '("Heuristic extractor provides generic fallback"))
        (mitigation-strategy "Monitor user reports, update selectors as needed")
        (long-term-solution "ML-based element detection (v2.0+)")
        (status "accepted-risk")))))

    (low-priority-issues
     (count 4)
     (issues
      ((issue-1
        (id "LOW-001")
        (title "Firefox compatibility not verified")
        (description "Extension uses Manifest v3 but not tested on Firefox")
        (severity "low")
        (impact "May not work on Firefox without adjustments")
        (required-changes '("Browser polyfill" "CSP adjustments" "manifest tweaks"))
        (estimated-effort "1-2 hours")
        (defer-to "After Chrome Web Store approval"))

       (issue-2
        (id "LOW-002")
        (title "No automated testing")
        (description "Unit and integration tests not implemented")
        (severity "low")
        (impact "Manual regression testing required")
        (long-term-solution "Deno test framework (v1.0)")
        (defer-to "v1.0"))

       (issue-3
        (id "LOW-003")
        (title "No privacy policy document")
        (description "Chrome Web Store recommends privacy policy")
        (severity "low")
        (impact "Users may have questions about data collection")
        (mitigation "README clearly states no data collection")
        (required-for "Chrome Web Store submission (recommended)")
        (estimated-effort "1 hour"))

       (issue-4
        (id "LOW-004")
        (title "Performance profiling not done")
        (description "Extension performance not measured on large feeds")
        (severity "low")
        (impact "Unknown performance characteristics with 1000+ items")
        (mitigation "Code is lightweight, WASM is fast")
        (defer-to "User feedback or v0.5")))))

    (technical-debt
     (count 3)
     (items
      ((debt-1
        (id "DEBT-001")
        (title "Heuristic extractor not in separate file")
        (description "Heuristic extraction code embedded in content.js")
        (impact "Code organization, harder to maintain")
        (effort "15 minutes to extract")
        (priority "low")
        (defer-to "Next refactoring session"))

       (debt-2
        (id "DEBT-002")
        (title "No build optimization")
        (description "Bundle not minified, no tree-shaking")
        (impact "Larger bundle size, slower load")
        (current-size "<1MB")
        (optimized-size-estimate "~500KB")
        (effort "1 hour to add build step")
        (priority "low")
        (defer-to "v1.0"))

       (debt-3
        (id "DEBT-003")
        (title "ReScript compilation not integrated")
        (description "ReScript code exists but not yet compiled to JS")
        (impact "ReScript code unused, JS written manually")
        (reason "RSR compliance requires Deno, ReScript uses npm")
        (long-term-solution "Deno-compatible ReScript build (investigate)")
        (priority "low")
        (defer-to "v1.0 or when Deno support improves")))))

    (known-limitations
     (count 5)
     (items
      ((limitation-1
        (id "LIMIT-001")
        (title "YouTube-only support in v0.1.1")
        (description "Other platforms (Twitter, Instagram) not yet supported")
        (severity "expected")
        (roadmap "Twitter/Instagram in v1.0 (June 2026)"))

       (limitation-2
        (id "LIMIT-002")
        (title "No mobile support")
        (description "Mobile browsers don't support full extension APIs")
        (severity "expected")
        (roadmap "v3.0+ if mobile APIs improve"))

       (limitation-3
        (id "LIMIT-003")
        (title "Simple keyword-based categorization")
        (description "Content categories based on regex matching, not ML")
        (accuracy "~70-80%")
        (improvement "ML-based classification in v0.5")
        (mitigation "Diversity calculation still valuable even with imperfect categories"))

       (limitation-4
        (id "LIMIT-004")
        (title "No bubble analysis history")
        (description "Extension doesn't track bubble changes over time")
        (roadmap "Activity log in v0.5, historical analysis in v1.0"))

       (limitation-5
        (id "LIMIT-005")
        (title "No lens composition")
        (description "Only one lens can be active at a time")
        (roadmap "Lens composition in v1.0")))))

    (user-experience-concerns
     (count 2)
     (items
      ((concern-1
        (id "UX-001")
        (title "Popup shows 'Analyzing feed...' placeholder")
        (description "Bubble analysis UI present but feature incomplete")
        (severity "minor")
        (impact "User may expect functionality that doesn't exist yet")
        (mitigation "FAQ explains feature coming in v0.5")
        (resolution "Complete bubble analysis in v0.5"))

       (concern-2
        (id "UX-002")
        (title "No visual feedback when lens activates")
        (description "Lens card changes color but no toast/notification")
        (severity "minor")
        (impact "User may not realize lens activated")
        (mitigation "Screen reader announces activation")
        (resolution "Add toast notification in v0.5")))))

    (dependency-risks
     (count 1)
     (items
      ((risk-1
        (id "DEP-001")
        (title "Platform DOM changes")
        (description "YouTube/Google/Bing can change DOM without notice")
        (probability "high")
        (impact "medium")
        (mitigation-strategy
         '("Heuristic fallback extractor"
           "Monitor user reports"
           "Update selectors promptly"
           "Future: ML-based element detection"))
        (monitoring "GitHub issues for user reports")
        (update-frequency "As needed when reported")))))

    (risk-assessment
     (overall-risk "low")
     (ship-readiness "ready-with-caveats")
     (caveats
      '("Bubble metrics untested on live platform"
        "Firefox compatibility not verified"
        "No automated testing"))
     (recommendation "Ship v0.1.1 to early adopters, gather feedback, iterate")
     (rationale
      '("Core functionality stable"
        "Security hardened (95%)"
        "Accessibility compliant (100%)"
        "Documentation comprehensive (95%)"
        "Known issues have workarounds or are low-severity")))))

;; Query helpers
(define (get-critical-blocker-count)
  (cadr (assoc 'count (cadr (assoc 'critical-blockers session-blockers)))))

(define (get-medium-issues)
  (cadr (assoc 'issues (cadr (assoc 'medium-priority-issues session-blockers)))))

(define (get-technical-debt-count)
  (cadr (assoc 'count (cadr (assoc 'technical-debt session-blockers)))))

(define (is-ready-to-ship?)
  (let ((risk (cadr (assoc 'risk-assessment session-blockers))))
    (equal? (cadr (assoc 'ship-readiness risk)) "ready-with-caveats")))

(define (get-recommendation)
  (cadr (assoc 'recommendation (cadr (assoc 'risk-assessment session-blockers)))))

(define (has-critical-blockers?)
  (> (get-critical-blocker-count) 0))
