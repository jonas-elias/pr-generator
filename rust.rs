use anyhow::Context as _;
use clap::{Parser, Subcommand};
use tokio::io::{self, AsyncReadExt};

use zksync_config::{ContractsConfig, DBConfig, ETHClientConfig, ETHSenderConfig};
use zksync_dal::{connection::DbVariant, ConnectionPool};
use zksync_types::{L1BatchNumber, U256};

use zksync_core::block_reverter::{
    BlockReverter, BlockReverterEthConfig, BlockReverterFlags, L1ExecutedBatchesRevert,
};

#[derive(Debug, Parser)]
#[command(author = "Matter Labs", version, about = "Block revert utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Displays suggested values to use.
    #[command(name = "print-suggested-values")]
    Display {
        /// Displays the values as a JSON object, so that they are machine-readable.
        #[arg(long)]
        json: bool,
    },
    /// Sends revert transaction to L1.
    #[command(name = "send-eth-transaction")]
    SendEthTransaction {
        /// L1 batch number used to rollback to.
        #[arg(long)]
        l1_batch_number: u32,
        /// Priority fee used for rollback ethereum transaction.
        // We operate only by priority fee because we want to use base fee from ethereum
        // and send transaction as soon as possible without any resend logic
        #[arg(long)]
        priority_fee_per_gas: Option<u64>,
        /// Nonce used for rollback Ethereum transaction.
        #[arg(long)]
        nonce: u64,
    },

    /// Reverts internal database state to previous block.
    #[command(name = "rollback-db")]
    RollbackDB {
        /// L1 batch number used to rollback to.
        #[arg(long)]
        l1_batch_number: u32,
        /// Flag that specifies if Postgres DB should be rolled back.
        #[arg(long)]
        rollback_postgres: bool,
        /// Flag that specifies if RocksDB with tree should be rolled back.
        #[arg(long)]
        rollback_tree: bool,
        /// Flag that specifies if RocksDB with state keeper cache should be rolled back.
        #[arg(long)]
        rollback_sk_cache: bool,
        /// Flag that allows to revert already executed blocks, it's ultra dangerous and required only for fixing external nodes
        #[arg(long)]
        allow_executed_block_reversion: bool,
    },

    /// Clears failed L1 transactions.
    #[command(name = "clear-failed-transactions")]
    ClearFailedL1Transactions,
}