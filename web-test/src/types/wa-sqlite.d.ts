declare module '@vlcn.io/wa-sqlite' {
    export interface SQLiteAPI {
        oo1: {
            DB: new (filename: string, options?: { vfs?: string }) => Database;
        };
        registerVFS: (vfs: VFS) => Promise<void>;
    }

    export interface Database {
        exec(sql: string): Promise<void>;
        prepare(sql: string): Promise<Statement>;
        close(): Promise<void>;
    }

    export interface Statement {
        bind(params: any[]): Promise<void>;
        step(): Promise<boolean>;
        get(): any;
        finalize(): Promise<void>;
    }

    export interface VFS {
        name: string;
    }
}

declare module '@vlcn.io/wa-sqlite/sqlite-wasm/wa-sqlite-async.wasm' {
    import { SQLiteAPI } from '@vlcn.io/wa-sqlite';
    const factory: () => Promise<SQLiteAPI>;
    export default factory;
}

declare module '@vlcn.io/wa-crsqlite' {
    import { VFS } from '@vlcn.io/wa-sqlite';
    const vfs: VFS;
    export default vfs;
}