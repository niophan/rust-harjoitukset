CREATE TABLE IF NOT EXISTS event(
  -- alias for auto-incrementing ROWID (see https://www.sqlite.org/autoinc.html)
  event_id INTEGER PRIMARY KEY,
  event_date DATE NOT NULL,
  event_description TEXT NOT NULL,
  category_id INTEGER NOT NULL,
  -- See https://www.sqlite.org/foreignkeys.html
  FOREIGN KEY (category_id) REFERENCES category(category_id)
);

CREATE TABLE IF NOT EXISTS category(
  category_id INTEGER PRIMARY KEY,
  primary_name TEXT NOT NULL,
  secondary_name TEXT -- is allowed to be NULL!
); 

PRAGMA foreign_keys = ON;

INSERT INTO category VALUES (100, 'history', NULL);

INSERT INTO event (event_date, event_description, category_id)
VALUES ('2003-03-19', 'The United States launches Operation Iraqi Freedom, beginning the invasion of Iraq.', 100);

INSERT INTO event (event_date, event_description, category_id)
VALUES ('1937-03-19', 'Astronomer Fritz Zwicky publishes his research on stellar explosions, coining the term "supernova" and hypothesizing that they are the origin of cosmic rays', 100);

INSERT INTO event (event_date, event_description, category_id)
VALUES ('1977-03-19', 'France performs nuclear test at Mururoa Atoll', 100);

INSERT INTO event (event_date, event_description, category_id)
VALUES ('1982-03-19', 'Falklands War: Argentinian forces land on South Georgia Island, precipitating war with the U.K.', 100);

INSERT INTO event (event_date, event_description, category_id)
VALUES ('2013-03-19', 'NASA''s Mars rover Curiosity discovers further evidence of water-bearing minerals', 100);
