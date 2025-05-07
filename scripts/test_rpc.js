const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

async function main() {
    // Wait for the crypto to be ready
    await cryptoWaitReady();

    // Create a keyring instance
    const keyring = new Keyring({ type: 'sr25519' });

    // Add Alice to our keyring with a hard-derived path (empty phrase, so the key is well-known)
    const alice = keyring.addFromUri('//Alice');

    // Create our API with a default connection to the local node
    const api = await ApiPromise.create({
        provider: new WsProvider('ws://127.0.0.1:9944')
    });

    // Set username
    console.log('Setting username for Alice...');
    const username = 'alice_username';

    try {
        // Create and send the transaction
        const tx = api.tx.usernameStorage.setUsername(username);
        const hash = await tx.signAndSend(alice);
        console.log('Transaction hash:', hash.toHex());

        // Wait for the transaction to be included in a block
        await new Promise(resolve => setTimeout(resolve, 5000));

        // Get username using the storage query
        console.log('Getting username...');
        const storedUsername = await api.query.usernameStorage.usernames(alice.address);
        console.log('Stored username:', storedUsername.toHuman());
    } catch (error) {
        console.error('Error:', error);
    }

    // Disconnect from the API
    await api.disconnect();
}

main().catch(console.error); 