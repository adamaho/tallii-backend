# Tallii Backend

This is the monolithic backend of the Tallii application.

### Development

In order to develop in the backend you will need to make sure you have rust installed, along with cargo. See [installing rustup](https://www.rust-lang.org/tools/install) for more info. Once you have rust installed following the repo specific instructions below.

1. Install the `sqlx` cli by running: `cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres`
    - This cli is used for managing the database, running migrations and creating new migrations
2. Make sure you have the following installed: `openssl libssl-dev clang llvm-dev libclang-dev`
    - On ubuntu run: `sudo apt install openssl libssl-dev clang llvm-dev libclang-dev`
    - On mac run: `tbd`
3. Start the database container: `docker-compose up database`
4. Build and run the application: `cargo run`

Next
 - Convert usernames to user_ids in paths
 - Done everything up to /api/v1/events/{event_id}/teams in api spec

