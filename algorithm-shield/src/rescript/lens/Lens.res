// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Lens module: Transform and reorder feeds based on selected perspective

open Observer

module LensConfig = {
  type t = {
    name: string,
    description: string,
    intensity: float, // 0.0 - 1.0
    parameters: Dict.t<JSON.t>,
  }
}

module TransformResult = {
  type action =
    | Reorder(array<int>) // New order of indices
    | Inject(array<string>) // URLs to inject
    | Hide(array<int>) // Indices to hide
    | Highlight(array<int>) // Indices to emphasize

  type t = {
    actions: array<action>,
    narrative: string,
  }
}

// Opposition Lens: Surface content unlike current distribution
let applyOppositionLens = (
  feedState: FeedState.t,
  config: LensConfig.t,
): TransformResult.t => {
  // Find underrepresented categories
  let allCategories = [
    ContentSignal.Tech,
    Politics,
    Art,
    Science,
    Entertainment,
    News,
    Social,
    Commerce,
    Education,
    Health,
  ]

  let dominantCats = feedState.dominantClusters->Array.map(c => c.category)

  // Categories NOT in dominant list
  let opposingCats = allCategories->Array.filter(cat =>
    !(dominantCats->Array.includes(cat))
  )

  // Category-specific search terms to find opposing viewpoints
  let categorySearchTerms = Dict.fromArray([
    ("Tech", ["low-tech lifestyle", "digital minimalism", "offline tools"]),
    ("Politics", ["non-partisan analysis", "policy wonks", "civic education"]),
    ("Art", ["documentary art", "functional design", "craft traditions"]),
    ("Science", ["citizen science", "DIY experiments", "science communication"]),
    ("Entertainment", ["educational content", "documentary series", "learning channels"]),
    ("News", ["explainer videos", "deep dives", "investigative journalism"]),
    ("Social", ["solo activities", "introspection", "mindfulness practices"]),
    ("Commerce", ["minimalism", "right to repair", "anti-consumerism"]),
    ("Education", ["street knowledge", "traditional skills", "oral histories"]),
    ("Health", ["public health", "preventive care", "community wellness"]),
  ])

  // Generate URLs for underrepresented categories
  let injectedUrls = []

  opposingCats->Array.forEach(cat => {
    let catName = switch cat {
    | ContentSignal.Tech => "Tech"
    | Politics => "Politics"
    | Art => "Art"
    | Science => "Science"
    | Entertainment => "Entertainment"
    | News => "News"
    | Social => "Social"
    | Commerce => "Commerce"
    | Education => "Education"
    | Health => "Health"
    }

    switch categorySearchTerms->Dict.get(catName) {
    | Some(terms) => {
        // Pick a random term from this category
        let randomIdx = Int.fromFloat(Random.float(Int.toFloat(Array.length(terms))))
        switch terms->Array.get(randomIdx) {
        | Some(term) => {
            let encodedTerm = term->String.replaceAll(" ", "+")
            let url = `https://www.youtube.com/results?search_query=${encodedTerm}`
            injectedUrls->Array.push(url)->ignore
          }
        | None => ()
        }
      }
    | None => ()
    }
  })

  // Limit to 4 URLs max
  let limitedUrls = injectedUrls->Array.slice(~start=0, ~end=min(4, Array.length(injectedUrls)))

  let narrative = switch Array.length(limitedUrls) {
  | 0 => "Feed already diverse across categories"
  | n => `Opening ${Int.toString(n)} opposing viewpoints from underrepresented categories`
  }

  {
    actions: if Array.length(limitedUrls) > 0 {
      [TransformResult.Inject(limitedUrls)]
    } else {
      []
    },
    narrative,
  }
}

// Random Walk Lens: Explore low-probability paths
let applyRandomWalkLens = (
  feedState: FeedState.t,
  config: LensConfig.t,
): TransformResult.t => {
  // Define diverse search queries outside typical bubbles
  let diverseTopics = [
    ("origami tutorials", ContentSignal.Art),
    ("mycology foraging", ContentSignal.Education),
    ("esperanto language", ContentSignal.Education),
    ("circuit bending music", ContentSignal.Entertainment),
    ("urban beekeeping", ContentSignal.Health),
    ("brutalist architecture", ContentSignal.Art),
    ("fermentation science", ContentSignal.Science),
    ("indigenous knowledge systems", ContentSignal.Education),
    ("mathematical art", ContentSignal.Art),
    ("permaculture design", ContentSignal.Health),
    ("linguistics documentary", ContentSignal.Education),
    ("avant-garde cinema", ContentSignal.Entertainment),
    ("ethnomusicology", ContentSignal.Entertainment),
    ("bio-architecture", ContentSignal.Science),
    ("systems thinking", ContentSignal.Education),
  ]

  // Find dominant categories to avoid
  let dominantCats = feedState.dominantClusters->Array.map(c => c.category)

  // Filter topics that are NOT in dominant categories
  let opposingTopics = diverseTopics->Array.filter(((_, cat)) =>
    !(dominantCats->Array.includes(cat))
  )

  // Randomly select 3-5 topics
  let numToSelect = Int.fromFloat(3.0 +. Random.float(3.0))  // 3-5
  let selectedTopics = []

  for _ in 0 to numToSelect - 1 {
    let availableTopics = if Array.length(opposingTopics) > 0 {
      opposingTopics
    } else {
      diverseTopics  // Fallback if all categories are dominant
    }

    let randomIdx = Int.fromFloat(
      Random.float(Int.toFloat(Array.length(availableTopics)))
    )

    switch availableTopics->Array.get(randomIdx) {
    | Some((topic, _)) => {
        let encodedTopic = topic
          ->String.replaceAll(" ", "+")
        let url = `https://www.youtube.com/results?search_query=${encodedTopic}`
        selectedTopics->Array.push(url)->ignore
      }
    | None => ()
    }
  }

  let narrative = switch Array.length(selectedTopics) {
  | 0 => "No random walk paths generated"
  | n => `Opening ${Int.toString(n)} exploratory paths outside your filter bubble`
  }

  {
    actions: [TransformResult.Inject(selectedTopics)],
    narrative,
  }
}

