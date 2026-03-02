// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Core orchestrator for Algorithm Shield
// Coordinates Observer, Actuator, Lens, and Persona systems

module State = {
  type mode =
    | Normal
    | Stealth
    | Persona(string)

  type lensType =
    | Opposition
    | RandomWalk
    | TimeShift
    | Locality
    | Serendipity
    | None

  type t = {
    mode: mode,
    activeLens: lensType,
    activePersona: option<string>,
    membraneIntensity: float, // 0.0 - 1.0
    isPaused: bool,
  }

  let initial = {
    mode: Normal,
    activeLens: None,
    activePersona: None,
    membraneIntensity: 0.5,
    isPaused: false,
  }
}

module Action = {
  type t =
    | SetMode(State.mode)
    | ActivateLens(State.lensType)
    | DeactivateLens
    | SetPersona(option<string>)
    | SetIntensity(float)
    | TogglePause
    | TriggerBreach
}

let reducer = (state: State.t, action: Action.t): State.t => {
  switch action {
  | SetMode(mode) => {...state, mode}
  | ActivateLens(lens) => {...state, activeLens: lens}
  | DeactivateLens => {...state, activeLens: None}
  | SetPersona(persona) => {...state, activePersona: persona}
  | SetIntensity(intensity) => {...state, membraneIntensity: intensity}
  | TogglePause => {...state, isPaused: !state.isPaused}
  | TriggerBreach => state // Will trigger side effects
  }
}

// Membrane thickness calculation
let calculateThickness = (state: State.t): float => {
  let baseThickness = state.membraneIntensity
  let lensModifier = switch state.activeLens {
  | None => 0.0
  | Opposition => 0.3
  | RandomWalk => 0.2
  | TimeShift => 0.15
  | Locality => 0.1
  | Serendipity => 0.25
  }
  let modeModifier = switch state.mode {
  | Normal => 0.0
  | Stealth => 0.4
  | Persona(_) => 0.2
  }

  baseThickness +. lensModifier +. modeModifier
}

// Narrative generation for current state
let narrateState = (state: State.t): string => {
  let modeStr = switch state.mode {
  | Normal => "Normal browsing"
  | Stealth => "Stealth mode active"
  | Persona(name) => `Persona: ${name}`
  }

  let lensStr = switch state.activeLens {
  | None => "no lens"
  | Opposition => "Opposition lens (surfacing contrasts)"
  | RandomWalk => "Random Walk lens (exploring low-probability paths)"
  | TimeShift => "Time-Shift lens (prioritizing non-trending content)"
  | Locality => "Locality lens (emphasizing nearby/local)"
  | Serendipity => "Serendipity lens (maximizing surprise)"
  }

  let thickness = calculateThickness(state)
  let thicknessStr = if thickness < 0.3 {
    "thin membrane"
  } else if thickness < 0.7 {
    "moderate membrane"
  } else {
    "thick membrane"
  }

  `${modeStr} | ${lensStr} | ${thicknessStr}`
}
