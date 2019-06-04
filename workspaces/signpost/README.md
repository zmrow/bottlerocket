# signpost

si**g**n**p**os**t** is Thar's utility for modifying GPT priority bits on its OS disk.

```plain
USAGE:
    signpost <SUBCOMMAND>

SUBCOMMANDS:
    status                  Show partition sets and priority status
    mark-successful-boot    Mark the active partitions as successfully booted
    clear-inactive          Clears inactive priority information to prepare writing images disk
    upgrade-to-inactive     Sets the inactive partitions as new upgrade partitions
    rollback-to-inactive    Deprioritizes the inactive partitions
    rewrite-table           Rewrite the partition table with no changes to disk (used for testing this code)
```

## Background

The Thar OS disk has two partition sets, each containing three partitions:

* the *boot* partition, containing the `vmlinuz` Linux kernel image and the GRUB configuration. The kernel command line contains the root of the dm-verity hash tree.
* the *root* partition, containing the read-only `/` filesystem.
* the *hash* partition, containing the full dm-verity hash tree for the root partition.

The Thar boot partition uses the same GPT partition attribute flags as Chrome OS, which are [used by GRUB to select the partition to read a grub.cfg from](../../packages/grub/gpt.patch):

| Bits | Content |
|-|-|
| 63-57 | Unused |
| 56 | Successful boot flag |
| 55-52 | Tries remaining |
| 51-48 | Priority |
| 47-0 | Reserved by GPT specification |

The boot partition GRUB selects contains a grub.cfg which references the root and hash partitions by offset, thus selecting all three partitions of a set.

## Upgrade procedure

1. Run `signpost clear-inactive` to clear the priority and successful bits before making any changes to the inactive partitions.
2. Copy the downloaded images to the inactive partitions on disk, then validate data was written correctly.
3. Run `signpost upgrade-to-inactive` to prioritize the inactive partitions and allow it one boot attempt before automatically rolling back.

## Rollback procedure

1. Run `signpost rollback-to-inactive` to prioritize the inactive partitions without modifying whether the active partitions were successful.