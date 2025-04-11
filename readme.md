### What is this branch about?
This is a very simple implementation of [diesel](https://docs.rs/diesel/latest/diesel/index.html)'s [```BoxableExpression```](https://docs.rs/diesel/latest/diesel/expression/trait.BoxableExpression.html) in combination with the [```alias```](https://docs.rs/diesel/latest/diesel/macro.alias.html) macro.

### Tables:
#### Contact:

| Id | Name    |
|----|---------|
| 1  | Lukas   |
| 2  | Dennis  |
| 3  | Tom     |
| 4  | Mareike |

#### User:

| Id | Name         | ContactId | CreatedById |
|----|--------------|-----------|-------------|
| 1  | Admin        | 1         | 1           |
| 2  | Lukas-User   | 1         | 1           |
| 3  | Dennis-User  | 2         | 1           |
| 4  | Tom-User     | 3         | 2           |
| 5  | Mareike-User | 4         | 2           |

### Run the code:
```cargo test -- --nocapture```