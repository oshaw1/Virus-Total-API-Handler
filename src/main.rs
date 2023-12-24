use std::error::Error;
use std::fs;
use std::io::{self};

fn list_files_in_uploads() -> Result<String, Box<dyn Error>> {
    println!("Type the exact file name of the file you want to upload:");
    let entries = std::fs::read_dir("uploads/")?;
    let files: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| e.file_name().into_string().ok())
        })
        .collect();

    for (index, file) in files.iter().enumerate() {
        println!("{}. {}", index + 1, file);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    input = input.trim().to_string();

    if files.contains(&input) {
        Ok(input)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid file name",
        ))?
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_name = list_files_in_uploads()?;
    let file_path = format!("uploads/{}", file_name);

    let api_key = read_api_key_from_file()?;
    let url = "https://www.virustotal.com/api/v3/files".to_string();
    let password = read_user_input("Enter the password (if any):")?;

    let result = upload_file(api_key, url, file_path, Some(password)).await;
    match result {
        Ok(_) => println!("File upload successful"),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn read_user_input(prompt: &str) -> Result<String, io::Error> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_api_key_from_file() -> Result<String, io::Error> {
    fs::read_to_string("api_key/api_key.txt")
}

async fn upload_file(
    api_key: String,
    url: String,
    file_path: String,
    password: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read(&file_path)?;

    let mut form = reqwest::multipart::Form::new()
        .text("password", password.clone().unwrap_or_else(|| "".to_string()));

    let part = reqwest::multipart::Part::bytes(file_content)
        .file_name(file_path.clone());

    form = form.part("file", part);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("x-apikey", api_key)
        .multipart(form)
        .send()
        .await?;

    println!("Response: {:?}", response.text().await?);
    Ok(())
}
