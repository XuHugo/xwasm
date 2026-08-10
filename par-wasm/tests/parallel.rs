use std::{
    fs,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use parallel::{
    counters::SPECULATIVE_ABORT_COUNT,
    state::{StateView, StateViewId},
};
use statedb::State;
use types::{
    address::AccountAddress,
    rwset::{AccessPath, StateKey, WriteOp, WriteSet},
    transaction::{
        ExecutionStatus, Transaction, TransactionOutput as RawTransactionOutput, TransactionStatus,
    },
};
use wasm::{
    types::{Metadata, PreprocessedTransaction, WasmTransactionOutput, WasmtimeRuntime},
    VM,
};
use wasmtime::Config;

struct EmptyStateView;

impl StateView for EmptyStateView {
    fn id(&self) -> StateViewId {
        StateViewId::Miscellaneous
    }

    fn get_state_value(&self, _state_key: &StateKey) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn is_genesis(&self) -> bool {
        false
    }
}

fn transaction(code: &Arc<[u8]>, name: &str, age: u64) -> PreprocessedTransaction {
    let sender = AccountAddress::new([1; AccountAddress::LENGTH]);
    let contract = AccountAddress::new([2; AccountAddress::LENGTH]);

    PreprocessedTransaction {
        txn: Transaction {
            sender,
            dest: contract,
            contract_name: "xq".to_string(),
            func_name: "xq_abc".to_string(),
            gas_limit: 10_000,
            gas: false,
            code: Arc::clone(code),
            param: serde_json::json!({
                "name": name,
                "age": age,
                "sex": "man"
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
            tx_hash: format!("tx-{name}"),
        },
        gas: false,
        gas_limit: 10_000,
    }
}

#[test]
fn executes_a_wasm_block_in_parallel_and_preserves_order() {
    if std::thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .unwrap_or(1)
        < 2
    {
        return;
    }

    let wasm_bytes = fs::read("./tests/wasmfile/contract.wasm").unwrap();
    let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true)).unwrap();
    let aot_bytes: Arc<[u8]> = engine.precompile_module(&wasm_bytes).unwrap().into();

    let outputs = WasmtimeRuntime::parallel_execute_transactions(
        vec![
            transaction(&aot_bytes, "alice", 18),
            transaction(&aot_bytes, "bob", 20),
        ],
        &EmptyStateView,
        2,
    )
    .unwrap();

    assert_eq!(outputs.len(), 2);
    for output in &outputs {
        assert_eq!(
            output.status(),
            TransactionStatus::Keep(ExecutionStatus::Success)
        );
    }

    let first: serde_json::Value = serde_json::from_str(&outputs[0].0.data).unwrap();
    let second: serde_json::Value = serde_json::from_str(&outputs[1].0.data).unwrap();

    assert_eq!(first["name"], "alice");
    assert_eq!(first["age"], 18);
    assert_eq!(second["name"], "bob");
    assert_eq!(second["age"], 20);
}
const STORAGE_WAT: &str = r#"
(module
  (import "xq" "get_store" (func $get_store (param i32 i32 i32) (result i32)))
  (import "xq" "set_store" (func $set_store (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "key")

  (func (export "increment") (param i64) (result i32)
    (drop (call $get_store (i32.const 0) (i32.const 3) (i32.const 32)))
    (i32.store8
      (i32.const 16)
      (i32.add (i32.load8_u (i32.const 32)) (i32.const 1)))
    (drop (call $set_store
      (i32.const 0) (i32.const 3) (i32.const 16) (i32.const 1)))
    (i32.const 1))

  (func (export "write_then_read") (param i64) (result i32)
    (i32.store8 (i32.const 16) (i32.const 7))
    (drop (call $set_store
      (i32.const 0) (i32.const 3) (i32.const 16) (i32.const 1)))
    (drop (call $get_store (i32.const 0) (i32.const 3) (i32.const 32)))
    (i32.eq (i32.load8_u (i32.const 32)) (i32.const 7))))
"#;

const MULTI_KEY_WAT: &str = r#"
(module
  (import "xq" "get_store" (func $get_store (param i32 i32 i32) (result i32)))
  (import "xq" "set_store" (func $set_store (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "a")
  (data (i32.const 1) "b")
  (data (i32.const 2) "c")

  (func $read (param $key i32) (result i32)
    (drop (call $get_store (local.get $key) (i32.const 1) (i32.const 32)))
    (i32.load8_u (i32.const 32)))

  (func $write (param $key i32) (param $value i32)
    (i32.store8 (i32.const 16) (local.get $value))
    (drop (call $set_store
      (local.get $key) (i32.const 1) (i32.const 16) (i32.const 1))))

  (func (export "slow_write_a") (param i64) (result i32)
    (local $i i32)
    (block $done
      (loop $spin
        (br_if $done (i32.ge_u (local.get $i) (i32.const 2000)))
        (drop (call $get_store
          (i32.const 0) (i32.const 1) (i32.const 32)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $spin)))
    (call $write (i32.const 0) (i32.const 1))
    (i32.const 1))

  (func (export "a_to_b") (param i64) (result i32)
    (call $write
      (i32.const 1) (i32.add (call $read (i32.const 0)) (i32.const 1)))
    (i32.const 1))

  (func (export "b_to_c") (param i64) (result i32)
    (call $write
      (i32.const 2) (i32.add (call $read (i32.const 1)) (i32.const 1)))
    (i32.const 1))

  (func (export "b_to_a") (param i64) (result i32)
    (call $write
      (i32.const 0) (i32.add (call $read (i32.const 1)) (i32.const 1)))
    (i32.const 1))

  (func (export "read_ab_write_c") (param i64) (result i32)
    (call $write
      (i32.const 2)
      (i32.add (call $read (i32.const 0)) (call $read (i32.const 1))))
    (i32.const 1))

  (func (export "read_bc_write_a") (param i64) (result i32)
    (call $write
      (i32.const 0)
      (i32.add (call $read (i32.const 1)) (call $read (i32.const 2))))
    (i32.const 1))

  (func (export "conditional_route") (param i64) (result i32)
    (if (i32.eqz (call $read (i32.const 0)))
      (then (call $write (i32.const 1) (i32.const 7)))
      (else (call $write (i32.const 2) (i32.const 9))))
    (i32.const 1))

  (func (export "c_to_b") (param i64) (result i32)
    (call $write
      (i32.const 1) (i32.add (call $read (i32.const 2)) (i32.const 1)))
    (i32.const 1)))
"#;

struct MultiKeyStateView {
    values: Vec<(StateKey, Vec<u8>)>,
}

impl StateView for MultiKeyStateView {
    fn get_state_value(&self, state_key: &StateKey) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(self
            .values
            .iter()
            .find(|(key, _)| key == state_key)
            .map(|(_, value)| value.clone()))
    }

    fn is_genesis(&self) -> bool {
        false
    }
}
struct HotStateView {
    key: StateKey,
    value: Vec<u8>,
}

impl StateView for HotStateView {
    fn get_state_value(&self, state_key: &StateKey) -> anyhow::Result<Option<Vec<u8>>> {
        Ok((state_key == &self.key).then(|| self.value.clone()))
    }

    fn is_genesis(&self) -> bool {
        false
    }
}

fn contract_address() -> AccountAddress {
    AccountAddress::new([2; AccountAddress::LENGTH])
}

fn storage_key() -> StateKey {
    StateKey::AccessPath(AccessPath::new(contract_address(), b"key".to_vec()))
}

fn storage_transaction(code: &Arc<[u8]>, func_name: &str, index: usize) -> PreprocessedTransaction {
    let sender = AccountAddress::new([1; AccountAddress::LENGTH]);

    PreprocessedTransaction {
        txn: Transaction {
            sender,
            dest: contract_address(),
            contract_name: "storage".to_string(),
            func_name: func_name.to_string(),
            gas_limit: 10_000,
            gas: false,
            code: Arc::clone(code),
            param: String::new(),
            amount: 0,
        },
        owner: sender,
        self_address: contract_address(),
        self_balance: 100,
        metadata: Metadata {
            block_time: 111,
            block_height: 222,
            tx_hash: format!("storage-tx-{index}"),
        },
        gas: false,
        gas_limit: 10_000,
    }
}

fn storage_module() -> Arc<[u8]> {
    let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true)).unwrap();
    let module = wasmtime::Module::new(&engine, STORAGE_WAT).unwrap();
    module.serialize().unwrap().into()
}

fn written_byte(output: &wasm::types::WasmTransactionOutput, key: &StateKey) -> u8 {
    let write_set = output.write_set();
    match write_set.get(key) {
        Some(WriteOp::Creation(value)) | Some(WriteOp::Modification(value)) => value[0],
        Some(WriteOp::Deletion) | None => panic!("expected a value write"),
    }
}
fn named_storage_key(name: &[u8]) -> StateKey {
    StateKey::AccessPath(AccessPath::new(contract_address(), name.to_vec()))
}

fn multi_key_module() -> Arc<[u8]> {
    let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true)).unwrap();
    let module = wasmtime::Module::new(&engine, MULTI_KEY_WAT).unwrap();
    module.serialize().unwrap().into()
}

fn execute_multi_key_block(functions: &[&str]) -> (Vec<WasmTransactionOutput>, u64) {
    let code = multi_key_module();
    let state_view = MultiKeyStateView {
        values: [b"a".as_slice(), b"b".as_slice(), b"c".as_slice()]
            .iter()
            .copied()
            .map(|name| (named_storage_key(name), vec![0]))
            .collect(),
    };
    let transactions = |code: &Arc<[u8]>| {
        functions
            .iter()
            .enumerate()
            .map(|(index, function)| storage_transaction(code, function, index))
            .collect()
    };

    let sequential =
        WasmtimeRuntime::sequential_execute_transactions(transactions(&code), &state_view).unwrap();
    let aborts_before = SPECULATIVE_ABORT_COUNT.get();
    let parallel =
        WasmtimeRuntime::parallel_execute_transactions(transactions(&code), &state_view, 4)
            .unwrap();
    let speculative_aborts = SPECULATIVE_ABORT_COUNT.get() - aborts_before;

    assert_eq!(parallel.len(), sequential.len());
    for (parallel_output, sequential_output) in parallel.iter().zip(&sequential) {
        assert_eq!(parallel_output.0.status, sequential_output.0.status);
        assert_eq!(parallel_output.0.data, sequential_output.0.data);
        assert_eq!(parallel_output.0.write_set, sequential_output.0.write_set);
        assert_eq!(parallel_output.0.events, sequential_output.0.events);
        assert_eq!(parallel_output.0.gas_used, sequential_output.0.gas_used);
    }

    (parallel, speculative_aborts)
}

#[test]
fn resolves_cascading_dependencies_across_three_keys() {
    let (outputs, _) = execute_multi_key_block(&["slow_write_a", "a_to_b", "b_to_c"]);
    let a = named_storage_key(b"a");
    let b = named_storage_key(b"b");
    let c = named_storage_key(b"c");

    assert_eq!(written_byte(&outputs[0], &a), 1);
    assert_eq!(written_byte(&outputs[1], &b), 2);
    assert_eq!(written_byte(&outputs[2], &c), 3);
}

#[test]
fn resolves_cross_key_and_overlapping_read_write_sets() {
    let (outputs, _) =
        execute_multi_key_block(&["b_to_a", "a_to_b", "read_ab_write_c", "read_bc_write_a"]);
    let a = named_storage_key(b"a");
    let b = named_storage_key(b"b");
    let c = named_storage_key(b"c");

    assert_eq!(written_byte(&outputs[0], &a), 1);
    assert_eq!(written_byte(&outputs[1], &b), 2);
    assert_eq!(written_byte(&outputs[2], &c), 3);
    assert_eq!(written_byte(&outputs[3], &a), 5);
}

#[test]
fn removes_stale_writes_when_reexecution_changes_the_write_set() {
    let (outputs, speculative_aborts) =
        execute_multi_key_block(&["slow_write_a", "conditional_route", "c_to_b"]);
    assert!(
        speculative_aborts > 0,
        "the delayed first transaction should force speculative re-execution"
    );
    let b = named_storage_key(b"b");
    let c = named_storage_key(b"c");

    assert!(outputs[1].write_set().get(&b).is_none());
    assert_eq!(written_byte(&outputs[1], &c), 9);
    assert_eq!(written_byte(&outputs[2], &b), 10);
}

#[test]
fn reads_its_own_writes_before_shared_state() {
    let code = storage_module();
    let state_view = HotStateView {
        key: storage_key(),
        value: vec![0],
    };

    let outputs = WasmtimeRuntime::parallel_execute_transactions(
        vec![
            storage_transaction(&code, "write_then_read", 0),
            storage_transaction(&code, "write_then_read", 1),
        ],
        &state_view,
        2,
    )
    .unwrap();

    assert!(outputs
        .iter()
        .all(|output| { output.status() == TransactionStatus::Keep(ExecutionStatus::Success) }));
}

#[test]
fn reexecutes_a_hot_key_against_the_latest_prior_write() {
    if std::thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .unwrap_or(1)
        < 2
    {
        return;
    }

    let code = storage_module();
    let key = storage_key();
    let state_view = HotStateView {
        key: key.clone(),
        value: vec![0],
    };

    let outputs = WasmtimeRuntime::parallel_execute_transactions(
        vec![
            storage_transaction(&code, "increment", 0),
            storage_transaction(&code, "increment", 1),
        ],
        &state_view,
        2,
    )
    .unwrap();

    assert_eq!(written_byte(&outputs[0], &key), 1);
    assert_eq!(written_byte(&outputs[1], &key), 2);
}

#[test]
fn parallel_hot_key_block_matches_sequential_execution() {
    if std::thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .unwrap_or(1)
        < 2
    {
        return;
    }

    let code = storage_module();
    let key = storage_key();
    let state_view = HotStateView {
        key,
        value: vec![0],
    };

    let sequential = WasmtimeRuntime::sequential_execute_transactions(
        vec![
            storage_transaction(&code, "increment", 0),
            storage_transaction(&code, "increment", 1),
        ],
        &state_view,
    )
    .unwrap();

    let parallel = WasmtimeRuntime::parallel_execute_transactions(
        vec![
            storage_transaction(&code, "increment", 0),
            storage_transaction(&code, "increment", 1),
        ],
        &state_view,
        2,
    )
    .unwrap();

    assert_eq!(parallel.len(), sequential.len());
    for (parallel_output, sequential_output) in parallel.iter().zip(&sequential) {
        assert_eq!(parallel_output.0.status, sequential_output.0.status);
        assert_eq!(parallel_output.0.data, sequential_output.0.data);
        assert_eq!(parallel_output.0.write_set, sequential_output.0.write_set);
        assert_eq!(parallel_output.0.events, sequential_output.0.events);
        assert_eq!(parallel_output.0.gas_used, sequential_output.0.gas_used);
    }
}
fn output_with_write(
    key: StateKey,
    operation: WriteOp,
    status: TransactionStatus,
) -> WasmTransactionOutput {
    let mut write_set = WriteSet::default();
    write_set.insert((key, operation));
    WasmTransactionOutput(RawTransactionOutput {
        data: String::new(),
        write_set,
        events: Vec::new(),
        gas_used: 0,
        status,
    })
}

#[test]
fn commits_successful_outputs_atomically_with_last_write_wins() {
    let mut state = State::new_mem().unwrap();
    let first_key = storage_key();
    let second_key = StateKey::Raw(b"second".to_vec());

    let outputs = vec![
        output_with_write(
            first_key.clone(),
            WriteOp::Creation(vec![1]),
            TransactionStatus::Keep(ExecutionStatus::Success),
        ),
        output_with_write(
            second_key.clone(),
            WriteOp::Creation(vec![9]),
            TransactionStatus::Keep(ExecutionStatus::Success),
        ),
        output_with_write(
            first_key.clone(),
            WriteOp::Modification(vec![2]),
            TransactionStatus::Keep(ExecutionStatus::Success),
        ),
        output_with_write(
            second_key.clone(),
            WriteOp::Modification(vec![7]),
            TransactionStatus::Retry,
        ),
    ];

    WasmtimeRuntime::commit_block(&mut state, &outputs).unwrap();

    let first_encoded = first_key.to_bytes().unwrap();
    let second_encoded = second_key.to_bytes().unwrap();
    assert_eq!(state.get_optional(&first_encoded), Some(vec![2]));
    assert_eq!(state.get_optional(&second_encoded), Some(vec![9]));

    WasmtimeRuntime::commit_block(
        &mut state,
        &[output_with_write(
            first_key,
            WriteOp::Deletion,
            TransactionStatus::Keep(ExecutionStatus::Success),
        )],
    )
    .unwrap();

    assert_eq!(state.get_optional(&first_encoded), None);
    assert_eq!(state.get_optional(&second_encoded), Some(vec![9]));
}

#[test]
fn executes_real_edsl_wasm_commits_and_recovers_state() {
    let wasm_path = std::env::var_os("XWASM_CONTRACT_WASM")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "./tests/wasmfile/contract.wasm".into());
    let wasm_bytes = fs::read(wasm_path).unwrap();
    let engine = wasmtime::Engine::new(Config::new().epoch_interruption(true)).unwrap();
    let aot_bytes: Arc<[u8]> = engine.precompile_module(&wasm_bytes).unwrap().into();

    let outputs = WasmtimeRuntime::parallel_execute_transactions(
        vec![
            transaction(&aot_bytes, "alice", 18),
            transaction(&aot_bytes, "bob", 20),
        ],
        &EmptyStateView,
        2,
    )
    .unwrap();

    assert!(outputs
        .iter()
        .all(|output| output.status() == TransactionStatus::Keep(ExecutionStatus::Success)));

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("xwasm-edsl-e2e-{}-{unique}", std::process::id()));

    {
        let mut state = State::open(&path).unwrap();
        WasmtimeRuntime::commit_block(&mut state, &outputs).unwrap();
    }

    {
        let mut recovered = State::open(&path).unwrap();
        let key = StateKey::AccessPath(AccessPath::new(contract_address(), b"last_call".to_vec()))
            .to_bytes()
            .unwrap();
        let value = recovered.get_optional(&key).unwrap();
        let stored: serde_json::Value = serde_json::from_slice(&value).unwrap();

        assert_eq!(stored["name"], "bob");
        assert_eq!(stored["age"], 20);
        assert_eq!(stored["sex"], "man");
    }

    fs::remove_dir_all(path).unwrap();
}
#[test]
fn parallel_api_falls_back_to_sequential_at_concurrency_one() {
    let code = storage_module();
    let key = storage_key();
    let state_view = HotStateView {
        key,
        value: vec![0],
    };
    let transactions = vec![
        storage_transaction(&code, "increment", 0),
        storage_transaction(&code, "increment", 1),
    ];

    let sequential =
        WasmtimeRuntime::sequential_execute_transactions(transactions, &state_view).unwrap();
    let fallback = WasmtimeRuntime::parallel_execute_transactions(
        vec![
            storage_transaction(&code, "increment", 0),
            storage_transaction(&code, "increment", 1),
        ],
        &state_view,
        1,
    )
    .unwrap();

    assert_eq!(fallback.len(), sequential.len());
    for (fallback, sequential) in fallback.iter().zip(&sequential) {
        assert_eq!(fallback.0.status, sequential.0.status);
        assert_eq!(fallback.0.data, sequential.0.data);
        assert_eq!(fallback.0.write_set, sequential.0.write_set);
    }
}
