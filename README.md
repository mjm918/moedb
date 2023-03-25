# ‼️ Work in Progress ‼️

# MoeDb - A Realtime Document Store Database

MoeDb is a fast and reliable document store and object storage database that supports custom JSON query language (JQL) for easy data retrieval and manipulation. It has a real-time capability that can store documents in both persistent storage and memory, making it ideal for building high-performance applications that require real-time updates.

**Features**
- Document store and object storage
- Custom JSON Query Language (JQL)
- Real-time database
- Support for persistent storage using RocksDB
- Mobile and IoT-friendly Lite version with auto-sync functionality
- High-performance database

**JQL Examples**

Creating a database:
```
{
    "_action": "create-db",
    "_database": "my_database",
    "_body": {}
}
```
Creating a collection:
```
{
    "_action": "create-collection",
    "_database": "my_database",
    "_collection": "my_collection",
    "_body": {}
}
```
Retrieving data:
```
{
    "_action": "get",
    "_database": "my_database",
    "_collection": "my_collection",
    "_body": {"_id": "document_id"}
}
```
Updating data:
```
{
    "_action": "upsert",
    "_database": "my_database",
    "_collection": "my_collection",
    "_body": {"_id": "document_id", "field": "new_value"}
}
```
Deleting data:
```
{
    "_action": "delete",
    "_database": "my_database",
    "_collection": "my_collection",
    "_body": {"_id": "document_id"}
}
```

**License**

MoeDb is open-source software licensed under the MIT License.

**Contribution**

If you'd like to contribute to MoeDb, please fork the repository and make changes as you'd like. Pull requests are warmly welcome. For major changes, please open an issue first to discuss what you would like to change.

**Acknowledgements**

MoeDb is built using various open-source software, including RocksDB, and the contributions of the open-source community are greatly appreciated.