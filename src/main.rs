use rcgen::{CertificateParams, IsCa, KeyPair, PKCS_ECDSA_P256_SHA256};
use time::{Duration, OffsetDateTime};

fn main() {
    let root_ca_key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).unwrap();
    let root_ca_cert = {
        let mut ca_params =
            CertificateParams::new(vec!["eu-west-1.aws.neon.build".into()]).unwrap();

        let nbf = OffsetDateTime::now_utc();
        let naf = nbf + Duration::weeks(1);

        ca_params.not_before = nbf;
        ca_params.not_after = naf;
        ca_params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Constrained(1));

        ca_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, "eu-west-1.aws.neon.build");

        ca_params.self_signed(&root_ca_key_pair).unwrap()
    };
    std::fs::write("certs/root.pem", root_ca_cert.pem()).unwrap();

    let proxy_ca_key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).unwrap();
    let proxy_ca_cert = {
        let mut ca_params =
            CertificateParams::new(vec!["proxy.eu-west-1.aws.neon.build".into()]).unwrap();

        let nbf = OffsetDateTime::now_utc();
        let naf = nbf + Duration::hours(24);

        ca_params.not_before = nbf;
        ca_params.not_after = naf;
        ca_params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Constrained(0));

        ca_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, "proxy.eu-west-1.aws.neon.build");

        ca_params
            .signed_by(&proxy_ca_key_pair, &root_ca_cert, &root_ca_key_pair)
            .unwrap()
    };
    std::fs::write("certs/proxy.pem", proxy_ca_cert.pem()).unwrap();

    let user1_key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).unwrap();
    let user1_cert = {
        let mut ca_params =
            CertificateParams::new(vec!["ep-foo-bar-1234.proxy.eu-west-1.aws.neon.build".into()])
                .unwrap();

        let nbf = OffsetDateTime::now_utc();
        let naf = nbf + Duration::minutes(5);

        ca_params.not_before = nbf;
        ca_params.not_after = naf;
        ca_params.is_ca = IsCa::ExplicitNoCa;

        ca_params.distinguished_name.push(
            rcgen::DnType::CommonName,
            "user1@ep-foo-bar-1234.proxy.eu-west-1.aws.neon.build",
        );

        ca_params
            .signed_by(&user1_key_pair, &proxy_ca_cert, &proxy_ca_key_pair)
            .unwrap()
    };
    std::fs::write("certs/user1.pem", user1_cert.pem()).unwrap();
    std::fs::write("certs/user1.key", user1_key_pair.serialize_pem()).unwrap();

    std::fs::write(
        "certs/user1_chain.pem",
        user1_cert.pem() + &proxy_ca_cert.pem(),
    )
    .unwrap();

    let user2_key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).unwrap();
    let user2_cert = {
        let mut ca_params =
            CertificateParams::new(vec!["ep-foo-baz-9876.proxy.eu-west-1.aws.neon.build".into()])
                .unwrap();

        let nbf = OffsetDateTime::now_utc();
        let naf = nbf + Duration::minutes(5);

        ca_params.not_before = nbf;
        ca_params.not_after = naf;
        ca_params.is_ca = IsCa::ExplicitNoCa;

        ca_params.distinguished_name.push(
            rcgen::DnType::CommonName,
            "user2@ep-foo-baz-9875.proxy.eu-west-1.aws.neon.build",
        );

        ca_params
            .signed_by(&user2_key_pair, &proxy_ca_cert, &proxy_ca_key_pair)
            .unwrap()
    };
    std::fs::write("certs/user2.pem", user2_cert.pem()).unwrap();
    std::fs::write("certs/user2.key", user2_key_pair.serialize_pem()).unwrap();

    std::fs::write(
        "certs/user2_chain.pem",
        user2_cert.pem() + &proxy_ca_cert.pem(),
    )
    .unwrap();

    let server_key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).unwrap();
    let server_cert = {
        let mut ca_params =
            CertificateParams::new(vec!["ep-foo-bar-1234.eu-west-1.aws.neon.build".into()])
                .unwrap();

        let nbf = OffsetDateTime::now_utc();
        let naf = nbf + Duration::hours(1);

        ca_params.not_before = nbf;
        ca_params.not_after = naf;
        ca_params.is_ca = IsCa::ExplicitNoCa;

        ca_params
            .signed_by(&server_key_pair, &root_ca_cert, &root_ca_key_pair)
            .unwrap()
    };
    std::fs::write("certs/server.pem", server_cert.pem()).unwrap();
    std::fs::write("certs/server.key", server_key_pair.serialize_pem()).unwrap();
}
