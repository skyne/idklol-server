# Character Service Updates

## Overview

The character service has been enhanced with two major improvements:

1. **Strongly-Typed Models** - Replaced simple data structs with proper domain models
2. **Combined Catalog API with Caching** - New single-call API for character creation with intelligent caching

## 1. Strongly-Typed Models

### Model Architecture

All database entities are now represented by strongly-typed models in `src/models/`:

#### Catalog Models (`models/catalog.rs`)
- `Race` - Playable races (Human, Elf, Dwarf, Orc)
- `Gender` - Character genders (Male, Female)
- `SkinColor` - Available skin tones
- `Class` - Character classes (Warrior, Mage, Rogue, Cleric, Ranger)
- `RaceGenderAllowed` - Valid race-gender combinations
- `RaceGenderSkinColorAllowed` - Valid race-gender-skin combinations
- `RaceGenderClassAllowed` - Valid race-gender-class combinations

All models use:
- SQLx `FromRow` derive for database mapping
- Serde `Serialize`/`Deserialize` for JSON support
- Proper type safety throughout the codebase

#### Character Model (`models/character.rs`)
- `Character` - Complete character entity with metadata
- Includes user_email, timestamps, and all attributes
- Full support for chrono datetime serialization

### Repository Pattern

Repositories now use strongly-typed return values:

```rust
// Before
async fn get_races(&self) -> Result<Vec<RaceData>, Box<dyn Error>>;

// After
use crate::models::catalog::Race;
async fn get_races(&self) -> Result<Vec<Race>, Box<dyn Error>>;
```

Benefits:
- Type safety across all layers
- Easy to swap database implementations
- Better IDE autocomplete and error detection
- Cleaner separation of concerns

## 2. Combined Catalog API with Caching

### Problem Solved

Previously, clients had to make 4+ API calls during character creation:
```
1. ListRaces()
2. ListGendersForRace(selectedRace)
3. ListSkinColors(race, gender)
4. ListClasses(race, gender)
```

This created:
- Multiple network round-trips
- Poor UX during character creation
- Server load from repeated queries
- Complex client-side state management

### Solution: GetCharacterCreationCatalog

New single-endpoint API that returns ALL data needed for character creation:

```protobuf
message GetCharacterCreationCatalogRequest {
    string cached_version = 1;  // Optional: client's cached version
}

message GetCharacterCreationCatalogResponse {
    bool is_current = 1;          // true if client's cache is valid
    CharacterCreationCatalog catalog = 2;  // null if is_current=true
}

message CharacterCreationCatalog {
    string version = 1;  // Cache version hash
    
    // All options with names
    repeated RaceOption races = 2;
    repeated GenderOption genders = 3;
    repeated SkinColorOption skin_colors = 4;
    repeated ClassOption classes = 5;
    
    // All valid combinations
    repeated RaceGenderCombination allowed_race_gender = 6;
    repeated RaceGenderSkinColorCombination allowed_race_gender_skin_color = 7;
    repeated RaceGenderClassCombination allowed_race_gender_class = 8;
}
```

### Caching Strategy

#### Server-Side Cache
- In-memory cache using `OnceLock<RwLock<>>`
- Thread-safe global singleton
- Version generated from content hash
- Automatic invalidation on data changes

```rust
// Cache implementation in catalog_cache.rs
pub struct CatalogCache;

impl CatalogCache {
    pub fn get() -> Option<(String, CharacterCreationCatalog)>;
    pub fn set(catalog: CharacterCreationCatalog) -> String;
    pub fn is_current(version: &str) -> bool;
}
```

#### Client-Side Cache Support
1. **First Call** - Client calls with empty `cached_version`:
   ```json
   { "cached_version": "" }
   ```
   Server returns full catalog with version:
   ```json
   {
     "is_current": false,
     "catalog": {
       "version": "a1b2c3d4",
       "races": [...],
       "genders": [...],
       ...
     }
   }
   ```

2. **Subsequent Calls** - Client sends cached version:
   ```json
   { "cached_version": "a1b2c3d4" }
   ```
   If current, server returns minimal response:
   ```json
   {
     "is_current": true,
     "catalog": null
   }
   ```

### Performance Benefits

- **Single network call** vs 4+ sequential calls
- **Client caching** - catalog persists across sessions
- **Server caching** - no database hits after first load
- **Bandwidth savings** - small response when cache is current
- **Parallel loading** - all DB queries run concurrently with `tokio::join!`

### Migration Guide

#### Old Client Code
```javascript
// Multiple sequential calls
const races = await client.listRaces();
const genders = await client.listGendersForRace(selectedRace);
const skinColors = await client.listSkinColors(selectedRace, selectedGender);
const classes = await client.listClasses(selectedRace, selectedGender);
```

#### New Client Code
```javascript
// Single call with caching
let cachedVersion = localStorage.getItem('catalog_version') || '';
const response = await client.getCharacterCreationCatalog({ cached_version: cachedVersion });

if (!response.is_current) {
  const catalog = response.catalog;
  localStorage.setItem('catalog_version', catalog.version);
  localStorage.setItem('catalog_data', JSON.stringify(catalog));
}

const catalog = response.is_current 
  ? JSON.parse(localStorage.getItem('catalog_data'))
  : response.catalog;

// Use catalog data for entire creation flow
```

### Example gRPC Call

```bash
# First call (no cache)
grpcurl -plaintext \
  -d '{"cached_version":""}' \
  localhost:50053 characters.CharacterService/GetCharacterCreationCatalog

# Returns full catalog with version "a1b2c3d4"

# Subsequent call (with cache)
grpcurl -plaintext \
  -d '{"cached_version":"a1b2c3d4"}' \
  localhost:50053 characters.CharacterService/GetCharacterCreationCatalog

# Returns: {"is_current": true}
```

## Testing

1. **Build and run**:
   ```bash
   cargo build -p idklol-characters
   docker-compose up -d
   ```

2. **Test new API**:
   ```bash
   grpcurl -plaintext localhost:50053 list characters.CharacterService
   grpcurl -plaintext -d '{}' localhost:50053 characters.CharacterService/GetCharacterCreationCatalog
   ```

3. **Verify caching**:
   - First call should show database queries in logs
   - Second call should return cached data instantly
   - Call with matching version should return `is_current: true`

## Traditional APIs Still Available

The original APIs remain functional for backward compatibility:
- `ListRaces`
- `ListGendersForRace`
- `ListSkinColors`
- `ListClasses`

New clients should use `GetCharacterCreationCatalog` for optimal performance.
