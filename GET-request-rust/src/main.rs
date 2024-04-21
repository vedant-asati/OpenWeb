// use anyhow::Result;
// use reqwest;
// use serde_json::Value; // Import serde_json for JSON parsing
// use std::io::Read;

// fn main() -> Result<()> {
//     let url = "https://api-sepolia.etherscan.io/api?module=account&action=balance&address=0x3c9F791975D50F32393910f905B569213c2742Ec&tag=latest&apikey=FATUAVKEXYM9PCPHTYE94PYZGY4NK4I978";

//     let mut res = reqwest::blocking::get(url)?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;

//     // Parse JSON response
//     let json: Value = serde_json::from_str(&body)?;

//     // Extract status, message, and result fields from JSON
//     let status = json["status"].as_str().unwrap_or("N/A");
//     let message = json["message"].as_str().unwrap_or("N/A");
//     let result = json["result"].as_str().unwrap_or("N/A");

//     println!("Status: {}", status);
//     println!("Message: {}", message);
//     println!("Result: {}", result);

//     Ok(())
// }

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
///
use google_oauth::AsyncClient;
use sha256::digest;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client_id = "343890256332-otlmb1slng5u0plp32sp2kt2j0tivh74.apps.googleusercontent.com";
    let access_token = "ya29.a0Ad52N3-PUxfH6qNwTxHmgtMhRUJufVsrO7PRPCD3n7FiRyWJOcnZEMisl7EuHzCJ5jwnIzfHf8mPsI5hOsm1VPyEL70KgXbPvydnALvM4JFbpMt_K48vYJqiBdxae8k40FvSYJQJa_y8oUftqkFuSj0vxPeMfKgFUQaCgYKAVASARMSFQHGX2MiNU6mnbx7KsAfPbeGv_34nA0169";
    // let id_token = "";

    let client = AsyncClient::new(client_id);
    // You can set the default timeout for fetching certificates from Google if needed
    // let client = AsyncClient::new(client_id).timeout(time::Duration::from_secs(30));

    // Validate the ID token
    let payload = match client.validate_access_token(access_token).await {
        Ok(payload) => payload,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    // When we get the payload, that means the ID token is valid.
    // Usually, we use `sub` as the identifier for our user...
    println!("Hello, I am {:?}", &payload);
    //payload.sub
    //payload.name
    //payload.email
    //payload.email_verified

    // Hashing a String JSR
    let gId = String::from(payload.sub);
    let gIdHash = digest(gId);
    println!("Hash: {}", &gIdHash);

    let mut gIdHash_ethAddress = HashMap::new();
    gIdHash_ethAddress.insert(
        gIdHash.to_string(),
        "0x337c787D769109Fc47686ccf816281Ad26e610B6".to_string(),
    );

    for (hash, string) in &gIdHash_ethAddress {
        println!("Hash: {}, String: {}", hash, string);
    }
}
