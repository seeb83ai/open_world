# Open World — Game Concept Document

## Game Overview

**Open World** is a persistent, AI-generated, multiplayer text-based exploration game. Players explore a dynamically generated graph of interconnected areas, discover items, solve puzzles, and navigate a world where resources are finite and shared. The game emphasizes emergent gameplay through exploration and consequence — every action is permanent, every item collected affects the world for the next player.

**Platforms:** Browser, CLI application

---

## Core Gameplay Loop

1. **Explore**: Move between connected areas, discovering new regions
2. **Observe**: Read detailed descriptions and examine surroundings
3. **Discover**: Find items scattered across areas
4. **Act**: Perform actions on puzzles and environmental elements
5. **Persist**: Changes remain for other players to encounter

---

## World Structure

### Graph-Based World

The world is a **directed graph**, not a grid:
- **Nodes**: Areas with a UUID, one-sentence description, and persistent state
- **Edges**: Connections between areas, each with a short contextual description

### Initial World

- **100 pre-generated areas** with AI-generated descriptions
- Variable exits per area (no fixed structure; some dead ends allowed if escapable)
- Connections between areas defined semantically (not via coordinates)

### World Growth

- When fewer than 50 unseen areas exist, new areas are generated
- When existing areas benefit from new connections, they are added
- New areas only connect to unvisited areas, **unless**:
  - A player creates a connection through action (dig tunnel, break wall)
  - Then existing visited areas can be re-connected

---

## Items System

### Item Properties

Each item has:
- **Name & description**
- **Quality** (starts high, degrades with use; below quality threshold, certain actions are restricted or fail)
- **Durability** (items can break; broken items can be repaired; quality and durability are tracked separately)
- **Carrying capacity modifier** (bags expand capacity, vehicles expand more dramatically)
- **Enabled actions** (what this item lets the player do; quality thresholds may restrict them)
- **Connection restrictions** (car can't enter house; small key can't open large locks; boots needed for muddy areas)
- **Connection requirements** (car needs roads; boat needs water; wings need open sky)
- **Quality-dependent actions** (worn key has lower success rate; rusty lockpick might break inside lock)
- **Repair recipe** (what tools/items fix this item if broken)

### Inventory & Capacity

- Players start with **minimal carrying capacity** (~1-2 items)
- Items themselves expand capacity (bag, cart, vehicle, etc.)
- Carrying capacity limits what a player can acquire and transport

### Resource Scarcity

- **No item respawning** — items are finite
- When a player collects an item, it's gone from that location forever
- This creates emergent competition and forces players to cooperate indirectly
- Lost items may reappear elsewhere (e.g., lost key found in another area for next player)
- **Feature, not bug:** The world gradually depletes as players progress

### Item Database

- All items catalogued with properties
- New items can be created during area generation
- Existing items are reused to minimize AI token usage

---

## Actions System

### Base Actions

Players always have basic actions (bare hands, feet):
- Push, pull, examine, etc.
- No items required

### Item-Enabled Actions

- Items unlock additional actions (key unlocks, axe chops, etc.)
- **N:N relationship**: Multiple items can enable the same action; one item can enable multiple actions
- Some actions modify item state (weapon degrades after use)

### Action Mechanics

- **All actions permanently modify world state** (chopped tree is gone forever)
- **Probabilistic outcomes**: Same action can have different results
  - Example: "smash chest" → nothing happens (30%), lock ruined (40%), chest opens (20%), chest explodes (10%)
- **Chainable consequences**: One action outcome triggers follow-up actions
  - Example: chest explodes → items scatter → pressure plate triggers collapse

### Action Database

- All possible actions catalogued with mechanics
- Reusable across areas
- New actions created during area generation if needed

---

## Puzzles System

### Puzzle Design

Each puzzle has:
- **Type** (locked chest, blocked path, environmental hazard, etc.)
- **Set of possible solution actions** with:
  - Probability weight
  - Result (nothing, item destroyed, puzzle solved, unintended consequence)
  - Follow-up actions triggered

