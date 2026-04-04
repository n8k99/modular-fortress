---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T03: Integration test — full lifecycle

Create a test entry in the_work (kind=task, title=test), read it back, update the title, read again, delete it, confirm 404. All via curl against running server.

## Inputs

- `T02 handlers`

## Expected Output

- `Passing integration test`

## Verification

Create → Read → Update → Read → Delete → 404 cycle passes
