
/* 
    ***************DRAFT IDEA*********************
    Multi-Relational Index Cache with a Dead-Letter TTL Buffer Mechanism

    A data will be cached via the cache_loader fn which
    applies the struct CachedData to a struct Data via transformation.

    A cache map houses the Least Recently Used cache which works
    via relational mappings to other maps.

    two types of cache maps:
    1. Lookup Cache Map
    2. Actual Cache Map

    e.g:

    lookup_map = {"alex": "1", "bob": "2", "charles": "3"}
    cache_map = {"1": alex_data, "2": bob_data, "3": charles_data}
    history_map = {"1": "2", "2": "3", "3": "4"}
    evicted_map = {jake_data: 5, tom_data: 6}

    cache_accessor(CachedData data)
    cache_accessor(alex_data)

    cache_accesor() {
    
        map the alex_data[pos] in cache_map
            if alex_data[pos] == lookup_map[alex_data.user_id or alex_data.username]
                cache_map[alex_data[pos]] = global counter tick variable for recent use since then
                lookup_map[alex_data.userid or alex_data.username] = global counter tick variable for recent use since then
                cache_loader([alex_data[pos]])
                return cache
            else:
                return cache does not exist
    }

    to determine the least recently use cache we can refer to the history map 
    and check the last key-val, remove and update accordingly
     when a cached data is evicted they go to a temporary
    evicted cache map which during softwre runtime, will only last up until 15 mins
    before they are no longer considered as a cache

*/

struct Data {
    token: u64,
    method: &str,
    endpoint: &str,
    host: &str,
}

struct CachedData {
    user_id: &str,
    data: Data,
    pos: u64,
    next: u32,
    prev: u32
}

static cache_map: [CachedData; 100] = [CachedData; 100];
static lookup_cache: [CachedData; 100] = [CachedData; 100];


fn cache_loader(UncachedData data) {

}

fn cache_accesor(CachedData data) {

}