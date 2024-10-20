-- schema.sql for Mindful Diary Application

-- Create the category_descriptions table if it doesn't exist
CREATE TABLE IF NOT EXISTS category_descriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    category TEXT NOT NULL UNIQUE,
    explanation TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (category IN ('personal', 'work', 'health', 'travel', 'family', 'hobby', 'finance', 'goal', 'memory', 'reflection', 'unspecified'))
);

-- Create the notes table if it doesn't exist
CREATE TABLE IF NOT EXISTS notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    content TEXT NOT NULL,
    analyzed BOOLEAN NOT NULL DEFAULT 0,
    category_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    analysis TEXT,
    FOREIGN KEY (category_id) REFERENCES category_descriptions(id)
);

-- Create the llm_categories table if it doesn't exist
CREATE TABLE IF NOT EXISTS llm_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    note_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (note_id) REFERENCES notes(id),
    FOREIGN KEY (category_id) REFERENCES category_descriptions(id)
);

-- Create indexes for improved query performance
CREATE INDEX IF NOT EXISTS idx_notes_category_id ON notes(category_id);
CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at);
CREATE INDEX IF NOT EXISTS idx_category_descriptions_category ON category_descriptions(category);
CREATE INDEX IF NOT EXISTS idx_llm_categories_note_id ON llm_categories(note_id);
CREATE INDEX IF NOT EXISTS idx_llm_categories_category_id ON llm_categories(category_id);

-- Insert initial category descriptions if they don't exist
INSERT OR IGNORE INTO category_descriptions (category, explanation) VALUES
('personal', 'Entries related to personal growth, self-discovery, and individual experiences. This category helps you track your personal journey and insights.'),
('work', 'Notes about your professional life, career goals, and work-related challenges or achievements. Use this to reflect on your work-life balance and professional growth.'),
('health', 'Entries focusing on physical and mental well-being, including exercise, nutrition, sleep patterns, and overall health. This category promotes mindfulness about your health habits.'),
('travel', 'Reflections on trips, new experiences in different places, and cultural observations. This category helps you cherish memories and lessons learned from your travels.'),
('family', 'Notes about family relationships, interactions, and dynamics. Use this category to reflect on your connections with loved ones and family events.'),
('hobby', 'Entries related to personal interests, leisure activities, and skills you''re developing outside of work. This category encourages mindfulness in your recreational pursuits.'),
('finance', 'Reflections on your financial situation, goals, and decisions. This category promotes mindful spending and saving habits.'),
('goal', 'Notes about personal or professional objectives, progress towards these goals, and strategies for achievement. Use this to stay focused and motivated.'),
('memory', 'Significant moments, experiences, or insights you want to remember. This category helps preserve important memories and the emotions associated with them.'),
('reflection', 'Deep thoughts, philosophical musings, or general reflections on life, society, or your place in the world. This category encourages deeper self-awareness and mindfulness.'),
('unspecified', 'Entries that don''t clearly fit into other categories or span multiple areas. This category ensures all your thoughts have a place, even if they''re not easily categorized.');