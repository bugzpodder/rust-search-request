use serde_json::{json, to_string, Map, Value};
use std::collections::HashMap;

pub struct Query {
    query: String,
    aliases: HashMap<String, String>,
    pub values: HashMap<String, String>,
}

impl Query {
    pub fn new(query: String, aliases: HashMap<String, String>) -> Self {
        Self {
            query,
            aliases,
            values: HashMap::new(),
        }
    }

    pub fn get_query(
        &mut self,
        clause: &Value,
        order_clauses: &Value,
        offset: u64,
        limit: u64,
    ) -> String {
        self.values = HashMap::new();
        let mut stmt = self.query.clone();
        let where_clause = self.get_where(clause, "AND");
        if !where_clause.is_empty() {
            stmt.push_str(&format!(" WHERE {where_clause}"));
        }
        let order_by = self.get_order_by(order_clauses);
        if !order_by.is_empty() {
            stmt.push_str(&format!(" ORDER BY {}", order_by));
        }
        if offset > 0 {
            stmt.push_str(&format!(" OFFSET {offset}").to_string());
        }
        if limit > 0 {
            stmt.push_str(&format!(" LIMIT {limit}").to_string());
        }

        stmt
    }

    fn get_field(&mut self, key: &String) -> String {
        match self.aliases.get(key) {
            Some(alias) => alias.to_string(),
            None => key.to_string(),
        }
    }

    fn get_where(&mut self, clause: &Value, op: &str) -> String {
        if !clause.is_object() {
            return "".to_string();
        }
        let mut conditions = Vec::new();
        for (key, value) in clause.as_object().unwrap() {
            let mut condition = "".to_string();
            if key == "and" || key == "or" || key == "not" {
                let op = if key == "and" { "AND" } else { "OR" };
                condition = self.get_where(value, op);
            } else if value.is_object() {
                let name = self.get_field(key);
                condition = self.get_condition(&name, value.as_object().unwrap());
            }
            if !condition.is_empty() {
                conditions.push(condition);
            }
        }
        if !conditions.is_empty() {
            format!("({})", conditions.join(&format!(" {op} ").to_string()))
        } else {
            "".to_string()
        }
    }

    fn get_condition(&mut self, name: &String, field: &Map<String, Value>) -> String {
        let mut conditions = Vec::new();
        for (key, value) in field {
            let condition = match key.as_ref() {
                "eq" => self.get_filter(name, value, "="),
                "neq" => self.get_filter(name, value, "!="),
                "gt" => self.get_filter(name, value, ">"),
                "lt" => self.get_filter(name, value, "<"),
                "gte" => self.get_filter(name, value, ">="),
                "lte" => self.get_filter(name, value, "<="),
                "ilike" => self.get_filter(name, value, "ILIKE"),
                "nilike" => self.get_filter(name, value, "NOT ILIKE"),
                "like" => self.get_filter(name, value, "LIKE"),
                "nlike" => self.get_filter(name, value, "NOT LIKE"),
                "iregex" => self.get_filter(name, value, "~"),
                "niregex" => self.get_filter(name, value, "!~"),
                "similar" => self.get_filter(name, value, "SIMILAR TO"),
                "nsimilar" => self.get_filter(name, value, "NOT SIMILAR TO"),
                "is_in" => self.get_filter_arr(name, value, "IN"),
                "is_not_in" => self.get_filter_arr(name, value, "NOT IN"),
                "is_null" => self.get_filter(name, value, "IS NULL"),
                "is_not_null" => self.get_filter(name, value, "IS NOT NULL"),
                _ => "".to_string(),
            };
            if !condition.is_empty() {
                conditions.push(condition);
            }
        }
        if !conditions.is_empty() {
            format!("({})", conditions.join(&format!(" AND ").to_string()))
        } else {
            "".to_string()
        }
    }

    fn get_filter_arr(&mut self, name: &str, value: &Value, op: &str) -> String {
        if value.is_null() || value.as_array().unwrap().is_empty() {
            return "".to_string();
        }
        let placeholders = self.make_placeholders(value);
        format!("{name} {op} ({placeholders})")
    }

    fn get_filter(&mut self, name: &str, value: &Value, op: &str) -> String {
        if value.is_null() {
            return "".to_string();
        } else if value.is_boolean() {
            return format!("{name} {op}");
        }
        let placeholder = self.make_placeholders(value);
        format!("{name} {op} {placeholder}")
    }

    fn make_placeholders(&mut self, value: &Value) -> String {
        let mut placeholders = Vec::new();
        let values;
        let single_value;
        if value.is_array() {
            values = value;
        } else {
            single_value = json!([value]);
            values = &single_value;
        }

        for value in values.as_array().unwrap() {
            let placeholder = format!("v{}", self.values.len());
            self.values.insert(
                placeholder.to_string(),
                to_string(value).unwrap().to_string(),
            );
            placeholders.push(format!(":{placeholder}"));
        }

        placeholders.join(", ")
    }

    fn get_order_by(&mut self, order_by: &Value) -> String {
        let mut clauses = Vec::new();
        if !order_by.is_array() {
            return "".to_string();
        }
        let order_by_arr = order_by.as_array().unwrap();
        if order_by_arr.is_empty() {
            return "".to_string();
        }

        for order in order_by_arr {
            let field = self.get_field(&order["field"].as_str().unwrap().to_string());
            let direction;
            if order["dir"].is_null() || order["dir"].as_str().unwrap() == "asc" {
                direction = "ASC";
            } else {
                direction = "DESC";
            }
            clauses.push(format!("{field} {direction}"))
        }
        clauses.join(", ")
    }
}
