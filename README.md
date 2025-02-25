# Goals

A lightweight, terminal-driven node on the Lightning network.

- Based on minimal LDK
- Looking at using PeerChannelEncryptor directly.
- LDK is a lot to insantiate to just get moving.

Some potential use cases:

- Runs on a small binary and accomplish some of the core features of Bitcoin SPV + lightning.
- Distributes the network more, or make the network more fault tolerant.
- Makes it easier for nodes to get online, to route payments, or spot bad actors.
- Perhaps accept payments or store information while your node is offline.
- Perhaps be designed to exist ephemerally, such as on phones or laptops with intermittent connections. Or perhaps on small cloud VMs.

Prior art:

- There is also [spruned](https://github.com/mempoolco/spruned), a bitcoin SPV client.
- Noise protocol [snow](https://github.com/mcginty/snow).
