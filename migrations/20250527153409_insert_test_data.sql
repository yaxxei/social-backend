-- Add migration script here

-- English

-- Used password: password123 (same hashed password for all users)
INSERT INTO users (nickname, role, email, hashed_password) VALUES
('AlexTheAdmin', 'admin', 'alex.admin@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('ModeratorMike', 'moderator', 'mike.moder@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('TechGuru', 'user', 'tech.guru@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('TravelLover', 'user', 'travel.lover@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('FoodieAnna', 'user', 'anna.food@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('Admin', 'admin', 'admin@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('Moderator', 'moderator', 'moderator@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('User', 'user', 'user@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8');

-- Adding communities
INSERT INTO communities (user_id, name, description, is_private) VALUES
((SELECT id FROM users WHERE nickname = 'TechGuru'), 'Tech Enthusiasts', 'A community for technology lovers and gadget geeks', FALSE),
((SELECT id FROM users WHERE nickname = 'TravelLover'), 'World Travelers', 'Share your travel experiences and tips', FALSE),
((SELECT id FROM users WHERE nickname = 'FoodieAnna'), 'Food Lovers Club', 'Everything about cooking and delicious food', TRUE),
((SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), 'Programming Hub', 'Advanced programming discussions', FALSE),
((SELECT id FROM users WHERE nickname = 'ModeratorMike'), 'Photography Pros', 'For professional photographers and enthusiasts', FALSE);

-- Adding posts
INSERT INTO posts (user_id, community_id, title, content) VALUES
((SELECT id FROM users WHERE nickname = 'TechGuru'), (SELECT id FROM communities WHERE name = 'Tech Enthusiasts'), 
'New smartphone reviews 2023',
'Today I want to share my thoughts about the latest smartphones released this year. The Samsung Galaxy S23 Ultra impresses with its camera system, especially the 200MP main sensor. Battery life is excellent, easily lasting a full day of heavy use. On the other hand, the iPhone 14 Pro Max shows significant improvements in video stabilization and the new Dynamic Island is quite innovative. What do you think about this year''s flagship models?'),

((SELECT id FROM users WHERE nickname = 'TravelLover'), (SELECT id FROM communities WHERE name = 'World Travelers'), 
'My adventure in Bali',
'Just returned from an amazing two-week trip to Bali! The island is absolutely breathtaking. I stayed in Ubud where I visited the famous Monkey Forest and Tegallalang Rice Terraces. The most memorable experience was climbing Mount Batur to watch the sunrise - it was challenging but totally worth it. The local food is delicious, especially nasi goreng and babi guling. Highly recommend visiting during the dry season (April to October) for the best weather.'),

((SELECT id FROM users WHERE nickname = 'FoodieAnna'), (SELECT id FROM communities WHERE name = 'Food Lovers Club'), 
'Authentic Italian pasta recipe',
'Here''s my grandmother''s authentic recipe for Spaghetti Carbonara (the real Roman way, no cream!): 

Ingredients:
- 400g spaghetti
- 150g guanciale (or pancetta)
- 4 egg yolks
- 50g pecorino romano
- 50g parmesan
- Freshly ground black pepper

Method:
1. Cook pasta in salted boiling water
2. Cut guanciale into small cubes and fry until crispy
3. Whisk egg yolks with grated cheeses and pepper
4. Drain pasta, mix with guanciale, then quickly stir in egg mixture
5. Serve immediately with extra cheese and pepper

The secret is to work quickly and use the pasta water to adjust consistency!'),

((SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), (SELECT id FROM communities WHERE name = 'Programming Hub'), 
'Rust vs Go in 2023',
'As both Rust and Go continue to grow in popularity, I wanted to share my comparison after using both for production systems. Rust excels in performance-critical applications where memory safety is crucial, like game engines or operating systems. The borrow checker, while challenging to learn, prevents entire classes of bugs. Go shines in cloud services and distributed systems - its simplicity, fast compilation, and excellent concurrency model make it perfect for microservices. Personally, I use Rust for system-level components and Go for web services. What''s your preference?'),

((SELECT id FROM users WHERE nickname = 'ModeratorMike'), (SELECT id FROM communities WHERE name = 'Photography Pros'), 
'Best lenses for portrait photography',
'After 10 years as a professional portrait photographer, here are my lens recommendations:

1. Canon EF 85mm f/1.2L II - The "king of portraits" with dreamy bokeh
2. Sony FE 135mm f/1.8 GM - Incredibly sharp with beautiful compression
3. Nikon 105mm f/1.4E - Exceptional for full-body portraits
4. Sigma 50mm f/1.4 DG HSM Art - Great all-rounder at affordable price

For beginners, I recommend starting with a fast 50mm (like the nifty fifty) to learn composition before investing in more specialized glass. Remember, lighting and posing are often more important than the lens itself!');

-- Adding comments
INSERT INTO comments (post_id, user_id, content) VALUES
((SELECT id FROM posts WHERE title LIKE 'New smartphone%'), (SELECT id FROM users WHERE nickname = 'TravelLover'), 
'I''m still using my 3-year-old phone and it works fine. Do we really need to upgrade every year?'),

((SELECT id FROM posts WHERE title LIKE 'My adventure in Bali%'), (SELECT id FROM users WHERE nickname = 'FoodieAnna'), 
'Bali is magical! Did you try the local coffee? Luwak coffee is quite an experience.'),

((SELECT id FROM posts WHERE title LIKE 'Authentic Italian%'), (SELECT id FROM users WHERE nickname = 'TechGuru'), 
'Thanks for sharing! I tried this recipe last night and it was delicious. Added some chili flakes for extra kick.'),

((SELECT id FROM posts WHERE title LIKE 'Rust vs Go%'), (SELECT id FROM users WHERE nickname = 'ModeratorMike'), 
'Great analysis! I''ve been using Go for our image processing microservices and it handles concurrent requests beautifully.'),

((SELECT id FROM posts WHERE title LIKE 'Best lenses%'), (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), 
'What about mirrorless options? The Sony 85mm f/1.4 GM is fantastic for portraits on the A7 series.');

-- Replies to comments (parent comments)
INSERT INTO comments (post_id, user_id, parent_comment_id, content) VALUES
((SELECT id FROM posts WHERE title LIKE 'New smartphone%'), (SELECT id FROM users WHERE nickname = 'TechGuru'), 
(SELECT id FROM comments WHERE content LIKE 'I''m still using my%'), 
'For casual users, definitely no need to upgrade often. But if you''re into mobile photography or gaming, the new chips and cameras make a noticeable difference.'),

((SELECT id FROM posts WHERE title LIKE 'My adventure in Bali%'), (SELECT id FROM users WHERE nickname = 'TravelLover'), 
(SELECT id FROM comments WHERE content LIKE 'Bali is magical%'), 
'Yes! The coffee tasting was incredible, though the luwak coffee was a bit too expensive for my budget. The regular Balinese coffee is amazing too.');

-- Adding likes
INSERT INTO likes (post_id, user_id, like_type) VALUES
((SELECT id FROM posts WHERE title LIKE 'New smartphone%'), (SELECT id FROM users WHERE nickname = 'ModeratorMike'), 1),
((SELECT id FROM posts WHERE title LIKE 'New smartphone%'), (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), 1),
((SELECT id FROM posts WHERE title LIKE 'Authentic Italian%'), (SELECT id FROM users WHERE nickname = 'TravelLover'), 1),
((SELECT id FROM posts WHERE title LIKE 'Best lenses%'), (SELECT id FROM users WHERE nickname = 'TechGuru'), 1),
((SELECT id FROM posts WHERE title LIKE 'Rust vs Go%'), (SELECT id FROM users WHERE nickname = 'FoodieAnna'), 1);

-- Likes on comments
INSERT INTO likes (comment_id, user_id, like_type) VALUES
((SELECT id FROM comments WHERE content LIKE 'I''m still using my%'), (SELECT id FROM users WHERE nickname = 'FoodieAnna'), 1),
((SELECT id FROM comments WHERE content LIKE 'Bali is magical%'), (SELECT id FROM users WHERE nickname = 'TechGuru'), 1),
((SELECT id FROM comments WHERE content LIKE 'Great analysis%'), (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), 1);

-- User subscriptions to communities
INSERT INTO follows (user_id, community_id) VALUES
((SELECT id FROM users WHERE nickname = 'TechGuru'), (SELECT id FROM communities WHERE name = 'Programming Hub')),
((SELECT id FROM users WHERE nickname = 'TravelLover'), (SELECT id FROM communities WHERE name = 'Food Lovers Club')),
((SELECT id FROM users WHERE nickname = 'FoodieAnna'), (SELECT id FROM communities WHERE name = 'World Travelers')),
((SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), (SELECT id FROM communities WHERE name = 'Tech Enthusiasts')),
((SELECT id FROM users WHERE nickname = 'ModeratorMike'), (SELECT id FROM communities WHERE name = 'Photography Pros'));

-- Russian

-- Adding Russian-speaking users
INSERT INTO users (nickname, role, email, hashed_password) VALUES
('RussianDev', 'user', 'dev.russian@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('MoscowTraveler', 'user', 'moscow.travel@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8'),
('SiberianCook', 'user', 'siberia.cook@example.com', '$argon2id$v=19$m=19456,t=2,p=1$dvhaFX+7HufoIsepJbuUdw$Y263VYojNGqoQIL9lEa5fcxjkaVkNZqsKuCahBD1WK8');

-- Adding Russian-speaking communities (names remain in English)
INSERT INTO communities (user_id, name, description, is_private) VALUES
((SELECT id FROM users WHERE nickname = 'RussianDev'), 'Russian IT', 'Сообщество русскоязычных IT-специалистов', FALSE),
((SELECT id FROM users WHERE nickname = 'MoscowTraveler'), 'Explore Russia', 'Путешествия по России и СНГ', FALSE),
((SELECT id FROM users WHERE nickname = 'SiberianCook'), 'Russian Cuisine', 'Традиционная и современная русская кухня', TRUE);

-- Adding Russian-language posts
INSERT INTO posts (user_id, community_id, title, content) VALUES
((SELECT id FROM users WHERE nickname = 'RussianDev'), (SELECT id FROM communities WHERE name = 'Russian IT'), 
'Работа в IT в России 2023',
'Как изменился рынок IT в России за последний год? В моей компании многие перешли на удалёнку, появилось больше проектов для внутреннего рынка. Зарплаты в долларовом эквиваленте упали, но в рублях остались на прежнем уровне. Какие технологии сейчас наиболее востребованы? В нашем регионе активно ищут 1С-разработчиков и Python-программистов.'),

((SELECT id FROM users WHERE nickname = 'MoscowTraveler'), (SELECT id FROM communities WHERE name = 'Explore Russia'), 
'Золотое кольцо России',
'Проехал по маршруту Золотого кольца за 10 дней. Особенно впечатлили Суздаль и Ростов Великий - настоящая русская история! Советы путешественникам: 1) Берите наличные - не везде принимают карты 2) Лучшее время - начало осени 3) Жильё лучше бронировать заранее. Кто-то ещё путешествовал по этому маршруту?'),

((SELECT id FROM users WHERE nickname = 'SiberianCook'), (SELECT id FROM communities WHERE name = 'Russian Cuisine'), 
'Рецепт настоящих сибирских пельменей',
'Семейный рецепт из Красноярска:

Тесто:
- 500 г муки
- 1 яйцо
- 150 мл воды
- щепотка соли

Начинка:
- 300 г говядины
- 300 г свинины
- 2 луковицы
- соль, перец по вкусу

Важное отличие сибирских пельменей - соотношение мяса 1:1 и обязательное добавление льда в фарш (около 50 мл на 600 г мяса). Лепим небольшие пельмени, варим 5-7 минут после всплытия. Подаём со сметаной, уксусом и черным перцем.');

-- Adding more community subscribers (varying amounts)
-- Russian IT - 5 subscribers
INSERT INTO follows (user_id, community_id) VALUES
((SELECT id FROM users WHERE nickname = 'TechGuru'), (SELECT id FROM communities WHERE name = 'Russian IT')),
((SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), (SELECT id FROM communities WHERE name = 'Russian IT')),
((SELECT id FROM users WHERE nickname = 'ModeratorMike'), (SELECT id FROM communities WHERE name = 'Russian IT')),
((SELECT id FROM users WHERE nickname = 'RussianDev'), (SELECT id FROM communities WHERE name = 'Russian IT')),
((SELECT id FROM users WHERE nickname = 'FoodieAnna'), (SELECT id FROM communities WHERE name = 'Russian IT'));

-- Explore Russia - 3 subscribers
INSERT INTO follows (user_id, community_id) VALUES
((SELECT id FROM users WHERE nickname = 'TravelLover'), (SELECT id FROM communities WHERE name = 'Explore Russia')),
((SELECT id FROM users WHERE nickname = 'MoscowTraveler'), (SELECT id FROM communities WHERE name = 'Explore Russia')),
((SELECT id FROM users WHERE nickname = 'SiberianCook'), (SELECT id FROM communities WHERE name = 'Explore Russia'));

-- Russian Cuisine - 2 subscribers
INSERT INTO follows (user_id, community_id) VALUES
((SELECT id FROM users WHERE nickname = 'FoodieAnna'), (SELECT id FROM communities WHERE name = 'Russian Cuisine')),
((SELECT id FROM users WHERE nickname = 'SiberianCook'), (SELECT id FROM communities WHERE name = 'Russian Cuisine'));

-- Adding Russian-language comments
INSERT INTO comments (post_id, user_id, content) VALUES
((SELECT id FROM posts WHERE title = 'Работа в IT в России 2023'), (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), 
'У нас в компании тоже перешли на гибридный формат. Интересно, как изменились зарплаты у фронтенд-разработчиков?'),

((SELECT id FROM posts WHERE title = 'Золотое кольцо России'), (SELECT id FROM users WHERE nickname = 'TravelLover'), 
'Я ездил зимой - совсем другие впечатления! Снежные пейзажи и мало туристов.'),

((SELECT id FROM posts WHERE title = 'Рецепт настоящих сибирских пельменей'), (SELECT id FROM users WHERE nickname = 'FoodieAnna'), 
'А вы пробовали добавлять немного свиного сала в фарш? Мой дед так делал, получается сочнее.');

-- Adding replies to Russian comments
INSERT INTO comments (post_id, user_id, parent_comment_id, content) VALUES
((SELECT id FROM posts WHERE title = 'Работа в IT в России 2023'), (SELECT id FROM users WHERE nickname = 'RussianDev'), 
(SELECT id FROM comments WHERE content LIKE 'У нас в компании%'), 
'По нашим данным, фронтенд-разработчики потеряли около 30% в долларовом эквиваленте, но многие перешли на более интересные проекты.'),

((SELECT id FROM posts WHERE title = 'Рецепт настоящих сибирских пельменей'), (SELECT id FROM users WHERE nickname = 'SiberianCook'), 
(SELECT id FROM comments WHERE content LIKE 'А вы пробовали добавлять%'), 
'Да, конечно! В традиционном рецепте иногда добавляют 50-100 г сала. Но я не стал усложнять рецепт для первого раза.');

-- Adding likes to Russian posts (varying amounts)
-- Russian IT post - 3 likes
INSERT INTO likes (post_id, user_id, like_type) VALUES
((SELECT id FROM posts WHERE title = 'Работа в IT в России 2023'), (SELECT id FROM users WHERE nickname = 'TechGuru'), 1),
((SELECT id FROM posts WHERE title = 'Работа в IT в России 2023'), (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'), 1),
((SELECT id FROM posts WHERE title = 'Работа в IT в России 2023'), (SELECT id FROM users WHERE nickname = 'ModeratorMike'), 1);

-- Explore Russia post - 1 like
INSERT INTO likes (post_id, user_id, like_type) VALUES
((SELECT id FROM posts WHERE title = 'Золотое кольцо России'), (SELECT id FROM users WHERE nickname = 'TravelLover'), 1);

-- Russian Cuisine post - 2 likes
INSERT INTO likes (post_id, user_id, like_type) VALUES
((SELECT id FROM posts WHERE title = 'Рецепт настоящих сибирских пельменей'), (SELECT id FROM users WHERE nickname = 'FoodieAnna'), 1),
((SELECT id FROM posts WHERE title = 'Рецепт настоящих сибирских пельменей'), (SELECT id FROM users WHERE nickname = 'MoscowTraveler'), 1);

-- Reporting test data (without status field)
-- Adding test reports for posts
INSERT INTO reports (report_type, reported_id, reporter_id, reason) VALUES
('post', 
 (SELECT id FROM posts WHERE title LIKE 'New smartphone%'), 
 (SELECT id FROM users WHERE nickname = 'TravelLover'),
 'This post contains inaccurate information about iPhone cameras'),

('post', 
 (SELECT id FROM posts WHERE title LIKE 'My adventure in Bali%'), 
 (SELECT id FROM users WHERE nickname = 'FoodieAnna'),
 'The post promotes dangerous activities (mountain climbing) without proper warnings'),

('post', 
 (SELECT id FROM posts WHERE title = 'Работа в IT в России 2023'), 
 (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'),
 'Пост содержит политически окрашенную информацию');

-- Reports for comments
INSERT INTO reports (report_type, reported_id, reporter_id, reason) VALUES
('comment', 
 (SELECT id FROM comments WHERE content LIKE 'I''m still using my%'), 
 (SELECT id FROM users WHERE nickname = 'TechGuru'),
 'Comment is off-topic and doesn''t contribute to the discussion'),

('comment', 
 (SELECT id FROM comments WHERE content LIKE 'У нас в компании%'), 
 (SELECT id FROM users WHERE nickname = 'RussianDev'),
 'Комментарий содержит некорректную информацию о зарплатах'),

('comment', 
 (SELECT id FROM comments WHERE content LIKE 'А вы пробовали добавлять%'), 
 (SELECT id FROM users WHERE nickname = 'SiberianCook'),
 'Не соответствует традиционному рецепту, может ввести в заблуждение');

-- Reports for users
INSERT INTO reports (report_type, reported_id, reporter_id, reason) VALUES
('user', 
 (SELECT id FROM users WHERE nickname = 'TechGuru'), 
 (SELECT id FROM users WHERE nickname = 'TravelLover'),
 'User consistently posts misleading tech information'),

('user', 
 (SELECT id FROM users WHERE nickname = 'MoscowTraveler'), 
 (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'),
 'Пользователь размещает коммерческие предложения под видом личного опыта'),

('user', 
 (SELECT id FROM users WHERE nickname = 'FoodieAnna'), 
 (SELECT id FROM users WHERE nickname = 'ModeratorMike'),
 'User shares recipes that could be dangerous if followed incorrectly');

-- Adding more varied reports for filtering testing
-- Reports in pending status
INSERT INTO reports (report_type, reported_id, reporter_id, reason) VALUES
('post', 
 (SELECT id FROM posts WHERE title LIKE 'Rust vs Go%'), 
 (SELECT id FROM users WHERE nickname = 'ModeratorMike'),
 'Post contains biased comparison without proper technical details'),

('comment', 
 (SELECT id FROM comments WHERE content LIKE 'Great analysis%'), 
 (SELECT id FROM users WHERE nickname = 'TechGuru'),
 'Comment promotes one technology over another without justification'),

('user', 
 (SELECT id FROM users WHERE nickname = 'RussianDev'), 
 (SELECT id FROM users WHERE nickname = 'SiberianCook'),
 'Пользователь ведет себя агрессивно в личных сообщениях');

-- Reports in processed status
INSERT INTO reports (report_type, reported_id, reporter_id, reason) VALUES
('post', 
 (SELECT id FROM posts WHERE title LIKE 'Best lenses%'), 
 (SELECT id FROM users WHERE nickname = 'AlexTheAdmin'),
 'Post contains affiliate links without disclosure'),

('comment', 
 (SELECT id FROM comments WHERE content LIKE 'Thanks for sharing%'), 
 (SELECT id FROM users WHERE nickname = 'FoodieAnna'),
 'Comment appears to be spam (generic praise without substance'),

('user', 
 (SELECT id FROM users WHERE nickname = 'TravelLover'), 
 (SELECT id FROM users WHERE nickname = 'TechGuru'),
 'User repeatedly posts same content across multiple communities');


-- Chats
-- Create private chats between the demo users
INSERT INTO chats (is_group) VALUES (FALSE), (FALSE), (FALSE);

-- Add chat members for these private chats
-- Chat 1: Admin and Moderator
INSERT INTO chat_members (chat_id, user_id) VALUES
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 0)), 
 (SELECT id FROM users WHERE nickname = 'Admin')),
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 0)), 
 (SELECT id FROM users WHERE nickname = 'Moderator'));

-- Chat 2: Admin and User
INSERT INTO chat_members (chat_id, user_id) VALUES
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 1)), 
 (SELECT id FROM users WHERE nickname = 'Admin')),
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 1)), 
 (SELECT id FROM users WHERE nickname = 'User'));

