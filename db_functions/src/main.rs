use mysql::*;
use prelude::Queryable;
// use mysql::prelude::*;

fn print_options(options: Vec<&str>) -> usize {
	let len = options.len();
	for (index, option) in options.into_iter().enumerate() {
		println!("  {}. {}", index + 1, option);
	}

	loop {
		let mut input = String::new();
		std::io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		let input: usize = match input.trim().parse() {
			Ok(value) => value,
			Err(_) => {
				println!("Please enter a number 1-{len}");
				continue;
			}
		};
		if input < 1 || input > len { 
			println!("Please enter a number 1-{len}");
			continue;
		}
		break input - 1;
	}
}

fn user_in_db(conn: &mut PooledConn, email: &str) -> bool {
	let query = r#"SELECT EXISTS(SELECT 1 FROM USER WHERE email = ?)"#;
	let result = conn
		.exec_first(query, (email.trim(),))
		.expect("failed to query DB");
	result.map(|(exists,): (u8,)| exists == 1).unwrap_or(false)
}
fn user_owns_song(conn: &mut PooledConn, email: &str, song: &Song) -> bool {
	let query = r#"
	SELECT EXISTS(
		SELECT 1
		FROM OWNS
		WHERE user_email=? AND song_title=? AND collection_title=? AND artist_name=?
	)"#;
	let result = conn
		.exec_first(query, (email.trim(), &song.song_title, &song.collection_title, &song.artist_name,))
		.expect("failed to query DB");
	result.map(|(exists,): (u8,)| exists == 1).unwrap_or(false)
}
fn get_user_email(conn: &mut PooledConn) -> String {
	// Ask user for email to search
	let mut input = String::new();
	loop {
		println!("Enter user's email: ");
		std::io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		input = input.trim().to_string();
		if user_in_db(conn, &input) { break; }
	}
	input.to_string()
}

struct Song {
	song_title: String,
	collection_title: String,
	artist_name: String
}
fn get_songs_like(conn: &mut PooledConn, song_title_input: &str) -> Vec<Song> {
	let query = r#"
		SELECT SONG.title AS song_title, artist_name, COLLECTION.title AS collection_title
		FROM SONG, COLLECTION
		WHERE SONG.collection_title = COLLECTION.title AND SONG.title LIKE ?;
	"#;

	conn.exec_map(
		query, 
		(format!("%{}%", song_title_input),),
		|(song_title, artist_name, collection_title)| Song { song_title, artist_name, collection_title, })
		.expect("Failed to query DB")
}

#[derive(Debug)]
struct MostListenedResult {
	song_title: String,
	collection_title: String,
	artist_name: String,
	listens: i32,
	collection_type: String,
}
fn find_most_listened(conn: &mut PooledConn) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// Ask user for email to search
	let email = get_user_email(conn);

	println!("Email: {}", email);

	// Query DB
	let query = r#"
		SELECT song_title, collection_title, OWNS.artist_name, listens, collection_type
		FROM OWNS, COLLECTION
		WHERE user_email = ? AND COLLECTION.title = OWNS.collection_title
	"#;
	let user_songs = conn.exec_map(query, (&email,), |(song_title, collection_title, artist_name, listens, collection_type)| {
			MostListenedResult { song_title, collection_title, artist_name, listens, collection_type }
		}).expect("failed to get user's listens");
	
	// Handle case with no results
	if user_songs.len() == 0 {
		println!("User has no owned songs");
		return Ok(())
	}

	// Extract most listened song
	let mut max_listened = &user_songs[0];
	for result in user_songs.iter() {
		if result.listens > max_listened.listens {
			max_listened = result;
		}
	}
	
	// Print result
	println!("{}'s most listened song is {} from the {} {} by {}", &email, max_listened.song_title, max_listened.collection_type, max_listened.collection_title, max_listened.artist_name);

	Ok(())
}

fn buy_song(conn: &mut PooledConn) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// Ask user for email to search
	let email = get_user_email(conn);

	// Get input for song
	println!("Enter song title: ");
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	let song_title_input = input.trim();

	// Search for songs like input and prompt user to choose a song
	let potential_songs = get_songs_like(conn, song_title_input);
	println!("Which song do you want:");
	let potential_songs_str: Vec<String> = potential_songs.iter().map(|song| format!("{} by {}", song.song_title, song.artist_name)).collect();
	let choice = print_options(potential_songs_str.iter().map(|s| s.as_str()).collect());
	let choice = &potential_songs[choice];

	// Insert song into database
	let query = r#"
		INSERT INTO OWNS VALUES (?, ?, ?, ?, ?)
	"#;
	conn.exec_drop(query, (0, &email, &choice.song_title, &choice.collection_title, &choice.artist_name,))?;

	println!("{} by {} added to account {}", choice.song_title, choice.artist_name, &email);

	Ok(())
}

fn listen_to_song(conn: &mut PooledConn) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// Ask user for email
	let email = get_user_email(conn);

	// Get input for song
	println!("Enter song title: ");
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	let song_title_input = input.trim();

	// Search for songs like input and prompt user to choose a song
	let potential_songs = get_songs_like(conn, song_title_input);
	println!("Which song do you want:");
	let potential_songs_str: Vec<String> = potential_songs.iter().map(|song| format!("{} by {}", song.song_title, song.artist_name)).collect();
	let choice = print_options(potential_songs_str.iter().map(|s| s.as_str()).collect());
	let choice = &potential_songs[choice];

	// Check that the user owns the song
	if !user_owns_song(conn, &email, &choice) {
		println!("User {} does not own that song", email);
		return Ok(())
	}

	// Update listen count
	let query = r#"
		UPDATE OWNS
		SET listens = listens + 1
		WHERE user_email=? AND song_title=? AND collection_title=? AND artist_name=?
	"#;
	conn.exec_drop(query, (&email, &choice.song_title, &choice.collection_title, &choice.artist_name,))?;
	println!("Updated listen count of {} by {}", choice.song_title, choice.artist_name);

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

	println!("Connected");

	conn.query_drop("USE music;")?;
	
	let functions = vec!["Get a user's most listened song", "Buy a song", "Listen to a song"];
	loop {
		println!("Choose what function to run: ");
		let call = print_options(functions.clone());
		
		match call {
			0 => find_most_listened(&mut conn)?,
			1 => buy_song(&mut conn)?,
			2 => listen_to_song(&mut conn)?,
			_ => println!("Unknown option")
		}
		println!("");
	}
}