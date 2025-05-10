#[macro_export]
macro_rules! vote_weight_record {
    ($id:expr) => {
        #[derive(Clone)]
        pub struct VoterWeightRecord(pub spl_governance_addin_api::voter_weight::VoterWeightRecord);

        impl anchor_lang::prelude::AccountDeserialize for VoterWeightRecord {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                use anchor_spl::token_2022_extensions::spl_token_metadata_interface::borsh::BorshDeserialize;                
                let vwr = spl_governance_addin_api::voter_weight::VoterWeightRecord::deserialize(buf)
                    .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;
                Ok(VoterWeightRecord(vwr))
            }

            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                use anchor_spl::token_2022_extensions::spl_token_metadata_interface::borsh::BorshDeserialize;
                let vwr = spl_governance_addin_api::voter_weight::VoterWeightRecord::deserialize(buf)
                    .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;
                Ok(VoterWeightRecord(vwr))
            }
        }

        impl anchor_lang::prelude::AccountSerialize for VoterWeightRecord {
            fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
                use anchor_spl::token_2022_extensions::spl_token_metadata_interface::borsh::BorshSerialize;
                self.0.serialize(writer)
                    .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotSerialize)?;
                Ok(())
            }
        }

        impl anchor_lang::prelude::Owner for VoterWeightRecord {
            fn owner() -> anchor_lang::prelude::Pubkey {
                $id
            }
        }

        impl std::ops::Deref for VoterWeightRecord {
            type Target = spl_governance_addin_api::voter_weight::VoterWeightRecord;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for VoterWeightRecord {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        #[cfg(feature = "idl-build")]
        impl anchor_lang::prelude::IdlBuild for VoterWeightRecord {}

        #[cfg(feature = "idl-build")]
        impl anchor_lang::prelude::Discriminator for VoterWeightRecord {
            const DISCRIMINATOR: &'static [u8] = &spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR;
        }

        impl VoterWeightRecord {
            pub fn discriminator() -> [u8; 8] {
                spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR
            }
        }
    };
}
