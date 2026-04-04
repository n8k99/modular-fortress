# S01: Create + Update + Delete + Move — UAT

**Milestone:** M003
**Written:** 2026-04-04T23:12:25.948Z

## UAT: CRUD + Move\n\n- [x] POST /api/the_work creates row with generated id (201)\n- [x] GET /api/the_work/{id} reads it back\n- [x] PATCH /api/the_work/{id} updates fields + auto updated_at\n- [x] POST /api/the_work/{id}/move transfers to the_commons with new kind\n- [x] Source row gone (404)\n- [x] Target row present with new kind\n- [x] DELETE /api/the_commons/{id} removes it\n- [x] Deleted row returns 404\n- [x] Invalid fields rejected with warning log
