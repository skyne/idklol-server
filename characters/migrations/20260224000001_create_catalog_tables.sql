-- Create catalog tables for character creation options

CREATE TABLE IF NOT EXISTS races (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS genders (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS skin_colors (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS classes (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Junction tables to define allowed combinations
CREATE TABLE IF NOT EXISTS race_gender_allowed (
    race_id INTEGER NOT NULL REFERENCES races(id),
    gender_id INTEGER NOT NULL REFERENCES genders(id),
    PRIMARY KEY (race_id, gender_id)
);

CREATE TABLE IF NOT EXISTS race_gender_skin_color_allowed (
    race_id INTEGER NOT NULL REFERENCES races(id),
    gender_id INTEGER NOT NULL REFERENCES genders(id),
    skin_color_id INTEGER NOT NULL REFERENCES skin_colors(id),
    PRIMARY KEY (race_id, gender_id, skin_color_id)
);

CREATE TABLE IF NOT EXISTS race_gender_class_allowed (
    race_id INTEGER NOT NULL REFERENCES races(id),
    gender_id INTEGER NOT NULL REFERENCES genders(id),
    class_id INTEGER NOT NULL REFERENCES classes(id),
    PRIMARY KEY (race_id, gender_id, class_id)
);

-- Characters table
CREATE TABLE IF NOT EXISTS characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    user_email VARCHAR(255) NOT NULL,
    race_id INTEGER NOT NULL REFERENCES races(id),
    gender_id INTEGER NOT NULL REFERENCES genders(id),
    skin_color_id INTEGER NOT NULL REFERENCES skin_colors(id),
    class_id INTEGER NOT NULL REFERENCES classes(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_race_gender FOREIGN KEY (race_id, gender_id) 
        REFERENCES race_gender_allowed(race_id, gender_id),
    CONSTRAINT valid_skin_color FOREIGN KEY (race_id, gender_id, skin_color_id) 
        REFERENCES race_gender_skin_color_allowed(race_id, gender_id, skin_color_id),
    CONSTRAINT valid_class FOREIGN KEY (race_id, gender_id, class_id) 
        REFERENCES race_gender_class_allowed(race_id, gender_id, class_id)
);

CREATE INDEX idx_characters_user_email ON characters(user_email);
CREATE INDEX idx_characters_created_at ON characters(created_at DESC);
