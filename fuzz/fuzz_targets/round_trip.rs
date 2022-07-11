#![no_main]

use libfuzzer_sys::fuzz_target;
use yaml_rust::{YamlEmitter, YamlLoader};

fuzz_target!(|data: &str| {
    if let Ok(objs) = YamlLoader::load_from_str(data) {
        let mut compact = String::new();
        let mut normal = String::new();

        for obj in objs {
            let mut emitter = YamlEmitter::new(&mut compact);
            emitter.compact(true);
            emitter.dump(&obj).expect("deserialized obj to serialize");
            let objs2 =
                YamlLoader::load_from_str(&compact).expect("serialized object to deserialize");
            assert_eq!(1, objs2.len(), "only one obj should be consumed");
            assert_eq!(obj, objs2[0], "roundtrip should equal");
            compact.clear();

            let mut emitter = YamlEmitter::new(&mut normal);
            emitter.compact(false);
            emitter.dump(&obj).expect("deserialized obj to serialize");
            let objs2 =
                YamlLoader::load_from_str(&normal).expect("serialized object to deserialize");
            assert_eq!(1, objs2.len(), "only one obj should be consumed");
            assert_eq!(obj, objs2[0], "roundtrip should equal");
            normal.clear();
        }
    }
});
