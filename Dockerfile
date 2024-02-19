FROM ruimarinho/bitcoin-core

# Create a directory for the wallet
RUN mkdir -p /data/wallet

# Start Bitcoin Core in the background
CMD ["bitcoind", "-datadir=/data"]

# Expose RPC port and data directory volume
EXPOSE 8332
VOLUME ["/data:/data"]

# Wait for bitcoind to start before creating wallet
ENTRYPOINT ["sh", "-c", "bitcoin-cli -datadir=/data -rpcuser=myuser -rpcpassword=strongpassword waitfornewblock 1 > /dev/null && bitcoin-cli -datadir=/data -rpcuser=myuser -rpcpassword=strongpassword createwallet mywallet"]

