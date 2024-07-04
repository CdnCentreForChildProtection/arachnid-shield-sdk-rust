# Arachnid Shield SDK

An SDK for consuming the [Arachnid Shield API](https://shield.projectarachnid.com/).

## Usage

First, obtain login credentials by contacting [Project Arachnid](https://projectarachnid.ca/en/contact). 

### Sync client

You may use the `ArachnidShield` client that has all the methods needed to consume the Arachnid Shield API.

```rust
use arachnid_shield::{ArachnidShield, ApiUser};
use mime::IMAGE_JPEG;

fn get_media() -> Vec<u8> {
    return vec![]
}

let client = ArachnidShield::new(
    ApiUser::new("<username>", "<password>")
);

// Suppose you have media contents and mime_type already available.
let contents = get_media();

// Request Arachnid Shield to scan the media.
let response = client.scan_media_from_bytes(contents, IMAGE_JPEG);

// Might want to handle errors in practice, but we'll
// just .unwrap() it here in this example.
let scanned_media = response.unwrap();

if scanned_media.matches_known_media() {
    eprintln!("Uh-oh, this media: {:#?} matches known media so it is harmful.", scanned_media);
}
```

### Async client

You may use the `AsyncArachnidShield` client that has exactly the same interface as the sync client but with all 
the methods being awaitable.

```rust
use arachnid_shield::{AsyncArachnidShield as ArachnidShield, ApiUser};
use mime::IMAGE_JPEG;

fn get_media() -> Vec<u8> {
    return vec![]
}

let client = ArachnidShield::new(
    ApiUser::new("<username>", "<password>")
);

#[tokio::main]  // Just an example. Could use any runtime here.
async fn main() {
    // Suppose you have media contents and mime_type already available.
    let contents = get_media();

    // Request Arachnid Shield to scan the media.
    let response = client.scan_media_from_bytes(contents, IMAGE_JPEG).await;

    // Might want to handle errors in practice, but we'll
    // just .unwrap() it here in this example.
    let scanned_media = response.unwrap();

    if scanned_media.matches_known_media() {
        eprintln!("Uh-oh, this media: {:#?} matches known media so it is harmful.", scanned_media);
    }
}

```