-- Chat 3: Moderator and User
INSERT INTO chat_members (chat_id, user_id) VALUES
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)), 
 (SELECT id FROM users WHERE nickname = 'Moderator')),
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)), 
 (SELECT id FROM users WHERE nickname = 'User'));

-- Create a group chat with all three users
INSERT INTO chats (name, is_group) VALUES ('Техподдержка', TRUE);

-- Add members to group chat
INSERT INTO chat_members (chat_id, user_id, role) VALUES
((SELECT id FROM chats WHERE name = 'Техподдержка'), 
 (SELECT id FROM users WHERE nickname = 'Admin'), 'owner'),
((SELECT id FROM chats WHERE name = 'Техподдержка'), 
 (SELECT id FROM users WHERE nickname = 'Moderator'), 'member'),
((SELECT id FROM chats WHERE name = 'Техподдержка'), 
 (SELECT id FROM users WHERE nickname = 'User'), 'member');

-- Add messages to Admin-Moderator chat (in Russian)
INSERT INTO messages (chat_id, sender_id, content) VALUES
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 0)),
 (SELECT id FROM users WHERE nickname = 'Admin'),
 'Привет! Как дела с модерацией сегодня?'),
 
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 0)),
 (SELECT id FROM users WHERE nickname = 'Moderator'),
 'Всё нормально, пока тихо. Было пару спам-постов, но я их удалил.'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 0)),
 (SELECT id FROM users WHERE nickname = 'Admin'),
 'Хорошо. Напомни, пожалуйста, проверить новые заявки в сообщества.');

