# Challenge 2020-01 — Bitmap

In this challenge, you have to implement a bitmap data structure.
Your bitmap will be non-resizeable.
The functions that your bitmap must support are described below.

- `new(num_entries)`: create a new bitmap that can old bits for
  num_entries, i.e., 0, 1, 2, ..., num_entries-1.
- `set(bitmap, entry)`: set the specified entry to `1` and return the
  *previous* value of that entry. If the entry is out of range
  (greater or equal to num_entry), return an error or raise an
  exception.
- `clear(bitmap, entry)`: set the specified entry to `0` and return
  the *previous* value of that entry. If the entry is out of range
  (greater or equal to num_entries), return an error or raise an
  exception.
- `get(bitmap, entry)`: return the value of the specified entry. If
  the entry is out of range (greater or equal to num_entries), return
  an error or raise an exception.
- `len(bitmap)`: return the number of entries that have the value 1.