### Multiple Solutions

- Each puzzle has **multiple possible solution paths**
- Different actions succeed with different probabilities
- A failed solution might have unintended consequences (chest explodes, lock is destroyed)
- Players must experiment or trade knowledge with others

### Puzzle Database

- Catalogue of reusable puzzle templates
- Customizable during area generation
- Reduces AI token usage through pattern reuse

---

## Multiplayer & Persistence

### Shared World State

- **Database tracks**:
  - Area state (items present, puzzle states, environmental changes)
  - Item properties (quality, durability, location, ownership)
  - Player snapshots (last position, inventory)
  - **Event log**: Every player action recorded with user ID, area ID, timestamp, action description, outcome

### Async Interaction Model (Initial)

- When a player enters an area, they see it in the **state other players left it**
- No real-time synchronization initially
- Players affect each other through shared resources and consequences

### Future Real-Time Enhancement

- Multiple players in the same area can see each other
- Potential collaboration mechanics (shared puzzles, joint actions)
- Potential competition (race for items, sabotage)

---

## AI Generation Pipeline

### Context Gathering (Pass 1)

- Load one-sentence descriptions of **all connected areas**
- Load short descriptions of **all connections** (edges)
- For visited connected areas: load full detailed descriptions and current state
- Summarize into a coherent context document

### Outline Generation (Pass 2)

Using the context summary and the target area's one-sentence description:
- Define area concept and atmosphere
- Outline rough item placement
- Determine puzzle types and complexity
- Define item/vehicle restrictions for this area and its connections
- Lock the outline to prevent drift during detailing

### Detailed Generation (Pass 3)

- Load existing items, actions, puzzles from database
- Reuse and extend existing patterns where possible
- Generate detailed area description
- Generate specific items with properties and restrictions
- Generate specific puzzles with solution actions and outcomes
- Verify generated items are compatible with area restrictions
- Update database with any new items/actions/puzzles

---

## Future Systems (Full Scope)

### Survival Mechanics

- **Biological needs**: Hunger, thirst, temperature, fatigue
- **Environmental hazards**: Lava, freezing cold, toxic areas, etc.
- **Consequences**: Neglect leads to incapacity or death
- **Permadeath model**: TBD during implementation

### Building & Crafting

- **Crafting**: Combine items to create new items
- **Construction**: Build permanent structures (shelter, bridge, farm)
- **Area modification**: Alter existing areas (build walls, fill holes)
- **Long-term projects**: Player-driven world changes

### Health & Status Effects

- Injuries and status conditions
- Recovery and treatment items
- Environmental effects on player abilities

---

## Anti-Cheating & Coherence

### Why UUID-Based (Not Coordinate-Based)

- **Random UUIDs** prevent players from memorizing or predicting area paths
- **Semantic connections** (based on descriptions, not math) prevent jumping via coordinate hacks
- **Unique descriptions per UUID** prevent replicating known areas
- **Probabilistic puzzle outcomes** mean the same action doesn't always succeed

### Coherence Through Context

- One-sentence descriptions guide AI generation without over-specifying
- Semantic connections (via descriptions) prevent impossible transitions
- Multi-pass generation allows refinement without full regeneration
- Edge descriptions maintain narrative flow between areas

---

## Server Configuration (GAME.md)

Each server instance can define:
- **Starting area UUID**
- **Initial 100 area descriptions** (AI-generated or custom)
- **Biome/theme rules** (e.g., "areas 1-30 are forest, 31-60 are desert")
- **Difficulty scaling** (puzzle complexity, item rarity, hazard frequency)
- **Feature flags** (survival mechanics enabled? building enabled?)
- **Player interaction rules** (PvP, trading, area ownership, etc.)

**Multiple servers = Different game worlds with different themes and rules.**

---

## Next Steps

1. **Technical Specification** (CLAUDE.md): Architecture, data models, API design, tech stack
2. **MVP Scope**: Define Phase 1 features vs. future phases
3. **Prototype Development**: Initial world generation, movement, basic interactions