-- Add messages to Admin-User chat (in Russian)
INSERT INTO messages (chat_id, sender_id, content) VALUES
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 1)),
 (SELECT id FROM users WHERE nickname = 'User'),
 'Здравствуйте! У меня вопрос по функционалу сайта.'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 1)),
 (SELECT id FROM users WHERE nickname = 'Admin'),
 'Здравствуйте! Слушаю вас.'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 1)),
 (SELECT id FROM users WHERE nickname = 'User'),
 'Как создать новое сообщество? Не могу найти кнопку.'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 1)),
 (SELECT id FROM users WHERE nickname = 'Admin'),
 'Нужно зайти в раздел "Сообщества" и там будет кнопка "Создать" в правом верхнем углу.');

-- Add messages to Moderator-User chat (in Russian)
INSERT INTO messages (chat_id, sender_id, content) VALUES
((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)),
 (SELECT id FROM users WHERE nickname = 'Moderator'),
 'Привет! Твой последний пост был помечен жалобой.'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)),
 (SELECT id FROM users WHERE nickname = 'User'),
 'Ой, а что не так?'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)),
 (SELECT id FROM users WHERE nickname = 'Moderator'),
 'Там была спорная информация о политике. Лучше избегать таких тем.'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)),
 (SELECT id FROM users WHERE nickname = 'User'),
 'Понял, больше не буду. Можно восстановить пост если я его исправлю?'),

