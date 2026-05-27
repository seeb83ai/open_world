# Open World — Architecture & Implementation Plan

## Overview

A persistent, AI-generated, multiplayer text-based exploration game playable in browser or CLI. Players explore a graph-based world of interconnected areas, discover items, solve puzzles, and gradually deplete shared resources.

---

## Core Systems

### 1. World Structure (Graph-Based)

**Not a grid.** The world is a **directed graph** where:
- **Nodes** = Areas (UUID, one-sentence description, persistent state)
- **Edges** = Connections between areas (each edge has a short description for context)

**Initial world:**
- 100 pre-generated areas with AI-generated descriptions
- Connections between areas (variable number of exits per area; some dead ends allowed if escapable)
- All descriptions AI-generated at server startup

**Graph expansion:**
- When < 50 unseen areas remain, generate new batch
- When a generated area "benefits from new connection" (TBD: criteria for this)
- New areas always connect to unvisited areas only (unless player creates connection via action)
- Player-created connections (dig tunnel, break wall) can connect to visited areas

---

### 2. Persistence & Shared State

**Database tracks:**
- Area state (items present, puzzle states, environmental changes)
- Item state (quality, durability, location, ownership)
- Player snapshots (last known position, inventory)
- **Event log**: Every player action recorded with user ID, area ID, timestamp, action description, outcome

**Behavior:**
- When a player enters an area, they see it in the state other players left it
- No item respawning
- Items can break during use; broken items can be repaired with right tools
- **Feature, not bug:** World gradually depletes as players collect items and solve puzzles
- Players inadvertently help/hinder each other through shared resources

---

### 3. Items System

**Item properties:**
- Name, description
- **Quality** (starts high, degrades with use; below quality threshold, certain actions are restricted or fail)
- **Durability** (items can break; broken items can be repaired; quality and durability are tracked separately)
- Carrying capacity modifier (bag increases capacity, car increases more, rocket enables different areas)
- Enabled actions (what this item lets the player do; quality thresholds may restrict them)
- **Connection restrictions** (car can't enter house; small key can't open large locks; boots needed for muddy areas)
- Connection requirements (car needs roads; boat needs water; wings need open sky)
- Quality-dependent actions (worn key has lower success rate; rusty lockpick might break inside lock)
- Repair recipe (what tools/items fix this item if broken)

**Inventory:**
- Players start with empty inventory
- Carrying capacity limited by items themselves (base capacity very low, ~1-2 items)
- Items like bags, carts, vehicles expand capacity
- Some items enable travel to new areas (boat for water, wings for sky, etc.)

**Item database:**
- All items catalogued with properties
- Reusable across areas
- Can be extended during generation (new item variants created as needed)

---

### 4. Actions System

**Base actions:**
- Players always have bare-hand actions (push, pull, examine, etc.)

**Item-unlocked actions:**
- Items enable additional actions (key unlocks, axe chops, etc.)
- N:N relationship: multiple items can enable same action; one item can enable multiple actions

