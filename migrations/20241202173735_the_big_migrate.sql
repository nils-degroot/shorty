CREATE TABLE url (
	url_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	url TEXT NOT NULL CHECK (url <> '')
);