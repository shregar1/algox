use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use crate::hashing::geo::Geohash;
use std::collections::BTreeMap;

/// Geographic Location Sharder (GeoSharder).
/// Maps latitude/longitude coordinates or geohash prefixes to localized database shards/datacenters.
pub struct GeoSharder {
    precision: usize,
    geo_map: BTreeMap<String, String>, // geohash_prefix -> shard_id
    fallback_shard: Option<String>,
}

impl GeoSharder {
    /// Creates a new GeoSharder with geohash matching precision.
    pub fn new(precision: usize) -> Self {
        Self {
            precision: precision.max(1),
            geo_map: BTreeMap::new(),
            fallback_shard: None,
        }
    }

    pub fn set_fallback(&mut self, fallback_shard: &str) {
        self.fallback_shard = Some(fallback_shard.to_string());
    }

    /// Registers a geohash prefix to a specific region / datacenter shard.
    pub fn register_region(&mut self, geohash_prefix: &str, shard_id: &str) {
        self.geo_map.insert(geohash_prefix.to_lowercase(), shard_id.to_string());
    }

    /// Routes geographic coordinates `(lat, lon)` to the nearest registered region shard.
    pub fn route_coords(&self, lat: f64, lon: f64) -> Option<String> {
        let full_hash = Geohash::encode(lat, lon, self.precision);
        self.route_geohash(&full_hash)
    }

    /// Routes a geohash string to its registered region shard by prefix matching.
    pub fn route_geohash(&self, geohash: &str) -> Option<String> {
        let gh = geohash.to_lowercase();
        for len in (1..=gh.len().min(self.precision)).rev() {
            let prefix = &gh[..len];
            if let Some(shard) = self.geo_map.get(prefix) {
                return Some(shard.clone());
            }
        }
        self.fallback_shard.clone()
    }
}

impl AlgorithmTrait for GeoSharder {
    fn name(&self) -> &'static str {
        "geo_sharding"
    }

    fn len(&self) -> usize {
        self.geo_map.len()
    }

    fn clear(&mut self) {
        self.geo_map.clear();
        self.fallback_shard = None;
    }
}

impl ShardingAlgorithmTrait for GeoSharder {
    /// Accepts `"lat,lon"` (e.g. `"42.6,-5.6"`) or raw geohash string key.
    fn get_shard(&self, key: &str) -> Option<String> {
        if let Some((lat_str, lon_str)) = key.split_once(',') {
            if let (Ok(lat), Ok(lon)) = (lat_str.trim().parse::<f64>(), lon_str.trim().parse::<f64>()) {
                return self.route_coords(lat, lon);
            }
        }
        self.route_geohash(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_sharding() {
        let mut sharder = GeoSharder::new(5);
        sharder.set_fallback("global-shard-us-east");
        
        // Register European region prefix "ezs"
        sharder.register_region("ezs", "shard-eu-west");
        // Register North America region prefix "dnq"
        sharder.register_region("dnq", "shard-us-central");

        // Coordinates (42.6, -5.6) -> geohash "ezs42" -> routes to "shard-eu-west"
        let shard_eu = sharder.route_coords(42.6, -5.6).unwrap();
        assert_eq!(shard_eu, "shard-eu-west");

        // Unregistered region falls back to default fallback shard
        let shard_unknown = sharder.route_coords(0.0, 0.0).unwrap();
        assert_eq!(shard_unknown, "global-shard-us-east");
    }
}
