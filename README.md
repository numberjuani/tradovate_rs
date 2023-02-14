# Tradovate.rs
## Example .env file to build client
```
TRADOVATE_CID=your-id
TRADOVATE_SECRET=your-secret
TRADOVATE_APP_ID=your-app-id
TRADOVATE_APP_VERSION=your-app-version
TRADOVATE_USERNAME=your-username
TRADOVATE_PASSWORD=your-password
```
## Tests
To run the tests, set the env to build the client and run `cargo test`

## To install, add this to your Cargo.toml
```
[dependencies]
tradovate_rs = { git = "https://github.com/numberjuani/tradovate_rs", branch = "main" }