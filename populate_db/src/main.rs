use mysql::*;
use mysql::prelude::*;

use std::fs;

fn create_tables(conn: &mut PooledConn) -> std::result::Result<(), Box<dyn std::error::Error>> {
	conn.query_drop(
		r"
		USE music;
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
		")?;
	Ok(())
}

fn parse_file(conn: &mut PooledConn, schema_filename: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let base_path = std::env::current_dir().unwrap();
	let mut schema_file_path = base_path.clone();
	schema_file_path.push(schema_filename);
	let contents = fs::read_to_string(&schema_file_path).expect(&format!("failed to read file {}", schema_file_path.to_str().unwrap()));

	let mut table: Option<String> = None;
	for line in contents.lines() {
		if line.len() == 0 { continue; } // ignore empty lines
		
		let mut chars = line.chars();
		if chars.next().unwrap() == '#' { // line is table name
			let new_table: String = chars.collect();
			table = Some(new_table.trim().to_string());
			// println!("{}", table.as_ref().unwrap());
		}
		else { // line is row to insert
			let table_name = table.as_ref().unwrap_or_else(|| { panic!("no table named specified") });
			// println!("    {line}");
			let query = "INSERT INTO ".to_owned() + table_name + " VALUES (" + line + ")";
			conn.query_drop(query)?;
		}
	}

	Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = std::env::args().collect();

	let default_password = "password".to_string();
	let password = args.get(1).unwrap_or(&default_password);
	println!("Connecting as root:{password}...");
	let url = format!("mysql://root:{password}@localhost:3306"); // assumes your password is 'password'

	let pool = Pool::new(url.as_str()).expect("HELP pass your root password with `programName.exe myPassword`");
	let mut conn = pool.get_conn()?;

	// Create DB
	println!("Creating database...");
	match conn.query_drop(r"CREATE DATABASE music;") {
		Err(err) => {
			let message = format!("failed to create database: {}", err);
			if message.contains("exists") {
				println!("{message}"); // print error and continue
			}
			else {
				panic!("{message}"); // exit program if it's a real error
			}
		},
		Ok(()) => (),
	};

	// Make sure in database
	conn.query_drop(r"USE music;")?;
	
	// Create tables
	println!("Creating tables...");
	match create_tables(&mut conn) {
		Err(err) => {
			let message = format!("failed to create tables: {}", err);
			if message.contains("exists") {
				println!("{message}"); // print error and continue
			}
			else {
				panic!("{message}"); // exit program if it's a real error
			}
		},
		Ok(()) => (),
	};
	
	// Make sure using music DB
	conn.query_drop(r"USE music;")?;

	// Parse input file
	println!("Populating database...");
	let schema_filename = args.get(2)
		.unwrap_or_else(|| {
			panic!("expected filename as second argument");
		});
	parse_file(&mut conn, &schema_filename)?;
	
	println!("Done");
	Ok(())
}
