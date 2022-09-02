# lock hosts
This software watches /etc/hosts file and overwrites /etc/hosts by original data if /etc/hosts is modified.

I use /etc/hosts to block addicted websites. However, I edit /etc/hosts and avoid those blocks sometimes. This software contributes as an obstacle to such an attempt.

# Usage

```
git clone
sudo cargo build
cargo install --path .
sudo lock-hosts
```

# Tips
I recommend to auto start this software somehow.