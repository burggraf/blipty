import initSqlJs from 'sql.js';

export class Database {
    private static instance: Database;
    private db: any = null;
    private initialized = false;

    private constructor() { }

    static getInstance(): Database {
        if (!Database.instance) {
            Database.instance = new Database();
        }
        return Database.instance;
    }

    async init() {
        if (this.initialized) return;

        try {
            const SQL = await initSqlJs({
                locateFile: file => `https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.12.0/${file}`
            });

            // Try to load existing database from IndexedDB
            const existingData = await this.loadFromIndexedDB('blipty.db');

            // Create new database or load existing one
            this.db = existingData
                ? new SQL.Database(existingData)
                : new SQL.Database();

            // Initialize schema if this is a new database
            if (!existingData) {
                await this.initSchema();
                // Save the initial database
                const data = this.db.export();
                await this.persistToIndexedDB('blipty.db', data);
            }

            this.initialized = true;
        } catch (err) {
            console.error('Failed to initialize SQLite:', err);
            throw err;
        }
    }

    private async initSchema() {
        const tables = [
            `CREATE TABLE IF NOT EXISTS playlists (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                server_url TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                epg_url TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT,
                last_updated TEXT,
                is_active INTEGER NOT NULL DEFAULT 1
            )`,
            `CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY,
                category_id INTEGER NOT NULL UNIQUE,
                name TEXT NOT NULL,
                content_type TEXT NOT NULL DEFAULT 'live',
                type TEXT CHECK(type IN ('live', 'vod', 'movie', 'series')) NOT NULL DEFAULT 'movie',
                parent_id INTEGER,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            )`,
            `CREATE TABLE IF NOT EXISTS streams (
                id INTEGER PRIMARY KEY,
                stream_id INTEGER NOT NULL UNIQUE,
                name TEXT NOT NULL,
                category_id INTEGER REFERENCES categories(id),
                stream_type TEXT CHECK(stream_type IN ('live', 'vod', 'series')) NOT NULL,
                type_name TEXT,
                category_name TEXT,
                epg_id TEXT,
                icon_url TEXT,
                added INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            )`,
            `CREATE TABLE IF NOT EXISTS epg_data (
                id INTEGER PRIMARY KEY,
                channel_id TEXT NOT NULL,
                start INTEGER NOT NULL,
                end INTEGER NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                season INTEGER,
                episode INTEGER,
                FOREIGN KEY(channel_id) REFERENCES streams(epg_id)
            )`,
            `CREATE TABLE IF NOT EXISTS vod_metadata (
                id INTEGER PRIMARY KEY,
                stream_id INTEGER REFERENCES streams(id),
                rating REAL,
                director TEXT,
                year INTEGER,
                plot TEXT,
                imdb_id TEXT,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            )`,
            `CREATE TABLE IF NOT EXISTS favorites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                playlist_id INTEGER NOT NULL,
                stream_id TEXT NOT NULL,
                content_type TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                UNIQUE(playlist_id, stream_id),
                FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
            )`
        ];

        for (const sql of tables) {
            this.db.run(sql);
        }
    }

    private async persistToIndexedDB(dbName: string, data: Uint8Array) {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(dbName, 1);

            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                const db = request.result;
                const tx = db.transaction(['sqlite'], 'readwrite');
                const store = tx.objectStore('sqlite');

                const putRequest = store.put(data, 'db');
                putRequest.onsuccess = () => resolve(undefined);
                putRequest.onerror = () => reject(putRequest.error);
            };

            request.onupgradeneeded = (event) => {
                const db = (event.target as IDBOpenDBRequest).result;
                db.createObjectStore('sqlite');
            };
        });
    }

    private async loadFromIndexedDB(dbName: string): Promise<Uint8Array | null> {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(dbName, 1);

            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                const db = request.result;
                const tx = db.transaction(['sqlite'], 'readonly');
                const store = tx.objectStore('sqlite');

                const getRequest = store.get('db');
                getRequest.onsuccess = () => resolve(getRequest.result || null);
                getRequest.onerror = () => reject(getRequest.error);
            };

            request.onupgradeneeded = (event) => {
                const db = (event.target as IDBOpenDBRequest).result;
                db.createObjectStore('sqlite');
            };
        });
    }

    async query(sql: string, params: any[] = []): Promise<any[]> {
        const stmt = this.db.prepare(sql);
        return stmt.getAsObject(params);
    }

    async exec(sql: string, params: any[] = []): Promise<void> {
        const stmt = this.db.prepare(sql);
        stmt.run(params);
        stmt.free();
    }

    async close(): Promise<void> {
        if (this.db) {
            this.db.close();
        }
    }
}