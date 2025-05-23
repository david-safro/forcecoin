// src/routes/api/blockchain/+server.js
import { json } from '@sveltejs/kit';
import { spawn } from 'child_process';
import { promisify } from 'util';
import net from 'net';
import path from 'path';

// Fix the path - from frontend/website/ to project root target directory
const BLOCKCHAIN_CLI_PATH = process.platform === 'win32'
    ? '../../target/release/blockchain.exe'  // Windows
    : '../../target/release/blockchain';     // Linux/Mac

const DEFAULT_NODE_ADDRESS = '127.0.0.1:7001';

// Helper function to execute Rust CLI commands
/**
 * @param {string[]} args
 */
function executeBlockchainCommand(args) {
    return new Promise((resolve, reject) => {
        const process = spawn(BLOCKCHAIN_CLI_PATH, args);
        let stdout = '';
        let stderr = '';

        process.stdout.on('data', (data) => {
            stdout += data.toString();
        });

        process.stderr.on('data', (data) => {
            stderr += data.toString();
        });

        process.on('close', (code) => {
            if (code === 0) {
                resolve(stdout.trim());
            } else {
                reject(new Error(`Command failed: ${stderr || stdout}`));
            }
        });

        process.on('error', (error) => {
            reject(error);
        });
    });
}

// Helper function to communicate with blockchain node via TCP
/**
 * @param {string} command
 * @param nodeAddress
 */
function sendNodeCommand(command, nodeAddress = DEFAULT_NODE_ADDRESS) {
    return new Promise((resolve, reject) => {
        const client = new net.Socket();
        let response = '';

        client.connect(parseInt(nodeAddress.split(':')[1], 10), nodeAddress.split(':')[0], () => {
            client.write(command);
        });

        client.on('data', (data) => {
            response += data.toString();
        });

        client.on('close', () => {
            resolve(response.trim());
        });

        client.on('error', (error) => {
            reject(error);
        });

        // Timeout after 10 seconds
        setTimeout(() => {
            client.destroy();
            reject(new Error('Connection timeout'));
        }, 10000);
    });
}

export async function POST({ request, cookies }) {
    try {
        const { action, ...params } = await request.json();
        const session = cookies.get('session');

        if (!session) {
            return json({ error: 'Not authenticated' }, { status: 401 });
        }

        const user = JSON.parse(session);
        const walletLabel = `user_${user.userId}`; // Create wallet label based on user ID

        switch (action) {
            case 'create_wallet':
                try {
                    const result = await executeBlockchainCommand(['create-wallet', walletLabel]);
                    return json({ success: true, message: result });
                } catch (error) {
                    // Wallet might already exist, that's okay
                    return json({ success: true, message: 'Wallet ready' });
                }

            case 'get_balance':
                try {
                    const balance = await sendNodeCommand(`GET_BALANCE ${params.address || walletLabel}`);
                    return json({ success: true, balance: parseFloat(balance) || 0 });
                } catch (error) {
                    return json({ success: true, balance: 0 }); // Default to 0 if node not running
                }

            case 'mine':
                try {
                    const result = await executeBlockchainCommand(['mine', walletLabel, DEFAULT_NODE_ADDRESS]);
                    return json({ success: true, message: result });
                } catch (error) {
                    return json({ error: error.message }, { status: 500 });
                }

            case 'send_transaction':
                try {
                    const { recipient, amount } = params;
                    const result = await executeBlockchainCommand([
                        'send', walletLabel, recipient, amount.toString(), DEFAULT_NODE_ADDRESS
                    ]);
                    return json({ success: true, message: result });
                } catch (error) {
                    return json({ error: error.message }, { status: 500 });
                }

            case 'get_stats':
                try {
                    const stats = await sendNodeCommand('GET_STATS');
                    return json({ success: true, stats: stats });
                } catch (error) {
                    return json({ success: true, stats: 'Node offline' });
                }

            case 'start_node':
                try {
                    // Start node in background
                    const nodeProcess = spawn(BLOCKCHAIN_CLI_PATH, ['start-node', DEFAULT_NODE_ADDRESS], {
                        detached: true,
                        stdio: 'ignore'
                    });
                    nodeProcess.unref();

                    // Wait a moment for node to start
                    await new Promise(resolve => setTimeout(resolve, 2000));

                    return json({ success: true, message: 'Node started' });
                } catch (error) {
                    return json({ error: error.message }, { status: 500 });
                }

            default:
                return json({ error: 'Unknown action' }, { status: 400 });
        }

    } catch (error) {
        console.error('Blockchain API error:', error);
        return json({ error: 'Internal server error' }, { status: 500 });
    }
}