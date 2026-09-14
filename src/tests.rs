use super::*;
use rand::rng;
use rand::RngExt;

#[test]
fn test_interpolate() {
    assert_eq!(interpolate(0.0, 100.0, 0.5), 50.0);
    assert_eq!(interpolate(100.0, 0.0, 0.5), 50.0);
    assert_eq!(interpolate(0.0, 0.0, 0.5), 0.0);
    assert_eq!(interpolate(5.0, 10.0, 0.2), 6.0);
}

#[test]
fn fuzz_inputs() {
    let mut latitude = -90.0;
    while latitude <= 90.0 {
        let mut longitude = -180.0;
        while longitude <= 180.0 {
            assert!(
                geoid_height(latitude, longitude).is_ok(),
                "geoid_height({}, {}) caused a crash",
                latitude,
                longitude
            );
            longitude += 0.09;
        }
        latitude += 0.11;
    }
}

#[test]
fn test_one_invalid_input() {
    let tests = [
        (-91.0, 0.0, Error::LatitudeOutOfRange),
        (91.0, 0.0, Error::LatitudeOutOfRange),
        (0.0, -181.0, Error::LongitudeOutOfRange),
        (0.0, 181.0, Error::LongitudeOutOfRange),
    ];
    for (lat, lon, expected) in tests {
        let result = geoid_height(lat, lon);
        assert!(
            result.is_err(),
            "Expected an error computing height for ({}, {})",
            lat,
            lon
        );
        if let Err(actual) = result {
            assert_eq!(expected, actual, "Got a different error than expected");
        }
    }
}

#[test]
fn test_two_invalid_inputs() {
    assert!(geoid_height(-95.0, -200.0).is_err());
}

#[test]
fn validate_known_points() {
    let tests = [
        (-90.0, -180.0, -30.15),
        (-90.0, 0.0, -30.15),
        (-90.0, 180.0, -30.15),
        (0.0, -180.0, 21.281),
        (0.0, 0.0, 17.225),
        (0.0, 180.0, 21.281),
        (90.0, -180.0, 14.899),
        (90.0, 0.0, 14.899),
        (90.0, 180.0, 14.899),
        (-81.0, -135.0, -45.184),
        (72.0, 141.0, -1.603),
        (-87.0, 49.5, -17.6775),
    ];
    for (lat, lon, expected) in tests {
        let result = geoid_height(lat, lon);
        assert!(
            result.is_ok(),
            "could not calculate geoid height at ({}, {})",
            lat,
            lon
        );
        if let Ok(actual) = result {
            assert!(
                (actual - expected).abs() < 1e-4,
                "got unexpected height at ({}, {}): expected {}, got {}",
                lat,
                lon,
                expected,
                actual
            );
        }
    }
}

#[test]
fn test_special_float_values() {
    assert_eq!(geoid_height(f32::NAN, 0.0), Err(Error::LatitudeOutOfRange));
    assert_eq!(
        geoid_height(0.0, f32::INFINITY),
        Err(Error::LongitudeOutOfRange)
    );
}

#[test]
fn test_micro_boundaries() {
    assert_eq!(geoid_height(90.00001, 0.0), Err(Error::LatitudeOutOfRange));
    assert_eq!(geoid_height(-90.00001, 0.0), Err(Error::LatitudeOutOfRange));
}

#[test]
fn fuzz_random_floats() {
    let mut rng = rng();

    for _ in 0..100_000 {
        let lat: f32 = rng.random_range(-90.0..=90.0);
        let lon: f32 = rng.random_range(-180.0..=180.0);

        assert!(
            geoid_height(lat, lon).is_ok(),
            "geoid_height({lat}, {lon}) caused a crash"
        );
    }
}

#[test]
fn fuzz_extreme_floats() {
    // Values that MUST produce errors
    let invalids = [
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MAX,
        -f32::MAX,
    ];

    for &lat in &invalids {
        assert!(geoid_height(lat, 0.0).is_err());
    }

    for &lon in &invalids {
        assert!(geoid_height(0.0, lon).is_err());
    }

    // Values that MUST be valid
    let valids = [f32::EPSILON, -f32::EPSILON];

    for &lat in &valids {
        assert!(geoid_height(lat, 0.0).is_ok());
    }

    for &lon in &valids {
        assert!(geoid_height(0.0, lon).is_ok());
    }
}

#[test]
fn fuzz_boundary_near_values() {
    let mut rng = rng();

    for _ in 0..50_000 {
        let lat = 90.0 - rng.random::<f32>() * 1e-6;
        let lon = 180.0 - rng.random::<f32>() * 1e-6;

        assert!(geoid_height(lat, lon).is_ok());
    }
}