((SELECT id FROM chats WHERE id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)),
 (SELECT id FROM users WHERE nickname = 'Moderator'),
 'Да, пришли мне исправленный текст, я посмотрю.');

-- Add messages to group chat (in Russian)
INSERT INTO messages (chat_id, sender_id, content) VALUES
((SELECT id FROM chats WHERE name = 'Техподдержка'),
 (SELECT id FROM users WHERE nickname = 'Admin'),
 'Всем привет! Сегодня планируем обновление системы в 23:00.'),

((SELECT id FROM chats WHERE name = 'Техподдержка'),
 (SELECT id FROM users WHERE nickname = 'User'),
 'Надолго ли?'),

((SELECT id FROM chats WHERE name = 'Техподдержка'),
 (SELECT id FROM users WHERE nickname = 'Admin'),
 'Примерно на 1 час. Постараемся уложиться быстрее.'),

((SELECT id FROM chats WHERE name = 'Техподдержка'),
 (SELECT id FROM users WHERE nickname = 'Moderator'),
 'Нужно ли предупредить пользователей о возможных перебоях?'),

((SELECT id FROM chats WHERE name = 'Техподдержка'),
 (SELECT id FROM users WHERE nickname = 'Admin'),
 'Да, размести уведомление в главных сообществах за час до начала.');

