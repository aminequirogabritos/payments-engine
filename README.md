### Payments Engine

A simple payments engine that processes a CSV file containing client transactions and produces a CSV representation of the resulting account state.

The engine supports deposits, withdrawals, disputes, resolves, and chargebacks. As transactions are processed, it keeps track of each client's available funds, held funds, total balance, and locked state.

#### How it works

The engine reads transactions sequentially from an input CSV file and applies them to the corresponding client account.

The supported transaction types are:

* `deposit`: Adds funds to the client's available balance.
* `withdrawal`: Removes funds from the client's available balance, provided sufficient funds are available.
* `dispute`: Moves the amount of the disputed deposit from available funds to held funds. A dispute may result in a negative available balance if the client has already spent part of the disputed funds.
* `resolve`: Releases the held funds associated with a previous dispute and returns them to the client's available balance.
* `chargeback`: Removes the disputed funds from the client's held balance and locks the client's account. This may result in a negative total balance if the client no longer has sufficient funds to cover the chargeback.

The resulting account state contains:

* `client`: Client identifier.
* `available`: Funds currently available to the client.
* `held`: Funds currently held due to disputes.
* `total`: The sum of available and held funds.
* `locked`: Whether the client's account has been locked following a chargeback.

Invalid transactions are rejected rather than being silently applied, with particular care taken around malformed input and inconsistent transaction relationships.

#### Technical decisions

##### Decimal arithmetic

Financial values are handled using [`rust_decimal`](https://crates.io/crates/rust_decimal) rather than floating-point types.

This avoids the precision and rounding issues that can arise when representing decimal financial values using binary floating-point arithmetic. It also allows the engine to preserve the expected decimal precision throughout the processing pipeline.

##### CSV processing

[`csv`](https://crates.io/crates/csv) and [`serde`](https://crates.io/crates/serde) are used for CSV parsing, deserialization, serialization, and output generation.

The `type` field from the input CSV is deserialized directly into a Rust enum rather than being handled as an arbitrary string. This makes the set of supported transaction types explicit and allows the compiler to help enforce the corresponding business logic.

##### Input consistency

The engine is intentionally strict when processing input. If a record cannot be parsed correctly, processing fails rather than continuing with potentially inconsistent account state.

The `amount` field is represented as `Option<Decimal>` because disputes, resolves, and chargebacks may contain an empty amount in the input format. Deposits and withdrawals, however, require an amount. If either of these transaction types is missing its amount, the transaction is considered invalid and processing stops.

Additional validation is performed for transactions that reference previous transactions:

* A dispute, resolve, or chargeback must refer to a transaction belonging to the same client as the referencing entry.
* A resolve or chargeback must have a corresponding previous dispute.
* Disputes are assumed to apply to deposits. This follows the interpretation that disputed funds are moved from a client's available balance into held funds. Under this model, a withdrawal cannot be disputed.
* A dispute does not require the client to have sufficient available funds to cover the disputed amount. If the disputed amount exceeds the client's current available balance, the available balance may become negative.
* A chargeback removes the disputed amount from held funds and locks the account. If the client does not have sufficient remaining funds to cover the chargeback, the resulting available and total balances may be negative.

When one of these relationships is inconsistent, the entry is treated as an error originating from the external transaction source rather than allowing it to modify the client's account state.

#### Maintainability and efficiency

The implementation intentionally prioritizes correctness, readability, maintainability, and safety over maximum processing efficiency.

There are several areas where the current implementation could be optimized. For example, when processing a dispute, the engine searches for the corresponding original transaction. Similarly, resolve and chargeback operations require finding a valid previous dispute. With sufficiently large datasets, repeatedly searching through previously processed transactions could become inefficient.

A more performance-oriented implementation could maintain additional indexes or data structures that allow these relationships to be resolved in constant or near-constant time. The current implementation deliberately avoids introducing that additional complexity because the focus of this project is keeping the business rules straightforward and easy to reason about.

The same consideration applies to memory and input processing. The current design is intended for the scope and requirements of this implementation rather than as a fully optimized high-throughput transaction processing system.

In other words, the current implementation favors:

1. Correct financial state transitions.
2. Explicit validation and failure behavior.
3. Clear and maintainable business logic.
4. Safe handling of financial values.
5. Simplicity over premature optimization.

The implementation could be extended with more sophisticated indexing and streaming-oriented processing if performance requirements increased.

#### Error handling philosophy

For financial data, silently ignoring malformed or inconsistent input can be more dangerous than stopping processing.

For that reason, the engine follows a fail-fast approach for malformed input. If a transaction cannot be deserialized or violates the structural requirements of the input format, the process stops instead of producing a potentially inconsistent output.

Business-level inconsistencies involving relationships between transactions are handled separately. When a dispute, resolve, or chargeback does not correspond to a valid transaction history, that entry is rejected without modifying the client's account state.

This distinction allows the engine to preserve a consistent account state while still recognizing that some invalid transactions may originate from the external system providing the transaction data.

#### Project structure

The project is implemented in Rust as a command-line application.

The general processing flow is:

```text
Input CSV
    |
    v
CSV deserialization
    |
    v
Transaction validation
    |
    v
Transaction processing
    |
    v
Client account state
    |
    v
Output CSV
```

#### Running the project

The application expects the input CSV path as a command-line argument and writes the resulting account state to standard output.

For example:

```bash
cargo run -- examples/transactions.csv > output.csv
```

The repository includes `examples/transactions.csv` as a representative input file for demonstration and manual testing. It contains a variety of valid transaction sequences, including deposits, withdrawals, disputes, resolves, chargebacks, multiple clients, and decimal amounts.

This file is provided only as an example of the expected input format and application behavior. It is not intended to represent a production dataset or an authoritative test fixture. The automated integration tests in `tests/` contain the specific input/output cases used to verify the application's behavior.

The output can be redirected directly to a CSV file or piped into another process.

#### Testing

The project includes automated integration tests covering different transaction sequences and expected account states.

Tests focus on the externally observable behavior of the application, including valid transaction processing and invalid or inconsistent input cases.

Run the test suite with:

```bash
cargo test
```

The test suite also verifies edge cases such as disputes exceeding the client's currently available funds, where the resulting balance may become negative.

#### Limitations and possible improvements

The current implementation intentionally leaves room for optimization.

Potential future improvements include:

* Maintaining indexes for faster transaction and dispute lookups.
* Reducing repeated searches when resolving transaction relationships.
* Supporting more streaming-oriented processing for very large inputs.
* Extending the test suite with additional property-based or unit-level tests.
* Introducing more specialized data structures if the transaction volume or performance requirements increase.

These improvements would primarily address scalability and performance rather than fundamental correctness of the current processing model.

#### AI Usage

Generative AI tools were used throughout the development of this project for technical discussion, debugging, code review, exploring implementation alternatives, and refining documentation.

The full transcripts of the AI interactions that contributed to the solution are available in [`docs/ai-transcript.md`](docs/ai-transcript.md).