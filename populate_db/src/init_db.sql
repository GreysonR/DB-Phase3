CREATE TABLE ARTIST (
	artist_name VARCHAR(255),
	label VARCHAR(255),
	biography VARCHAR(255),
	PRIMARY KEY (artist_name)
);
CREATE TABLE USER (
	email VARCHAR(255),
	username VARCHAR(255),
	PRIMARY KEY (email)
);
CREATE TABLE COLLECTION (
	collection_type VARCHAR(15) NOT NULL,
	title VARCHAR(255),
	artist_name VARCHAR(255),
	PRIMARY KEY (title),
	FOREIGN KEY (artist_name) REFERENCES ARTIST(artist_name)
);
CREATE TABLE SONG (
	title VARCHAR(255),
	length INT NOT NULL,
	collection_title VARCHAR(255),
	PRIMARY KEY (title),
	FOREIGN KEY (collection_title) REFERENCES COLLECTION(title)
);
CREATE TABLE OWNS (
	listens INT NOT NULL,
	user_email VARCHAR(255),
	song_title VARCHAR(255),
	collection_title VARCHAR(255),
	artist_name VARCHAR(255),
	FOREIGN KEY (user_email) REFERENCES USER(email),
	FOREIGN KEY (song_title) REFERENCES SONG(title),
	FOREIGN KEY (collection_title) REFERENCES COLLECTION(title),
	FOREIGN KEY (artist_name) REFERENCES ARTIST(artist_name)
);
CREATE TABLE RELEASE_DATES (
	collection_title VARCHAR(255),
	date DATE,
	PRIMARY KEY (date),
	FOREIGN KEY (collection_title) REFERENCES COLLECTION(title)
);