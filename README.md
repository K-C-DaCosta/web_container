# A format for lazily loading files
A simple container format that makes it easy to fetch files within the container, without loading the entire container into memory. For browser and the web. On desktop tokio is used, and on the browser the fetch API + Partial content is used to fetch only releveant files within container.  

## note:
**Doesn't do any compression**