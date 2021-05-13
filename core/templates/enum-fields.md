| Name | Value |
|------|-------|
{% for field in fields -%}
| `{{field.name}}` | `{{field.value}}` |
{% endfor %}
