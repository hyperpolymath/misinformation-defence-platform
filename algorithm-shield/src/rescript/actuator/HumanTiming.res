// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Human-like timing to evade bot detection

// Random number generator helpers
module Random = {
  @val external random: unit => float = "Math.random"

  // Random float in range [min, max]
  let range = (min: float, max: float): float => {
    min +. random() *. (max -. min)
  }

  // Random int in range [min, max] inclusive
  let rangeInt = (min: int, max: int): int => {
    Float.toInt(Float.fromInt(min) +. random() *. Float.fromInt(max - min + 1))
  }

  // Random boolean with probability p
  let probability = (p: float): bool => {
    random() < p
  }
}

// Timing profiles for different interaction types
module TimingProfile = {
  type t = {
    baseDelay: float,        // Base delay in ms
    jitter: float,           // Random variation (±)
    accelerationCurve: bool, // Whether to use acceleration (slower start)
  }

  // Reading/thinking before click
  let preClick: t = {
    baseDelay: 800.0,
    jitter: 400.0,
    accelerationCurve: true,
  }

  // Brief pause between actions
  let betweenActions: t = {
    baseDelay: 300.0,
    jitter: 200.0,
    accelerationCurve: false,
  }

  // Scroll pacing (per scroll event)
  let scrollPace: t = {
    baseDelay: 150.0,
    jitter: 100.0,
    accelerationCurve: false,
  }

  // Tab switch hesitation
  let tabSwitch: t = {
    baseDelay: 1200.0,
    jitter: 600.0,
    accelerationCurve: true,
  }

  // Generate actual delay from profile
  let generateDelay = (profile: t): float => {
    let base = profile.baseDelay
    let variation = Random.range(-.profile.jitter, profile.jitter)
    let delay = base +. variation

    // Apply acceleration curve if enabled (slower reaction at start)
    if profile.accelerationCurve && Random.probability(0.3) {
      delay *. Random.range(1.2, 1.8)
    } else {
      delay
    }
  }
}

// Mouse movement simulation
module MouseMovement = {
  type point = {x: int, y: int}

  // Generate random click point within element bounds
  let generateClickPoint = (rect: {
    "left": float,
    "top": float,
    "width": float,
    "height": float,
  }): point => {
    // Avoid perfect center, use gaussian-ish distribution
    let xNorm = Random.range(0.3, 0.7) // 30-70% across width
    let yNorm = Random.range(0.3, 0.7)

    {
      x: Float.toInt(rect["left"] +. rect["width"] *. xNorm),
      y: Float.toInt(rect["top"] +. rect["height"] *. yNorm),
    }
  }

  // Simulate mouse movement curve (simple linear for now)
  // Future: Bezier curves for more realistic movement
  let generateMovementPath = (
    _from: point,
    _to: point,
    _steps: int,
  ): array<point> => {
    // Placeholder - actual implementation would generate
    // a curved path with velocity variation
    []
  }
}

// Scroll behavior
module ScrollBehavior = {
  type scrollPattern =
    | Continuous(int) // Pixels per scroll event
    | Discrete(int)   // Fixed jumps
    | Variable        // Random amounts

  // Generate scroll amount based on pattern
  let generateScrollAmount = (pattern: scrollPattern): int => {
    switch pattern {
    | Continuous(base) => base + Random.rangeInt(-20, 20)
    | Discrete(jump) => jump
    | Variable => Random.rangeInt(100, 500)
    }
  }

  // Scroll inertia simulation (gradually slow down)
  let generateInertia = (initialVelocity: float): array<float> => {
    let friction = 0.92 // Deceleration factor
    let threshold = 1.0  // Stop when velocity < threshold

    let velocities = []
    let mutableVel = initialVelocity

    while mutableVel > threshold {
      velocities->Array.push(mutableVel)->ignore
      mutableVel = mutableVel *. friction
    }

    velocities
  }
}

// Typing simulation (for search/comments)
module TypingBehavior = {
  // Words per minute (human average: 40-60 WPM)
  let wpmRange = (35, 65)

