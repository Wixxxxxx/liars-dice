use anchor_client::solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Keypair,
    signature::Signer,
    transaction::Transaction,
};
use anchor_lang::{pubkey, InstructionData, ToAccountMetas};
use liars_dice::accounts::Join as JoinAccounts;
use liars_dice::instruction::Join as JoinInstruction;
use litesvm::LiteSVM;

#[tokio::test]
pub async fn test_basic_program() {
    // 1. start up new SVM instance (local validator)
    let mut svm = LiteSVM::new();

    // 2. generate host keypair to create game account
    let host_kp = Keypair::new();
    let host_pk = host_kp.pubkey();

    // 3. generate player keypair to sign transaction and join the game
    let player_kp = Keypair::new();
    let player_pk = player_kp.pubkey();

    // 3. create test game account PDA
    let program_id = pubkey!("AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B");
    let (game_pda, _) =
        Pubkey::find_program_address(&[b"liarsdicesession", host_pk.as_ref()], &program_id);

    svm.set_account(
        game_pda,
        Account {
            lamports: 1_000_000_000,
            owner: program_id,
            data: vec![],
            executable: false,
            rent_epoch: 0,
        },
    );

    // 3. read in program binary and airdop sol to payer
    let program_id = pubkey!("AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B");
    let bytes = include_bytes!("../../target/deploy/liars_dice.so");
    svm.add_program(program_id, bytes);
    svm.airdrop(&host_pk, 1_000_000_000).unwrap();

    // 4. create instruction
    let accounts = JoinAccounts {}.to_account_metas(Some(false));
    let data = JoinInstruction.data();
    let ix = Instruction {
        program_id,
        accounts,
        data,
    };

    // 5. create message and send transaction
    let msg = Message::new_with_blockhash(&[ix], Some(&host_pk), &svm.latest_blockhash());
    let tx = Transaction::new(&[host_kp], msg, svm.latest_blockhash());
    let meta = svm.send_transaction(tx).unwrap();
    assert_eq!(
        meta.logs[2],
        "Program log: Greetings from: AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B"
    )
}

// #[tokio::test]
// pub async fn test_basic_program() {
//     // 1. start up new SVM instance (local validator)
//     let mut svm = LiteSVM::new();

//     // 2. generate payer keypair to sign
//     let host_kp = Keypair::new();
//     let host_pk = host_kp.pubkey();

//     // 3. read in program binary and airdop sol to payer
//     let program_id = pubkey!("AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B");
//     let bytes = include_bytes!("../../../../target/deploy/liars_dice.so");
//     svm.add_program(program_id, bytes);
//     svm.airdrop(&host_pk, 1_000_000_000).unwrap();

//     // 4. create instruction
//     let accounts = InitializeAccounts {}.to_account_metas(Some(false));
//     let data = InitializeInstruction.data();
//     let ix = Instruction {
//         program_id,
//         accounts,
//         data,
//     };

//     // 5. create message and send transaction
//     let msg = Message::new_with_blockhash(&[ix], Some(&host_pk), &svm.latest_blockhash());
//     let tx = Transaction::new(&[host_kp], msg, svm.latest_blockhash());
//     let meta = svm.send_transaction(tx).unwrap();
//     assert_eq!(
//         meta.logs[2],
//         "Program log: Greetings from: AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B"
//     )
// }
