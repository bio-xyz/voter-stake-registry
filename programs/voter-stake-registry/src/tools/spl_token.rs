use {
    crate::error::VsrError,
    anchor_lang::prelude::*,
    anchor_spl::token_2022::spl_token_2022::{
        self,
        extension::{
            transfer_fee::TransferFeeConfig, transfer_hook, BaseStateWithExtensions,
            PodStateWithExtensions, StateWithExtensions,
        },
        pod::PodMint,
        state::Mint,
    },
    anchor_lang::solana_program::program::{invoke_signed, invoke},
    spl_transfer_hook_interface::onchain::add_extra_accounts_for_execute_cpi,
};

/// Transfers SPL Tokens
/// works for both spl-token and spl-token-2022 
pub fn transfer_checked_spl_tokens<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    additional_accounts: &[AccountInfo<'a>],
    mint_decimals: u8,
) -> Result<()> {
    let spl_token_program_id = spl_token_info.key;

    let mut transfer_instruction = spl_token_2022::instruction::transfer_checked(
        spl_token_program_id,
        source_info.key,
        mint_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
        mint_decimals
    )
    .unwrap();

    let mut cpi_account_infos = vec![
        source_info.clone(),
        mint_info.clone(),
        destination_info.clone(),
        authority_info.clone(),
    ];

    // if it's a signer, it might be a multisig signer, throw it in!
    additional_accounts
        .iter()
        .filter(|ai| ai.is_signer)
        .for_each(|ai| {
            cpi_account_infos.push(ai.clone());
            transfer_instruction
                .accounts
                .push(AccountMeta::new_readonly(*ai.key, ai.is_signer));
        });
    // used for transfer_hooks
    // scope the borrowing to avoid a double-borrow during CPI
    {
        let mint_data = mint_info.try_borrow_data()?;
        let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
        if let Some(program_id) = transfer_hook::get_program_id(&mint) {
            add_extra_accounts_for_execute_cpi(
                &mut transfer_instruction,
                &mut cpi_account_infos,
                &program_id,
                source_info.clone(),
                mint_info.clone(),
                destination_info.clone(),
                authority_info.clone(),
                amount,
                additional_accounts,
            )?;
        }
    }

    invoke(&transfer_instruction, &cpi_account_infos)?;

    Ok(())
}


/// Transfers SPL Tokens checked from a token account with seeds
pub fn transfer_spl_tokens_signed<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    authority_seeds: &[&[u8]],
    amount: u64,
    spl_token_program_id: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    additional_accounts: &[AccountInfo<'a>],
    mint_decimals: u8,
) -> Result<()> {
    let mut transfer_instruction = spl_token_2022::instruction::transfer_checked(
        spl_token_program_id.key,
        source_info.key,
        mint_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
        mint_decimals,
    )
    .unwrap();

    let mut cpi_account_infos = vec![
        source_info.clone(),
        mint_info.clone(),
        destination_info.clone(),
        authority_info.clone(),
    ];

    // if it's a signer, it might be a multisig signer, throw it in!
    additional_accounts
        .iter()
        .filter(|ai| ai.is_signer)
        .for_each(|ai| {
            cpi_account_infos.push(ai.clone());
            transfer_instruction
                .accounts
                .push(AccountMeta::new_readonly(*ai.key, ai.is_signer));
        });

    // used for transfer_hooks
    // scope the borrowing to avoid a double-borrow during CPI
    {
        let mint_data = mint_info.try_borrow_data()?;
        let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
        if let Some(program_id) = transfer_hook::get_program_id(&mint) {
            add_extra_accounts_for_execute_cpi(
                &mut transfer_instruction,
                &mut cpi_account_infos,
                &program_id,
                source_info.clone(),
                mint_info.clone(),
                destination_info.clone(),
                authority_info.clone(),
                amount,
                additional_accounts,
            )?;
        }
    }

    invoke_signed(
        &transfer_instruction,
        &cpi_account_infos,
        &[&authority_seeds[..]],
    )?;
    Ok(())
}

/// Get current TransferFee, returns 0 if no TransferFeeConfig exist.
pub fn get_current_mint_fee(mint_info: &AccountInfo, amount: u64) -> Result<u64> {
    let mint_data = mint_info.try_borrow_data()?;
    let mint = PodStateWithExtensions::<PodMint>::unpack(&mint_data)?;

    if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        Ok(transfer_fee_config
            .calculate_epoch_fee(Clock::get()?.epoch, amount)
            .ok_or(VsrError::MathematicalOverflow)?)
    } else {
        Ok(0)
    }
}
