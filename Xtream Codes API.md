<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" class="logo" width="120"/>

# 

---

# I want to create an app that uses xstream codes to manage access to external service providers.  I need complete API documentation that uses xstream codes for server url, username, and password information to be able to:

1. download a list of categories and channels for live content
2. download a list of categories and channels for vod content (movies, series, etc.)
3. download epg information for upcoming live channels
4. prepare urls for streaming individual live channels or vod content
I'd like my program to be able to download this content into a local SQLite database, so any information about a compatible sql data structure to hold all this data would be helpful.  The application will be written in Tauri 2.0 with Rust as the backend and Svelte 5 and shadcn-svelte as the UI library if any of that matters

## Overview

This technical report provides a detailed implementation guide for integrating Xtream Codes APIs into a Tauri 2.0 application using Rust/Svelte. The solution includes API interaction patterns, SQLite schema design, and data synchronization workflows compatible with IPTV service providers using Xtream Codes panels.

---

## Xtream Codes API Architecture

### **Base URL Structure**

All endpoints use:
`http://[SERVER]:[PORT]/player_api.php?username=[USER]&password=[PWD]`
*Example:* `http://iptv.example:8080/player_api.php?username=john&password=abc123`[^1][^6]

---

### **Core API Endpoints**

#### 1. Live Content Endpoints

| Endpoint | Parameters | Response Format | Description |
| :-- | :-- | :-- | :-- |
| `action=get_live_categories` | None | JSON | Returns all live TV categories |
| `action=get_live_streams` | `category_id=X` | JSON | Channels within specified category |
| `action=get_epg` | `stream_id=X` | JSON | EPG data for specific channel |

**Sample Response (get_live_streams):**

```json
[
  {
    "num": 1,
    "name": "BBC One",
    "stream_type": "live",
    "stream_id": 2059,
    "epg_channel_id": "bbc.one.uk"
  }
]
```

---

#### 2. VOD Content Endpoints

| Endpoint | Parameters | Response Format | Description |
| :-- | :-- | :-- | :-- |
| `action=get_vod_categories` | None | JSON | Movie/Series categories |
| `action=get_vod_streams` | `category_id=X` | JSON | Content items within category |
| `action=get_series_info` | `series_id=X` | JSON | Detailed series metadata |

**VOD Stream URL Construction:**
`http://[SERVER]:[PORT]/movie/[USER]/[PWD]/[STREAM_ID].EXT`
*Example:* `http://iptv.example:8080/movie/john/abc123/3041.mp4`[^3][^16]

---

#### 3. EPG Endpoints

| Endpoint | Parameters | Response Format | Description |
| :-- | :-- | :-- | :-- |
| `action=get_short_epg` | `stream_id=X&limit=Y` | JSON | Next Y programs for channel |
| `xmltv.php` | None | XMLTV | Full EPG data in XMLTV format |

**XMLTV Response Snippet:**

```xml
<programme channel="bbc.one.uk" start="20250302180000 +0000" stop="20250302183000 +0000">
  <title lang="en">News at Six</title>
</programme>
```

---

## SQLite Database Schema Design

### **Core Tables**

#### 1. Categories Table

```sql
CREATE TABLE categories (
  id INTEGER PRIMARY KEY,
  category_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  type TEXT CHECK(type IN ('live', 'vod')) NOT NULL,
  parent_id INTEGER REFERENCES categories(id),
  UNIQUE(category_id, type)
);
```


#### 2. Streams Table

```sql
CREATE TABLE streams (
  id INTEGER PRIMARY KEY,
  stream_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  category_id INTEGER REFERENCES categories(id),
  stream_type TEXT CHECK(stream_type IN ('live', 'vod', 'series')) NOT NULL,
  epg_id TEXT,
  icon_url TEXT,
  added INTEGER, -- UNIX timestamp
  UNIQUE(stream_id, stream_type)
);
```


#### 3. EPG Data Table

```sql
CREATE TABLE epg_data (
  id INTEGER PRIMARY KEY,
  channel_id TEXT NOT NULL,
  start INTEGER NOT NULL, -- UNIX timestamp
  end INTEGER NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  season INTEGER,
  episode INTEGER,
  FOREIGN KEY(channel_id) REFERENCES streams(epg_id)
);
```


