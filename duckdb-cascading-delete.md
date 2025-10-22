Since DuckDB doesn't support `ON DELETE CASCADE` constraints, here are the best practices for handling cascading deletes:

## 1. **Manual Deletion with Transactions**
The most straightforward approach is to manually delete related records in the correct order within a transaction:

```sql
BEGIN TRANSACTION;

-- Delete child records first
DELETE FROM order_items WHERE order_id = 123;
DELETE FROM order_history WHERE order_id = 123;

-- Then delete the parent record
DELETE FROM orders WHERE id = 123;

COMMIT;
```

## 2. **Create Helper Procedures/Functions**
Implement deletion logic in your application layer or as SQL scripts:

```sql
-- Create a view or temporary function-like pattern
-- Example: Delete a customer and all related data
CREATE OR REPLACE VIEW delete_customer AS
WITH deleted_orders AS (
    DELETE FROM order_items 
    WHERE order_id IN (SELECT id FROM orders WHERE customer_id = ?)
),
deleted_order_records AS (
    DELETE FROM orders WHERE customer_id = ?
)
DELETE FROM customers WHERE id = ?;
```

## 3. **Application-Level Cascade Logic**
Handle cascading deletes in your application code:

```python
# Python example with DuckDB
def delete_customer_cascade(conn, customer_id):
    with conn.begin() as transaction:
        # Get related records first if needed for logging
        conn.execute("DELETE FROM order_items WHERE order_id IN (SELECT id FROM orders WHERE customer_id = ?)", [customer_id])
        conn.execute("DELETE FROM orders WHERE customer_id = ?", [customer_id])
        conn.execute("DELETE FROM customers WHERE id = ?", [customer_id])
```

## 4. **Use Soft Deletes Instead**
Consider implementing soft deletes with a `deleted_at` timestamp:

```sql
-- Instead of DELETE, use UPDATE
UPDATE customers SET deleted_at = CURRENT_TIMESTAMP WHERE id = 123;
UPDATE orders SET deleted_at = CURRENT_TIMESTAMP WHERE customer_id = 123;

-- Filter out soft-deleted records in queries
SELECT * FROM customers WHERE deleted_at IS NULL;
```

## 5. **Design Considerations**

### Use Nullable Foreign Keys Where Appropriate
```sql
-- Allow orphaned records where it makes sense
ALTER TABLE orders ALTER COLUMN customer_id SET DEFAULT NULL;
UPDATE orders SET customer_id = NULL WHERE customer_id = 123;
DELETE FROM customers WHERE id = 123;
```

### Create a Deletion Order Documentation
Document the dependency hierarchy:
```
Level 1: order_items, order_history
Level 2: orders, customer_addresses  
Level 3: customers
```

## 6. **Automated Script Generation**
Create a script to generate deletion statements:

```sql
-- Generate CASCADE DELETE statements
SELECT 
    'DELETE FROM ' || table_name || ' WHERE ' || column_name || ' = ?;' as delete_statement
FROM information_schema.columns 
WHERE column_name LIKE '%customer_id%'
ORDER BY table_name;
```

## Best Practice Recommendations

1. **Always use transactions** - Ensure atomicity of cascading deletes
2. **Delete in dependency order** - Children before parents
3. **Test thoroughly** - Verify all related records are handled
4. **Consider data retention** - Soft deletes might be preferable for audit trails
5. **Document relationships** - Maintain clear documentation of table dependencies
6. **Add checks in application layer** - Verify referential integrity before operations

For production systems, I'd recommend combining transactions with application-level cascade logic, as this gives you the most control and visibility over what's being deleted while maintaining data integrity.