// Time-Shift Lens: Prioritize older or non-trending content
let applyTimeShiftLens = (
  feedState: FeedState.t,
  config: LensConfig.t,
): TransformResult.t => {
  // Topics with temporal depth (evergreen subjects that were popular in the past)
  let timelessTopics = [
    ("early internet culture", "2000s"),
    ("classic documentary films", "1990s"),
    ("vintage computing", "1980s"),
    ("forgotten inventions", "historical"),
    ("archival footage", "historical"),
    ("retro technology", "pre-2010"),
    ("old school animation", "classic"),
    ("vintage educational films", "archival"),
    ("historical lectures", "classic"),
    ("classic music performances", "archival"),
  ]

  // Randomly select 2-3 topics
  let numToSelect = Int.fromFloat(2.0 +. Random.float(2.0))  // 2-3
  let selectedUrls = []

  for _ in 0 to numToSelect - 1 {
    let randomIdx = Int.fromFloat(
      Random.float(Int.toFloat(Array.length(timelessTopics)))
    )

    switch timelessTopics->Array.get(randomIdx) {
    | Some((topic, era)) => {
        let encodedTopic = topic->String.replaceAll(" ", "+")
        // Use YouTube's upload date filters via sp parameter
        // For simplicity, just add the search query - users will see older content
        let url = `https://www.youtube.com/results?search_query=${encodedTopic}+${era}`
        selectedUrls->Array.push(url)->ignore
      }
    | None => ()
    }
  }

  let narrative = switch Array.length(selectedUrls) {
  | 0 => "No time-shifted content generated"
  | n => `Opening ${Int.toString(n)} paths to non-trending and archival content`
  }

  {
    actions: [TransformResult.Inject(selectedUrls)],
    narrative,
  }
}

// Locality Lens: Emphasize geographically nearby content
let applyLocalityLens = (
  feedState: FeedState.t,
  config: LensConfig.t,
): TransformResult.t => {
  // Local-focused search terms (works without geolocation API)
  // Uses general "local" modifiers that prompt YouTube to show regional content
  let localTopics = [
    "local news near me",
    "community events",
    "neighborhood stories",
    "local businesses",
    "regional history",
    "local food culture",
    "community projects",
    "local music scene",
    "neighborhood tours",
    "local activism",
    "community organizations",
    "regional traditions",
  ]

  // Randomly select 3-4 topics
  let numToSelect = Int.fromFloat(3.0 +. Random.float(2.0))  // 3-4
  let selectedUrls = []

  for _ in 0 to numToSelect - 1 {
    let randomIdx = Int.fromFloat(
      Random.float(Int.toFloat(Array.length(localTopics)))
    )

    switch localTopics->Array.get(randomIdx) {
    | Some(topic) => {
        let encodedTopic = topic->String.replaceAll(" ", "+")
        let url = `https://www.youtube.com/results?search_query=${encodedTopic}`
        selectedUrls->Array.push(url)->ignore
      }
    | None => ()
    }
  }

  let narrative = switch Array.length(selectedUrls) {
  | 0 => "No local content generated"
  | n => `Opening ${Int.toString(n)} paths to local and nearby content`
  }

  {
    actions: [TransformResult.Inject(selectedUrls)],
    narrative,
  }
}

