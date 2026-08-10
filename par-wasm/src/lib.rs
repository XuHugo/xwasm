mod hostfunc;
pub mod types;
pub mod vm;
use parallel::state::StateView;
use types::{PreprocessedTransaction, WasmTransactionOutput};
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExecutionProfile {
    pub average_transaction_micros: u64,
    pub conflict_rate_bps: u16,
}

pub fn recommended_concurrency(
    transaction_count: usize,
    max_concurrency: usize,
    profile: ExecutionProfile,
) -> usize {
    // The current MVP is tuned and validated for at most four workers. Keeping
    // the cap here prevents callers from accidentally selecting an unproven
    // higher parallelism level on machines with many logical CPUs.
    let max_concurrency = max_concurrency.clamp(1, 4);
    if transaction_count < 32 || profile.average_transaction_micros < 250 {
        return 1;
    }

    if profile.conflict_rate_bps >= 2_000 {
        return max_concurrency.min(2);
    }
    if profile.conflict_rate_bps >= 500 {
        return max_concurrency.min(4);
    }

    max_concurrency
}

pub trait VM {
    fn parallel_execute_transactions<S: StateView>(
        transactions: Vec<PreprocessedTransaction>,
        state_view: &S,
        concurrency_level: usize,
    ) -> Result<Vec<WasmTransactionOutput>, i64>;

    fn adaptive_execute_transactions<S: StateView>(
        transactions: Vec<PreprocessedTransaction>,
        state_view: &S,
        max_concurrency: usize,
        profile: ExecutionProfile,
    ) -> Result<Vec<WasmTransactionOutput>, i64> {
        let concurrency = recommended_concurrency(transactions.len(), max_concurrency, profile);
        Self::parallel_execute_transactions(transactions, state_view, concurrency)
    }
    fn sequential_execute_transactions<S: StateView>(
        transactions: Vec<PreprocessedTransaction>,
        state_view: &S,
    ) -> Result<Vec<WasmTransactionOutput>, i64>;

    fn commit_block(
        state: &mut statedb::State,
        outputs: &[WasmTransactionOutput],
    ) -> anyhow::Result<()>;
}
#[cfg(test)]
mod tests {
    use super::{recommended_concurrency, ExecutionProfile};

    #[test]
    fn recommends_sequential_for_small_or_cheap_blocks() {
        assert_eq!(
            recommended_concurrency(
                500,
                8,
                ExecutionProfile {
                    average_transaction_micros: 75,
                    conflict_rate_bps: 0,
                },
            ),
            1
        );
        assert_eq!(
            recommended_concurrency(
                16,
                8,
                ExecutionProfile {
                    average_transaction_micros: 1_000,
                    conflict_rate_bps: 0,
                },
            ),
            1
        );
    }

    #[test]
    fn reduces_concurrency_as_conflicts_rise() {
        let heavy = |conflict_rate_bps| ExecutionProfile {
            average_transaction_micros: 600,
            conflict_rate_bps,
        };

        assert_eq!(recommended_concurrency(500, 8, heavy(0)), 4);
        assert_eq!(recommended_concurrency(500, 8, heavy(500)), 4);
        assert_eq!(recommended_concurrency(500, 8, heavy(2_000)), 2);
    }
}
