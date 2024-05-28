
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{ 
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey, system_instruction::{self, SystemInstruction}, system_program
    };
use crate::{error::RNGProgramError::InvalidInstruction, instruction::RNGProgramInstruction, state::Toplama, };




pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction: RNGProgramInstruction = RNGProgramInstruction::unpack(instruction_data)?;

    match instruction {
      RNGProgramInstruction::CallProgram => {
        Self::call_program(accounts,program_id)
      }
      RNGProgramInstruction::WriteData {data}=> {
        Self::write_data(accounts,program_id,data)
      }

    }
  }


  pub fn  call_program(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let new_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let payer: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  let ix = system_instruction::create_account(payer.key, new_account.key, 10000000, 10, program_id);

  invoke(&ix, &[payer.clone(),new_account.clone()])?;


    Ok(())
  }

  pub fn  write_data(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    data:Toplama
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let payer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let new_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;


    let mut new_account_data: Toplama = Toplama::try_from_slice(&new_account.data.borrow())?;

    new_account_data.sayi += data.sayi;
    new_account_data.random_number += data.random_number;


    if new_account_data.sayi % 2 == 0 {

      **new_account.try_borrow_mut_lamports()? -= 1000000;
      **payer.try_borrow_mut_lamports()? += 1000000;

    }



    new_account_data.serialize(&mut &mut new_account.data.borrow_mut()[..])?;

    Ok(())
  }



}