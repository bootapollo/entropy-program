mod close;
mod next;
mod open;
mod reveal;
mod sample;

use close::*;
use next::*;
use open::*;
use reveal::*;
use sample::*;

use entropy_api::prelude::*;
use steel::*;
use solana_security_txt::security_txt;


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&entropy_api::ID, program_id, data)?;

    match ix {
        EntropyInstruction::Open => process_open(accounts, data)?,
        EntropyInstruction::Close => process_close(accounts, data)?,
        EntropyInstruction::Next => process_next(accounts, data)?,
        EntropyInstruction::Reveal => process_reveal(accounts, data)?,
        EntropyInstruction::Sample => process_sample(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);


security_txt! { 
    name: "Entropy",
    project_url: "https://entropy.godl.dev",
    contacts: "email:bootapollo@pm.me,telegram:bootapollo",
    policy: "https://github.com/zenx-labs/entropy-program/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/zenx-labs/entropy-program"
}
