use std::{fs, sync::Arc, time::Instant};

use parallel::{
    counters::SPECULATIVE_ABORT_COUNT,
    state::{StateView, StateViewId},
};
use types::{
    address::AccountAddress,
    rwset::{AccessPath, StateKey},
    transaction::{ExecutionStatus, Transaction, TransactionStatus},
};
use wasm::{
    recommended_concurrency,
    types::{Metadata, PreprocessedTransaction, WasmTransactionOutput, WasmtimeRuntime},
    ExecutionProfile, VM,
};
use wasmtime::Config;

#[derive(Clone, Copy)]
enum Mode {
    Compute,
    Independent,
    Hotspot,
}

impl Mode {
    fn parse(value: &str) -> anyhow::Result<Self> {
        match value {
            "independent" => Ok(Self::Independent),
            "hotspot" => Ok(Self::Hotspot),
            "compute" => Ok(Self::Compute),
            _ => anyhow::bail!("mode must be independent, hotspot, or compute"),
        }
    }

    fn function(self) -> &'static str {
        match self {
            Self::Independent => "xq_store",
            Self::Hotspot => "xq_increment",
            Self::Compute => "xq_compute",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Independent => "independent",
            Self::Hotspot => "hotspot",
            Self::Compute => "compute",
        }
    }

    fn age(self, index: usize) -> usize {
        match self {
            Self::Compute => 200_000,
            Self::Independent | Self::Hotspot => index,
        }
    }
}

struct BenchmarkState {
    counter_key: StateKey,
}

impl StateView for BenchmarkState {
    fn id(&self) -> StateViewId {
        StateViewId::Miscellaneous
    }

    fn get_state_value(&self, key: &StateKey) -> anyhow::Result<Option<Vec<u8>>> {
        Ok((key == &self.counter_key).then(|| b"0".to_vec()))
    }

    fn is_genesis(&self) -> bool {
        false
    }
}

fn contract_address() -> AccountAddress {
    AccountAddress::new([2; AccountAddress::LENGTH])
}

fn transactions(code: &Arc<[u8]>, count: usize, mode: Mode) -> Vec<PreprocessedTransaction> {
    let sender = AccountAddress::new([1; AccountAddress::LENGTH]);
    let contract = contract_address();

    (0..count)
        .map(|index| PreprocessedTransaction {
            txn: Transaction {
                sender,
                dest: contract,
                contract_name: "xq".to_string(),
                func_name: mode.function().to_string(),
                gas_limit: 100_000,
                gas: false,
                code: Arc::clone(code),
                param: serde_json::json!({
                    "name": format!("account-{index}"),
                    "age": mode.age(index),
                    "sex": "benchmark"
                })
                .to_string(),
                amount: 0,
            },
            owner: sender,
            self_address: contract,
            self_balance: 100,
            metadata: Metadata {
                block_time: 111,
                block_height: 222,
                tx_hash: format!("benchmark-{index}"),
            },
            gas: false,
            gas_limit: 100_000,
        })
        .collect()
}

fn assert_success(outputs: &[WasmTransactionOutput], expected: usize) {
    assert_eq!(outputs.len(), expected);
    assert!(outputs
        .iter()
        .all(|output| output.status() == TransactionStatus::Keep(ExecutionStatus::Success)));
}

fn assert_deterministic(sequential: &[WasmTransactionOutput], parallel: &[WasmTransactionOutput]) {
    for (sequential, parallel) in sequential.iter().zip(parallel) {
        assert_eq!(parallel.0.status, sequential.0.status);
        assert_eq!(parallel.0.data, sequential.0.data);
        assert_eq!(parallel.0.write_set, sequential.0.write_set);
        assert_eq!(parallel.0.events, sequential.0.events);
        assert_eq!(parallel.0.gas_used, sequential.0.gas_used);
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let transaction_count = args
        .next()
        .unwrap_or_else(|| "100".to_string())
        .parse::<usize>()?;
    let concurrency = args
        .next()
        .map(|value| value.parse::<usize>())
        .transpose()?
        .unwrap_or(4)
        .max(1);
    let mode = Mode::parse(&args.next().unwrap_or_else(|| "independent".to_string()))?;
    anyhow::ensure!(transaction_count > 0, "transaction count must be positive");

    let wasm_path = std::env::var_os("XWASM_CONTRACT_WASM")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            "../contract/target/wasm32-unknown-unknown/release/contract.wasm".into()
        });
    let wasm_bytes = fs::read(wasm_path)?;
    let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true))?;
    let aot_bytes: Arc<[u8]> = engine.precompile_module(&wasm_bytes)?.into();
    let state = BenchmarkState {
        counter_key: StateKey::AccessPath(AccessPath::new(contract_address(), b"counter".to_vec())),
    };

    let warmup =
        WasmtimeRuntime::sequential_execute_transactions(transactions(&aot_bytes, 1, mode), &state)
            .map_err(|code| anyhow::anyhow!("warmup failed with code {code}"))?;
    assert_success(&warmup, 1);

    let sequential_transactions = transactions(&aot_bytes, transaction_count, mode);
    let sequential_start = Instant::now();
    let sequential =
        WasmtimeRuntime::sequential_execute_transactions(sequential_transactions, &state)
            .map_err(|code| anyhow::anyhow!("sequential execution failed with code {code}"))?;
    let sequential_elapsed = sequential_start.elapsed();
    assert_success(&sequential, transaction_count);

    let parallel_transactions = transactions(&aot_bytes, transaction_count, mode);
    let aborts_before = SPECULATIVE_ABORT_COUNT.get();
    let parallel_start = Instant::now();
    let parallel =
        WasmtimeRuntime::parallel_execute_transactions(parallel_transactions, &state, concurrency)
            .map_err(|code| anyhow::anyhow!("parallel execution failed with code {code}"))?;
    let parallel_elapsed = parallel_start.elapsed();
    let speculative_aborts = SPECULATIVE_ABORT_COUNT.get() - aborts_before;
    assert_success(&parallel, transaction_count);
    assert_deterministic(&sequential, &parallel);

    let sequential_seconds = sequential_elapsed.as_secs_f64();
    let parallel_seconds = parallel_elapsed.as_secs_f64();
    let sequential_tps = transaction_count as f64 / sequential_seconds;
    let parallel_tps = transaction_count as f64 / parallel_seconds;
    let profile = ExecutionProfile {
        average_transaction_micros: (sequential_elapsed.as_micros() / transaction_count as u128)
            as u64,
        conflict_rate_bps: ((speculative_aborts.saturating_mul(10_000) / transaction_count as u64)
            .min(10_000)) as u16,
    };
    let recommended = recommended_concurrency(transaction_count, concurrency, profile);

    println!("mode={}", mode.name());
    println!("transactions={transaction_count}");
    println!("concurrency={concurrency}");
    println!("sequential_ms={:.3}", sequential_seconds * 1_000.0);
    println!("parallel_ms={:.3}", parallel_seconds * 1_000.0);
    println!("sequential_tps={sequential_tps:.2}");
    println!("parallel_tps={parallel_tps:.2}");
    println!("speedup={:.3}", sequential_seconds / parallel_seconds);
    println!("speculative_aborts={speculative_aborts}");
    println!(
        "average_transaction_micros={}",
        profile.average_transaction_micros
    );
    println!("conflict_rate_bps={}", profile.conflict_rate_bps);
    println!("recommended_concurrency={recommended}");

    Ok(())
}