-- Update message statuses (mark messages as sent and read)
-- For Admin-Moderator chat
INSERT INTO message_statuses (message_id, chat_id, user_id, is_send, is_read, read_at)
SELECT m.id, m.chat_id, cm.user_id, TRUE, 
       CASE WHEN u.nickname = 'Moderator' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Admin') THEN TRUE
            WHEN u.nickname = 'Admin' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Moderator') THEN TRUE
            ELSE FALSE END,
       CASE WHEN u.nickname = 'Moderator' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Admin') THEN NOW()
            WHEN u.nickname = 'Admin' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Moderator') THEN NOW()
            ELSE NULL END
FROM messages m
JOIN chats c ON m.chat_id = c.id
JOIN chat_members cm ON c.id = cm.chat_id
JOIN users u ON cm.user_id = u.id
WHERE c.id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 0)
AND u.nickname IN ('Admin', 'Moderator');

-- For Admin-User chat
INSERT INTO message_statuses (message_id, chat_id, user_id, is_send, is_read, read_at)
SELECT m.id, m.chat_id, cm.user_id, TRUE, 
       CASE WHEN u.nickname = 'User' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Admin') THEN TRUE
            WHEN u.nickname = 'Admin' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'User') THEN TRUE
            ELSE FALSE END,
       CASE WHEN u.nickname = 'User' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Admin') THEN NOW()
            WHEN u.nickname = 'Admin' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'User') THEN NOW()
            ELSE NULL END
