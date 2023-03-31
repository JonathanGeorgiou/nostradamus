CREATE TABLE IF NOT EXISTS players(
  id INT PRIMARY KEY,
  first_name varchar(250) NOT NULL,
  last_name varchar(250) NOT NULL,
  email_address varchar(250),
  favourite_team varchar(250)
);

INSERT INTO players(id, first_name, last_name, email_address, favourite_team) VALUES
(1, 'Jonathan', 'Georgiou', 'jonathangeorgiou94@proton.me', 'Manchester United'),
(2, 'George', 'Georgiou', 'pallouras64@hotmail.com', 'Manchester United'),
(3, 'Nicolas', 'Georgiou', 'nicolasgoergiou04@gmail.com', 'Manchester United'),
(4, 'Papi', 'Georgiou', 'kantopapi@outlook.com', 'Manchester United');
