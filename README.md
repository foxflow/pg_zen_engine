# pg_zen_engine

<p>
<a href=""><img src="https://img.shields.io/badge/postgresql-12+-blue.svg" alt="PostgreSQL version" height="18"></a>
<img src="https://img.shields.io/pypi/l/markdown-subtemplate.svg" alt="License" height="18"></a>

</p>

---

**Source Code**: <a href="https://github.com/supabase/pg_zen_engine" target="_blank">https://github.com/foxflow/pg_zen_engine</a>

---

## Summary

`pg_zen_engine` is a PostgreSQL extension adding support for [JSON Decision Model](https://gorules.io/docs/developers/bre/json-decision-model) evaluation on `jsonb` data types.


## API
- evaluate_jdm
```sql
-- Validates a json *instance* against a *schema*
evaluate_jdm(graph jsonb, data json) returns jsonb
```
