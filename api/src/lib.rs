pub mod consts;
pub mod error;
pub mod event;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use gemm_utils as utils;

use solana_program::declare_id;

declare_id!("9x1a9dKgfA3kKDLwgJEhQPQK6h2AMvVjH4pEtbnYaESz");
