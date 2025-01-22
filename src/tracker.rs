use crate::Action;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, Datelike};

#[derive(Debug, Serialize, Deserialize)]
struct Expense {
    id: u32,
    description: String,
    amount: f64,
    created_at: DateTime<Local>,
}

impl Expense {
    fn new(id: u32, description: impl Into<String>, amount: f64) -> Self {
        Expense { 
            id, 
            description: description.into(),
            amount,
            created_at: Local::now(),
        }
    }

    fn description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    fn amount(&mut self, amount: f64) {
        self.amount = amount;
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExpenseTracker {
    expenses: Vec<Expense>,
    last_id: u32,
}

impl ExpenseTracker {
    pub fn new() -> Self {
        ExpenseTracker::default()
    }

    fn add(&mut self, description: impl Into<String>, amount: f64) -> u32 {
        let id = self.last_id + 1;
        let expense = Expense::new(id, description, amount);
        self.expenses.push(expense);
        self.last_id = id;
        
        id
    }

    fn update_description(&mut self, id: u32, description: impl Into<String>) -> Option<u32> {
        let expense = self.expenses.iter_mut().find(|exp| exp.id == id)?; 
        expense.description(description);
        
        Some(id)
    }

    fn update_amount(&mut self, id: u32, amount: f64) -> Option<u32> {
        let expense = self.expenses.iter_mut().find(|exp| exp.id == id)?; 
        expense.amount(amount);
        
        Some(id)
    }

    fn delete(&mut self, id: u32) -> Option<()> {
        if self.expenses.iter().any(|exp| exp.id == id) {
            self.expenses.retain(|exp| exp.id != id);
            Some(())
        } else {
            None
        }
    }

    fn list_all(&self) -> String {
        let mut msg = String::from("# ID  Date         Amount     Description\n");
        
        for exp in &self.expenses {
            msg.push_str(
                format!("# {}   {}   ${}        {}\n", 
                    exp.id, 
                    exp.created_at.format("%d/%m/%Y"), 
                    exp.amount,
                    exp.description, 
                ).as_str()
            );
        }

        msg
    }

    fn summary(&self) -> String {
        let total_expenses: f64 = self.expenses.iter().map(|exp| exp.amount).sum();
        
        format!("# Total expenses: ${}", total_expenses)
    }

    fn summary_by_month(&self, month: u8) -> String {
        let months = ["January", "Febrary", "March", "May", "June", "July", "August", "September", "October", "November", "December"];
        let mut total_expenses: f64 = self.expenses.iter().filter(|exp| exp.created_at.month() == month as u32).map(|exp| exp.amount).sum();

        if total_expenses == 0.0 {
            total_expenses = 0.0;
        }
        
        format!("# Total expenses for {}: ${}", months[(month - 1) as usize], total_expenses)
    }


    pub fn process_action(&mut self, action: Action) -> String {
        match action {
            Action::Add { description, amount } => {
                let id = self.add(description, amount);
                format!("# Expense added successfully (ID: {})", id)
            },
            Action::Update { id, description, amount } => {
                match (description, amount) {
                    (Some(description), Some(amount)) => {
                        let desc_opt = self.update_description(id, description);
                        let amount_opt = self.update_amount(id, amount);

                        if desc_opt.is_none() || amount_opt.is_none() {
                            format!("# Expense does not exist (ID: {})", id)
                        } else {
                            format!("# Expense updated successfully (ID: {})", id)
                        } 
                    }, 
                    (Some(description), None) => {
                        let opt = self.update_description(id, description);
                        if opt.is_none() {
                            format!("# Expense does not exist (ID: {})", id)
                        } else {
                            format!("# Expense updated successfully (ID: {})", id)
                        } 
                    },
                    (None, Some(amount)) => {
                        let opt = self.update_amount(id, amount);
                        if opt.is_none() {
                            format!("# Expense does not exist (ID: {})", id)
                        } else {
                            format!("# Expense updated successfully (ID: {})", id)
                        } 
                    },
                    (None, None) => {
                        format!("Nothing to update for expense (ID: {})", id)
                    },
                }
            }
            Action::Delete { id } => {
                let opt = self.delete(id);
                if opt.is_none() {
                    format!("# Expense does not exist (ID: {})", id)
                } else {
                    format!("# Expense deleted successfully (ID: {})", id)
                } 
            }, 
            Action::List => self.list_all(), 
            Action::Summary { month } => match month {
                Some(m) => self.summary_by_month(m),
                None => self.summary(),
            }
        }
    }
}