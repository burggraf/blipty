import { writable } from 'svelte/store';
import { Database } from './db';
import { DbService } from './services/db-service';

export const db = writable<Database | null>(null);
export const dbService = writable<DbService | null>(null);

export async function initDatabase() {
    const database = Database.getInstance();
    await database.init();
    db.set(database);

    const service = new DbService(database);
    dbService.set(service);

    return { database, service };
}