A record abstraction with a derive on top of sled.

Rough sketch to start a discussion on implementing https://github.com/spacejam/sled/issues/562

### Random notes

- Additional API: MyType::find_by_id(&db, MyTypeKey { id: 33 });
- Can add indexes, with methods for querying them!
    - Can implement unique indexes on top of that
    - MyType::find_by_email(&db);

Challenge: zero-copy parsing of records from the database will probably require parsing the types and rewriting the lifetimes in there, but it's possible.
