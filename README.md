# Expense Tracker

Xpenser is a command line interface to manage your finances. This tool is inspired in the [Roadmap.sh expense tracker guidelines](https://roadmap.sh/projects/expense-tracker)

## Quick Start

Ensure you have [Rust installed](https://www.rust-lang.org/tools/install). Then run the following commands.

```bash
$ cargo run -- add --description "Lunch" --amount 20  
$ cargo run -- add --description "Dinner" --amount 30  
$ cargo run -- list
```

## Features
With this tool the user is able to:

- add an expense with a description and amount.
- update an expense.
- delete an expense.
- view all expenses.
- view a summary of all expenses.
- view a summary of expenses for a specific month (of current year).

## Example
The list of commands and their usage is given below:
```bash
$ cargo run -- add --description "Lunch" --amount 20
# Expense added successfully (ID: 1)

$ cargo run -- add --description "Dinner" --amount 10
# Expense added successfully (ID: 2)

$ cargo run -- list
# ID  Date       Description  Amount
# 1   2024-08-06  Lunch        $20
# 2   2024-08-06  Dinner       $10

$ cargo run -- summary
# Total expenses: $30

$ cargo run -- delete --id 2
# Expense deleted successfully

$ cargo run -- summary
# Total expenses: $20

$ cargo run -- summary --month 8
# Total expenses for August: $20
```