FROM messages m
JOIN chats c ON m.chat_id = c.id
JOIN chat_members cm ON c.id = cm.chat_id
JOIN users u ON cm.user_id = u.id
WHERE c.id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 1)
AND u.nickname IN ('Admin', 'User');

-- For Moderator-User chat
INSERT INTO message_statuses (message_id, chat_id, user_id, is_send, is_read, read_at)
SELECT m.id, m.chat_id, cm.user_id, TRUE, 
       CASE WHEN u.nickname = 'User' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Moderator') THEN TRUE
            WHEN u.nickname = 'Moderator' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'User') THEN TRUE
            ELSE FALSE END,
       CASE WHEN u.nickname = 'User' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'Moderator') THEN NOW()
            WHEN u.nickname = 'Moderator' AND m.sender_id = (SELECT id FROM users WHERE nickname = 'User') THEN NOW()
            ELSE NULL END
FROM messages m
JOIN chats c ON m.chat_id = c.id
JOIN chat_members cm ON c.id = cm.chat_id
JOIN users u ON cm.user_id = u.id
WHERE c.id = (SELECT id FROM chats ORDER BY created_at LIMIT 1 OFFSET 2)
AND u.nickname IN ('Moderator', 'User');

-- For group chat
INSERT INTO message_statuses (message_id, chat_id, user_id, is_send, is_read, read_at)
SELECT m.id, m.chat_id, cm.user_id, TRUE, 
       CASE WHEN u.nickname != (SELECT nickname FROM users WHERE id = m.sender_id) THEN TRUE ELSE FALSE END,
       CASE WHEN u.nickname != (SELECT nickname FROM users WHERE id = m.sender_id) THEN NOW() ELSE NULL END
FROM messages m
JOIN chats c ON m.chat_id = c.id
JOIN chat_members cm ON c.id = cm.chat_id
JOIN users u ON cm.user_id = u.id
WHERE c.name = 'Техподдержка';