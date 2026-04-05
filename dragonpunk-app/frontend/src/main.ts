// Dragonpunk Desktop — frontend entry point
// Calls Go bindings to display master_chronicle status

import * as DbService from '../bindings/github.com/n8k99/modular-fortress/dragonpunk-app/dbservice';

const statusEl = document.getElementById('status')!;
const contentEl = document.getElementById('content')!;

async function init() {
    try {
        // Health check
        const health = await DbService.Health();
        if (health.connected) {
            statusEl.textContent = `master_chronicle connected — ${health.table_count} tables`;
            statusEl.classList.add('connected');
        } else {
            statusEl.textContent = `Database: ${health.status}`;
            return;
        }

        // List tables with row counts
        const tables = await DbService.ListTables();
        tables.sort((a, b) => b.row_count - a.row_count);

        let html = '<h2 class="section-title">Nine Tables</h2>';
        for (const table of tables) {
            html += `
                <div class="table-row" data-table="${table.name}">
                    <span class="name">${table.name}</span>
                    <span class="count">${table.row_count.toLocaleString()} rows</span>
                </div>`;
        }
        html += '<div id="kinds" class="kinds-panel"></div>';
        contentEl.innerHTML = html;

        // Click handler for kind breakdown
        document.querySelectorAll('.table-row').forEach(row => {
            row.addEventListener('click', async () => {
                const tableName = (row as HTMLElement).dataset.table!;
                const kindsEl = document.getElementById('kinds')!;

                // Highlight selected
                document.querySelectorAll('.table-row').forEach(r => r.classList.remove('selected'));
                row.classList.add('selected');

                try {
                    const kinds = await DbService.ListKinds(tableName);
                    if (kinds.length === 0) {
                        kindsEl.innerHTML = `<div class="kind-header">${tableName} — no kinds</div>`;
                        return;
                    }

                    let kindHtml = `<div class="kind-header">${tableName} kinds</div>`;
                    for (const k of kinds) {
                        kindHtml += `
                            <div class="kind-row">
                                <span class="kind-name">${k.kind}</span>
                                <span class="kind-count">${k.count.toLocaleString()}</span>
                            </div>`;
                    }
                    kindsEl.innerHTML = kindHtml;
                } catch (err) {
                    kindsEl.innerHTML = `<div class="kind-header">Error: ${err}</div>`;
                }
            });
        });

    } catch (err) {
        statusEl.textContent = `Error: ${err}`;
    }
}

init();
