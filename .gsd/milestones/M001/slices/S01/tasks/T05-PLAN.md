---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T05: Integration verification + README

Full end-to-end test: build binary, start it, curl /api/health, verify JSON response. Add brief README.md or comment header in main.go with quickstart instructions.

## Inputs

- `T04 working server`

## Expected Output

- `dragonpunk/README.md`
- `Passing integration test`

## Verification

go build -o dragonpunk/bin/dragonpunk ./dragonpunk && ./dragonpunk/bin/dragonpunk & sleep 1 && curl -s localhost:PORT/api/health | jq . && kill %1