  // Calculate delay between keystrokes
  let generateKeystrokeDelay = (): float => {
    // Average: 5 chars per word, so chars/min = WPM * 5
    let wpm = Random.rangeInt(fst(wpmRange), snd(wpmRange))
    let charsPerMin = Float.fromInt(wpm * 5)
    let msPerChar = 60000.0 /. charsPerMin

    // Add variation (some keys faster, some slower)
    msPerChar +. Random.range(-50.0, 100.0)
  }

  // Occasional typo + backspace
  let shouldMakeTypo = (): bool => {
    Random.probability(0.05) // 5% typo rate
  }
}

// Wait/sleep helpers
module Sleep = {
  @val external setTimeout: (unit => unit, float) => int = "setTimeout"

  let sleep = (ms: float): promise<unit> => {
    Promise.make((resolve, _reject) => {
      let _ = setTimeout(() => resolve(), ms)
    })
  }

  // Sleep with timing profile
  let sleepWithProfile = async (profile: TimingProfile.t): unit => {
    let delay = TimingProfile.generateDelay(profile)
    await sleep(delay)
  }

  // Random pause (mimics user thinking/reading)
  let pause = async (): unit => {
    let delay = Random.range(500.0, 2000.0)
    await sleep(delay)
  }
}

// High-level human-like action wrappers
module HumanAction = {
  // Human-like click with pre-delay and jittered position
  let click = async (element: Dom.element): unit => {
    // Pre-click thinking delay
    await Sleep.sleepWithProfile(TimingProfile.preClick)

    // In real implementation:
    // 1. Get element bounding rect
    // 2. Generate click point with jitter
    // 3. Optionally simulate mouse movement
    // 4. Dispatch click event

    // For now, just dispatch click
    // (actual implementation in Actuator will use this)
    ()
  }

  // Human-like scroll with inertia
  let scroll = async (amount: int): unit => {
    let velocities = ScrollBehavior.generateInertia(Float.fromInt(amount))

    // Scroll gradually with inertia
    for velocity in velocities {
      // Dispatch scroll event with current velocity
      // await sleep for next frame
      await Sleep.sleep(16.0) // ~60fps
    }
  }

  // Human-like typing
  let type_ = async (text: string): unit => {
    let chars = String.split(text, "")

    for char in chars {
      // Type character
      // (dispatch keydown, keypress, keyup events)

      // Delay before next character
      let delay = TypingBehavior.generateKeystrokeDelay()
      await Sleep.sleep(delay)

      // Occasional typo
      if TypingBehavior.shouldMakeTypo() {
        // Type wrong character, then backspace
        await Sleep.sleep(Random.range(100.0, 300.0))
        // Dispatch backspace
        await Sleep.sleep(Random.range(50.0, 150.0))
      }
    }
  }

  // Human-like tab navigation
  let openInNewTab = async (url: string): unit => {
    // Hesitation before opening new tab
    await Sleep.sleepWithProfile(TimingProfile.tabSwitch)

    // Open tab (actual implementation will use ChromeTabs)
    ()
  }
}

// Detection evasion checks
module DetectionEvasion = {
  // Check if timing is too regular (bot-like)
  let isTimingTooRegular = (delays: array<float>): bool => {
    if Array.length(delays) < 3 {
      false
    } else {
      // Calculate variance
      let mean = delays->Array.reduce(0.0, (acc, d) => acc +. d) /. Float.fromInt(Array.length(delays))
      let variance = delays->Array.reduce(0.0, (acc, d) => {
        let diff = d -. mean
        acc +. diff *. diff
      }) /. Float.fromInt(Array.length(delays))

      // Low variance = too regular = suspicious
      // Threshold: variance should be > 10% of mean
      variance < (mean *. 0.1)
    }
  }

  // Add additional jitter if pattern too regular
  let adaptiveJitter = (baseDelay: float, recentDelays: array<float>): float => {
    if isTimingTooRegular(recentDelays) {
      // Inject more randomness
      baseDelay +. Random.range(-baseDelay *. 0.5, baseDelay *. 0.5)
    } else {
      baseDelay
    }
  }
}
