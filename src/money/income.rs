use std::cmp;

use indexmap::IndexMap;

use crate::{
    config,
    money::transactions::{Amount, Transactions},
};

pub fn allocate(transactions: &mut Transactions, config: config::Budgets) {
    if config.targets.is_empty() {
        // The rest of this function assumes that we have at lease one target
        // budget configured.
        return;
    }

    let mut monthly_budgets = IndexMap::new();
    let mut budget_totals = IndexMap::new();

    for budget in config.targets {
        let existing_entry = monthly_budgets
            .insert(budget.name.clone(), Amount::from(budget.monthly));
        assert!(existing_entry.is_none());

        *budget_totals
            .entry(budget.name.clone())
            .or_insert(Amount::zero()) +=
            transactions.account_total(budget.name);
    }

    'outer: for transaction in transactions {
        loop {
            let unallocated_budget =
                transaction.budgets.amount_for(&config.unallocated);

            let unallocated_budget = match unallocated_budget {
                Some(unallocated_budget) => unallocated_budget,
                None => {
                    // If there is no unallocated budget, we're done with this
                    // transaction.
                    continue 'outer;
                }
            };

            if unallocated_budget <= Amount::zero() {
                continue 'outer;
            }

            let mut budget_totals_in_months = IndexMap::new();

            for (name, &amount) in &monthly_budgets {
                let budget_total = *budget_totals
                    .entry(name.clone())
                    .or_insert(Amount::zero());
                let budget_total_in_months = (budget_total / amount).floor();

                budget_totals_in_months.insert(name, budget_total_in_months);
            }

            let mut min_budget_in_months = budget_totals_in_months
                .first()
                .map(|(&name, &total_in_months)| (name, total_in_months))
                .unwrap();

            for (name, &total_in_months) in &budget_totals_in_months {
                if total_in_months < min_budget_in_months.1 {
                    min_budget_in_months = (name, total_in_months);
                }
            }

            let min_budget_name = min_budget_in_months.0;
            let min_budget_in_months = min_budget_in_months.1;

            let monthly_budget = monthly_budgets[min_budget_name];
            let total = budget_totals[min_budget_name];

            let target_total =
                monthly_budget * (min_budget_in_months + 1.0).floor();
            let missing = target_total - total;

            let transfer_amount = cmp::min(missing, unallocated_budget);

            transaction.budgets.transfer(
                transfer_amount,
                &config.unallocated,
                min_budget_name.into(),
            );
            budget_totals[min_budget_name] += transfer_amount;
        }
    }
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use crate::{
        config,
        money::transactions::{Accounts, Amount, Transaction, Transactions},
    };

    use super::allocate;

    #[test]
    fn allocate_should_allocate_budgets_according_to_priority() {
        let config = config::Budgets {
            unallocated: "Unallocated".into(),
            targets: vec![
                config::Budget {
                    name: "A".into(),
                    monthly: 100_00,
                },
                config::Budget {
                    name: "B".into(),
                    monthly: 50_00,
                },
            ],
        };

        let amount = Amount::from(100_00);
        let mut transactions = Transactions::from(vec![
            Transaction {
                amount,
                budgets: Accounts::new().insert(&config.unallocated, amount),
                ..transaction()
            },
            Transaction {
                amount,
                budgets: Accounts::new().insert(&config.unallocated, amount),
                ..transaction()
            },
        ]);

        allocate(&mut transactions, config);

        assert_eq!(transactions.account_total("A"), Amount::from(150_00));
        assert_eq!(transactions.account_total("B"), Amount::from(50_00));
    }

    #[test]
    fn allocate_should_take_existing_budgets_into_account() {
        let config = config::Budgets {
            unallocated: "Unallocated".into(),
            targets: vec![
                config::Budget {
                    name: "A".into(),
                    monthly: 100_00,
                },
                config::Budget {
                    name: "B".into(),
                    monthly: 50_00,
                },
            ],
        };

        let amount = Amount::from(100_00);
        let mut transactions = Transactions::from(vec![
            Transaction {
                amount,
                budgets: Accounts::new()
                    .insert(&config.unallocated, amount / 2.0)
                    .insert("A", amount / 2.0),
                ..transaction()
            },
            Transaction {
                amount: amount * 2.0,
                budgets: Accounts::new()
                    .insert(&config.unallocated, amount)
                    .insert("B", amount),
                ..transaction()
            },
        ]);

        allocate(&mut transactions, config);

        assert_eq!(transactions.account_total("A"), Amount::from(200_00));
        assert_eq!(transactions.account_total("B"), Amount::from(100_00));
    }

    #[test]
    fn allocate_should_fill_highest_priority_budget_to_next_month() {
        let config = config::Budgets {
            unallocated: "Unallocated".into(),
            targets: vec![
                config::Budget {
                    name: "A".into(),
                    monthly: 100_00,
                },
                config::Budget {
                    name: "B".into(),
                    monthly: 100_00,
                },
            ],
        };

        let amount = Amount::from(50_00);
        let mut transactions = Transactions::from(vec![
            Transaction {
                amount,
                budgets: Accounts::new().insert(&config.unallocated, amount),
                ..transaction()
            },
            Transaction {
                amount,
                budgets: Accounts::new().insert(&config.unallocated, amount),
                ..transaction()
            },
        ]);

        allocate(&mut transactions, config);

        assert_eq!(transactions.account_total("A"), Amount::from(100_00));
        assert_eq!(transactions.account_total("B"), Amount::from(0_00));
    }

    #[test]
    fn allocate_should_not_panic_if_no_targets_are_configured() {
        let config = config::Budgets {
            unallocated: "Unallocated".into(),
            targets: Vec::new(),
        };

        let amount = Amount::from(100_00);
        let mut transactions = Transactions::from(vec![
            Transaction {
                amount,
                budgets: Accounts::new().insert(&config.unallocated, amount),
                ..transaction()
            },
            Transaction {
                amount,
                budgets: Accounts::new().insert(&config.unallocated, amount),
                ..transaction()
            },
        ]);

        allocate(&mut transactions, config);
    }

    fn transaction() -> Transaction {
        Transaction {
            #[rustfmt::skip]
            date: date!(2021-07-18),
            description: "A transaction".into(),
            amount: Amount::zero(),
            budgets: Accounts::new(),
        }
    }
}
