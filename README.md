# Goals

The goal is to create a lightweight node on the Lightning network. How it is "lightweight" is still open for interpretation.

For example it could be lightweight:

- it could run in very low resource environments to simulate thousands of nodes, or to run on small hardware
- it could have limited blockchain access, e.g. it could have high network connectivity but low storage
- lightweight could mean no funds, e.g. it could be purely a relay like watchtowers

Some potential use cases:

- Runs on a small binary and accomplish some of the core features of Bitcoin SPV + lightning.
- Distributes the network more, or make the network more fault tolerant.
- Makes it easier for nodes to get online, to route payments, or spot bad actors.
- Perhaps accept payments or store information while your node is offline.
- Perhaps be designed to exist ephemerally, such as on phones or laptops with intermittent connections. Or perhaps on small cloud VMs.

Prior art:

- There is also [spruned](https://github.com/mempoolco/spruned), a bitcoin SPV client.
- Noise protocol [snow](https://github.com/mcginty/snow).