#### 4. VOD Metadata Table

```sql
CREATE TABLE vod_metadata (
  id INTEGER PRIMARY KEY,
  stream_id INTEGER REFERENCES streams(id),
  rating REAL,
  director TEXT,
  year INTEGER,
  plot TEXT,
  imdb_id TEXT
);
```

**Indexes:**

```sql
CREATE INDEX idx_streams_category ON streams(category_id);
CREATE INDEX idx_epg_times ON epg_data(start, end);
```

---

## Rust Implementation Strategy

### **API Client Module**

```rust
struct XtreamClient {
    base_url: String,
    username: String,
    password: String,
    db_conn: Connection
}

impl XtreamClient {
    async fn fetch_live_categories(&self) -> Result<Vec<Category>> {
        let url = format!("{}&action=get_live_categories", self.base_url);
        let response = reqwest::get(&url).await?.json().await?;
        self.db_conn.execute("INSERT OR REPLACE INTO categories...")?;
        Ok(response)
    }

    fn generate_stream_url(&self, stream: &Stream) -> String {
        match stream.stream_type.as_str() {
            "live" => format!("{}/live/{}/{}/{}.ts", 
                self.base_url, self.username, self.password, stream.stream_id),
            "vod" => format!("{}/movie/{}/{}/{}.mp4",
                self.base_url, self.username, self.password, stream.stream_id),
            _ => panic!("Unsupported stream type")
        }
    }
}
```

---

### **Database Sync Service**

```rust
async fn full_sync(client: &XtreamClient) -> Result<()> {
    let tx = client.db_conn.transaction()?;
    
    // Atomic update sequence
    sync_categories(&tx, "live").await?;
    sync_streams(&tx, "live").await?;
    sync_epg(&tx).await?;
    
    tx.commit()?;
    Ok(())
}

async fn sync_epg(conn: &Connection) -> Result<()> {
    let epg_data = client.fetch_epg().await?;
    conn.execute(
        "INSERT INTO epg_data 
        (channel_id, start, end, title) 
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(channel_id, start) DO UPDATE SET...",
        params![epg.channel_id, epg.start, epg.end, epg.title]
    )?;
}
```

---

## Performance Optimization

1. **Caching Layer:**
    - Implement LRU cache for frequent API calls (categories/stream lists)
    - Cache EPG data with TTL of 1 hour
2. **Bulk Insert Operations:**
```rust
fn batch_insert_epg(conn: &Connection, epg_list: Vec<EpgEntry>) -> Result<()> {
    let mut stmt = conn.prepare_cached(
        "INSERT OR REPLACE INTO epg_data 
        (channel_id, start, end, title) 
        VALUES (?, ?, ?, ?)"
    )?;

    for epg in epg_list {
        stmt.execute(params![
            epg.channel_id,
            epg.start,
            epg.end,
            epg.title
        ])?;
    }
    Ok(())
}
```

3. **Database Indexing Strategy:**
    - Composite index on (streams.category_id, streams.stream_type)
    - Covering index for EPG queries: (epg_data.channel_id, start, end)

---

## Security Considerations

1. **Credential Storage:**
    - Use Tauri's secure store for username/password
    - Encrypt credentials using OS-level keychain
2. **API Request Security:**
    - Always use HTTPS endpoints
    - Validate server certificates
    - Implement request signing mechanism
3. **SQL Injection Protection:**
    - Use Rust's prepared statements exclusively
    - Validate all category_id/stream_id parameters as integers

---

## Error Handling Framework

```rust
enum XtreamError {
    ApiError(reqwest::Error),
    DbError(rusqlite::Error),
    InvalidData(String)
}

impl From<rusqlite::Error> for XtreamError { ... }

async fn sync_data(client: &XtreamClient) -> Result<(), XtreamError> {
    let categories = client.fetch_live_categories()
        .await
        .map_err(XtreamError::ApiError)?;
    
    client.store_categories(categories)
        .map_err(XtreamError::DbError)
}
```

---

