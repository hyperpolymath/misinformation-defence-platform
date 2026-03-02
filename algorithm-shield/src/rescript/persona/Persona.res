// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Persona module: Behavioral shells for polymorphic browsing

module BehavioralTrait = {
  type category = Observer.ContentSignal.category

  type t = {
    interests: array<category>,
    avoidances: array<category>,
    clickProbability: float, // Base probability of clicking
    scrollSpeed: float, // Multiplier for scroll behavior
    sessionDuration: float, // Average session length in minutes
  }
}

module PersonaDefinition = {
  type t = {
    id: string,
    name: string,
    description: string,
    traits: BehavioralTrait.t,
    iconGlyph: string,
  }

  // Predefined personas
  let gardener: t = {
    id: "gardener",
    name: "Gardener",
    description: "Interested in plants, ecology, slow living",
    traits: {
      interests: [Observer.ContentSignal.Health, Education, Art],
      avoidances: [Tech, Politics, Commerce],
      clickProbability: 0.3,
      scrollSpeed: 0.7,
      sessionDuration: 20.0,
    },
    iconGlyph: "🌱",
  }

  let techSkeptic: t = {
    id: "tech-skeptic",
    name: "Tech Skeptic",
    description: "Critical of tech industry, privacy-focused",
    traits: {
      interests: [Politics, News, Social],
      avoidances: [Tech, Commerce, Entertainment],
      clickProbability: 0.4,
      scrollSpeed: 0.9,
      sessionDuration: 15.0,
    },
    iconGlyph: "🔒",
  }

  let artStudent: t = {
    id: "art-student",
    name: "Art Student",
    description: "Exploring visual culture, creativity, theory",
    traits: {
      interests: [Art, Education, Entertainment],
      avoidances: [Tech, Politics, Commerce],
      clickProbability: 0.5,
      scrollSpeed: 0.6,
      sessionDuration: 30.0,
    },
    iconGlyph: "🎨",
  }

  let all = [gardener, techSkeptic, artStudent]
}

module PersonaState = {
  type t = {
    activePersona: option<PersonaDefinition.t>,
    confidence: float, // How aligned current behavior is (0.0 - 1.0)
    sessionStart: option<float>,
  }

  let initial = {
    activePersona: None,
    confidence: 0.0,
    sessionStart: None,
  }

  let activate = (state: t, persona: PersonaDefinition.t): t => {
    {
      activePersona: Some(persona),
      confidence: 1.0,
      sessionStart: Some(Date.now()),
    }
  }

  let deactivate = (state: t): t => {
    {
      activePersona: None,
      confidence: 0.0,
      sessionStart: None,
    }
  }
}

// Calculate behavioral parameters based on active persona
let calculateBehaviorParams = (
  personaState: PersonaState.t,
  signal: Observer.ContentSignal.t,
): option<{
  "shouldClick": bool,
  "scrollAmount": int,
  "dwellTime": int,
}> => {
  switch personaState.activePersona {
  | None => None
  | Some(persona) => {
      // Check if signal matches persona interests
      let isInteresting = signal.categories->Array.some(cat =>
        persona.traits.interests->Array.includes(cat)
      )

      let isAvoided = signal.categories->Array.some(cat =>
        persona.traits.avoidances->Array.includes(cat)
      )

      let shouldClick = if isInteresting {
        Math.random() < persona.traits.clickProbability
      } else if isAvoided {
        false
      } else {
        Math.random() < (persona.traits.clickProbability *. 0.5)
      }

      Some({
        "shouldClick": shouldClick,
        "scrollAmount": Float.toInt(300.0 *. persona.traits.scrollSpeed),
        "dwellTime": if isInteresting { 5000 } else { 1000 },
      })
    }
  }
}

// Narrate persona state
let narratePersonaState = (state: PersonaState.t): string => {
  switch state.activePersona {
  | None => "No active persona"
  | Some(persona) => {
      let confidenceStr = if state.confidence > 0.8 {
        "strong"
      } else if state.confidence > 0.5 {
        "moderate"
      } else {
        "weak"
      }

      `${persona.iconGlyph} ${persona.name} (${confidenceStr} alignment)`
    }
  }
}
