use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use crate::hashing::geo::Geohash;
use std::collections::BTreeMap;

/// Geographic Location Sharder (GeoSharding).
pub struct GeoSharding {
    precision: usize,
    geo_map: BTreeMap<String, String>, // geohash_prefix -> shard_id
    fallback_shard: Option<String>,
}

impl GeoSharding {
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

    pub fn register_region(&mut self, geohash_prefix: &str, shard_id: &str) {
        self.geo_map.insert(geohash_prefix.to_lowercase(), shard_id.to_string());
    }

    pub fn route_coords(&self, lat: f64, lon: f64) -> Option<String> {
        let full_hash = Geohash::encode(lat, lon, self.precision);
        self.route_geohash(&full_hash)
    }

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

impl AlgorithmTrait for GeoSharding {
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

impl ShardingAlgorithmTrait for GeoSharding {
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
        let mut sharder = GeoSharding::new(5);
        sharder.set_fallback("global-shard-us-east");
        sharder.register_region("ezs", "shard-eu-west");
        sharder.register_region("dnq", "shard-us-central");

        let shard_eu = sharder.route_coords(42.6, -5.6).unwrap();
        assert_eq!(shard_eu, "shard-eu-west");

        let shard_unknown = sharder.route_coords(0.0, 0.0).unwrap();
        assert_eq!(shard_unknown, "global-shard-us-east");
    }
}
