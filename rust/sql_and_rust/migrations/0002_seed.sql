-- Idempotent seed data: safe to re-run in dev loop.

INSERT INTO users (name, email) VALUES
    ('Alice', 'alice@example.com'),
    ('Bob',   'bob@example.com'),
    ('Carol', 'carol@example.com')
ON CONFLICT (email) DO NOTHING;

INSERT INTO posts (user_id, title, body) VALUES
    ((SELECT id FROM users WHERE email = 'alice@example.com'), 'Hello World',         'My first post on this blog.'),
    ((SELECT id FROM users WHERE email = 'alice@example.com'), 'Rust is fun',          'Exploring ownership today.'),
    ((SELECT id FROM users WHERE email = 'bob@example.com'),   'Postgres tips',        'Indexes are your friend.'),
    ((SELECT id FROM users WHERE email = 'carol@example.com'), 'Window functions',     'ROW_NUMBER and RANK demystified.');

INSERT INTO comments (post_id, user_id, body) VALUES
    (1, (SELECT id FROM users WHERE email = 'bob@example.com'),   'Welcome!'),
    (1, (SELECT id FROM users WHERE email = 'carol@example.com'), 'Nice first post.'),
    (2, (SELECT id FROM users WHERE email = 'bob@example.com'),   'Love the borrow checker.'),
    (3, (SELECT id FROM users WHERE email = 'alice@example.com'), 'Indexes saved my life.'),
    (4, (SELECT id FROM users WHERE email = 'alice@example.com'), 'Great explanation.'),
    (4, (SELECT id FROM users WHERE email = 'bob@example.com'),   'I use PARTITION BY constantly.');