// Serendipity Lens: Maximize surprise and novelty
let applySerendipityLens = (
  feedState: FeedState.t,
  config: LensConfig.t,
): TransformResult.t => {
  // Utterly random and surprising topics - no category constraints
  let serendipitousTopics = [
    "competitive marble racing",
    "underwater basket weaving",
    "extreme ironing",
    "cheese rolling championships",
    "synchronized swimming history",
    "professional whistling",
    "ice sculpting competitions",
    "ostrich racing",
    "bog snorkeling",
    "wife carrying championships",
    "air guitar world championships",
    "ferrofluid sculptures",
    "cymatics patterns",
    "slime molds intelligence",
    "bioluminescent organisms",
    "murmuration starlings",
    "sacred geometry",
    "sand mandalas",
    "glass harmonica music",
    "theremin performances",
    "carnivorous plants",
    "axolotl regeneration",
    "tardigrade extremophiles",
    "quantum eraser experiment",
    "double-slit experiment",
    "holographic universe theory",
    "synesthesia experiences",
    "lucid dreaming techniques",
    "bibliomancy divination",
    "kinetic rain sculptures",
  ]

  // Randomly select 4-6 completely random topics
  let numToSelect = Int.fromFloat(4.0 +. Random.float(3.0))  // 4-6
  let selectedUrls = []

  for _ in 0 to numToSelect - 1 {
    let randomIdx = Int.fromFloat(
      Random.float(Int.toFloat(Array.length(serendipitousTopics)))
    )

    switch serendipitousTopics->Array.get(randomIdx) {
    | Some(topic) => {
        let encodedTopic = topic->String.replaceAll(" ", "+")
        let url = `https://www.youtube.com/results?search_query=${encodedTopic}`
        selectedUrls->Array.push(url)->ignore
      }
    | None => ()
    }
  }

  let narrative = switch Array.length(selectedUrls) {
  | 0 => "No serendipitous discoveries generated"
  | n => `Opening ${Int.toString(n)} completely unexpected paths for maximum serendipity`
  }

  {
    actions: [TransformResult.Inject(selectedUrls)],
    narrative,
  }
}

// Downstream Lens: Flow with the algorithm instead of resisting it
// "Surfing" - see where the algorithm wants to take you
let applyDownstreamLens = (
  feedState: FeedState.t,
  config: LensConfig.t,
): TransformResult.t => {
  // Find the strongest gradient in current feed
  let dominantCategory = feedState.dominantClusters
    ->Array.get(0)
    ->Option.map(c => c.category)
    ->Option.getOr(ContentSignal.Tech)

  // FLOW WITH IT: Amplify the filter bubble instead of opposing it
  // This shows users where the algorithm wants to take them
  let extremeSearches = switch dominantCategory {
  | ContentSignal.Tech => [
      "cutting edge AI research",
      "quantum computing breakthroughs",
      "future of technology",
      "transhumanism",
    ]
  | Politics => [
      "political theory deep dives",
      "geopolitical analysis",
      "political philosophy",
      "constitutional law",
    ]
  | Art => [
      "experimental art movements",
      "contemporary art theory",
      "avant-garde installations",
      "conceptual art",
    ]
  | Science => [
      "theoretical physics",
      "neuroscience frontiers",
      "synthetic biology",
      "complexity theory",
    ]
  | Entertainment => [
      "film theory",
      "narrative structure",
      "cinematography analysis",
      "auteur directors",
    ]
  | News => [
      "investigative journalism",
      "media analysis",
      "fact-checking methods",
      "journalism history",
    ]
  | Social => [
      "social psychology",
      "group dynamics",
      "cultural anthropology",
      "sociology of networks",
    ]
  | Commerce => [
      "behavioral economics",
      "consumer psychology",
      "market dynamics",
      "business strategy",
    ]
  | Education => [
      "pedagogy theory",
      "learning science",
      "educational philosophy",
      "cognitive development",
    ]
  | Health => [
      "public health policy",
      "epidemiology",
      "health systems",
      "preventive medicine",
    ]
  }

  // Select 2-3 extreme topics from dominant category
  let numToSelect = Int.fromFloat(2.0 +. Random.float(2.0))  // 2-3
  let selectedUrls = []

  for _ in 0 to numToSelect - 1 {
    let randomIdx = Int.fromFloat(
      Random.float(Int.toFloat(Array.length(extremeSearches)))
    )

    switch extremeSearches->Array.get(randomIdx) {
    | Some(search) => {
        let encoded = search->String.replaceAll(" ", "+")
        let url = `https://www.youtube.com/results?search_query=${encoded}`
        selectedUrls->Array.push(url)->ignore
      }
    | None => ()
    }
  }

  let categoryName = switch dominantCategory {
  | ContentSignal.Tech => "Tech"
  | Politics => "Politics"
  | Art => "Art"
  | Science => "Science"
  | Entertainment => "Entertainment"
  | News => "News"
  | Social => "Social"
  | Commerce => "Commerce"
  | Education => "Education"
  | Health => "Health"
  }

  let narrative = switch Array.length(selectedUrls) {
  | 0 => "No downstream paths generated"
  | n => `Flowing downstream: exploring the deep end of ${categoryName} (${Int.toString(n)} paths)`
  }

  {
    actions: [TransformResult.Inject(selectedUrls)],
    narrative,
  }
}

// Main lens application dispatcher
let applyLens = (
  lensType: Membrane.State.lensType,
  feedState: FeedState.t,
  config: LensConfig.t,
): option<TransformResult.t> => {
  switch lensType {
  | Membrane.State.None => None
  | Opposition => Some(applyOppositionLens(feedState, config))
  | RandomWalk => Some(applyRandomWalkLens(feedState, config))
  | TimeShift => Some(applyTimeShiftLens(feedState, config))
  | Locality => Some(applyLocalityLens(feedState, config))
  | Serendipity => Some(applySerendipityLens(feedState, config))
  }
}
