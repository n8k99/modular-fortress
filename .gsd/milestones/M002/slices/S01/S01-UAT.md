# S01: Generic List + Get endpoints for all Nine Tables — UAT

**Milestone:** M002
**Written:** 2026-04-04T23:06:34.797Z

## UAT: Generic List + Get for all Nine Tables\n\n- [x] All 14 tables return paginated JSON via GET /api/{table}\n- [x] GET /api/{table}/{id} returns full row\n- [x] kind=task filter narrows the_work to 2,554 rows\n- [x] q=orbis search finds 670 realm entries\n- [x] Invalid table returns 404 JSON\n- [x] Missing row returns 404 JSON\n- [x] the_links (370K rows) paginates in <25ms
