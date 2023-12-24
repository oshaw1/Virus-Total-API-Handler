# Virus-Total-API-Handler

Virus Total API Handler is a API request Handler capable of interacting with the virus total api website to scan files for malware.
I developed this in Rust to help with learning. 

## The handler currently has the following functionality:
POST request to upload files for scanning

## Running the handler
To run the handler in your chosen terminal simply:
```
cargo run --{request flag e.g. post}
```

> [!IMPORTANT]
> Replace the contents of "api_key.txt" with your own VT API key. Please ensure all files in uploads are of a .zip format
