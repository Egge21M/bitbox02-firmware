// Copyright 2023 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::pb;
use super::Error;

use pb::response::Response;

use crate::workflow::confirm;

use bitbox02::keystore;

use alloc::vec::Vec;

/// Processes a BIP-85 API call.
pub async fn process(request: &pb::Bip85Request) -> Result<Response, Error> {
    match &request.app {
        None => Err(Error::InvalidInput),
        #[cfg(not(feature = "app-bip85-bip39"))]
        Some(pb::bip85_request::App::Bip39(())) => Err(Error::Disabled),
        #[cfg(feature = "app-bip85-bip39")]
        Some(pb::bip85_request::App::Bip39(())) => Ok(Response::Bip85(pb::Bip85Response {
            app: Some(pb::bip85_response::App::Bip39(process_bip39().await?)),
        })),
        Some(pb::bip85_request::App::Ln(request)) => Ok(Response::Bip85(pb::Bip85Response {
            app: Some(pb::bip85_response::App::Ln(process_ln(request).await?)),
        })),
    }
}

/// Derives and displays a BIP-39 seed according to BIP-85:
/// https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39.
#[cfg(feature = "app-bip85-bip39")]
async fn process_bip39() -> Result<(), Error> {
    use crate::workflow::trinary_choice::{choose, TrinaryChoice};
    use crate::workflow::{menu, mnemonic, status, trinary_input_string};

    confirm::confirm(&confirm::Params {
        title: "BIP-85",
        body: "Derive BIP-39\nmnemonic?",
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    confirm::confirm(&confirm::Params {
        title: "BIP-85",
        body: "This is an advanced feature. Proceed only if you know what you are doing.",
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    let num_words: u32 = match choose("How many words?", "12", "18", "24").await {
        TrinaryChoice::TRINARY_CHOICE_LEFT => 12,
        TrinaryChoice::TRINARY_CHOICE_MIDDLE => 18,
        TrinaryChoice::TRINARY_CHOICE_RIGHT => 24,
    };

    status::status(&format!("{} words", num_words), true).await;

    // Pick index. The first few are quick-access. "More" leads to a full number input keyboard.
    let index: u32 =
        match menu::pick(&["0", "1", "2", "3", "4", "More"], Some("Select index")).await? {
            i @ 0..=4 => i.into(),
            5 => {
                let number_string = trinary_input_string::enter(
                    &trinary_input_string::Params {
                        title: "Enter index",
                        number_input: true,
                        longtouch: true,
                        ..Default::default()
                    },
                    trinary_input_string::CanCancel::Yes,
                    "",
                )
                .await?;
                match number_string.as_str().parse::<u32>() {
                    Ok(i) if i < util::bip32::HARDENED => i,
                    _ => {
                        status::status("Invalid index", false).await;
                        return Err(Error::InvalidInput);
                    }
                }
            }
            6.. => panic!("bip85 error"),
        };

    status::status(&format!("Index: {}", index), true).await;

    confirm::confirm(&confirm::Params {
        title: "Keypath",
        body: &format!("m/83696968'/39'/0'/{}'/{}'", num_words, index),
        scrollable: true,
        longtouch: true,
        ..Default::default()
    })
    .await?;

    confirm::confirm(&confirm::Params {
        title: "",
        body: &format!("{} word mnemonic\nfollows", num_words),
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    let mnemonic = keystore::bip85_bip39(num_words, index)?;
    let words: Vec<&str> = mnemonic.split(' ').collect();
    mnemonic::show_and_confirm_mnemonic(&words).await?;

    status::status("Finished", true).await;

    Ok(())
}

/// Derives and displays a LN seed according to BIP-85.
/// It is the same as BIP-85 with app number 39', but instead using app number 19534' (= 0x4c4e = 'LN'),
/// and restricted to 12 word mnemonics.
/// https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39
async fn process_ln(
    &pb::bip85_request::AppLn { account_number }: &pb::bip85_request::AppLn,
) -> Result<Vec<u8>, Error> {
    // We allow only one LN account until we see a reason to have more.
    if account_number != 0 {
        return Err(Error::InvalidInput);
    }
    confirm::confirm(&confirm::Params {
        title: "",
        body: "Create\nLightning wallet\non host device?",
        longtouch: true,
        ..Default::default()
    })
    .await?;

    keystore::bip85_ln(account_number).map_err(|_| Error::Generic)
}
