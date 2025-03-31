# ForceCoin Blockchain Testing Commands
# -----------------------------------------

# 1. SETUP AND INITIALIZATION
# --------------------------

# Compile the project
cargo build --release

# 2. WALLET MANAGEMENT
# -------------------

# Create wallets for testing
cargo run --release -- create-wallet alice
cargo run --release -- create-wallet bob
cargo run --release -- create-wallet charlie
cargo run --release -- create-wallet miner1

# List all created wallets to verify and note addresses
cargo run --release -- list-wallets

# 3. NODE SETUP (Run these in separate terminal windows)
# ---------------------------------------------------

# Start first node
cargo run --release -- start-node 127.0.0.1:7001

# Start second node connected to first node (in a new terminal)
cargo run --release -- start-node 127.0.0.1:7002 127.0.0.1:7001

# Start third node connected to both previous nodes (in a new terminal)
cargo run --release -- start-node 127.0.0.1:7003 127.0.0.1:7001 127.0.0.1:7002

# 4. INITIAL MINING TESTS
# ---------------------

# Check initial blockchain stats - should show empty chain
cargo run --release -- blockchain-stats 127.0.0.1:7001

# Mine first block with miner1 (no transactions yet, just the mining reward)
cargo run --release -- mine miner1 127.0.0.1:7001

# Check stats again - should now show 1 block
cargo run --release -- blockchain-stats 127.0.0.1:7001

# Check balance of miner1 - should show mining reward
cargo run --release -- get-balance miner1 127.0.0.1:7001
# 5. TRANSACTION TESTS
# ------------------

# Send coins from miner1 to alice (replace ALICE_ADDRESS with actual address from list-wallets output)
cargo run --release -- send miner1 ALICE_ADDRESS 10 127.0.0.1:7001

# Check pending transactions
cargo run --release -- get-pending 127.0.0.1:7001

# Mine a block to include the transaction
cargo run --release -- mine bob 127.0.0.1:7002

# Check balances after transaction
cargo run --release -- get-balance miner1 127.0.0.1:7001
cargo run --release -- get-balance alice 127.0.0.1:7001

# 6. INSUFFICIENT BALANCE TEST
# --------------------------

# Try to send more coins than alice has (should fail with insufficient balance)
cargo run --release -- send alice BOB_ADDRESS 50 127.0.0.1:7001

# 7. MULTI-NODE SYNCHRONIZATION TEST
# --------------------------------

# Send a transaction from alice to charlie on node 1
cargo run --release -- send alice CHARLIE_ADDRESS 2 127.0.0.1:7001

# Mine it on node 3 (different from where transaction was submitted)
cargo run --release -- mine miner1 127.0.0.1:7003

# Check that all nodes are synchronized by checking balances on different nodes
cargo run --release -- get-balance charlie 127.0.0.1:7001
cargo run --release -- get-balance charlie 127.0.0.1:7002
cargo run --release -- get-balance charlie 127.0.0.1:7003

# 8. BLOCKCHAIN INSPECTION
# ----------------------

# Get blockchain statistics
cargo run --release -- blockchain-stats 127.0.0.1:7001

# Check block details (replace index with valid block number)
cargo run --release -- get-block 0 127.0.0.1:7001  # Genesis block
cargo run --release -- get-block 1 127.0.0.1:7001  # First mined block
# 9. ADVANCED TESTS
# ---------------

# Create and send multiple transactions
cargo run --release -- send miner1 BOB_ADDRESS 1 127.0.0.1:7001
cargo run --release -- send bob CHARLIE_ADDRESS 0.5 127.0.0.1:7002
cargo run --release -- send alice MINER1_ADDRESS 0.25 127.0.0.1:7003

# Check pending transactions on any node
cargo run --release -- get-pending 127.0.0.1:7001

# Mine to include all transactions
cargo run --release -- mine miner1 127.0.0.1:7001

# Final balance check for all wallets
cargo run --release -- get-balance miner1 127.0.0.1:7001
cargo run --release -- get-balance alice 127.0.0.1:7001
cargo run --release -- get-balance bob 127.0.0.1:7001
cargo run --release -- get-balance charlie 127.0.0.1:7001
