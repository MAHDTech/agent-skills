use skills_core::installer::{compute_dir_checksum, compute_file_sha256, compute_sha256};
use skills_core::{SkillDownloader, SkillFileEntry};
use tempfile::TempDir;

#[test]
fn sha256_preserves_lowercase_hex_and_leading_zeros() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("input");
    for (input, expected) in [
        (
            b"".as_slice(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"s".as_slice(),
            "043a718774c572bd8a25adbeb1bfcd5c0256ae11cecf9f9c3f925d0e52beaf89",
        ),
    ] {
        assert_eq!(compute_sha256(input), expected);
        assert_eq!(SkillDownloader::compute_sha256(input), expected);
        assert!(SkillDownloader::verify_sha256(input, expected));
        std::fs::write(&path, input).unwrap();
        assert_eq!(compute_file_sha256(&path).unwrap(), expected);
    }
}

#[test]
fn file_sha256_preserves_digest_across_read_buffers() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("input");
    std::fs::write(&path, vec![b'a'; 8193]).unwrap();
    assert_eq!(
        compute_file_sha256(&path).unwrap(),
        "9c10c48d1f1d6618db88fde2c25409181c9201ed34ec6815d62bcf57c10d177b"
    );
}

#[test]
fn directory_checksum_preserves_sorted_entry_digest() {
    let mut entries = vec![
        SkillFileEntry::builder()
            .path("z.txt")
            .sha256("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            .size_bytes(0)
            .build(),
        SkillFileEntry::builder()
            .path("a.txt")
            .sha256("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
            .size_bytes(3)
            .build(),
    ];
    let expected = "a923b9536ca6bcc28baad53954454856bdae4559a3891ecb2a7f520918b03145";
    assert_eq!(compute_dir_checksum(&entries), expected);
    entries.reverse();
    assert_eq!(compute_dir_checksum(&entries), expected);
    assert_eq!(
        compute_dir_checksum(&[]),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}
