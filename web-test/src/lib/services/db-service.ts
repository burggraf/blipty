import type { Database } from '../db';

export interface Playlist {
    id?: number;
    name: string;
    server_url: string;
    username: string;
    password: string;
    epg_url?: string;
    created_at: string;
    updated_at?: string;
    last_updated?: string;
    is_active: number;
}

export class DbService {
    constructor(private db: Database) { }

    async addPlaylist(playlist: Omit<Playlist, 'id' | 'created_at'>): Promise<number> {
        const now = new Date().toISOString();
        const sql = `
            INSERT INTO playlists (
                name, server_url, username, password, epg_url, 
                created_at, is_active
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
        `;

        await this.db.exec(sql, [
            playlist.name,
            playlist.server_url,
            playlist.username,
            playlist.password,
            playlist.epg_url || null,
            now,
            playlist.is_active
        ]);

        const [{ id }] = await this.db.query('SELECT last_insert_rowid() as id');
        return id;
    }

    async getPlaylists(): Promise<Playlist[]> {
        return this.db.query('SELECT * FROM playlists ORDER BY created_at DESC');
    }

    async updatePlaylist(id: number, playlist: Partial<Playlist>): Promise<void> {
        const sets = Object.entries(playlist)
            .map(([key]) => `${key} = ?`)
            .join(', ');

        const sql = `UPDATE playlists SET ${sets}, updated_at = ? WHERE id = ?`;
        const values = [...Object.values(playlist), new Date().toISOString(), id];

        await this.db.exec(sql, values);
    }

    async deletePlaylist(id: number): Promise<void> {
        await this.db.exec('DELETE FROM playlists WHERE id = ?', [id]);
    }
}