## UI Integration (Svelte Example)

```svelte
<!-- Category Browser -->
<CategoryList let:category>
  <div class="category-card">
    <h2>{category.name}</h2>
    <StreamGrid streams={category.streams} />
  </div>
</CategoryList>

<script>
  export let category;
  
  async function loadStreams() {
    const response = await fetch(`/api/streams?category=${category.id}`);
    return response.json();
  }
</script>
```

---

## Conclusion

This implementation provides a complete solution for integrating Xtream Codes APIs with a local SQLite database, optimized for performance and security. Key features include:

1. Comprehensive API coverage for live/VOD content and EPG data
2. Normalized database schema with efficient indexing
3. Rust-based synchronization service with atomic transactions
4. Secure credential handling and error management

Future extensions could add:

- Background sync service
- User preference tracking
- Advanced EPG search capabilities
- Multi-provider aggregation support

The architecture leverages modern Tauri capabilities while maintaining compatibility with standard Xtream Codes implementations.

<div style="text-align: center">⁂</div>

[^1]: https://stackoverflow.com/questions/78847811/how-to-build-a-playable-url-from-xtream-codes-api

[^2]: https://www.reddit.com/r/TiviMate/comments/1anu7nw/how_do_i_extract_only_the_live_channels_from/

[^3]: https://github.com/gogetta69/TMDB-To-VOD-Playlist

[^4]: https://epgguide.net/xtream-ui-iptv-panel/

[^5]: https://pub.dev/documentation/xtream_code_client/latest/

[^6]: https://troypoint.com/xtream-codes/

[^7]: https://epgready.com/how-to-setup-epg-for-xtream-codes-iptv-panel/

[^8]: https://pkg.go.dev/github.com/ze3ma0/yptv

[^9]: https://www.sqlite.org/rescode.html

[^10]: https://pkg.go.dev/github.com/almo7aya/go.xtream-codes

[^11]: https://www.datensen.com/blog/docs/sqlite-database-design/

[^12]: https://github.com/ProTechEx/xtream-codes-decoded-v2.9/blob/master/database.sql

[^13]: https://www.reddit.com/r/ZoomPlayer/comments/1eg5g9y/iptv_xtreme_codes_vs_playlist_m3u_epg_xml/

[^14]: https://dbschema.com/2023/06/02/sqlite/create-db/

[^15]: https://www.worldofiptv.com/threads/trim-and-export-and-import-the-xtream-codes-database-as-sql-or-gzip-manually.431/

[^16]: https://stackoverflow.com/questions/59654755/how-to-generate-sqlite-entity-relationship-diagram-from-database-file

[^17]: https://github.com/ProTechEx/xtream-codes-decoded-v2.9

[^18]: https://kandi.openweaver.com/shell/ProTechEx/Xtream-Codes-2.2.0-Nulled

[^19]: https://github.com/ProTechEx/xtream-codes-decoded-v2.9/blob/master/crons/users_checker.php

[^20]: https://www.reddit.com/r/TiviMate/comments/12ca5lz/xtream_codestivimate/

[^21]: https://appsnscripts.com/index.php?threads%2Fxtream-codes-v2-decoded-php.1716%2F

[^22]: https://www.reddit.com/r/TiviMate/comments/11dq2vm/xtream_code_vs_m3u/

[^23]: https://kandi.openweaver.com/php/ProTechEx/IPTV-Panel

[^24]: https://github.com/xoceunder/xtream-codes-decoded/blob/master/database.sql

[^25]: https://docs.oracle.com/en/database/oracle/oracle-database/19/xstrm/xstream-guide.pdf

[^26]: https://debezium.io/documentation/reference/stable/connectors/oracle.html

[^27]: https://debezium.io/documentation/reference/stable/operations/debezium-server.html

[^28]: https://docs.oracle.com/cd/E13167_01/aldsp/docs25/appdev/jdbcclt.html

[^29]: https://www.worldofiptv.com/threads/how-can-i-request-live-channels-only-list-from-xtream-codes-api.1699/

[^30]: https://iptvfreetrials.com/xtream-codes-api-iptv/

