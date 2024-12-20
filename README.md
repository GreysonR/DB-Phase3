# Setup
1. Download `populate_db.exe` and `db_functions.exe` from the GitHub release page, or build from source (requires [Rust installed](https://www.rust-lang.org/tools/install)) with `$ cargo build --release` in both directories (output will be under ./target/release for each dir). Note that I have had issues with the release build of db_functions, so try running `$ cargo build` without --release and check ./target/debug to see of that executable works.
3. Setup a MySQL database on localhost:3306
4. Run `$ populate_db.exe [password] [data file]`, ie `$ populate_db.exe myRootPass data.txt`. The data.txt file used for testing can be found at the root of the repo

# Usage
1. Run `$ db_functions.exe [password]`
2. Follow prompts to execute a function

## Example
`>` indicates user input
```
$ db_functions.exe myRootPass
Connecting as root:myRootPass...
Connected
Choose what function to run:
  1. Get a user's most listened song
  2. Buy a song
  3. Listen to a song
> 2
Enter user's email:
> user1@email.com
Enter song title:
> neighborhood
Which song do you want:
  1. Neighborhood #1 (Tunnels) by Arcade Fire
  2. Neighborhood #2 (Laika) by Arcade Fire
> 2
Neighborhood #2 (Laika) by Arcade Fire added to account user1@email.com

Choose what function to run:
  1. Get a user's most listened song
  2. Buy a song
  3. Listen to a song
> ^C
```
