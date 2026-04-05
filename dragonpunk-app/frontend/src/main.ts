// Dragonpunk Desktop — Foundry VTT-style layout
// Scene canvas with pan/zoom + collapsible icon-rail sidebars

import * as DbService from '../bindings/github.com/n8k99/modular-fortress/dragonpunk-app/dbservice';

// ── Health check ──
async function checkHealth() {
    const statusDot = document.getElementById('status-indicator')!;
    const statusText = document.getElementById('status-text')!;
    try {
        const health = await DbService.Health();
        if (health.connected) {
            statusDot.classList.add('connected');
            statusText.textContent = `master_chronicle · ${health.table_count} tables`;
        } else {
            statusText.textContent = `db: ${health.status}`;
        }
    } catch (err) {
        statusText.textContent = `error: ${err}`;
    }
}

// ── Scene canvas pan/zoom ──
function initCanvas() {
    const viewport = document.getElementById('scene-viewport')!;
    const content = document.getElementById('scene-content')!;

    let scale = 0.5;    // start zoomed out to see more of the map
    let panX = -200;
    let panY = -100;
    let isPanning = false;
    let startX = 0;
    let startY = 0;

    function applyTransform() {
        content.style.transform = `translate(${panX}px, ${panY}px) scale(${scale})`;
    }

    // Mouse drag to pan
    viewport.addEventListener('mousedown', (e: MouseEvent) => {
        if (e.button !== 0) return; // left click only
        isPanning = true;
        startX = e.clientX - panX;
        startY = e.clientY - panY;
        e.preventDefault();
    });

    window.addEventListener('mousemove', (e: MouseEvent) => {
        if (!isPanning) return;
        panX = e.clientX - startX;
        panY = e.clientY - startY;
        applyTransform();
    });

    window.addEventListener('mouseup', () => {
        isPanning = false;
    });

    // Scroll wheel to zoom (around cursor position)
    viewport.addEventListener('wheel', (e: WheelEvent) => {
        e.preventDefault();
        const zoomFactor = e.deltaY > 0 ? 0.9 : 1.1;
        const newScale = Math.min(Math.max(scale * zoomFactor, 0.1), 3);

        // Zoom toward cursor position
        const rect = viewport.getBoundingClientRect();
        const mx = e.clientX - rect.left;
        const my = e.clientY - rect.top;

        panX = mx - (mx - panX) * (newScale / scale);
        panY = my - (my - panY) * (newScale / scale);
        scale = newScale;

        applyTransform();
    }, { passive: false });

    applyTransform();
}

// ── Sidebar panel toggle ──
function initSidebars() {
    // LHSB domain icons
    document.querySelectorAll('#lhsb .sb-icon[data-domain]').forEach(btn => {
        btn.addEventListener('click', () => {
            const domain = (btn as HTMLElement).dataset.domain!;
            const panel = document.getElementById('lhsb-panel')!;
            const wasActive = btn.classList.contains('active');

            // Clear all active states on this sidebar
            document.querySelectorAll('#lhsb .sb-icon').forEach(b => b.classList.remove('active'));

            if (wasActive) {
                panel.hidden = true;
            } else {
                btn.classList.add('active');
                panel.hidden = false;
                loadDomainPanel(domain, panel);
            }
        });
    });

    // RHSB composed view icons
    document.querySelectorAll('#rhsb .sb-icon[data-panel]').forEach(btn => {
        btn.addEventListener('click', () => {
            const panelName = (btn as HTMLElement).dataset.panel!;
            const panel = document.getElementById('rhsb-panel')!;
            const wasActive = btn.classList.contains('active');

            document.querySelectorAll('#rhsb .sb-icon').forEach(b => b.classList.remove('active'));

            if (wasActive) {
                panel.hidden = true;
            } else {
                btn.classList.add('active');
                panel.hidden = false;
                panel.innerHTML = `<div class="panel-title">${panelName}</div><div class="panel-placeholder" style="color:var(--text-muted);font-size:11px;margin-top:1rem;">Coming soon...</div>`;
            }
        });
    });

    // Settings gear
    document.querySelector('#lhsb .sb-icon[data-panel="settings"]')?.addEventListener('click', () => {
        const panel = document.getElementById('lhsb-panel')!;
        const btn = document.querySelector('#lhsb .sb-icon[data-panel="settings"]')!;
        const wasActive = btn.classList.contains('active');

        document.querySelectorAll('#lhsb .sb-icon').forEach(b => b.classList.remove('active'));

        if (wasActive) {
            panel.hidden = true;
        } else {
            btn.classList.add('active');
            panel.hidden = false;
            panel.innerHTML = `<div class="panel-title">Settings</div><div style="color:var(--text-muted);font-size:11px;">Domain configuration and options</div>`;
        }
    });
}

// ── Load domain entries into panel ──
async function loadDomainPanel(domain: string, panel: HTMLElement) {
    panel.innerHTML = `<div class="panel-title">${formatDomainName(domain)}</div><div style="color:var(--text-muted);">Loading...</div>`;

    try {
        const kinds = await DbService.ListKinds(domain);
        let html = `<div class="panel-title">${formatDomainName(domain)}</div>`;

        if (kinds.length === 0) {
            html += `<div style="color:var(--text-muted);font-size:11px;">No entries</div>`;
        } else {
            for (const k of kinds) {
                html += `<div class="panel-entry">
                    <div class="entry-title">${k.kind}</div>
                    <div class="entry-kind">${k.count.toLocaleString()} entries</div>
                </div>`;
            }
        }
        panel.innerHTML = html;
    } catch (err) {
        panel.innerHTML = `<div class="panel-title">${formatDomainName(domain)}</div><div style="color:#E24B4A;">Error: ${err}</div>`;
    }
}

function formatDomainName(domain: string): string {
    return domain.replace('the_', 'The ').replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
}

// ── Init ──
checkHealth();
initCanvas();
initSidebars();
