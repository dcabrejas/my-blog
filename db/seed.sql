#Dummy data to get started developing
#Insert authors
INSERT INTO authors(id, firstname, lastname, email, alias) VALUES
  (1, 'Diego', 'Cabrejas', 'dcabrejas@icloud.com', 'dcabrejas'),
  (2, 'John', 'Doe', 'johndoe@example.com', 'johndoe')
;

#Insert categories
INSERT INTO categories(id, name, description, url_key) VALUES
  (1, 'Programming', 'Programming related blog posts', 'programming'),
  (2, 'Traveling', 'Traveling related blog posts', 'traveling')
;

#Insert blog posts todo
INSERT INTO posts(id, author_id, title, body, url_key, published, published_at) VALUES
  (1, 1, 'The art of programming', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'the-art-of-programming', 1, '2018-03-11 00:00:00'),
  (2, 2, 'OOP concepts', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'oop-concepts', 1, '2018-03-10 00:00:00'),
  (3, 1, 'The MVC pattern', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'the-mvc-pattern', 1, '2018-03-09 00:00:00'),
  (4, 2, 'PHP Closures explained', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'php-closures-explained', 0, null),
  (5, 1, '10 things to do in tokyo', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'top-things-tokio', 1, '2018-03-05 00:00:00'),
  (6, 2, 'Southern Spain', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'southern-spain', 1, '2018-03-03 00:00:00'),
  (7, 1, 'San Francisco nightlife', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'sf-nightlife', 1, '2018-03-04 00:00:00'),
  (8, 2, 'A week in cape town', REPEAT("Lorem ipsum dolor sit amet. ", 50), 'cape-town-week', 0, null)
;

#Insert category post mappings
INSERT INTO category_post(category_id, post_id) VALUES (1, 1),(1, 2),(1, 3),(1, 4),(2, 5),(2, 6),(2, 7),(2, 8);

