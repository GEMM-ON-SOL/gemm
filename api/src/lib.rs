pub mod consts;
pub mod error;
pub mod event;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use gemm_utils as utils;

use solana_program::declare_id;

declare_id!("GemmNd98XaPYbPtfaAiM32ynMgsqotF1qRJ9mWBt2VgK");
