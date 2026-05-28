# Open World — Server Configuration (GAME.md)

This document describes per-server configuration for different game worlds.

## Server Instance Configuration

Each server can define its own game world settings:

```yaml
starting_area_id: "uuid-of-starting-area"
biome_rules:
  - "areas 1-50: forest with trees and wildlife"
  - "areas 51-100: desert with sand and heat"

difficulty:
  puzzle_complexity: "medium"  # easy, medium, hard
  item_rarity: "balanced"      # common, balanced, rare
  environmental_hazards: true

features:
  survival_enabled: false
  building_enabled: false
  permadeath: false
  
player_interactions:
  pvp_enabled: false
  trading_enabled: false
  area_ownership: false

world:
  initial_areas: 100
  expansion_threshold: 50  # Generate new areas when unseen < 50
  max_areas_per_batch: 25
```

## Multiple Server Worlds

You can run multiple game servers with different themes:

### World 1: Standard Exploration
```
name: "Endless Frontier"
theme: "varied biomes"
difficulty: "medium"
max_players: 100
```

### World 2: Hardcore Survival
```
name: "Harsh Lands"
theme: "desert and tundra"
difficulty: "hard"
features:
  survival_enabled: true
  permadeath: true
max_players: 50
```

### World 3: Collaborative Quest
```
name: "The Great Dungeon"
theme: "underground caverns"
difficulty: "medium"
features:
  trading_enabled: true
  area_ownership: false
max_players: 200
```

## Configuration Loading

Servers load configuration from:
1. `.env` - Global configuration
2. `GAME.md` - This file (manual reference)
3. `config/servers/` - Per-server YAML files

Example: `config/servers/default.yaml`

```yaml
name: Default
starting_area: 00000000-0000-0000-0000-000000000001
difficulty: medium
features:
  survival: false
  building: false
```

## Environment Variables

These affect all server instances:

```bash
DATABASE_URL=sqlite://./open_world.db
JWT_SECRET=<secure-key>
LOG_LEVEL=INFO
RATE_LIMIT_IP=100
RATE_LIMIT_AUTH=500
```

## TBD - Future Server Features

- Player persistence across server restarts
- Backup/restore game world snapshots
- Admin commands for testing
- Server migration (move players between servers)
- Scheduled area resets (seasonal content)
