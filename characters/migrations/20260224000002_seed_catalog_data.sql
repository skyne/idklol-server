-- Seed catalog data matching proto enum values

-- Insert races (matching proto Race enum)
INSERT INTO races (id, name) VALUES
    (1, 'Human'),
    (2, 'Elf'),
    (3, 'Dwarf'),
    (4, 'Orc')
ON CONFLICT DO NOTHING;

-- Insert genders (matching proto Gender enum)
INSERT INTO genders (id, name) VALUES
    (1, 'Male'),
    (2, 'Female')
ON CONFLICT DO NOTHING;

-- Insert skin colors (matching proto SkinColor enum)
INSERT INTO skin_colors (id, name) VALUES
    (1, 'Pale'),
    (2, 'Fair'),
    (3, 'Tan'),
    (4, 'Brown'),
    (5, 'Dark'),
    (6, 'Green'),
    (7, 'Gray')
ON CONFLICT DO NOTHING;

-- Insert classes (matching proto CharacterClass enum)
INSERT INTO classes (id, name) VALUES
    (1, 'Warrior'),
    (2, 'Mage'),
    (3, 'Rogue'),
    (4, 'Cleric'),
    (5, 'Ranger')
ON CONFLICT DO NOTHING;

-- Define allowed race-gender combinations
-- Human: Male, Female
INSERT INTO race_gender_allowed (race_id, gender_id) VALUES
    (1, 1), (1, 2)
ON CONFLICT DO NOTHING;

-- Elf: Male, Female
INSERT INTO race_gender_allowed (race_id, gender_id) VALUES
    (2, 1), (2, 2)
ON CONFLICT DO NOTHING;

-- Dwarf: Male, Female
INSERT INTO race_gender_allowed (race_id, gender_id) VALUES
    (3, 1), (3, 2)
ON CONFLICT DO NOTHING;

-- Orc: Male, Female
INSERT INTO race_gender_allowed (race_id, gender_id) VALUES
    (4, 1), (4, 2)
ON CONFLICT DO NOTHING;

-- Define allowed skin colors for race-gender combinations
-- Human Male/Female: Pale, Fair, Tan, Brown, Dark
INSERT INTO race_gender_skin_color_allowed (race_id, gender_id, skin_color_id) VALUES
    (1, 1, 1), (1, 1, 2), (1, 1, 3), (1, 1, 4), (1, 1, 5),
    (1, 2, 1), (1, 2, 2), (1, 2, 3), (1, 2, 4), (1, 2, 5)
ON CONFLICT DO NOTHING;

-- Elf Male/Female: Pale, Fair
INSERT INTO race_gender_skin_color_allowed (race_id, gender_id, skin_color_id) VALUES
    (2, 1, 1), (2, 1, 2),
    (2, 2, 1), (2, 2, 2)
ON CONFLICT DO NOTHING;

-- Dwarf Male/Female: Fair, Tan, Brown
INSERT INTO race_gender_skin_color_allowed (race_id, gender_id, skin_color_id) VALUES
    (3, 1, 2), (3, 1, 3), (3, 1, 4),
    (3, 2, 2), (3, 2, 3), (3, 2, 4)
ON CONFLICT DO NOTHING;

-- Orc Male/Female: Green, Gray
INSERT INTO race_gender_skin_color_allowed (race_id, gender_id, skin_color_id) VALUES
    (4, 1, 6), (4, 1, 7),
    (4, 2, 6), (4, 2, 7)
ON CONFLICT DO NOTHING;

-- Define allowed classes for race-gender combinations
-- Human Male/Female: All classes
INSERT INTO race_gender_class_allowed (race_id, gender_id, class_id) VALUES
    (1, 1, 1), (1, 1, 2), (1, 1, 3), (1, 1, 4), (1, 1, 5),
    (1, 2, 1), (1, 2, 2), (1, 2, 3), (1, 2, 4), (1, 2, 5)
ON CONFLICT DO NOTHING;

-- Elf Male/Female: Mage, Rogue, Ranger
INSERT INTO race_gender_class_allowed (race_id, gender_id, class_id) VALUES
    (2, 1, 2), (2, 1, 3), (2, 1, 5),
    (2, 2, 2), (2, 2, 3), (2, 2, 5)
ON CONFLICT DO NOTHING;

-- Dwarf Male/Female: Warrior, Cleric
INSERT INTO race_gender_class_allowed (race_id, gender_id, class_id) VALUES
    (3, 1, 1), (3, 1, 4),
    (3, 2, 1), (3, 2, 4)
ON CONFLICT DO NOTHING;

-- Orc Male/Female: Warrior, Rogue
INSERT INTO race_gender_class_allowed (race_id, gender_id, class_id) VALUES
    (4, 1, 1), (4, 1, 3),
    (4, 2, 1), (4, 2, 3)
ON CONFLICT DO NOTHING;
