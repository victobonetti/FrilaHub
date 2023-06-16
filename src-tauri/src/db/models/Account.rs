// Estrutura para a conta de fiado
use crate::db::models::item::Item;
use crate::db::models::payment::Payment;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub items: Option<Vec<Item>>,
    pub payments: Option<Vec<Payment>>,
    pub paid_amount: f64,
    pub account_total: f64,
}

use serde::{Serialize, Serializer, ser::SerializeMap};
impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut payment_map = serializer.serialize_map(Some(6))?;
        payment_map.serialize_entry("id", &self.id)?;
        payment_map.serialize_entry("user_id", &self.user_id)?;
        payment_map.serialize_entry("items", &self.items)?;
        payment_map.serialize_entry("payments", &self.payments)?;
        payment_map.serialize_entry("paid_amount", &self.paid_amount)?;
        payment_map.serialize_entry("account_total", &self.account_total)?;
        payment_map.end()
    }
}


impl Account {
    fn get_payments(
        conn: &PooledConnection<SqliteConnectionManager>,
        account_id: String,
    ) -> Result<Vec<Payment>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT * FROM payments WHERE account_id = ?1")?;
        let rows = stmt.query_map(params![account_id], |row| {
            Ok(Payment {
                id: row.get(0)?,
                amount: row.get(1)?,
                account_id: row.get(2)?,
            })
        })?;

        let payments: Result<Vec<_>, rusqlite::Error> = rows.collect();
        println!("Payments found: {:?}", payments);
        payments
    }

    fn get_items(
        conn: &PooledConnection<SqliteConnectionManager>,
        account_id: String,
    ) -> Result<Vec<Item>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT * FROM items WHERE account_id = ?1")?;
        let rows = stmt.query_map(params![account_id], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                quantity: row.get(2)?,
                product_id: row.get(3)?,
                price: row.get(4)?,
                notes: row.get(5)?,
                account_id: row.get(6)?,
            })
        })?;

        let items: Result<Vec<_>, rusqlite::Error> = rows.collect();
        println!("Items found: {:?}", items);
        items
    }

    pub fn find_one(
        conn: &PooledConnection<SqliteConnectionManager>,
        account_id: String,
    ) -> Result<Option<Account>, rusqlite::Error> {
        let items = Account::get_items(conn, account_id.clone())?;
        let payments = Account::get_payments(conn, account_id.clone())?;

        let paid_amount = payments.iter().map(|payment| payment.amount as f64).sum();
        let account_total = items
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum();

        let mut stmt = conn.prepare("SELECT * FROM accounts WHERE id = ?1")?;
        let mut rows = stmt.query(params![account_id.clone()])?;

        if let Some(row) = rows.next()? {
            let account = Account {
                id: row.get(0)?,
                user_id: row.get(1)?,
                items: Some(items),
                payments: Some(payments),
                paid_amount,
                account_total,
            };                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    pub fn delete_one(
        conn: &PooledConnection<SqliteConnectionManager>,
        account_id: String,
    ) -> Result<(), rusqlite::Error> {
        // Excluir os registros da tabela payments relacionados à conta
        conn.execute(
            "DELETE FROM payments WHERE account_id = ?1",
            params![account_id],
        )?;

        // Excluir os registros da tabela items relacionados à conta
        conn.execute(
            "DELETE FROM items WHERE account_id = ?1",
            params![account_id],
        )?;

        // Verificar se algum usuário possui a conta
        let mut stmt = conn.prepare("SELECT id FROM users WHERE account_id = ?1 LIMIT 1")?;
        let mut rows = stmt.query(params![account_id])?;
        if let Some(row) = rows.next()? {
            let user_id: String = row.get(0)?;
            // Atualizar o usuário, removendo o valor da coluna account_id
            conn.execute(
                "UPDATE users SET account_id = NULL WHERE id = ?1",
                params![user_id],
            )?;
        }

        // Excluir a conta da tabela accounts
        conn.execute("DELETE FROM accounts WHERE id = ?1", params![account_id])?;

        Ok(())
    }

    fn find_one_by_user(conn: &PooledConnection<SqliteConnectionManager>, user_id: String) -> Result<bool> {
        let sql = "SELECT COUNT(*) FROM accounts WHERE user_id = ?1";
        let count: i64 = conn.query_row(sql, params![user_id], |row| row.get(0))?;
    
        Ok(count > 0)
    }

    pub fn create_account(
        conn: &PooledConnection<SqliteConnectionManager>,
        user_id: String,
    ) -> Result<String, rusqlite::Error> {
        // Verificar se o usuário já possui uma conta cadastrada

        let existing_account = Account::find_one_by_user(conn, user_id.clone())?;

        if existing_account {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let uuid = Uuid::new_v4().to_string();

        // Criar a conta
        conn.execute(
            "INSERT INTO accounts (id, user_id) VALUES (?1, ?2)",
            params![uuid, user_id.clone()],
        )?;

        // Atualizar usuário
        conn.execute(
            "UPDATE users SET account_id = ?1 WHERE id = ?2",
            params![uuid, user_id.clone()],
        )?;

        Ok(uuid)
    }
}