**Action effects:**
- All actions **permanently modify state** (tree chopped → it's gone forever)
- Actions are **probabilistic** with multiple possible outcomes
- Outcomes can be **chainable** (chest explodes → triggers follow-up actions)
- Quality may affect success rates (worn key has lower success chance)

**Action database:**
- All possible actions catalogued
- Reusable across areas
- New actions created as needed during generation

---

### 5. Puzzles System

**Puzzle definition:**
- Puzzle type (locked chest, blocked path, etc.)
- Set of possible solution actions with outcomes
- Each action outcome has:
  - Probability weight
  - Result (nothing, item destroyed, puzzle solved, unintended consequence)
  - Follow-up actions triggered (if any)

**Puzzle database:**
- Catalogue of reusable puzzle templates
- Can be extended and customized during area generation
- Reduces AI token usage by pattern reuse

**Multiple solutions:**
- Each puzzle has multiple possible actions that might solve it
- Different actions have different probabilities of success
- Lost items reappear elsewhere for other players (e.g., lost key hidden in area for next player to find)

---

## AI Generation Pipeline

### Three-Pass Area Generation

**Pass 1: Context gathering**
- Load one-sentence descriptions of all connected areas
- Load short descriptions of all connections (edges)
- For visited connected areas: load their full detailed descriptions and current state
- Summarize findings into context document

**Pass 2: Outline**
- Use context summary + area's one-sentence description to outline:
  - Area concept and atmosphere
  - Rough item placement
  - Puzzle types to include
  - Expected complexity
  - Item/vehicle restrictions for this area and its connections
- Lock outline to prevent drift

**Pass 3: Detailing**
- Load existing items, actions, puzzles from database
- Match/reuse where possible; extend/create only if needed
- Generate detailed area description
- Generate specific items (with properties and restrictions)
- Generate specific puzzles (with solution actions/outcomes)
- Verify generated items are compatible with area restrictions
- Update database with any new items/actions/puzzles

---

## Multiplayer & Real-Time

**Async model (initial):**
- Players see area snapshot when entering (state from when others left)
- No real-time interaction

**Future enhancement:**
- Real-time events if multiple players in same area
- Potential collaboration/competition mechanics (TBD during implementation)

---

## Future Systems (In Scope, TBD)

### Survival Mechanics
- Hunger, thirst, temperature, fatigue (or subset)
- Environmental hazards (lava, cold, toxic areas)
- Consequences for neglect (player incapacity or death)
- Permadeath vs. respawn model (TBD)

### Building & Crafting
- Craft items from components
- Construct permanent structures
- Modify existing areas
- Long-term player projects (TBD during implementation)

### Health & Status Effects
- Injury, status conditions
- Recovery/treatment items
- Environmental effects on player ability

---

## Anti-Cheating & Randomization

- UUIDs are randomly assigned (prevents coordinate memorization)
- Area descriptions are unique per UUID (prevents replicating known areas)
- Connections are semantic (based on descriptions) not coordinate-based (no jumping via grid math)
- Puzzle outcomes are probabilistic (action A doesn't always succeed)
- Database persistence makes cheating meaningless (other players see results)

---

## Server Configuration (GAME.md)

Each server instance defines:
- Starting area UUID
- Initial 100 area descriptions (AI-generated or custom)
- Biome/theme rules (e.g., "areas 1-30 are forest")
- Difficulty scaling
- Survival/building feature flags
- Player interaction rules

Multiple servers = different game worlds with different settings.

---

## Critical Implementation Notes

### Token Optimization
- Puzzle database reduces regeneration (pattern reuse)
- Item database reduces redundant generation
- Action database prevents duplicate action creation
- Context summarization (Pass 1) distills multi-area info into compact prompt

### Coherence Strategy
- One-sentence descriptions act as "skeleton keys" guiding generation
- Semantic connections (via descriptions) prevent impossible transitions
- Multi-pass generation allows refinement without full regeneration
- Edge descriptions maintain narrative flow between areas

### Scalability Considerations
- Graph structure allows infinite world growth
- Database lookup (existing items/actions/puzzles) faster than regeneration
- Expansion threshold (50 unseen areas) prevents unbounded generation

---

## Data Models (TBD - Detail in Implementation)

- **Area**: UUID, description, connections, items, puzzle states
- **Item**: UUID, name, quality, durability, restrictions, enabled actions
- **Action**: UUID, name, item requirements, effects, quality modifiers
- **Puzzle**: UUID, type, solution actions, outcomes
- **Event**: user_id, area_id, timestamp, action_description, outcome

---

## Next Steps

1. **Tech Stack Decision**: Framework (backend/frontend), database, LLM integration
2. **Data Schema**: Detailed database design
3. **MVP Scope**: Which systems in Phase 1 vs. future
4. **Prototype**: Initial world generation, basic movement/interaction
