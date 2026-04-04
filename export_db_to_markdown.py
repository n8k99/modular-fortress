#!/usr/bin/env python3
"""
Deterministic script to export master_chronicle database to markdown files.

This script connects to the PostgreSQL database and exports all tables
into organized markdown files under markdown/ directory.

No AI required - pure data extraction.
"""

import psycopg2
import os
import json
from datetime import datetime
from pathlib import Path

# Database connection settings from config.json
DB_CONFIG = {
    'host': '127.0.0.1',
    'port': 5432,
    'database': 'master_chronicle',
    'user': 'chronicle',
    'password': 'chronicle2026'
}

# Output directory
OUTPUT_DIR = Path('markdown')

def connect_db():
    """Connect to PostgreSQL database."""
    try:
        conn = psycopg2.connect(**DB_CONFIG)
        return conn
    except Exception as e:
        print(f"Failed to connect: {e}")
        print("Note: Database might need SSH tunnel: ssh -L 5432:127.0.0.1:5432 root@144.126.251.126")
        return None

def sanitize_filename(name):
    """Convert a name to a safe filename."""
    return "".join(c if c.isalnum() or c in (' ', '-', '_') else '_' for c in name).strip()

def export_table_to_markdown(table_name):
    """Export a single table to markdown files. Uses isolated connection."""
    # Create a fresh connection just for this table
    conn = connect_db()
    if not conn:
        print(f"Skipping {table_name}: connection failed")
        return False

    # Set autocommit to avoid transaction cascades
    conn.autocommit = True
    cursor = conn.cursor()

    try:
        # Set timeout for this specific query (2 minutes)
        cursor.execute("SET statement_timeout = '120000'")

        # Get column names
        cursor.execute(f"""
            SELECT column_name
            FROM information_schema.columns
            WHERE table_name = '{table_name}'
            ORDER BY ordinal_position
        """)
        columns = [row[0] for row in cursor.fetchall()]

        # Get all rows
        cursor.execute(f'SELECT * FROM {table_name}')
        rows = cursor.fetchall()

        print(f"Exporting {table_name}: {len(rows)} records")

        # Create table-specific subdirectory
        output_path = OUTPUT_DIR / table_name
        output_path.mkdir(parents=True, exist_ok=True)

        for idx, row in enumerate(rows):
            data = dict(zip(columns, row))

            # Try to create a meaningful filename
            if 'title' in data and data['title']:
                filename = sanitize_filename(str(data['title']))[:100]
            elif 'name' in data and data['name']:
                filename = sanitize_filename(str(data['name']))[:100]
            elif 'id' in data:
                filename = f"{table_name}_{data['id']}"
            else:
                filename = f"{table_name}_{idx}"

            filepath = output_path / f"{filename}.md"

            # Build markdown content
            md_content = f"---\n"
            md_content += f"table: {table_name}\n"

            for col, val in data.items():
                if val is not None:
                    if isinstance(val, (dict, list)):
                        md_content += f"{col}: |\n"
                        json_str = json.dumps(val, indent=2)
                        for line in json_str.split('\n'):
                            md_content += f"  {line}\n"
                    elif isinstance(val, datetime):
                        md_content += f"{col}: {val.isoformat()}\n"
                    else:
                        # Escape special characters for YAML
                        val_str = str(val).replace('\n', '\\n').replace('"', '\\"')
                        if '\n' in str(val) or len(str(val)) > 80:
                            md_content += f"{col}: |\n  {val}\n"
                        else:
                            md_content += f"{col}: {val_str}\n"

            md_content += "---\n\n"

            # Add body content if there's a content/body/text field
            body_fields = ['content', 'body', 'text', 'notes', 'description']
            for field in body_fields:
                if field in data and data[field]:
                    md_content += f"# {data.get('title', data.get('name', 'Content'))}\n\n"
                    md_content += str(data[field])
                    break

            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(md_content)

        return True

    except Exception as e:
        print(f"Error exporting {table_name}: {e}")
        return False
    finally:
        cursor.close()
        conn.close()

def export_all_tables():
    """Export all tables from the database."""
    # Get list of tables
    conn = connect_db()
    if not conn:
        return

    cursor = conn.cursor()
    cursor.execute("""
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
        ORDER BY table_name
    """)
    tables = [row[0] for row in cursor.fetchall()]
    cursor.close()
    conn.close()

    print(f"Found {len(tables)} tables to export\n")

    success_count = 0
    failed_tables = []

    for table in tables:
        try:
            if export_table_to_markdown(table):
                success_count += 1
            else:
                failed_tables.append(table)
        except Exception as e:
            print(f"Fatal error exporting {table}: {e}")
            failed_tables.append(table)

    print(f"\n{'='*60}")
    print(f"Success: {success_count}/{len(tables)} tables")
    if failed_tables:
        print(f"Failed: {', '.join(failed_tables)}")

def main():
    print("=" * 60)
    print("Master Chronicle Database → Markdown Exporter")
    print("=" * 60)
    print()

    export_all_tables()
    print("\n✓ Export complete!")
    print(f"Files written to: {OUTPUT_DIR.absolute()}")

if __name__ == '__main__':
    main()
