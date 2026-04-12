;; Algorithm Shield - Next Steps
;; Generated: 2026-01-24
;; What needs to happen next

(define session-next
  '((immediate-actions
     (priority "high")
     (timeframe "today")
     (actions
      ((action-1
        (task "Load extension in Chrome")
        (steps
         '("Open chrome://extensions"
           "Enable Developer mode"
           "Click Load unpacked"
           "Select dist/ folder"))
        (expected-outcome "Extension icon appears in toolbar")
        (estimated-time "2 minutes"))

       (action-2
        (task "Test on YouTube homepage")
        (steps
         '("Navigate to https://youtube.com"
           "Open DevTools (F12) → Console"
           "Look for Algorithm Shield logs"
           "Click extension icon"
           "Verify popup opens"))
        (expected-outcome "Extension loads, popup functional")
        (estimated-time "5 minutes"))

       (action-3
        (task "Verify bubble metrics calculation")
        (steps
         '("Check console for extraction logs"
           "Look for category distribution"
           "Verify diversity score calculated"))
        (expected-outcome "Metrics calculate correctly or heuristic fallback works")
        (estimated-time "10 minutes")))))

    (this-week
     (priority "medium")
     (timeframe "2-7 days")
     (actions
      ((action-1
        (task "Complete Task #4 - Test bubble metrics")
        (blockers '("Requires manual testing on live platform"))
        (estimated-time "1 hour")
        (outcome "Task #4 marked complete or deferred to v0.5"))

       (action-2
        (task "Test all lenses on YouTube")
        (lenses '("Opposition" "Random Walk" "Time-Shift" "Serendipity"))
        (verification '("Verify URL generation" "Check tab opening" "Confirm diversity"))
        (estimated-time "30 minutes"))

       (action-3
        (task "Test all personas")
        (personas '("Gardener" "Tech Skeptic" "Art Student"))
        (verification '("Verify activation" "Check behavior changes"))
        (estimated-time "30 minutes"))

       (action-4
        (task "Create session checkpoint commit")
        (files '(".machine_readable/*.scm"))
        (message "chore: Add machine-readable session state files")
        (estimated-time "5 minutes")))))

    (this-month
     (priority "medium")
     (timeframe "1-4 weeks")
     (actions
      ((action-1
        (task "Prepare Chrome Web Store listing")
        (requirements
         '("Register developer account ($5 fee)"
           "Create ZIP bundle"
           "Write store description"
           "Create screenshots"
           "Privacy policy (optional but recommended)"))
        (estimated-time "2-3 hours"))

       (action-2
        (task "Add Firefox support")
        (changes-needed
         '("Add browser polyfill"
           "Add browser_specific_settings to manifest"
           "Test on Firefox Developer Edition"
           "Adjust CSP if needed"))
        (estimated-time "1-2 hours"))

       (action-3
        (task "Create privacy policy")
        (scope '("Data collection (none)" "Permissions justification" "Open source license"))
        (estimated-time "1 hour"))

       (action-4
        (task "Prepare release notes for v0.1.1")
        (sections '("Security improvements" "Accessibility compliance" "Documentation" "Known issues"))
        (estimated-time "30 minutes")))))

    (next-milestone-v0.5
     (target-date "March 2026")
     (priority "low")
     (timeframe "1-3 months")
     (major-features
      ((feature-1
        (name "Live bubble analysis")
        (description "Real-time feed diversity calculation in popup")
        (dependencies '("Reliable DOM extraction on all platforms")))

       (feature-2
        (name "YouTube platform support (complete)")
        (description "Full extraction and lens application on YouTube")
        (dependencies '("Stable YouTube DOM selectors")))

       (feature-3
        (name "Activity log")
        (description "Show user what the shield has done")
        (dependencies '("State persistence" "History tracking")))

       (feature-4
        (name "Bubble map visualization")
        (description "Visual representation of filter bubble")
        (dependencies '("D3.js or similar" "Chart rendering"))))))

    (deferred-items
     (priority "low")
     (reason "Not critical for v0.1.1")
     (items
      ((item-1
        (task "Comprehensive platform testing")
        (reason "DOM extraction fragile, platforms change frequently")
        (defer-to "v0.5 or user testing phase"))

       (item-2
        (task "Automated unit tests")
        (reason "Infrastructure not yet in place")
        (defer-to "v1.0"))

       (item-3
        (task "Firefox Add-ons submission")
        (reason "Chrome Web Store first, Firefox after validation")
        (defer-to "After Chrome approval"))

       (item-4
        (task "Mobile support")
        (reason "Browser extension APIs limited on mobile")
        (defer-to "v3.0+")))))

    (decision-points
     ((decision-1
       (question "Should we test Task #4 now or defer?")
       (options
        '(("test-now" "Manual testing on YouTube - 1 hour effort, may be frustrating")
          ("defer-to-v0.5" "Wait for user testing when more features complete")
          ("mark-done" "75% completion is solid, testing can be optional")))
       (recommendation "defer-to-v0.5")
       (reason "User expressed frustration with platform testing, and extension is in good shape"))

      (decision-2
       (question "Chrome Web Store submission timeline?")
       (options
        '(("submit-now" "v0.1.1 is stable, good for early adopters")
          ("wait-for-v0.5" "More features = better first impression")
          ("wait-for-v1.0" "Full feature set before public release")))
       (recommendation "submit-now")
       (reason "Get real user feedback early, iterate based on usage"))

      (decision-3
       (question "Firefox support priority?")
       (options
        '(("high" "Maximize reach, Firefox users value privacy")
          ("medium" "After Chrome validation")
          ("low" "Chrome first, Firefox later")))
       (recommendation "medium")
       (reason "1-2 hour effort after Chrome submission approved"))))

    (resources-needed
     ((resource-1
       (type "developer account")
       (item "Chrome Web Store developer registration")
       (cost "$5 one-time")
       (timeline "immediate"))

      (resource-2
       (type "testing infrastructure")
       (item "Automated testing framework")
       (cost "time investment")
       (timeline "v1.0"))

      (resource-3
       (type "user feedback")
       (item "Beta testers for real-world usage")
       (cost "none")
       (timeline "after Chrome Web Store submission")))))

;; Query helpers
(define (get-immediate-actions)
  (cadr (assoc 'actions (cadr (assoc 'immediate-actions session-next)))))

(define (get-next-milestone-features)
  (cadr (assoc 'major-features (cadr (assoc 'next-milestone-v0.5 session-next)))))

(define (get-deferred-items)
  (cadr (assoc 'items (cadr (assoc 'deferred-items session-next)))))

(define (get-decision-recommendation decision-name)
  (let ((decisions (cadr (assoc 'decision-points session-next))))
    (let ((decision (assoc decision-name decisions)))
      (cadr (assoc 'recommendation (cdr decision))))))

(define (should-defer-testing?)
  (equal? (get-decision-recommendation 'decision-1) "defer-to-v0.5"))
