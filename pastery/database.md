# Database Management

## Database file name
- `clip.db`

## Key Formats

### Clipboard key format
- Pattern: `clipboard-YYYY-MM-DD-sequence`
- Example: `clipboard-2025-08-17-1`, `clipboard-2025-08-17-2`
- Description: Stores clipboard history with date and sequence number

### Memo key format  
- Pattern: `memo-sequence`
- Example: `memo-1`, `memo-2`, `memo-3`
- Description: Stores standalone memos with simple sequential numbering

## Database Structure

### CLIPBOARD_TABLE
- **Key**: `clipboard-YYYY-MM-DD-sequence` (String)
- **Value**: clipboard content (String)
- **Purpose**: Stores clipboard history organized by date and sequence

### MEMO_TABLE
- **Key**: `memo-sequence` (String)  
- **Value**: memo content (String)
- **Purpose**: Stores user-created memos with simple sequential IDs

## API Changes

### MemoData Methods
- `add_memo(memo: &str) -> String`: Adds a new memo and returns the generated key
- `get_memo(sequence: u64) -> Option<String>`: Retrieves memo by sequence number
- `update_memo(sequence: u64, memo: &str)`: Updates existing memo
- `delete_memo(sequence: u64)`: Deletes memo by sequence number
- `get_memo_items(count: Option<usize>) -> Vec<MemoItem>`: Gets memo items sorted by sequence (descending)

### Server API Endpoints
- `POST /memo`: Add new memo (body: `{"memo": "content"}`)
- `PUT /memo`: Update memo (body: `{"sequence": 1, "memo": "new content"}`)
- `DELETE /memo/{sequence}`: Delete memo by sequence number   