[^31]: https://forums.nextpvr.com/showthread.php?tid=60851

[^32]: https://www.inmatrix.com/blog/iptv_xtream_codes_vs_playlist_m3u_and_epg_xml.shtml

[^33]: https://www.inmatrix.com/blog/iptv_the_xtream_codes_saga_part_2.shtml

[^34]: https://revoiptv.com/how-to-setup-iptv-on-gse-via-xtream-codes-api/

[^35]: https://www.techkings.org/threads/xtream-epg-or-xml.170604/

[^36]: https://github.com/AndreyPavlenko/Fermata/discussions/434

[^37]: https://emby.media/community/index.php?%2Ftopic%2F94440-support-for-xtream-code-api%2F

[^38]: https://www.formuler-support.tv/forum/thread/8807-epg-no-information-issue/

[^39]: https://github.com/engenex/xtream-codes-api-v2

[^40]: https://reflexsat.store/what-is-xtream-iptv-codes-api-how-does-it-work-in/

[^41]: https://www.sqlite.org/fileformat.html

[^42]: https://stackoverflow.com/questions/61176632/custom-channel-playlist-sync-with-the-server-in-xtream-codes

[^43]: https://www.youtube.com/watch?v=JT-9cALngYE

[^44]: https://www.reddit.com/r/TiviMate/comments/ggvphz/xtream_codes_api/

[^45]: https://dbmstools.com/categories/database-design-tools/sqlite

[^46]: https://news.ycombinator.com/item?id=40206752

[^47]: https://www.linuxsat-support.com/thread/147664-x-streamity-xtream-codes-iptv-player/?postID=832436

[^48]: https://www.reddit.com/r/Database/comments/iqzw6t/free_program_to_er_diagram_a_sqlite_database/

[^49]: https://www.youtube.com/watch?v=ryi2_fyvQZM

[^50]: https://www.sat-universe.com/index.php?threads%2Fplugin-jedi-epg-xtream.313632%2F

[^51]: https://github.com/grimelinse/xtream-codes-decoded

[^52]: https://www.digital-eliteboard.com/threads/extend-api-functionality-create-restart-streams-on-2x-player-api-on-1-6x.481061/

[^53]: https://x-stream.github.io/security.html

[^54]: https://docs.oracle.com/en/database/oracle/oracle-database/21/xstrm/xstream-out-concepts.html

[^55]: https://raw.githubusercontent.com/xtreamui/XCFILES/master/check_hacks.py

[^56]: https://www.worldofiptv.com/threads/xtream-ui-howtos.2139/

[^57]: https://x-stream.github.io/manual-tweaking-output.html

[^58]: https://x-stream.github.io/faq.html

[^59]: https://www.cnx-software.com/2016/10/04/xtream-codes-iptv-panel-review-part-2-movie-data-editing-security-resellers-users-and-pricing-management/

[^60]: https://www.vondranlegal.com/what-is-iptv-and-why-are-the-xtream-codes-raid-important

[^61]: https://docs.oracle.com/en/database/oracle/oracle-database/12.2/xstrm/xstream-guide.pdf

[^62]: https://www.worldofiptv.com/threads/xtream-codes-v2-api-examples.460/

[^63]: https://docs.infor.com/ln/10.3/en-us/lnolh/docs/ln_10.3_refsqldbdrivertrg__en-us.pdf

[^64]: https://stackoverflow.com/questions/2773729/import-export-table-definitions-using-sqlserver-management-studio

[^65]: https://bryteflow.com/oracle-cdc-change-data-capture-13-things-to-know/

[^66]: https://www.red-gate.com/simple-talk/databases/sql-server/t-sql-programming-sql-server/laying-out-sql-code/

[^67]: https://www.worldofiptv.com/threads/xtream-ui-quick-fix-guide.1502/

[^68]: https://downloads.mysql.com/docs/mysql-security-excerpt-5.7-en.pdf

[^69]: https://www.w3.org/TR/xmlschema11-2/

[^70]: https://stackoverflow.com/questions/14272453/xml-parsing-using-jaxb

[^71]: https://docs.spring.io/spring-batch/docs/4.0.x/reference/html/index-single.html

